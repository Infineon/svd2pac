mod svd2temp;
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

/// Create the intermediate representation of device used by template engine
fn get_device(device: &svd::Device) -> Device {
    Device {
        name: device.name.clone(),
        description: device.description.clone(),
        peripheral_mod: get_peripherals_types(device)
    }
}

fn get_register(reg: &svd::Register) -> Register {
    let register_name = reg.get_name_id_internal();
    let (dim, dim_increment) = get_dim_dim_increment(reg);
    // Get fields
    let mut fields = vec![];
    for field in reg.fields() {
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
        })
    }
    let size = match reg
        .properties
        .size
        .expect("All registers shall have a defined size")
    {
        64 => BitSize::BIT64,
        32 => BitSize::BIT32,
        16 => BitSize::BIT16,
        8 => BitSize::BIT8,
        register_size => {
            panic!("Unsupported register size {register_size}")
        }
    };
    let reset_value = reg
        .properties
        .reset_value
        .expect("All registers shall have a reset value defined");
    let has_enumerated_fields = fields.iter().any(|f| f.enum_type.is_some());

    let access = match reg.properties.access {
        Some(reg_access) => match reg_access {
            svd::Access::ReadOnly => RegisterAccess::R,
            svd::Access::WriteOnly => RegisterAccess::W,
            svd::Access::ReadWrite => RegisterAccess::RW,
            svd::Access::WriteOnce => RegisterAccess::W,
            svd::Access::ReadWriteOnce => RegisterAccess::RW,
        },
        // If register access mode is not defined. The value is inferred from access mode of bitfields
        None => {
            warn!(
                "Access mode is not defined for register ({}) inferring from bitfield",
                register_name
            );
            let is_register_writable = fields.iter().any(|f| {
                f.access == RegisterBitfieldAccess::W || f.access == RegisterBitfieldAccess::RW
            });
            let is_register_readable = fields.iter().any(|f| {
                f.access == RegisterBitfieldAccess::R || f.access == RegisterBitfieldAccess::RW
            });
            match (is_register_readable, is_register_writable) {
                (true, true) => RegisterAccess::RW,
                (true, false) => RegisterAccess::R,
                (false, true) => RegisterAccess::W,
                (false, false) => {
                    error!("No bitfield in register '{}' specifies an access mode. Not able to infer register access mode", &register_name);
                    RegisterAccess::R
                }
            }
        }
    };
    Register {
        name: register_name,
        offset: reg.address_offset,
        dim,
        dim_increment,
        access,
        description: reg.description.clone().unwrap_or_default(),
        fields,
        size,
        reset_value,
        has_enumerated_fields,
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
                panic!("Unsupport is default, all value in enumeration shall have a value defined")
            };

            values.push(EnumeratedSigleValue {
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

fn get_parent_struct_name_cluster<T: PeripheralClusterT>(
    device: &svd::Device,
    container: &T,
    derived_from: &str,
) -> String {
    let splitted: Vec<&str> = derived_from.split('.').collect();
    if splitted.len() == 1 {
        /* Accordingly with cmsis svd spec when there is a single string. The name shall be found in
           the container scope
        */
        if let Some(parent) = container.get_clusters().find(|&c| c.name == splitted[0]) {
            let mut struct_name = parent.struct_name().to_sanitized_struct_ident();
            struct_name.insert_str(0, "self::");
            struct_name
        } else {
            panic!("{} is referenced in a derivedFrom attribute but it doesn't exist a cluster with this name",derived_from);
        }
    } else {
        let mut result: Vec<String> = Vec::new();
        // In this case we expect an absolute path starting from device level
        let mut present_item: &dyn PeripheralClusterT =
            match device.peripherals.iter().find(|p| p.name == splitted[0]) {
                Some(item) => item,
                None => panic!(
                    "in a derivedFrom={} attribute is referenced an item {} that cannot be found",
                    derived_from, splitted[0]
                ),
            };

        result.push(present_item.struct_name());

        splitted.iter().skip(1).for_each(|&item| {
            let item = match present_item.get_clusters().find(|c| c.name == item) {
                Some(item) => item,
                None => panic!(
                    "in a derivedFrom={} attribute is referenced an item {} that cannot be found",
                    derived_from, item
                ),
            };
            result.push(item.struct_name());
            present_item = item;
        });
        /*
        Modules and struct have different coding guidelines
         */
        for x in 0..result.len() {
            if x == result.len() - 1 {
                result[x] = result[x].to_sanitized_struct_ident()
            } else {
                result[x] = result[x].to_sanitized_mod_ident()
            }
        }
        result.insert(0, "crate".to_string());
        result.join("::")
    }
}

fn get_cluster<T: PeripheralClusterT>(
    device: &svd::Device,
    container: &T,
    cluster: &svd::Cluster,
) -> Cluster {
    let name = cluster.name.to_internal_ident();
    let (dim, dim_increment) = get_dim_dim_increment(cluster);
    let (registers, clusters) = get_register_clusters(device, container, &cluster.children);
    let struct_id = cluster.struct_name();
    let struct_path: String = if let Some(ref derived_path) = cluster.derived_from {
        if !registers.is_empty() || !clusters.is_empty() {
            error!("Cluster with derivedFrom attributes are supported only if they doesn't contains registers and/or clusters. Included registers and clusters are ignored")
        }
        get_parent_struct_name_cluster(device, container, derived_path)
    } else {
        let mut struct_name = cluster.struct_name().to_sanitized_struct_ident();
        struct_name.insert_str(0, "self::");
        struct_name
    };
    Cluster {
        name,
        description: cluster.description.clone().unwrap_or_default(),
        offset: cluster.address_offset,
        dim,
        dim_increment,
        registers,
        clusters,
        is_derived_from: cluster.derived_from.as_ref().cloned(),
        struct_id,
        struct_path,
    }
}

fn get_register_clusters<T: PeripheralClusterT>(
    device: &svd::Device,
    container: &T,
    registers_clusters: &Vec<svd::RegisterCluster>,
) -> (
    LinkedHashMap<String, Register>,
    LinkedHashMap<String, Cluster>,
) {
    let mut registers = LinkedHashMap::new();
    let mut clusters = LinkedHashMap::new();
    for reg_clu in registers_clusters {
        match reg_clu {
            svd::RegisterCluster::Register(reg_svd) => {
                let register = get_register(reg_svd);
                registers.insert(register.name.clone(), register);
            }
            svd::RegisterCluster::Cluster(cluster_svd) => {
                let cluster = get_cluster(device, container, cluster_svd);
                clusters.insert(cluster_svd.name.clone(), cluster);
            }
        }
    }
    (registers, clusters)
}

fn get_peripherals_types(svd_device: &svd::Device) -> LinkedHashMap<String, PeripheralMod> {
    let mut result = LinkedHashMap::new();

    for p in svd_device.peripherals.iter() {
        let svd_name = p.name();
        /*I don't generate any type if the peripheral has derivedFrom attribute.
        Overriding of peripheral content is not supported*/
        assert!(
            p.derived_from.is_none(),
            "derived_from is not supported in peripherals"
        );
        debug!("Parsing peripheral: {}", svd_name);
        let (registers, clusters) =
            get_register_clusters(svd_device, p, p.registers.as_ref().unwrap_or(&Vec::new()));
        let peripheral_struct_name = svd_name.to_internal_ident();
        let (dim, dim_increment) = get_dim_dim_increment(p);
        let base_addr = (0..dim)
            .map(|index| p.base_address + (index * dim_increment) as u64)
            .collect();
        let interrupts = p
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
        let peripheral_mod = PeripheralMod {
            name: peripheral_struct_name,
            description: p.description.clone().unwrap_or_default(),
            clusters,
            registers,
            base_addr,
            interrupts,
        };
        result.insert(peripheral_mod.name.clone(), peripheral_mod);
    }
    result
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
fn parse_xml(xml: &mut str, svd_validation_level: SvdValidationLevel) -> Result<svd::Device> {
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
    peripheral_types: &LinkedHashMap<String, PeripheralMod>,
) -> Vec<Option<Interrupt>> {
    match peripheral_types
        .values()
        .flat_map(|x| x.interrupts.iter().map(|x| x.value))
        .max()
    {
        None => Vec::new(),
        Some(max_int_index) => {
            let mut result = vec![None; max_int_index as usize + 1];
            for interrupt in peripheral_types.values().flat_map(|x| x.interrupts.iter()) {
                if result[interrupt.value as usize].is_some() {
                    error!(
                        "Duplicated interrupt definition at index {}",
                        interrupt.value
                    );
                }
                result[interrupt.value as usize] = Some(interrupt.clone());
            }
            result
        }
    }
}

pub(super) fn parse_xml2ir(
    xml: &mut str,
    svd_validation_level: SvdValidationLevel,
    custom_license_text: &Option<String>,
) -> Result<IR> {
    let svd_device = parse_xml(xml, svd_validation_level)?;
    let entity_db = get_entity_db(&svd_device);
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
    let device = get_device(&svd_device);
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
