#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// Child : Child information








/// Child information

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Child {

    /// current rebuild progress (%)
    #[serde(default, rename = "rebuildProgress", skip_serializing_if = "Option::is_none")]
    pub rebuild_progress: Option<usize>,

    /// State of a Nexus Child
    #[serde(default, rename = "state")]
    pub state: crate::models::ChildState,

    /// Reason for the state of a Nexus Child
    #[serde(default, rename = "state_reason", skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<crate::models::ChildStateReason>,

    /// uri of the child device
    #[serde(default, rename = "uri")]
    pub uri: String,

}

impl Child {
    /// Child using only the required fields
    pub fn new(state: impl Into<crate::models::ChildState>, uri: impl Into<String>) -> Child {
        Child {
            rebuild_progress: None,
            state: state.into(),
            state_reason: None,
            uri: uri.into(),
            
        }
    }
    /// Child using all fields
    pub fn new_all(rebuild_progress: impl Into<Option<usize>>, state: impl Into<crate::models::ChildState>, state_reason: impl Into<Option<crate::models::ChildStateReason>>, uri: impl Into<String>) -> Child {
        Child {
            rebuild_progress: rebuild_progress.into(),
            state: state.into(),
            state_reason: state_reason.into(),
            uri: uri.into(),
            
        }
    }
}
















