#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// Node : io-engine storage node information








/// io-engine storage node information

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Node {

    /// storage node identifier
    #[serde(default, rename = "id")]
    pub id: String,

    /// Node spec
    #[serde(default, rename = "spec", skip_serializing_if = "Option::is_none")]
    pub spec: Option<crate::models::NodeSpec>,

    /// io-engine storage node information
    #[serde(default, rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<crate::models::NodeState>,

}

impl Node {
    /// Node using only the required fields
    pub fn new(id: impl Into<String>) -> Node {
        Node {
            id: id.into(),
            spec: None,
            state: None,
            
        }
    }
    /// Node using all fields
    pub fn new_all(id: impl Into<String>, spec: impl Into<Option<crate::models::NodeSpec>>, state: impl Into<Option<crate::models::NodeState>>) -> Node {
        Node {
            id: id.into(),
            spec: spec.into(),
            state: state.into(),
            
        }
    }
}














