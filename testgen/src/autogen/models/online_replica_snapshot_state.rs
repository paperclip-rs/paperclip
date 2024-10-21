#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// OnlineReplicaSnapshotState : Online ReplicaSnapshotState representation.








/// Online ReplicaSnapshotState representation.

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct OnlineReplicaSnapshotState {

    
    #[serde(default, rename = "uuid")]
    pub uuid: uuid::Uuid,

    
    #[serde(default, rename = "source_id")]
    pub source_id: uuid::Uuid,

    /// storage pool identifier
    #[serde(default, rename = "pool_id")]
    pub pool_id: String,

    /// storage pool unique identifier
    #[serde(default, rename = "pool_uuid")]
    pub pool_uuid: uuid::Uuid,

    /// Timestamp when the replica snapshot is taken on the storage system.
    #[serde(default, rename = "timestamp")]
    pub timestamp: String,

    /// Replica snapshot size.
    #[serde(default, rename = "size")]
    pub size: u64,

    /// Runtime size in bytes of the snapshot. Equal to the volume allocation at the time of the snapshot creation. It may grow larger if any of its predecessors are deleted.
    #[serde(default, rename = "allocated_size")]
    pub allocated_size: u64,

    /// Total allocated size of all the snapshot predecessors.
    #[serde(default, rename = "predecessor_alloc_size")]
    pub predecessor_alloc_size: u64,

}

impl OnlineReplicaSnapshotState {
    /// OnlineReplicaSnapshotState using only the required fields
    pub fn new(uuid: impl Into<uuid::Uuid>, source_id: impl Into<uuid::Uuid>, pool_id: impl Into<String>, pool_uuid: impl Into<uuid::Uuid>, timestamp: impl Into<String>, size: impl Into<u64>, allocated_size: impl Into<u64>, predecessor_alloc_size: impl Into<u64>) -> OnlineReplicaSnapshotState {
        OnlineReplicaSnapshotState {
            uuid: uuid.into(),
            source_id: source_id.into(),
            pool_id: pool_id.into(),
            pool_uuid: pool_uuid.into(),
            timestamp: timestamp.into(),
            size: size.into(),
            allocated_size: allocated_size.into(),
            predecessor_alloc_size: predecessor_alloc_size.into(),
            
        }
    }
    /// OnlineReplicaSnapshotState using all fields
    pub fn new_all(uuid: impl Into<uuid::Uuid>, source_id: impl Into<uuid::Uuid>, pool_id: impl Into<String>, pool_uuid: impl Into<uuid::Uuid>, timestamp: impl Into<String>, size: impl Into<u64>, allocated_size: impl Into<u64>, predecessor_alloc_size: impl Into<u64>) -> OnlineReplicaSnapshotState {
        OnlineReplicaSnapshotState {
            uuid: uuid.into(),
            source_id: source_id.into(),
            pool_id: pool_id.into(),
            pool_uuid: pool_uuid.into(),
            timestamp: timestamp.into(),
            size: size.into(),
            allocated_size: allocated_size.into(),
            predecessor_alloc_size: predecessor_alloc_size.into(),
            
        }
    }
}
























