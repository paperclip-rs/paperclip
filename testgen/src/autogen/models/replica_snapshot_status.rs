#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// ReplicaSnapshotStatus : Current ReplicaSnapshot status



/// Current ReplicaSnapshot status
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ReplicaSnapshotStatus {


    #[serde(rename = "Unknown")]
    Unknown,

    #[serde(rename = "Online")]
    Online,

    #[serde(rename = "Offline")]
    Offline,

}

impl ToString for ReplicaSnapshotStatus {
    fn to_string(&self) -> String {
        match self {
            
            
            Self::Unknown => String::from("Unknown"),
            
            Self::Online => String::from("Online"),
            
            Self::Offline => String::from("Offline"),
            
            
        }
    }
}

impl Default for ReplicaSnapshotStatus {
    fn default() -> Self {
        Self::Unknown
    }
}









