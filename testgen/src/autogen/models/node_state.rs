#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// NodeState : io-engine storage node information








/// io-engine storage node information

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct NodeState {

    /// gRPC endpoint of the io-engine instance
    #[serde(default, rename = "grpcEndpoint")]
    pub grpc_endpoint: String,

    /// storage node identifier
    #[serde(default, rename = "id")]
    pub id: String,

    /// deemed state of the node
    #[serde(default, rename = "status")]
    pub status: crate::models::NodeStatus,

    /// NVMe Qualified Names (NQNs) are used to uniquely describe a host or NVM subsystem for the purposes of identification and authentication
    #[serde(default, rename = "node_nqn", skip_serializing_if = "Option::is_none")]
    pub node_nqn: Option<String>,

}

impl NodeState {
    /// NodeState using only the required fields
    pub fn new(grpc_endpoint: impl Into<String>, id: impl Into<String>, status: impl Into<crate::models::NodeStatus>) -> NodeState {
        NodeState {
            grpc_endpoint: grpc_endpoint.into(),
            id: id.into(),
            status: status.into(),
            node_nqn: None,
            
        }
    }
    /// NodeState using all fields
    pub fn new_all(grpc_endpoint: impl Into<String>, id: impl Into<String>, status: impl Into<crate::models::NodeStatus>, node_nqn: impl Into<Option<String>>) -> NodeState {
        NodeState {
            grpc_endpoint: grpc_endpoint.into(),
            id: id.into(),
            status: status.into(),
            node_nqn: node_nqn.into(),
            
        }
    }
}
















