mod svd2temp;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::ir::*;
use super::util::*;
use crate::svd_util::*;
use crate::SvdValidationLevel;
use anyhow::Result;
use linked_hash_map::LinkedHashMap;
use log::{debug, error, warn};
use svd2temp::*;
use svd_parser::svd;
use svd_parser::svd::Name;

trait RegisterHelper {
    /// Get name of register considering the presence of alternate group
    /// Alternate group is appended to the id of register
    fn get_name_id_internal(&self) -> String;
}

impl RegisterHelper for svd::RegisterInfo {
    fn get_name_id_internal(&self) -> String {
        match self.alternate_group {
            None => self.name.to_internal_ident(),
            Some(ref alt_group_name) => {
                (self.name.to_owned() + "_" + alt_group_name).to_internal_ident()
            }
        }
    }
}

enum PeripheralClusterE<'a> {
    Peripheral(&'a mut PeripheralMod),
    Cluster(&'a mut Cluster),
}

impl<'a> PeripheralClusterE<'a> {
    pub fn get_mut_registers(&mut self) -> &mut LinkedHashMap<String, Rc<RefCell<Register>>> {
        match self {
            PeripheralClusterE::Peripheral(p) => &mut p.registers,
            PeripheralClusterE::Cluster(c) => &mut c.registers,
        }
    }
    pub fn get_mut_clusters(&mut self) -> &mut LinkedHashMap<String, Rc<RefCell<Cluster>>> {
        match self {
            PeripheralClusterE::Peripheral(p) => &mut p.clusters,
            PeripheralClusterE::Cluster(c) => &mut c.clusters,
        }
    }
}

/// Utility function to get number of instances and increment between the distances
/// This function can be used anytime in svd::array::MaybeArray is used
///
///
/// # Arguments
///
/// * `array` any MaybeArray type
///
/// # Result
///
/// (`dimension of array`,`increment between two element of array`) If `array`=Single() the result default to (1,0)
fn get_dim_dim_increment<T>(array: &svd::array::MaybeArray<T>) -> (u32, u32) {
    match array {
        svd::array::MaybeArray::Single(_) => (1, 0),
        svd::array::MaybeArray::Array(_, dim_element) => {
            (dim_element.dim, dim_element.dim_increment)
        }
    }
}

#[derive(Debug)]
enum DeviceItem {
    Register(Rc<RefCell<Register>>),
    Cluster(Rc<RefCell<Cluster>>),
    //TODO Remove the allow
    #[allow(dead_code)]
    Peripheral(Rc<RefCell<PeripheralMod>>),
    //Field(Rc<RefCell<FieldGetterSetter>>),
}

#[derive(Default)]
struct Visitor {
    device: Device,
    svd_ref_to_ir_item: HashMap<String, DeviceItem>,
    // Current item svd path that is used to build
    // the key of svd_ref_to_ir_item. In case of array only the first item will be considered
    current_item_svd_path: Vec<String>,
    // Path to the module of item in Rust code
    current_mod_ir_path: Vec<String>,
}
impl Visitor {
    /// Create the intermediate representation of device used by template engine
    fn visit_device(&mut self, device: &svd::Device) {
        self.device.name.clone_from(&device.name);
        self.device.description.clone_from(&device.description);

        for peripheral in device.peripherals.iter() {
            let name = peripheral.name().to_internal_ident();
            let mut peripheral_mod = PeripheralMod {
                name: name.clone(),
                ..Default::default()
            };
            self.push_current_item_svd_path(peripheral);
            self.visit_peripheral(peripheral, &mut peripheral_mod);
            let peripheral_mod = Rc::new(RefCell::new(peripheral_mod));
            self.device
                .peripheral_mod
                .insert(name, peripheral_mod.clone());

            self.svd_ref_to_ir_item.insert(
                peripheral.name().to_string(),
                DeviceItem::Peripheral(peripheral_mod.clone()),
            );
            self.pop_current_item_svd_path(DeviceItem::Peripheral(peripheral_mod));
        }
    }
    fn visit_peripheral(
        &mut self,
        peripheral: &svd::Peripheral,
        target_peripheral: &mut PeripheralMod,
    ) {
        /*I don't generate any type if the peripheral has derivedFrom attribute.
        Overriding of peripheral content is not supported*/
        assert!(
            peripheral.derived_from.is_none(),
            "derived_from is not supported in peripherals"
        );
        debug!("Parsing peripheral: {}", &target_peripheral.name);

        let (dim, dim_increment) = get_dim_dim_increment(peripheral);
        target_peripheral.base_addr = (0..dim)
            .map(|index| peripheral.base_address + (index * dim_increment) as u64)
            .collect();
        target_peripheral.interrupts = peripheral
            .interrupt
            .iter()
            .map(|x| Interrupt {
                name: x.name.clone(),
                value: x.value,
                description: x
                    .description
                    .as_ref()
                    .map_or_else(String::new, |x| x.clone()),
            })
            .collect();
        target_peripheral.description = peripheral.description.clone().unwrap_or_default();

        for cluster_register in peripheral.registers.as_ref().unwrap_or(&Vec::new()) {
            self.visit_cluster_register(
                cluster_register,
                PeripheralClusterE::Peripheral(target_peripheral),
            );
        }
    }

    fn visit_register(&mut self, reg: &svd::Register, register: &mut Register) {
        register.name = reg.get_name_id_internal();
        register.description = reg.description.clone().unwrap_or_default();
        register.offset = reg.address_offset;
        (register.dim, register.dim_increment) = get_dim_dim_increment(reg);

        if register.struct_id.is_empty() {
            register.struct_module_path = Vec::with_capacity(10);
            register.struct_module_path.extend_from_slice(
                &self.current_mod_ir_path[0..self.current_mod_ir_path.len() - 1],
            );
            register.struct_id = register.name.to_sanitized_struct_ident();
        }
        // Get fields
        let mut fields = Vec::new();
        for field in reg.fields() {
            assert!(
                field.derived_from.is_none(),
                "derived_from is not supported in field"
            );
            let description = field.description.clone().unwrap_or_default();
            let offset = field.bit_range.offset;
            let mask = (0..field.bit_range.width - 1).fold(0x1u32, |acc, _| (acc << 1) | 0x1);
            let name = field.name.to_internal_ident();
            let svd_field_access = match field.access {
                None => {
                    error!("Inheritance of access is not supported. Bitfield: {} access shall be specified. Bitfield skipped",name);
                    continue;
                }
                Some(acc) => acc,
            };
            let access = match svd_field_access {
                svd::Access::ReadOnly => RegisterBitfieldAccess::R,
                svd::Access::WriteOnly => RegisterBitfieldAccess::W,
                svd::Access::ReadWrite => RegisterBitfieldAccess::RW,
                svd::Access::WriteOnce => RegisterBitfieldAccess::W,
                svd::Access::ReadWriteOnce => RegisterBitfieldAccess::RW,
            };
            let enum_type = get_values_types(field);
            let (dim, dim_increment) = get_dim_dim_increment(field);
            fields.push(FieldGetterSetter {
                name,
                description,
                offset,
                mask,
                enum_type,
                access,
                size: BitSize::val_2_bit_size(mask.into()),
                dim,
                dim_increment,
            });
        }
        match reg.properties.size {
            Some(value) => {
                register.size = match value {
                    64 => BitSize::BIT64,
                    32 => BitSize::BIT32,
                    16 => BitSize::BIT16,
                    8 => BitSize::BIT8,
                    register_size => {
                        panic!("Unsupported register size {register_size}")
                    }
                }
            }
            None => assert!(
                reg.derived_from.is_some(),
                "register {} is not derived and it has no specified size",
                &register.name
            ),
        }
        match reg.properties.reset_value {
            Some(value) => register.reset_value = value,
            None => assert!(
                reg.derived_from.is_some(),
                "register {} is not derived and it has no specified reset value",
                &register.name
            ),
        }

        register.has_enumerated_fields = fields.iter().any(|f| f.enum_type.is_some());

        match reg.properties.access {
            Some(reg_access) => {
                register.access = match reg_access {
                    svd::Access::ReadOnly => RegisterAccess::R,
                    svd::Access::WriteOnly => RegisterAccess::W,
                    svd::Access::ReadWrite => RegisterAccess::RW,
                    svd::Access::WriteOnce => RegisterAccess::W,
                    svd::Access::ReadWriteOnce => RegisterAccess::RW,
                }
            }
            // If register access mode is not defined. The value is inferred from access mode of bitfields
            None => {
                // If the register is derived from we relay on parent value
                if reg.derived_from.is_none() {
                    warn!(
                        "Access mode is not defined for register ({}) inferring from bitfield",
                        &register.name
                    );
                    let is_register_writable = fields.iter().any(|f| {
                        f.access == RegisterBitfieldAccess::W
                            || f.access == RegisterBitfieldAccess::RW
                    });
                    let is_register_readable = fields.iter().any(|f| {
                        f.access == RegisterBitfieldAccess::R
                            || f.access == RegisterBitfieldAccess::RW
                    });
                    register.access = match (is_register_readable, is_register_writable) {
                        (true, true) => RegisterAccess::RW,
                        (true, false) => RegisterAccess::R,
                        (false, true) => RegisterAccess::W,
                        (false, false) => {
                            error!("No bitfield in register '{}' specifies an access mode. Not able to infer register access mode", &register.name);
                            RegisterAccess::R
                        }
                    }
                }
            }
        };
        register.fields = fields
            .into_iter()
            .map(|f| (f.name.clone(), Rc::new(RefCell::new(f))))
            .collect();
    }

    fn visit_cluster(&mut self, cluster_svd: &svd::Cluster, cluster: &mut Cluster) {
        cluster.name = cluster_svd.name.to_internal_ident();
        cluster.description = cluster_svd.description.clone().unwrap_or_default();
        cluster.offset = cluster_svd.address_offset;
        (cluster.dim, cluster.dim_increment) = get_dim_dim_increment(cluster_svd);

        if let Some(header_struct_name) = &cluster_svd.header_struct_name {
            cluster.struct_module_path = Vec::with_capacity(10);
            cluster.struct_module_path.extend_from_slice(
                &self.current_mod_ir_path[0..self.current_mod_ir_path.len() - 1],
            );
            cluster.struct_id = header_struct_name.to_sanitized_struct_ident();
        } else if cluster.struct_id.is_empty() {
            cluster.struct_module_path = Vec::with_capacity(10);
            cluster.struct_module_path.extend_from_slice(
                &self.current_mod_ir_path[0..self.current_mod_ir_path.len() - 1],
            );
            cluster.struct_id = cluster.name.to_sanitized_struct_ident();
        }
        for cluster_register in &cluster_svd.children {
            self.visit_cluster_register(cluster_register, PeripheralClusterE::Cluster(cluster));
        }
    }
    fn visit_cluster_register(
        &mut self,
        register_cluster: &svd::RegisterCluster,
        mut parent_peripheral_cluster: PeripheralClusterE,
    ) {
        match register_cluster {
            svd::RegisterCluster::Register(ref reg_svd) => {
                let derived_register: Option<Register> = if let Some(derived_ref) =
                    register_cluster.derived_from()
                {
                    let absolute_reference_path = self.get_absolute_svd_path(derived_ref);
                    if let Some(ref_item) = self.svd_ref_to_ir_item.get(&absolute_reference_path) {
                        if let DeviceItem::Register(ref_register) = ref_item {
                            Some(ref_register.borrow().clone())
                        } else {
                            panic!(
                                "reference {:} in {:} point to not register svd item",
                                derived_ref, reg_svd.name
                            );
                        }
                    } else {
                        panic!("Missing reference {:} in {:}", derived_ref, reg_svd.name);
                    }
                } else {
                    None
                };
                // Push the target register svd and ir path in corresponding FIFO stack
                self.push_current_item_svd_path(reg_svd);
                let mut register = derived_register
                    .as_ref()
                    .map_or_else(Register::default, |x| x.clone());

                self.visit_register(reg_svd, &mut register);

                let name = register.name.clone();
                // If after visiting the svd node and updating the cluster_svd we get cluster that has the same type
                // set derived_register and replace the struct id
                register.is_derived_from = derived_register
                    .is_some_and(|derived_register| register.has_same_type(&derived_register));

                let register = Rc::new(RefCell::new(register));

                parent_peripheral_cluster
                    .get_mut_registers()
                    .insert(name, register.clone());
                // Pop out the paths and the just updated cluster in svd to it index
                self.pop_current_item_svd_path(DeviceItem::Register(register));
            }
            svd::RegisterCluster::Cluster(ref cluster_svd) => {
                let derived_cluster: Option<Cluster> = if let Some(derived_ref) =
                    register_cluster.derived_from()
                {
                    let absolute_reference_path = self.get_absolute_svd_path(derived_ref);
                    if let Some(ref_item) = self.svd_ref_to_ir_item.get(&absolute_reference_path) {
                        if let DeviceItem::Cluster(ref_cluster) = ref_item {
                            Some(ref_cluster.borrow().clone())
                        } else {
                            panic!(
                                "reference {:} in {:} point to not cluster svd item",
                                derived_ref, cluster_svd.name
                            );
                        }
                    } else {
                        panic!(
                            "Missing reference {:} in {:}",
                            derived_ref, cluster_svd.name
                        );
                    }
                } else {
                    None
                };

                // Push the target cluster svd and ir path in corresponding FIFO stack
                self.push_current_item_svd_path(cluster_svd);
                let mut cluster = derived_cluster
                    .as_ref()
                    .map_or_else(Cluster::default, |x| x.clone());
                self.visit_cluster(cluster_svd, &mut cluster);
                // If after visiting the svd node and updating the cluster_svd we get cluster that has the same type
                // set derived_cluster and replace the struct id
                cluster.is_derived_from = derived_cluster
                    .is_some_and(|derived_cluster| cluster.has_same_type(&derived_cluster));

                let name = cluster.name.clone();
                let cluster = Rc::new(RefCell::new(cluster));
                parent_peripheral_cluster
                    .get_mut_clusters()
                    .insert(name, cluster.clone());
                // Pop out the paths and the just updated cluster in svd to it index
                self.pop_current_item_svd_path(DeviceItem::Cluster(cluster));
            }
        }
    }

    fn pop_current_item_svd_path(&mut self, ir_item: DeviceItem) {
        self.svd_ref_to_ir_item
            .insert(self.current_item_svd_path.join("."), ir_item);
        assert!(self.current_item_svd_path.pop().is_some());
        assert!(self.current_mod_ir_path.pop().is_some());
    }
    fn push_current_item_svd_path(&mut self, svd_item: &impl ExpandedName) {
        self.current_item_svd_path
            .push(svd_item.get_expanded_name());
        self.current_mod_ir_path
            .push(svd_item.name().to_sanitized_mod_ident());
    }
    fn get_absolute_svd_path(&self, local_svd_name: &str) -> String {
        if local_svd_name.contains('.') {
            local_svd_name.to_string()
        } else {
            let mut result: Vec<&str> = Vec::with_capacity(self.current_item_svd_path.len() + 1);
            self.current_item_svd_path
                .iter()
                .for_each(|s| result.push(s));
            result.push(local_svd_name);
            result.join(".")
        }
    }
}

fn get_values_types(field: &svd::Field) -> Option<EnumeratedValueType> {
    if field.enumerated_values.is_empty() {
        return None;
    };

    let name = field.name.to_internal_ident();
    let mut values = Vec::new();
    let mut max_value = 0u64; // Compute max value of bitfiled to define the size of bitfiled in bits.
    for enum_values in &field.enumerated_values {
        for val_entry in &enum_values.values {
            assert!(
                !val_entry.name.is_empty(),
                "Value of enumeration shall have a name"
            );
            let description = val_entry.description.clone().unwrap_or_default();
            let val_name: String = if let Some(ref enumerated_values_name) = enum_values.name {
                format!("{}_{}", enumerated_values_name, val_entry.name)
            } else {
                val_entry.name.clone()
            };

            let value = if let Some(value) = val_entry.value {
                value
            } else {
                panic!("Default value is unsupported, all value in enumeration shall have a value defined")
            };

            values.push(EnumeratedSingleValue {
                name: val_name,
                value,
                description,
            });
            max_value = max_value.max(value);
        }
    }
    Some(EnumeratedValueType {
        name,
        size: BitSize::val_2_bit_size(max_value),
        values,
    })
}

/// Parse xml and trasform to device description of svd_rs module
///
/// # Arguments
///
/// * `path` path to svd file
///
/// # Result
///
/// Device containing all information of svd file
pub(super) fn parse_xml(
    xml: &mut str,
    svd_validation_level: SvdValidationLevel,
) -> Result<svd::Device> {
    let mut parser_config = svd_parser::Config::default();
    parser_config.expand_properties = true;
    parser_config.ignore_enums = false;
    parser_config.validate_level = match svd_validation_level {
        SvdValidationLevel::Disabled => svd::ValidateLevel::Disabled,
        SvdValidationLevel::Weak => svd::ValidateLevel::Weak,
        SvdValidationLevel::Strict => svd::ValidateLevel::Strict,
    };
    let result = svd_parser::parse_with_config(xml, &parser_config);
    if let Err(err) = &result {
        if let Some(error_at) = err.downcast_ref::<svd_parser::SVDErrorAt>() {
            error!("Error while parsing {}", error_at);
        }
    }
    result
}

/// Generate interrupt table including holes that will be used to create required function for cortex-m-rt
fn get_interrupt_table(
    peripheral_types: &LinkedHashMap<String, Rc<RefCell<PeripheralMod>>>,
) -> Vec<Option<Interrupt>> {
    match peripheral_types
        .values()
        .flat_map(|x| {
            x.borrow()
                .interrupts
                .iter()
                .map(|x| x.value)
                .collect::<Vec<_>>()
        })
        .max()
    {
        None => Vec::new(),
        Some(max_int_index) => {
            let mut result = vec![None; max_int_index as usize + 1];
            for interrupt in peripheral_types
                .values()
                .flat_map(|x| x.borrow().interrupts.clone())
            {
                let interrupt_id = interrupt.value as usize;
                if result[interrupt_id].is_some() {
                    error!(
                        "Duplicated interrupt definition at index {}",
                        interrupt.value
                    );
                }
                result[interrupt_id] = Some(interrupt);
            }
            result
        }
    }
}

pub(super) fn svd_device2ir(
    svd_device: &svd::Device,
    custom_license_text: &Option<String>,
) -> Result<IR> {
    let entity_db = get_entity_db(svd_device);
    // Use custom license if available otherwise use license in svd and if it not present use empty string.
    let license_text = custom_license_text.as_ref().map_or_else(
        || {
            svd_device.license_text.as_ref().map_or_else(
                || {
                    error!("No license defined in SVD. Use --license-file option in command line");
                    String::new()
                },
                |license_txt| license_txt.replace("\\n", "\n"),
            )
        },
        |file_license| file_license.clone(),
    );
    let mut visitor = Visitor::default();
    visitor.visit_device(svd_device);
    let device = visitor.device;
    let interrupt_table = get_interrupt_table(&device.peripheral_mod);
    Ok(IR {
        device,
        register_addresses: entity_db.register_addresses,
        license_text,
        interrupt_table,
        nvic_prio_bits: svd_device.cpu.as_ref().map(|x| x.nvic_priority_bits),
        vendor_systick_config: svd_device.cpu.as_ref().map(|x| x.has_vendor_systick),
        fpu_present: svd_device.cpu.as_ref().map(|x| x.fpu_present),
        mpu_present: svd_device.cpu.as_ref().map(|x| x.mpu_present),
    })
}
