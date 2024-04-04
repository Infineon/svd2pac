use linked_hash_map::LinkedHashMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Device {
    pub name: String,
    pub description: String,
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnumeratedSigleValue {
    pub name: String,
    pub value: u64,
    pub description: String,
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnumeratedValueType {
    pub name: String,
    pub size: BitSize, // Used generate the smallest numeric type to contain the value
    pub values: Vec<EnumeratedSigleValue>,
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
    pub fields: Vec<FieldGetterSetter>,
    pub size: BitSize,
    pub reset_value: u64,
    pub has_enumerated_fields: bool,
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cluster {
    pub name: String,
    pub description: String,
    pub offset: u32,
    pub dim: u32,
    pub dim_increment: u32,
    pub registers: LinkedHashMap<String, Register>,
    pub clusters: LinkedHashMap<String, Cluster>,
    pub is_derived_from: Option<String>,
    pub struct_id: String,
    pub struct_path: String,
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeripheralMod {
    pub name: String,
    pub description: String,
    pub clusters: LinkedHashMap<String, Cluster>,
    pub registers: LinkedHashMap<String, Register>,
    pub base_addr: Vec<u64>,
    pub interrupts: Vec<Interrupt>,
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
    pub peripheral_mod: LinkedHashMap<String, PeripheralMod>,
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
