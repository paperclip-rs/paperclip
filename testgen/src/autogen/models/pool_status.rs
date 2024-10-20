#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// PoolStatus : current status of the pool



/// current status of the pool
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PoolStatus {


    #[serde(rename = "Unknown")]
    Unknown,

    #[serde(rename = "Online")]
    Online,

    #[serde(rename = "Degraded")]
    Degraded,

    #[serde(rename = "Faulted")]
    Faulted,

}

impl ToString for PoolStatus {
    fn to_string(&self) -> String {
        match self {
            
            
            Self::Unknown => String::from("Unknown"),
            
            Self::Online => String::from("Online"),
            
            Self::Degraded => String::from("Degraded"),
            
            Self::Faulted => String::from("Faulted"),
            
            
        }
    }
}

impl Default for PoolStatus {
    fn default() -> Self {
        Self::Unknown
    }
}









