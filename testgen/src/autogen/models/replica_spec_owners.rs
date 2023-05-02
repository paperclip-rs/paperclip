#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// ReplicaSpecOwners : Owner Resource








/// Owner Resource

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReplicaSpecOwners {

    
    #[serde(default, rename = "nexuses")]
    pub nexuses: Vec<uuid::Uuid>,

    
    #[serde(default, rename = "volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<uuid::Uuid>,

}

impl ReplicaSpecOwners {
    /// ReplicaSpecOwners using only the required fields
    pub fn new(nexuses: impl IntoVec<uuid::Uuid>) -> ReplicaSpecOwners {
        ReplicaSpecOwners {
            nexuses: nexuses.into_vec(),
            volume: None,
            
        }
    }
    /// ReplicaSpecOwners using all fields
    pub fn new_all(nexuses: impl IntoVec<uuid::Uuid>, volume: impl Into<Option<uuid::Uuid>>) -> ReplicaSpecOwners {
        ReplicaSpecOwners {
            nexuses: nexuses.into_vec(),
            volume: volume.into(),
            
        }
    }
}












