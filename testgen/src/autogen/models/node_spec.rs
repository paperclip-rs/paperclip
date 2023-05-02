#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// NodeSpec : Node spec








/// Node spec

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct NodeSpec {

    /// gRPC endpoint of the io-engine instance
    #[serde(default, rename = "grpcEndpoint")]
    pub grpc_endpoint: String,

    /// storage node identifier
    #[serde(default, rename = "id")]
    pub id: String,

    /// labels to be set on the node
    #[serde(default, rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,

    /// the drain state
    #[serde(default, rename = "cordondrainstate", skip_serializing_if = "Option::is_none")]
    pub cordondrainstate: Option<crate::models::CordonDrainState>,

    /// NVMe Qualified Names (NQNs) are used to uniquely describe a host or NVM subsystem for the purposes of identification and authentication
    #[serde(default, rename = "node_nqn", skip_serializing_if = "Option::is_none")]
    pub node_nqn: Option<String>,

}

impl NodeSpec {
    /// NodeSpec using only the required fields
    pub fn new(grpc_endpoint: impl Into<String>, id: impl Into<String>) -> NodeSpec {
        NodeSpec {
            grpc_endpoint: grpc_endpoint.into(),
            id: id.into(),
            labels: None,
            cordondrainstate: None,
            node_nqn: None,
            
        }
    }
    /// NodeSpec using all fields
    pub fn new_all(grpc_endpoint: impl Into<String>, id: impl Into<String>, labels: impl Into<Option<::std::collections::HashMap<String, String>>>, cordondrainstate: impl Into<Option<crate::models::CordonDrainState>>, node_nqn: impl Into<Option<String>>) -> NodeSpec {
        NodeSpec {
            grpc_endpoint: grpc_endpoint.into(),
            id: id.into(),
            labels: labels.into(),
            cordondrainstate: cordondrainstate.into(),
            node_nqn: node_nqn.into(),
            
        }
    }
}


















