use linked_hash_map::LinkedHashMap;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;

pub trait HasSameType {
    fn has_same_type(&self, other: &Self) -> bool;
}

impl HasSameType for PeripheralMod {
    fn has_same_type(&self, other: &Self) -> bool {
        self.clusters == other.clusters && self.registers == other.registers
    }
}

impl HasSameType for Cluster {
    fn has_same_type(&self, other: &Self) -> bool {
        self.clusters == other.clusters
            && self.registers == other.registers
            && self.struct_id == other.struct_id
            && self.struct_module_path == other.struct_module_path
    }
}

impl HasSameType for Register {
    fn has_same_type(&self, other: &Self) -> bool {
        self.fields == other.fields
            && self.struct_id == other.struct_id
            && self.struct_module_path == other.struct_module_path
            && self.access == other.access
            && self.size == other.size
            && self.reset_value == other.reset_value
    }
}

#[derive(Default, Clone, Debug, PartialEq, Serialize)]
pub struct Device {
    pub name: String,
    pub description: String,
    pub peripheral_mod: LinkedHashMap<String, Rc<RefCell<PeripheralMod>>>,
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnumeratedSingleValue {
    pub name: String,
    pub value: u64,
    pub description: String,
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnumeratedValueType {
    pub name: String,
    pub size: BitSize, // Used generate the smallest numeric type to contain the value
    pub values: Vec<EnumeratedSingleValue>,
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FieldGetterSetter {
    pub name: String,
    pub description: String,
    pub offset: u32,
    pub mask: u32,
    pub size: BitSize,
    pub enum_type: Option<EnumeratedValueType>,
    pub access: RegisterBitfieldAccess,
    pub dim: u32,
    pub dim_increment: u32,
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RegisterAccess {
    #[default]
    R,
    W,
    RW,
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RegisterBitfieldAccess {
    #[default]
    R,
    W,
    RW,
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum BitSize {
    BIT64,
    #[default]
    BIT32,
    BIT16,
    BIT8,
}

impl BitSize {
    pub fn val_2_bit_size(val: u64) -> BitSize {
        if val <= u8::MAX.into() {
            BitSize::BIT8
        } else if val <= u16::MAX.into() {
            BitSize::BIT16
        } else if val <= u32::MAX.into() {
            BitSize::BIT32
        } else {
            BitSize::BIT64
        }
    }
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Register {
    pub name: String,
    pub description: String,
    pub offset: u32,
    pub dim: u32,
    pub dim_increment: u32,
    pub access: RegisterAccess,
    pub fields: LinkedHashMap<String, Rc<RefCell<FieldGetterSetter>>>,
    pub size: BitSize,
    pub reset_value: u64,
    pub has_enumerated_fields: bool,
    pub is_derived_from: bool,
    /// Full Rust path to module that contains the struct
    pub struct_module_path: Vec<String>,
    /// Id of the struct
    pub struct_id: String,
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cluster {
    pub name: String,
    pub description: String,
    pub offset: u32,
    pub dim: u32,
    pub dim_increment: u32,
    pub registers: LinkedHashMap<String, Rc<RefCell<Register>>>,
    pub clusters: LinkedHashMap<String, Rc<RefCell<Cluster>>>,
    pub is_derived_from: bool,
    /// Full Rust path to module that contains the struct
    pub struct_module_path: Vec<String>,
    /// Id of the struct
    pub struct_id: String,
}

/// Describe Rust module that maps to a peripheral
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeripheralMod {
    pub name: String,
    pub description: String,
    pub clusters: LinkedHashMap<String, Rc<RefCell<Cluster>>>,
    pub registers: LinkedHashMap<String, Rc<RefCell<Register>>>,
    pub base_addr: Vec<u64>,
    pub interrupts: Vec<Interrupt>,
    // pub is_derived_from: bool,
    // /// Struct identifier of the peripheral.
    // /// It can be different from cluster name in case derivedFrom and/or headerStructName are used
    // pub struct_id: String,
}

/// Represents a part of a fully qualified path name for registers.
///
/// Used to flatten the SVD tree into a vector of paths to registers while
/// preserving their index information if parts of the path are arrays of
/// peripherals/clusters/registers.
#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct PathChunk {
    /// Identifier of the path chunk
    pub path: String,
    /// Optional index if the current path is an array.
    /// Necessary because SVD supports arrays on all levels: peripherals, clusters
    /// and registers.
    pub index: Option<u32>,
}

/// Description of single interrupt
#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct Interrupt {
    pub name: String,
    pub value: u32,
    pub description: String,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IR {
    pub device: Device,
    pub register_addresses: LinkedHashMap<u64, Vec<Vec<PathChunk>>>,
    pub license_text: String,
    /// Interrupt table to be created in the lib.rs. Interrupt table hole has value None
    pub interrupt_table: Vec<Option<Interrupt>>,
    /// used only for cortex m target
    /// This could be none if no CPU is defined.
    /// If not defined NVIC_PRIO_BITS constant will be not generated
    pub nvic_prio_bits: Option<u32>,
    /// used only for cortex m target. If it is false or None cortex-m::Systick module will be re-exported
    /// This could be none if no CPU is defined.
    pub vendor_systick_config: Option<bool>,
    /// This could be none if no CPU is defined.
    pub fpu_present: Option<bool>,
    /// This could be none if no CPU is defined.
    pub mpu_present: Option<bool>,
}
