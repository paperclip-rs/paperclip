#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// ReplicaSpaceUsage : Replica space usage information. Useful for capacity management, eg: figure out how much of a thin-provisioned replica is allocated. 








/// Replica space usage information. Useful for capacity management, eg: figure out how much of a thin-provisioned replica is allocated. 

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReplicaSpaceUsage {

    /// Replica capacity in bytes.
    #[serde(default, rename = "capacity_bytes")]
    pub capacity_bytes: u64,

    /// Amount of actually allocated disk space for this replica in bytes.
    #[serde(default, rename = "allocated_bytes")]
    pub allocated_bytes: u64,

    /// Amount of actually allocated disk space for this replica\'s snapshots in bytes.
    #[serde(default, rename = "allocated_bytes_snapshots")]
    pub allocated_bytes_snapshots: u64,

    /// Amount of actually allocated disk space for this replica\'s snapshots and its predecessors in bytes. For a restored/cloned replica this includes snapshots from the parent source. 
    #[serde(default, rename = "allocated_bytes_all_snapshots", skip_serializing_if = "Option::is_none")]
    pub allocated_bytes_all_snapshots: Option<u64>,

    /// Cluster size in bytes.
    #[serde(default, rename = "cluster_size")]
    pub cluster_size: u64,

    /// Total number of clusters.
    #[serde(default, rename = "clusters")]
    pub clusters: u64,

    /// Number of actually used clusters.
    #[serde(default, rename = "allocated_clusters")]
    pub allocated_clusters: u64,

}

impl ReplicaSpaceUsage {
    /// ReplicaSpaceUsage using only the required fields
    pub fn new(capacity_bytes: impl Into<u64>, allocated_bytes: impl Into<u64>, allocated_bytes_snapshots: impl Into<u64>, cluster_size: impl Into<u64>, clusters: impl Into<u64>, allocated_clusters: impl Into<u64>) -> ReplicaSpaceUsage {
        ReplicaSpaceUsage {
            capacity_bytes: capacity_bytes.into(),
            allocated_bytes: allocated_bytes.into(),
            allocated_bytes_snapshots: allocated_bytes_snapshots.into(),
            allocated_bytes_all_snapshots: None,
            cluster_size: cluster_size.into(),
            clusters: clusters.into(),
            allocated_clusters: allocated_clusters.into(),
            
        }
    }
    /// ReplicaSpaceUsage using all fields
    pub fn new_all(capacity_bytes: impl Into<u64>, allocated_bytes: impl Into<u64>, allocated_bytes_snapshots: impl Into<u64>, allocated_bytes_all_snapshots: impl Into<Option<u64>>, cluster_size: impl Into<u64>, clusters: impl Into<u64>, allocated_clusters: impl Into<u64>) -> ReplicaSpaceUsage {
        ReplicaSpaceUsage {
            capacity_bytes: capacity_bytes.into(),
            allocated_bytes: allocated_bytes.into(),
            allocated_bytes_snapshots: allocated_bytes_snapshots.into(),
            allocated_bytes_all_snapshots: allocated_bytes_all_snapshots.into(),
            cluster_size: cluster_size.into(),
            clusters: clusters.into(),
            allocated_clusters: allocated_clusters.into(),
            
        }
    }
}






















