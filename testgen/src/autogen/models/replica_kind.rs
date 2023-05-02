#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// ReplicaKind : Type of replica, example regular or snapshot.



/// Type of replica, example regular or snapshot.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ReplicaKind {


    #[serde(rename = "Regular")]
    Regular,

    #[serde(rename = "Snapshot")]
    Snapshot,

    #[serde(rename = "SnapshotClone")]
    SnapshotClone,

}

impl ToString for ReplicaKind {
    fn to_string(&self) -> String {
        match self {
            
            
            Self::Regular => String::from("Regular"),
            
            Self::Snapshot => String::from("Snapshot"),
            
            Self::SnapshotClone => String::from("SnapshotClone"),
            
            
        }
    }
}

impl Default for ReplicaKind {
    fn default() -> Self {
        Self::Regular
    }
}









