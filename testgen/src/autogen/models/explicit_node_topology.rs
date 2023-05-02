#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// ExplicitNodeTopology : volume topology, explicitly selected








/// volume topology, explicitly selected

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct ExplicitNodeTopology {

    /// replicas can only be placed on these nodes
    #[serde(default, rename = "allowed_nodes")]
    pub allowed_nodes: Vec<String>,

    /// preferred nodes to place the replicas
    #[serde(default, rename = "preferred_nodes")]
    pub preferred_nodes: Vec<String>,

}

impl ExplicitNodeTopology {
    /// ExplicitNodeTopology using only the required fields
    pub fn new(allowed_nodes: impl IntoVec<String>, preferred_nodes: impl IntoVec<String>) -> ExplicitNodeTopology {
        ExplicitNodeTopology {
            allowed_nodes: allowed_nodes.into_vec(),
            preferred_nodes: preferred_nodes.into_vec(),
            
        }
    }
    /// ExplicitNodeTopology using all fields
    pub fn new_all(allowed_nodes: impl IntoVec<String>, preferred_nodes: impl IntoVec<String>) -> ExplicitNodeTopology {
        ExplicitNodeTopology {
            allowed_nodes: allowed_nodes.into_vec(),
            preferred_nodes: preferred_nodes.into_vec(),
            
        }
    }
}












