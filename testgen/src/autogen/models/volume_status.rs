#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// VolumeStatus : current volume status



/// current volume status
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VolumeStatus {


    #[serde(rename = "Unknown")]
    Unknown,

    #[serde(rename = "Online")]
    Online,

    #[serde(rename = "Degraded")]
    Degraded,

    #[serde(rename = "Faulted")]
    Faulted,

    #[serde(rename = "Shutdown")]
    Shutdown,

}

impl ToString for VolumeStatus {
    fn to_string(&self) -> String {
        match self {
            
            
            Self::Unknown => String::from("Unknown"),
            
            Self::Online => String::from("Online"),
            
            Self::Degraded => String::from("Degraded"),
            
            Self::Faulted => String::from("Faulted"),
            
            Self::Shutdown => String::from("Shutdown"),
            
            
        }
    }
}

impl Default for VolumeStatus {
    fn default() -> Self {
        Self::Unknown
    }
}









