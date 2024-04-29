use svd_parser::svd;


pub trait PeripheralClusterT: svd::Name + svd::Description {
    fn get_clusters(&self) -> svd::registercluster::ClusterIter<'_>;
    fn get_registers(&self) -> svd::registercluster::RegisterIter<'_>;
    fn struct_name(&self) -> String;
}

impl PeripheralClusterT for svd::Peripheral {
    fn get_clusters(&self) -> svd::registercluster::ClusterIter<'_> {
        self.clusters()
    }

    fn get_registers(&self) -> svd::registercluster::RegisterIter<'_> {
        self.registers()
    }

    fn struct_name(&self) -> String {
        match self.header_struct_name {
            Some(ref struct_name) => struct_name.clone(),
            None => self.name.clone(),
        }
    }
}

impl PeripheralClusterT for svd::Cluster {
    fn get_clusters(&self) -> svd::registercluster::ClusterIter<'_> {
        self.clusters()
    }

    fn get_registers(&self) -> svd::registercluster::RegisterIter<'_> {
        self.registers()
    }

    fn struct_name(&self) -> String {
        match self.header_struct_name {
            Some(ref struct_name) => struct_name.clone(),
            None => self.name.clone(),
        }
    }
}

pub trait ExpandedName: svd_parser::svd::Name  {
    fn get_expanded_name(&self) -> String;
}

impl ExpandedName for svd::Cluster {
    fn get_expanded_name(&self) -> String {
        match self {
            svd::MaybeArray::Single(cluster_info) => cluster_info.name.clone(),
            svd::MaybeArray::Array(cluster_info,dim_info) => svd::cluster::expand(cluster_info,dim_info).next().expect("Empty").name.to_string(),
        }
    }
}

impl ExpandedName for svd::Peripheral {
    fn get_expanded_name(&self) -> String {
        match self {
            svd::MaybeArray::Single(info) => info.name.clone(),
            svd::MaybeArray::Array(info,dim_info) => svd::peripheral::expand(info,dim_info).next().expect("Empty").name.to_string(),
        }
    }
}
