#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// VolumeUsage : Volume space usage








/// Volume space usage

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct VolumeUsage {

    /// Capacity of the volume in bytes.
    #[serde(default, rename = "capacity")]
    pub capacity: u64,

    /// -| Allocated size in bytes, related the largest healthy replica, including snapshots. For example, if a volume has 2 replicas, each with 1MiB allocated space, then this field will be 1MiB.
    #[serde(default, rename = "allocated")]
    pub allocated: u64,

    /// -| Allocated size in bytes, related to the largest healthy replica, excluding snapshots.
    #[serde(default, rename = "allocated_replica")]
    pub allocated_replica: u64,

    /// -| Allocated size in bytes, related the healthy replica with the highest snapshot usage.
    #[serde(default, rename = "allocated_snapshots")]
    pub allocated_snapshots: u64,

    /// -| For a restored/cloned volume, allocated size in bytes, related to the healthy replica with largest parent snapshot allocation.
    #[serde(default, rename = "allocated_all_snapshots")]
    pub allocated_all_snapshots: u64,

    /// -| Allocated size in bytes, accrued from all the replicas, including snapshots. For example, if a volume has 2 replicas, each with 1MiB allocated space, then this field will be 2MiB.
    #[serde(default, rename = "total_allocated")]
    pub total_allocated: u64,

    /// -| Allocated size in bytes, accrued from all the replicas, excluding snapshots.
    #[serde(default, rename = "total_allocated_replicas")]
    pub total_allocated_replicas: serde_json::Value,

    /// -| Allocated size in bytes, accrued from all the replica\'s snapshots.
    #[serde(default, rename = "total_allocated_snapshots")]
    pub total_allocated_snapshots: u64,

}

impl VolumeUsage {
    /// VolumeUsage using only the required fields
    pub fn new(capacity: impl Into<u64>, allocated: impl Into<u64>, allocated_replica: impl Into<u64>, allocated_snapshots: impl Into<u64>, allocated_all_snapshots: impl Into<u64>, total_allocated: impl Into<u64>, total_allocated_replicas: impl Into<serde_json::Value>, total_allocated_snapshots: impl Into<u64>) -> VolumeUsage {
        VolumeUsage {
            capacity: capacity.into(),
            allocated: allocated.into(),
            allocated_replica: allocated_replica.into(),
            allocated_snapshots: allocated_snapshots.into(),
            allocated_all_snapshots: allocated_all_snapshots.into(),
            total_allocated: total_allocated.into(),
            total_allocated_replicas: total_allocated_replicas.into(),
            total_allocated_snapshots: total_allocated_snapshots.into(),
            
        }
    }
    /// VolumeUsage using all fields
    pub fn new_all(capacity: impl Into<u64>, allocated: impl Into<u64>, allocated_replica: impl Into<u64>, allocated_snapshots: impl Into<u64>, allocated_all_snapshots: impl Into<u64>, total_allocated: impl Into<u64>, total_allocated_replicas: impl Into<serde_json::Value>, total_allocated_snapshots: impl Into<u64>) -> VolumeUsage {
        VolumeUsage {
            capacity: capacity.into(),
            allocated: allocated.into(),
            allocated_replica: allocated_replica.into(),
            allocated_snapshots: allocated_snapshots.into(),
            allocated_all_snapshots: allocated_all_snapshots.into(),
            total_allocated: total_allocated.into(),
            total_allocated_replicas: total_allocated_replicas.into(),
            total_allocated_snapshots: total_allocated_snapshots.into(),
            
        }
    }
}
























