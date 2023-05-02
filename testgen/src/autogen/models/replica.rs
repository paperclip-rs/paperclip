#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// Replica : Replica information








/// Replica information

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Replica {

    /// storage node identifier
    #[serde(default, rename = "node")]
    pub node: String,

    /// storage pool identifier
    #[serde(default, rename = "pool")]
    pub pool: String,

    /// storage pool unique identifier
    #[serde(default, rename = "poolUuid", skip_serializing_if = "Option::is_none")]
    pub pool_uuid: Option<uuid::Uuid>,

    /// Common Protocol
    #[serde(default, rename = "share")]
    pub share: crate::models::Protocol,

    /// size of the replica in bytes
    #[serde(default, rename = "size")]
    pub size: u64,

    /// Replica space usage information. Useful for capacity management, eg: figure out how much of a thin-provisioned replica is allocated. 
    #[serde(default, rename = "space", skip_serializing_if = "Option::is_none")]
    pub space: Option<crate::models::ReplicaSpaceUsage>,

    /// state of the replica
    #[serde(default, rename = "state")]
    pub state: crate::models::ReplicaState,

    /// thin provisioning
    #[serde(default, rename = "thin")]
    pub thin: bool,

    /// uri usable by nexus to access it
    #[serde(default, rename = "uri")]
    pub uri: String,

    /// uuid of the replica
    #[serde(default, rename = "uuid")]
    pub uuid: uuid::Uuid,

    /// NQNs of hosts allowed to connect to this replica
    #[serde(default, rename = "allowed-hosts", skip_serializing_if = "Option::is_none")]
    pub allowed_hosts: Option<Vec<String>>,

    /// Type of replica, example regular or snapshot.
    #[serde(default, rename = "kind")]
    pub kind: crate::models::ReplicaKind,

}

impl Replica {
    /// Replica using only the required fields
    pub fn new(node: impl Into<String>, pool: impl Into<String>, share: impl Into<crate::models::Protocol>, size: impl Into<u64>, state: impl Into<crate::models::ReplicaState>, thin: impl Into<bool>, uri: impl Into<String>, uuid: impl Into<uuid::Uuid>, kind: impl Into<crate::models::ReplicaKind>) -> Replica {
        Replica {
            node: node.into(),
            pool: pool.into(),
            pool_uuid: None,
            share: share.into(),
            size: size.into(),
            space: None,
            state: state.into(),
            thin: thin.into(),
            uri: uri.into(),
            uuid: uuid.into(),
            allowed_hosts: None,
            kind: kind.into(),
            
        }
    }
    /// Replica using all fields
    pub fn new_all(node: impl Into<String>, pool: impl Into<String>, pool_uuid: impl Into<Option<uuid::Uuid>>, share: impl Into<crate::models::Protocol>, size: impl Into<u64>, space: impl Into<Option<crate::models::ReplicaSpaceUsage>>, state: impl Into<crate::models::ReplicaState>, thin: impl Into<bool>, uri: impl Into<String>, uuid: impl Into<uuid::Uuid>, allowed_hosts: impl IntoOptVec<String>, kind: impl Into<crate::models::ReplicaKind>) -> Replica {
        Replica {
            node: node.into(),
            pool: pool.into(),
            pool_uuid: pool_uuid.into(),
            share: share.into(),
            size: size.into(),
            space: space.into(),
            state: state.into(),
            thin: thin.into(),
            uri: uri.into(),
            uuid: uuid.into(),
            allowed_hosts: allowed_hosts.into_opt_vec(),
            kind: kind.into(),
            
        }
    }
}
































