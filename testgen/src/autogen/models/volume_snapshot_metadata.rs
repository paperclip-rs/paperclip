#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// VolumeSnapshotMetadata : Volume Snapshot Metadata information.








/// Volume Snapshot Metadata information.

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct VolumeSnapshotMetadata {

    /// Common base state for a resource
    #[serde(default, rename = "status")]
    pub status: crate::models::SpecStatus,

    /// Timestamp when snapshot is taken on the storage system.
    #[serde(default, rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,

    /// Size in bytes of the snapshot (which is equivalent to its source size).
    #[serde(default, rename = "size")]
    pub size: u64,

    /// Spec size in bytes of the snapshot (which is equivalent to its source spec size).
    #[serde(default, rename = "spec_size")]
    pub spec_size: u64,

    /// Size in bytes taken by the snapshot and its predecessors.
    #[serde(default, rename = "total_allocated_size")]
    pub total_allocated_size: u64,

    
    #[serde(default, rename = "txn_id")]
    pub txn_id: String,

    
    #[serde(default, rename = "transactions")]
    pub transactions: ::std::collections::HashMap<String, Vec<crate::models::ReplicaSnapshot>>,

    /// Number of restores done from this snapshot.
    #[serde(default, rename = "num_restores")]
    pub num_restores: u32,

    /// Number of snapshot replicas for a volumesnapshot.
    #[serde(default, rename = "num_snapshot_replicas")]
    pub num_snapshot_replicas: u32,

}

impl VolumeSnapshotMetadata {
    /// VolumeSnapshotMetadata using only the required fields
    pub fn new(status: impl Into<crate::models::SpecStatus>, size: impl Into<u64>, spec_size: impl Into<u64>, total_allocated_size: impl Into<u64>, txn_id: impl Into<String>, transactions: impl Into<::std::collections::HashMap<String, Vec<crate::models::ReplicaSnapshot>>>, num_restores: impl Into<u32>, num_snapshot_replicas: impl Into<u32>) -> VolumeSnapshotMetadata {
        VolumeSnapshotMetadata {
            status: status.into(),
            timestamp: None,
            size: size.into(),
            spec_size: spec_size.into(),
            total_allocated_size: total_allocated_size.into(),
            txn_id: txn_id.into(),
            transactions: transactions.into(),
            num_restores: num_restores.into(),
            num_snapshot_replicas: num_snapshot_replicas.into(),
            
        }
    }
    /// VolumeSnapshotMetadata using all fields
    pub fn new_all(status: impl Into<crate::models::SpecStatus>, timestamp: impl Into<Option<String>>, size: impl Into<u64>, spec_size: impl Into<u64>, total_allocated_size: impl Into<u64>, txn_id: impl Into<String>, transactions: impl Into<::std::collections::HashMap<String, Vec<crate::models::ReplicaSnapshot>>>, num_restores: impl Into<u32>, num_snapshot_replicas: impl Into<u32>) -> VolumeSnapshotMetadata {
        VolumeSnapshotMetadata {
            status: status.into(),
            timestamp: timestamp.into(),
            size: size.into(),
            spec_size: spec_size.into(),
            total_allocated_size: total_allocated_size.into(),
            txn_id: txn_id.into(),
            transactions: transactions.into(),
            num_restores: num_restores.into(),
            num_snapshot_replicas: num_snapshot_replicas.into(),
            
        }
    }
}


























