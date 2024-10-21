#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// Protocol : Common Protocol



/// Common Protocol
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Protocol {


    #[serde(rename = "none")]
    None,

    #[serde(rename = "nvmf")]
    Nvmf,

    #[serde(rename = "iscsi")]
    Iscsi,

    #[serde(rename = "nbd")]
    Nbd,

}

impl ToString for Protocol {
    fn to_string(&self) -> String {
        match self {
            
            
            Self::None => String::from("none"),
            
            Self::Nvmf => String::from("nvmf"),
            
            Self::Iscsi => String::from("iscsi"),
            
            Self::Nbd => String::from("nbd"),
            
            
        }
    }
}

impl Default for Protocol {
    fn default() -> Self {
        Self::None
    }
}









