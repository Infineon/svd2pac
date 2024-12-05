use super::super::ir::PathChunk;
use super::super::util::*;
use super::RegisterHelper;
use linked_hash_map::LinkedHashMap;
use svd_parser::svd::{self, MaybeArray};

pub(super) struct EntityDb {
    /// Flat map of absolute addresses to paths of registers at that address.
    pub register_addresses: LinkedHashMap<u64, Vec<Vec<PathChunk>>>,
}

/// Register struct wrapper with base address.
#[derive(Copy, Clone)]
pub(super) struct RegisterAbs<'a> {
    pub register: &'a svd::RegisterInfo,
    pub base_addr: u64,
}

/// Cluster struct wrapper with base address.
#[derive(Copy, Clone)]
pub(super) struct ClusterAbs<'a> {
    pub cluster: &'a svd::ClusterInfo,
    pub base_addr: u64,
}

/// Trait to get the absolute address of a [`RegisterAbs`] and [`ClusterAbs`].
trait AbsoluteAddress {
    fn abs_address(&self) -> u64;
}
impl AbsoluteAddress for RegisterAbs<'_> {
    fn abs_address(&self) -> u64 {
        self.base_addr + self.register.address_offset as u64
    }
}
impl AbsoluteAddress for ClusterAbs<'_> {
    fn abs_address(&self) -> u64 {
        self.base_addr + self.cluster.address_offset as u64
    }
}

/// Generate complete flat map of paths to registers and clusters in an SVD file.
/// Maps include possible indices if path contains an array of peripherals/clusters/registers.
/// Values in maps store the respective base address of clusters/registers.
struct FQNFlatMaps<'a> {
    registers: LinkedHashMap<Vec<PathChunk>, RegisterAbs<'a>>,
    #[allow(dead_code)]
    clusters: LinkedHashMap<Vec<PathChunk>, ClusterAbs<'a>>,
}
impl<'svd> FQNFlatMaps<'svd> {
    /// Build [`FQNFlatMaps`] from [`svd::Device`] reference.
    ///
    /// Iterates over all (arrays of-) peripherals and collects their content
    /// into `registers` and `clusters` respectively.
    pub(super) fn generate(device: &'svd svd::Device) -> Self {
        let mut ret: Self = Self {
            registers: LinkedHashMap::new(),
            clusters: LinkedHashMap::new(),
        };

        for p in &device.peripherals {
            let p_name = p.name.to_internal_ident().to_sanitized_const_ident();
            match p {
                MaybeArray::Single(p) => {
                    let prefix: Vec<PathChunk> = vec![PathChunk {
                        path: p_name,
                        index: None,
                    }];
                    for register_cluster in p.registers.as_deref().unwrap_or_default() {
                        ret.collect_register_cluster_arrays(
                            &prefix,
                            register_cluster,
                            p.base_address,
                        );
                    }
                }
                MaybeArray::Array(p, dim) => {
                    for p_index in 0..dim.dim {
                        let prefix: Vec<PathChunk> = vec![PathChunk {
                            path: p_name.clone(),
                            index: Some(p_index),
                        }];
                        for register_cluster in p.registers.as_deref().unwrap_or_default() {
                            ret.collect_register_cluster_arrays(
                                &prefix,
                                register_cluster,
                                p.base_address + (p_index * dim.dim_increment) as u64,
                            );
                        }
                    }
                }
            }
        }
        ret
    }

    /// Collect individual [`svd::RegisterCluster`] objects into [`FQNFlatMaps`].
    /// Calls respective collector functions.
    /// Recursion point.
    pub(super) fn collect_register_cluster_arrays<'prefix>(
        &mut self,
        prefix: &'prefix [PathChunk],
        rg: &'svd svd::RegisterCluster,
        base_addr: u64,
    ) {
        match rg {
            svd::RegisterCluster::Register(register) => {
                self.collect_register_array(prefix, register, base_addr);
            }
            svd::RegisterCluster::Cluster(cluster) => {
                self.collect_cluster_array(prefix, cluster, base_addr);
            }
        }
    }
    /// Collect individual [`svd::Register`]s into [`FQNFlatMaps.registers`].
    pub(super) fn collect_register_array<'prefix>(
        &mut self,
        prefix: &'prefix [PathChunk],
        register: &'svd svd::Register,
        base_addr: u64,
    ) {
        match register {
            MaybeArray::Single(register) => {
                let mut key = prefix.to_owned();
                key.push(PathChunk {
                    path: register.get_name_id_internal().to_sanitized_func_ident(),
                    index: None,
                });
                self.registers.insert(
                    key,
                    RegisterAbs {
                        register,
                        base_addr,
                    },
                );
            }
            MaybeArray::Array(register, dim) => {
                for register_index in 0..dim.dim {
                    let mut key = prefix.to_owned();
                    key.push(PathChunk {
                        path: register.get_name_id_internal().to_sanitized_func_ident(),
                        index: Some(register_index),
                    });
                    self.registers.insert(
                        key,
                        RegisterAbs {
                            register,
                            base_addr: base_addr + (register_index * dim.dim_increment) as u64,
                        },
                    );
                }
            }
        }
    }
    /// Collect individual [`svd::Cluster`] into the FQN register_map.
    pub(super) fn collect_cluster_array<'prefix>(
        &mut self,
        prefix: &'prefix [PathChunk],
        cluster: &'svd svd::Cluster,
        base_addr: u64,
    ) {
        match cluster {
            MaybeArray::Single(cluster) => {
                let mut key = prefix.to_owned();
                key.push(PathChunk {
                    path: cluster.name.to_internal_ident().to_sanitized_func_ident(),
                    index: None,
                });
                for child in &cluster.children {
                    self.collect_register_cluster_arrays(&key, child, base_addr);
                }
            }
            MaybeArray::Array(cluster, dim) => {
                // `cluster.address_offset` is relative to the containing
                // element, even if the docs of `svd::ClusterInfo` state
                // otherwise. See SVD spec: https://www.keil.com/pack/doc/CMSIS/SVD/html/elem_registers.html#elem_cluster
                for cluster_index in 0..dim.dim {
                    let mut key = prefix.to_owned();
                    key.push(PathChunk {
                        path: cluster.name.to_internal_ident().to_sanitized_func_ident(),
                        index: Some(cluster_index),
                    });
                    for child in &cluster.children {
                        self.collect_register_cluster_arrays(
                            &key,
                            child,
                            base_addr + (cluster_index * dim.dim_increment) as u64,
                        );
                    }
                }
            }
        }
    }
}

pub(super) fn get_entity_db(device: &svd::Device) -> EntityDb {
    let flat_maps = FQNFlatMaps::generate(device);

    // Build flat map of enumerated_values and addresses mapping to register names.
    //
    // This is needed for
    let mut register_addresses: LinkedHashMap<u64, Vec<Vec<PathChunk>>> = LinkedHashMap::new();
    for (reg_name, &reg) in &flat_maps.registers {
        register_addresses
            .entry(reg.abs_address())
            .or_default()
            .push(reg_name.clone());
    }

    EntityDb { register_addresses }
}
