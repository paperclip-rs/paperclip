#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// NexusSpecOperation : Record of the operation in progress








/// Record of the operation in progress

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct NexusSpecOperation {

    /// Record of the operation
    #[serde(default, rename = "operation")]
    pub operation: Operation,

    /// Result of the operation
    #[serde(default, rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<bool>,

}

impl NexusSpecOperation {
    /// NexusSpecOperation using only the required fields
    pub fn new(operation: impl Into<Operation>) -> NexusSpecOperation {
        NexusSpecOperation {
            operation: operation.into(),
            result: None,
            
        }
    }
    /// NexusSpecOperation using all fields
    pub fn new_all(operation: impl Into<Operation>, result: impl Into<Option<bool>>) -> NexusSpecOperation {
        NexusSpecOperation {
            operation: operation.into(),
            result: result.into(),
            
        }
    }
}








/// Record of the operation
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Operation {


    #[serde(rename = "Create")]
    Create,

    #[serde(rename = "Destroy")]
    Destroy,

    #[serde(rename = "Share")]
    Share,

    #[serde(rename = "Unshare")]
    Unshare,

    #[serde(rename = "AddChild")]
    AddChild,

    #[serde(rename = "RemoveChild")]
    RemoveChild,

    #[serde(rename = "Shutdown")]
    Shutdown,


}

impl Default for Operation {
    fn default() -> Self {
        Self::Create
    }
}






