#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// AppNode : Represents an application node, which connects to the target node via a share protocol (eg: nvmf).








/// Represents an application node, which connects to the target node via a share protocol (eg: nvmf).

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct AppNode {

    /// App node identifier.
    #[serde(default, rename = "id")]
    pub id: String,

    /// App node attributes.
    #[serde(default, rename = "spec")]
    pub spec: crate::models::AppNodeSpec,

    /// Deemed state of the app node.
    #[serde(default, rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<crate::models::AppNodeState>,

}

impl AppNode {
    /// AppNode using only the required fields
    pub fn new(id: impl Into<String>, spec: impl Into<crate::models::AppNodeSpec>) -> AppNode {
        AppNode {
            id: id.into(),
            spec: spec.into(),
            state: None,
            
        }
    }
    /// AppNode using all fields
    pub fn new_all(id: impl Into<String>, spec: impl Into<crate::models::AppNodeSpec>, state: impl Into<Option<crate::models::AppNodeState>>) -> AppNode {
        AppNode {
            id: id.into(),
            spec: spec.into(),
            state: state.into(),
            
        }
    }
}














