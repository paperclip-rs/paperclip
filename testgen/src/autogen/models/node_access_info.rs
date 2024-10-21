#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// NodeAccessInfo : Frontend Node access information.








/// Frontend Node access information.

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct NodeAccessInfo {

    /// The nodename of the node.
    #[serde(default, rename = "name")]
    pub name: String,

    /// The Nvme Nqn of the node\'s initiator.
    #[serde(default, rename = "nqn")]
    pub nqn: String,

}

impl NodeAccessInfo {
    /// NodeAccessInfo using only the required fields
    pub fn new(name: impl Into<String>, nqn: impl Into<String>) -> NodeAccessInfo {
        NodeAccessInfo {
            name: name.into(),
            nqn: nqn.into(),
            
        }
    }
    /// NodeAccessInfo using all fields
    pub fn new_all(name: impl Into<String>, nqn: impl Into<String>) -> NodeAccessInfo {
        NodeAccessInfo {
            name: name.into(),
            nqn: nqn.into(),
            
        }
    }
}












