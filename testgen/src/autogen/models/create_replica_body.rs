#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// CreateReplicaBody : Create Replica Body








/// Create Replica Body

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct CreateReplicaBody {

    /// Replica Share Protocol
    #[serde(default, rename = "share", skip_serializing_if = "Option::is_none")]
    pub share: Option<crate::models::ReplicaShareProtocol>,

    
    #[serde(default, rename = "allowed-hosts", skip_serializing_if = "Option::is_none")]
    pub allowed_hosts: Option<Vec<String>>,

    /// size of the replica in bytes
    #[serde(default, rename = "size")]
    pub size: u64,

    /// thin provisioning
    #[serde(default, rename = "thin")]
    pub thin: bool,

}

impl CreateReplicaBody {
    /// CreateReplicaBody using only the required fields
    pub fn new(size: impl Into<u64>, thin: impl Into<bool>) -> CreateReplicaBody {
        CreateReplicaBody {
            share: None,
            allowed_hosts: None,
            size: size.into(),
            thin: thin.into(),
            
        }
    }
    /// CreateReplicaBody using all fields
    pub fn new_all(share: impl Into<Option<crate::models::ReplicaShareProtocol>>, allowed_hosts: impl IntoOptVec<String>, size: impl Into<u64>, thin: impl Into<bool>) -> CreateReplicaBody {
        CreateReplicaBody {
            share: share.into(),
            allowed_hosts: allowed_hosts.into_opt_vec(),
            size: size.into(),
            thin: thin.into(),
            
        }
    }
}
















