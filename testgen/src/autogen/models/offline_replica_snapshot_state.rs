#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// OfflineReplicaSnapshotState : Offline ReplicaSnapshotState representation.








/// Offline ReplicaSnapshotState representation.

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct OfflineReplicaSnapshotState {

    
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

}

impl OfflineReplicaSnapshotState {
    /// OfflineReplicaSnapshotState using only the required fields
    pub fn new(uuid: impl Into<uuid::Uuid>, source_id: impl Into<uuid::Uuid>, pool_id: impl Into<String>, pool_uuid: impl Into<uuid::Uuid>) -> OfflineReplicaSnapshotState {
        OfflineReplicaSnapshotState {
            uuid: uuid.into(),
            source_id: source_id.into(),
            pool_id: pool_id.into(),
            pool_uuid: pool_uuid.into(),
            
        }
    }
    /// OfflineReplicaSnapshotState using all fields
    pub fn new_all(uuid: impl Into<uuid::Uuid>, source_id: impl Into<uuid::Uuid>, pool_id: impl Into<String>, pool_uuid: impl Into<uuid::Uuid>) -> OfflineReplicaSnapshotState {
        OfflineReplicaSnapshotState {
            uuid: uuid.into(),
            source_id: source_id.into(),
            pool_id: pool_id.into(),
            pool_uuid: pool_uuid.into(),
            
        }
    }
}
















