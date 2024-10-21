#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// VolumeSnapshotState : Volume Snapshot State information.








/// Volume Snapshot State information.

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct VolumeSnapshotState {

    
    #[serde(default, rename = "uuid")]
    pub uuid: uuid::Uuid,

    /// Runtime size in bytes of the snapshot. Equal to the volume allocation at the time of the snapshot creation. It may grow larger if any of its predecessors are deleted.
    #[serde(default, rename = "allocated_size")]
    pub allocated_size: u64,

    
    #[serde(default, rename = "source_volume")]
    pub source_volume: uuid::Uuid,

    /// Timestamp when snapshot is taken on the storage system.
    #[serde(default, rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,

    /// Indicates if a snapshot is ready to be used as a new volume source.
    #[serde(default, rename = "ready_as_source")]
    pub ready_as_source: bool,

    /// List of individual ReplicaSnapshotStates.
    #[serde(default, rename = "replica_snapshots")]
    pub replica_snapshots: Vec<crate::models::ReplicaSnapshotState>,

}

impl VolumeSnapshotState {
    /// VolumeSnapshotState using only the required fields
    pub fn new(uuid: impl Into<uuid::Uuid>, allocated_size: impl Into<u64>, source_volume: impl Into<uuid::Uuid>, ready_as_source: impl Into<bool>, replica_snapshots: impl IntoVec<crate::models::ReplicaSnapshotState>) -> VolumeSnapshotState {
        VolumeSnapshotState {
            uuid: uuid.into(),
            allocated_size: allocated_size.into(),
            source_volume: source_volume.into(),
            timestamp: None,
            ready_as_source: ready_as_source.into(),
            replica_snapshots: replica_snapshots.into_vec(),
            
        }
    }
    /// VolumeSnapshotState using all fields
    pub fn new_all(uuid: impl Into<uuid::Uuid>, allocated_size: impl Into<u64>, source_volume: impl Into<uuid::Uuid>, timestamp: impl Into<Option<String>>, ready_as_source: impl Into<bool>, replica_snapshots: impl IntoVec<crate::models::ReplicaSnapshotState>) -> VolumeSnapshotState {
        VolumeSnapshotState {
            uuid: uuid.into(),
            allocated_size: allocated_size.into(),
            source_volume: source_volume.into(),
            timestamp: timestamp.into(),
            ready_as_source: ready_as_source.into(),
            replica_snapshots: replica_snapshots.into_vec(),
            
        }
    }
}




















