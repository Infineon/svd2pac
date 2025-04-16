use svd_parser::svd::{self};

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("Invalid peripheral {peripheral_name:?}: {msg:?}")]
    InvalidPeripheral {
        peripheral_name: String,
        msg: String,
    },
    #[error("Invalid register {register_name:?}: {msg:?}")]
    InvalidRegister { register_name: String, msg: String },
    #[error("Invalid cluster {cluster_name:?}: {msg:?}")]
    InvalidCluster { cluster_name: String, msg: String },
    #[error("Invalid field {field_name:?}: {msg:?}")]
    InvalidField { field_name: String, msg: String },
    #[error("Unsupported feature {0}")]
    Unsupported(String),
}

pub(crate) trait ExpandedName: svd_parser::svd::Name {
    /// Generate an identifier that can be used in derivedFrom tags
    /// CMSIS svd.xsd specification is not consisted with svdconv.exe.
    /// In xsd file derivedFrom is of type dimableIdentifierType and
    /// this means that we can use the name of an array including the index placeholder.
    /// This is not supported by svdconv.exe. svdconv.exe want to have name of register after array unrolling.
    /// This function shall return the name of first element after array unrolling
    /// If the element is not an array it return just a clone of the name.
    fn get_expanded_name(&self) -> Result<String, ParseError>;
}

impl ExpandedName for svd::Cluster {
    fn get_expanded_name(&self) -> Result<String, ParseError> {
        match self {
            svd::MaybeArray::Single(info) => Ok(info.name.clone()),
            svd::MaybeArray::Array(info, dim_info) => Ok(svd::cluster::expand(info, dim_info)
                .next()
                .ok_or(ParseError::InvalidCluster {
                    cluster_name: self.name.clone(),
                    msg: "Array of size 0 is not allowed".to_string(),
                })?
                .name
                .to_string()),
        }
    }
}

impl ExpandedName for svd::Register {
    fn get_expanded_name(&self) -> Result<String, ParseError> {
        match self {
            svd::MaybeArray::Single(info) => Ok(info.name.clone()),
            svd::MaybeArray::Array(info, dim_info) => Ok(svd::register::expand(info, dim_info)
                .next()
                .ok_or(ParseError::InvalidRegister {
                    register_name: self.name.clone(),
                    msg: "Array of size 0 is not allowed".to_string(),
                })?
                .name
                .to_string()),
        }
    }
}

impl ExpandedName for svd::Peripheral {
    fn get_expanded_name(&self) -> Result<String, ParseError> {
        match self {
            svd::MaybeArray::Single(info) => Ok(info.name.clone()),
            svd::MaybeArray::Array(info, dim_info) => Ok(svd::peripheral::expand(info, dim_info)
                .next()
                .ok_or(ParseError::InvalidPeripheral {
                    peripheral_name: self.name.clone(),
                    msg: "Array of size 0 is not allowed".to_string(),
                })?
                .name
                .to_string()),
        }
    }
}

/// Trait to ger headerStructName field
pub(crate) trait HeaderStructName {
    fn header_struct_name(&self) -> Option<String>;
}

impl HeaderStructName for svd::Peripheral {
    fn header_struct_name(&self) -> Option<String> {
        self.header_struct_name.clone()
    }
}

impl HeaderStructName for svd::Cluster {
    fn header_struct_name(&self) -> Option<String> {
        self.header_struct_name.clone()
    }
}

impl HeaderStructName for svd::Register {
    fn header_struct_name(&self) -> Option<String> {
        None
    }
}
