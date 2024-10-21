#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// VolumeState : Runtime state of the volume








/// Runtime state of the volume

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct VolumeState {

    /// target exposed via a Nexus
    #[serde(default, rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: Option<crate::models::Nexus>,

    /// size of the volume in bytes
    #[serde(default, rename = "size")]
    pub size: u64,

    /// current volume status
    #[serde(default, rename = "status")]
    pub status: crate::models::VolumeStatus,

    /// name of the volume
    #[serde(default, rename = "uuid")]
    pub uuid: uuid::Uuid,

    /// replica topology information
    #[serde(default, rename = "replica_topology")]
    pub replica_topology: ::std::collections::HashMap<String, crate::models::ReplicaTopology>,

    /// Volume space usage
    #[serde(default, rename = "usage", skip_serializing_if = "Option::is_none")]
    pub usage: Option<crate::models::VolumeUsage>,

}

impl VolumeState {
    /// VolumeState using only the required fields
    pub fn new(size: impl Into<u64>, status: impl Into<crate::models::VolumeStatus>, uuid: impl Into<uuid::Uuid>, replica_topology: impl Into<::std::collections::HashMap<String, crate::models::ReplicaTopology>>) -> VolumeState {
        VolumeState {
            target: None,
            size: size.into(),
            status: status.into(),
            uuid: uuid.into(),
            replica_topology: replica_topology.into(),
            usage: None,
            
        }
    }
    /// VolumeState using all fields
    pub fn new_all(target: impl Into<Option<crate::models::Nexus>>, size: impl Into<u64>, status: impl Into<crate::models::VolumeStatus>, uuid: impl Into<uuid::Uuid>, replica_topology: impl Into<::std::collections::HashMap<String, crate::models::ReplicaTopology>>, usage: impl Into<Option<crate::models::VolumeUsage>>) -> VolumeState {
        VolumeState {
            target: target.into(),
            size: size.into(),
            status: status.into(),
            uuid: uuid.into(),
            replica_topology: replica_topology.into(),
            usage: usage.into(),
            
        }
    }
}




















