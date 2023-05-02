#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// VolumeShareProtocol : Volume Share Protocol



/// Volume Share Protocol
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VolumeShareProtocol {


    #[serde(rename = "nvmf")]
    Nvmf,

    #[serde(rename = "iscsi")]
    Iscsi,

}

impl ToString for VolumeShareProtocol {
    fn to_string(&self) -> String {
        match self {
            
            
            Self::Nvmf => String::from("nvmf"),
            
            Self::Iscsi => String::from("iscsi"),
            
            
        }
    }
}

impl Default for VolumeShareProtocol {
    fn default() -> Self {
        Self::Nvmf
    }
}









