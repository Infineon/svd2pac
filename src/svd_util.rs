use svd_parser::svd::{self};

pub(crate) trait ExpandedName: svd_parser::svd::Name {
    /// Generate an identifier that can be used in derivedFrom tags
    /// CMSIS svd.xsd specification is not consisted with svdconv.exe.
    /// In xsd file derivedFrom is of type dimableIdentifierType and
    /// this means that we can use the name of an array including the index placeholder.
    /// This is not supported by svdconv.exe. svdconv.exe want to have name of register after array unrolling.
    /// This function shall return the name of first element after array unrolling
    /// If the element is not an array it return just a clone of the name.
    fn get_expanded_name(&self) -> String;
}

impl ExpandedName for svd::Cluster {
    fn get_expanded_name(&self) -> String {
        match self {
            svd::MaybeArray::Single(info) => info.name.clone(),
            svd::MaybeArray::Array(info, dim_info) => svd::cluster::expand(info, dim_info)
                .next()
                .expect("Empty")
                .name
                .to_string(),
        }
    }
}

impl ExpandedName for svd::Register {
    fn get_expanded_name(&self) -> String {
        match self {
            svd::MaybeArray::Single(info) => info.name.clone(),
            svd::MaybeArray::Array(info, dim_info) => svd::register::expand(info, dim_info)
                .next()
                .unwrap_or_else(|| panic!("Register {} is array of size 0", self.name))
                .name
                .to_string(),
        }
    }
}

impl ExpandedName for svd::Peripheral {
    fn get_expanded_name(&self) -> String {
        match self {
            svd::MaybeArray::Single(info) => info.name.clone(),
            svd::MaybeArray::Array(info, dim_info) => svd::peripheral::expand(info, dim_info)
                .next()
                .unwrap_or_else(|| panic!("Peripheral {} is array of size 0", self.name))
                .name
                .to_string(),
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
