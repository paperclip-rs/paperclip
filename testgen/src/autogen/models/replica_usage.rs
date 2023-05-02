#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// ReplicaUsage : Replica space usage information. Useful for capacity management, eg: figure out how much of a thin-provisioned replica is allocated. 








/// Replica space usage information. Useful for capacity management, eg: figure out how much of a thin-provisioned replica is allocated. 

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReplicaUsage {

    /// Replica capacity in bytes.
    #[serde(default, rename = "capacity")]
    pub capacity: u64,

    /// Amount of actually allocated disk space for this replica in bytes.
    #[serde(default, rename = "allocated")]
    pub allocated: u64,

    /// Amount of actually allocated disk space for this replica\'s snapshots in bytes.
    #[serde(default, rename = "allocated_snapshots")]
    pub allocated_snapshots: u64,

    /// Amount of actually allocated disk space for this replica\'s snapshots and its predecessors in bytes. For a restored/cloned replica this includes snapshots from the parent source. 
    #[serde(default, rename = "allocated_all_snapshots")]
    pub allocated_all_snapshots: u64,

}

impl ReplicaUsage {
    /// ReplicaUsage using only the required fields
    pub fn new(capacity: impl Into<u64>, allocated: impl Into<u64>, allocated_snapshots: impl Into<u64>, allocated_all_snapshots: impl Into<u64>) -> ReplicaUsage {
        ReplicaUsage {
            capacity: capacity.into(),
            allocated: allocated.into(),
            allocated_snapshots: allocated_snapshots.into(),
            allocated_all_snapshots: allocated_all_snapshots.into(),
            
        }
    }
    /// ReplicaUsage using all fields
    pub fn new_all(capacity: impl Into<u64>, allocated: impl Into<u64>, allocated_snapshots: impl Into<u64>, allocated_all_snapshots: impl Into<u64>) -> ReplicaUsage {
        ReplicaUsage {
            capacity: capacity.into(),
            allocated: allocated.into(),
            allocated_snapshots: allocated_snapshots.into(),
            allocated_all_snapshots: allocated_all_snapshots.into(),
            
        }
    }
}
















