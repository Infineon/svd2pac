use svd_parser::svd;

pub trait ExpandedName: svd_parser::svd::Name {
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
