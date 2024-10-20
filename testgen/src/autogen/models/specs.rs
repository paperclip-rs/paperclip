#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// Specs : Specs detailing the requested configuration of the objects.








/// Specs detailing the requested configuration of the objects.

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Specs {

    /// Nexus Specs
    #[serde(default, rename = "nexuses")]
    pub nexuses: Vec<crate::models::NexusSpec>,

    /// Pool Specs
    #[serde(default, rename = "pools")]
    pub pools: Vec<crate::models::PoolSpec>,

    /// Replica Specs
    #[serde(default, rename = "replicas")]
    pub replicas: Vec<crate::models::ReplicaSpec>,

    /// Volume Specs
    #[serde(default, rename = "volumes")]
    pub volumes: Vec<crate::models::VolumeSpec>,

}

impl Specs {
    /// Specs using only the required fields
    pub fn new(nexuses: impl IntoVec<crate::models::NexusSpec>, pools: impl IntoVec<crate::models::PoolSpec>, replicas: impl IntoVec<crate::models::ReplicaSpec>, volumes: impl IntoVec<crate::models::VolumeSpec>) -> Specs {
        Specs {
            nexuses: nexuses.into_vec(),
            pools: pools.into_vec(),
            replicas: replicas.into_vec(),
            volumes: volumes.into_vec(),
            
        }
    }
    /// Specs using all fields
    pub fn new_all(nexuses: impl IntoVec<crate::models::NexusSpec>, pools: impl IntoVec<crate::models::PoolSpec>, replicas: impl IntoVec<crate::models::ReplicaSpec>, volumes: impl IntoVec<crate::models::VolumeSpec>) -> Specs {
        Specs {
            nexuses: nexuses.into_vec(),
            pools: pools.into_vec(),
            replicas: replicas.into_vec(),
            volumes: volumes.into_vec(),
            
        }
    }
}
















