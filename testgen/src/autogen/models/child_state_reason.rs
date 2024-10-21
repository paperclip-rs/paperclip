#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// ChildStateReason : Reason for the state of a Nexus Child



/// Reason for the state of a Nexus Child
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ChildStateReason {


    #[serde(rename = "OutOfSpace")]
    OutOfSpace,

}

impl ToString for ChildStateReason {
    fn to_string(&self) -> String {
        match self {
            
            
            Self::OutOfSpace => String::from("OutOfSpace"),
            
            
        }
    }
}

impl Default for ChildStateReason {
    fn default() -> Self {
        Self::OutOfSpace
    }
}









