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
