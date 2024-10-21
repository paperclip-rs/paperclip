#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// NexusShareProtocol : Nexus Share Protocol



/// Nexus Share Protocol
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NexusShareProtocol {


    #[serde(rename = "nvmf")]
    Nvmf,

    #[serde(rename = "iscsi")]
    Iscsi,

}

impl ToString for NexusShareProtocol {
    fn to_string(&self) -> String {
        match self {
            
            
            Self::Nvmf => String::from("nvmf"),
            
            Self::Iscsi => String::from("iscsi"),
            
            
        }
    }
}

impl Default for NexusShareProtocol {
    fn default() -> Self {
        Self::Nvmf
    }
}









