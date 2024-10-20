#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// ReplicaSnapshot : Replica Snapshot information.








/// Replica Snapshot information.

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReplicaSnapshot {

    
    #[serde(default, rename = "uuid")]
    pub uuid: uuid::Uuid,

    
    #[serde(default, rename = "source_id")]
    pub source_id: uuid::Uuid,

    /// Common base state for a resource
    #[serde(default, rename = "status")]
    pub status: crate::models::SpecStatus,

}

impl ReplicaSnapshot {
    /// ReplicaSnapshot using only the required fields
    pub fn new(uuid: impl Into<uuid::Uuid>, source_id: impl Into<uuid::Uuid>, status: impl Into<crate::models::SpecStatus>) -> ReplicaSnapshot {
        ReplicaSnapshot {
            uuid: uuid.into(),
            source_id: source_id.into(),
            status: status.into(),
            
        }
    }
    /// ReplicaSnapshot using all fields
    pub fn new_all(uuid: impl Into<uuid::Uuid>, source_id: impl Into<uuid::Uuid>, status: impl Into<crate::models::SpecStatus>) -> ReplicaSnapshot {
        ReplicaSnapshot {
            uuid: uuid.into(),
            source_id: source_id.into(),
            status: status.into(),
            
        }
    }
}














