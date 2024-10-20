#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// Topology : node and pool topology for volumes








/// node and pool topology for volumes

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Topology {

    /// Used to determine how to place/distribute the data during volume creation and replica replacement.  If left empty then the control plane will select from all available resources.
    #[serde(default, rename = "node_topology", skip_serializing_if = "Option::is_none")]
    pub node_topology: Option<crate::models::NodeTopology>,

    /// Used to determine how to place/distribute the data during volume creation and replica replacement.  If left empty then the control plane will select from all available resources.
    #[serde(default, rename = "pool_topology", skip_serializing_if = "Option::is_none")]
    pub pool_topology: Option<crate::models::PoolTopology>,

}

impl Topology {
    /// Topology using only the required fields
    pub fn new() -> Topology {
        Topology {
            node_topology: None,
            pool_topology: None,
            
        }
    }
    /// Topology using all fields
    pub fn new_all(node_topology: impl Into<Option<crate::models::NodeTopology>>, pool_topology: impl Into<Option<crate::models::PoolTopology>>) -> Topology {
        Topology {
            node_topology: node_topology.into(),
            pool_topology: pool_topology.into(),
            
        }
    }
}












