#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// VolumeTarget : Specification of a volume target








/// Specification of a volume target

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct VolumeTarget {

    /// The node where front-end IO will be sent to
    #[serde(default, rename = "node")]
    pub node: String,

    /// Volume Share Protocol
    #[serde(default, rename = "protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<crate::models::VolumeShareProtocol>,

    /// The nodes where the front-end workload resides. If the workload moves then the volume must be republished.
    #[serde(default, rename = "frontend_nodes", skip_serializing_if = "Option::is_none")]
    pub frontend_nodes: Option<Vec<crate::models::NodeAccessInfo>>,

}

impl VolumeTarget {
    /// VolumeTarget using only the required fields
    pub fn new(node: impl Into<String>) -> VolumeTarget {
        VolumeTarget {
            node: node.into(),
            protocol: None,
            frontend_nodes: None,
            
        }
    }
    /// VolumeTarget using all fields
    pub fn new_all(node: impl Into<String>, protocol: impl Into<Option<crate::models::VolumeShareProtocol>>, frontend_nodes: impl IntoOptVec<crate::models::NodeAccessInfo>) -> VolumeTarget {
        VolumeTarget {
            node: node.into(),
            protocol: protocol.into(),
            frontend_nodes: frontend_nodes.into_opt_vec(),
            
        }
    }
}














