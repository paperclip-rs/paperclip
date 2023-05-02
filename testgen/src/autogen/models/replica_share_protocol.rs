#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// ReplicaShareProtocol : Replica Share Protocol



/// Replica Share Protocol
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ReplicaShareProtocol {


    #[serde(rename = "nvmf")]
    Nvmf,

}

impl ToString for ReplicaShareProtocol {
    fn to_string(&self) -> String {
        match self {
            
            
            Self::Nvmf => String::from("nvmf"),
            
            
        }
    }
}

impl Default for ReplicaShareProtocol {
    fn default() -> Self {
        Self::Nvmf
    }
}









