#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// Nexus : Nexus information








/// Nexus information

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Nexus {

    /// Array of Nexus Children
    #[serde(default, rename = "children")]
    pub children: Vec<crate::models::Child>,

    /// URI of the device for the volume (missing if not published).  Missing property and empty string are treated the same.
    #[serde(default, rename = "deviceUri")]
    pub device_uri: String,

    /// id of the io-engine instance
    #[serde(default, rename = "node")]
    pub node: String,

    /// total number of rebuild tasks
    #[serde(default, rename = "rebuilds")]
    pub rebuilds: u32,

    /// Common Protocol
    #[serde(default, rename = "protocol")]
    pub protocol: crate::models::Protocol,

    /// size of the volume in bytes
    #[serde(default, rename = "size")]
    pub size: u64,

    /// State of the Nexus
    #[serde(default, rename = "state")]
    pub state: crate::models::NexusState,

    /// uuid of the nexus
    #[serde(default, rename = "uuid")]
    pub uuid: uuid::Uuid,

}

impl Nexus {
    /// Nexus using only the required fields
    pub fn new(children: impl IntoVec<crate::models::Child>, device_uri: impl Into<String>, node: impl Into<String>, rebuilds: impl Into<u32>, protocol: impl Into<crate::models::Protocol>, size: impl Into<u64>, state: impl Into<crate::models::NexusState>, uuid: impl Into<uuid::Uuid>) -> Nexus {
        Nexus {
            children: children.into_vec(),
            device_uri: device_uri.into(),
            node: node.into(),
            rebuilds: rebuilds.into(),
            protocol: protocol.into(),
            size: size.into(),
            state: state.into(),
            uuid: uuid.into(),
            
        }
    }
    /// Nexus using all fields
    pub fn new_all(children: impl IntoVec<crate::models::Child>, device_uri: impl Into<String>, node: impl Into<String>, rebuilds: impl Into<u32>, protocol: impl Into<crate::models::Protocol>, size: impl Into<u64>, state: impl Into<crate::models::NexusState>, uuid: impl Into<uuid::Uuid>) -> Nexus {
        Nexus {
            children: children.into_vec(),
            device_uri: device_uri.into(),
            node: node.into(),
            rebuilds: rebuilds.into(),
            protocol: protocol.into(),
            size: size.into(),
            state: state.into(),
            uuid: uuid.into(),
            
        }
    }
}
























