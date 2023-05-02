#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// ReplicaSpec : User specification of a replica.








/// User specification of a replica.

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReplicaSpec {

    /// Managed by our control plane
    #[serde(default, rename = "managed")]
    pub managed: bool,

    /// Record of the operation in progress
    #[serde(default, rename = "operation", skip_serializing_if = "Option::is_none")]
    pub operation: Option<crate::models::ReplicaSpecOperation>,

    /// Owner Resource
    #[serde(default, rename = "owners")]
    pub owners: crate::models::ReplicaSpecOwners,

    /// The pool that the replica should live on.
    #[serde(default, rename = "pool")]
    pub pool: String,

    /// storage pool unique identifier
    #[serde(default, rename = "poolUuid", skip_serializing_if = "Option::is_none")]
    pub pool_uuid: Option<uuid::Uuid>,

    /// Common Protocol
    #[serde(default, rename = "share")]
    pub share: crate::models::Protocol,

    /// The size that the replica should be.
    #[serde(default, rename = "size")]
    pub size: u64,

    /// Common base state for a resource
    #[serde(default, rename = "status")]
    pub status: crate::models::SpecStatus,

    /// Thin provisioning.
    #[serde(default, rename = "thin")]
    pub thin: bool,

    /// uuid of the replica
    #[serde(default, rename = "uuid")]
    pub uuid: uuid::Uuid,

    /// Type of replica, example regular or snapshot.
    #[serde(default, rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<crate::models::ReplicaKind>,

}

impl ReplicaSpec {
    /// ReplicaSpec using only the required fields
    pub fn new(managed: impl Into<bool>, owners: impl Into<crate::models::ReplicaSpecOwners>, pool: impl Into<String>, share: impl Into<crate::models::Protocol>, size: impl Into<u64>, status: impl Into<crate::models::SpecStatus>, thin: impl Into<bool>, uuid: impl Into<uuid::Uuid>) -> ReplicaSpec {
        ReplicaSpec {
            managed: managed.into(),
            operation: None,
            owners: owners.into(),
            pool: pool.into(),
            pool_uuid: None,
            share: share.into(),
            size: size.into(),
            status: status.into(),
            thin: thin.into(),
            uuid: uuid.into(),
            kind: None,
            
        }
    }
    /// ReplicaSpec using all fields
    pub fn new_all(managed: impl Into<bool>, operation: impl Into<Option<crate::models::ReplicaSpecOperation>>, owners: impl Into<crate::models::ReplicaSpecOwners>, pool: impl Into<String>, pool_uuid: impl Into<Option<uuid::Uuid>>, share: impl Into<crate::models::Protocol>, size: impl Into<u64>, status: impl Into<crate::models::SpecStatus>, thin: impl Into<bool>, uuid: impl Into<uuid::Uuid>, kind: impl Into<Option<crate::models::ReplicaKind>>) -> ReplicaSpec {
        ReplicaSpec {
            managed: managed.into(),
            operation: operation.into(),
            owners: owners.into(),
            pool: pool.into(),
            pool_uuid: pool_uuid.into(),
            share: share.into(),
            size: size.into(),
            status: status.into(),
            thin: thin.into(),
            uuid: uuid.into(),
            kind: kind.into(),
            
        }
    }
}






























