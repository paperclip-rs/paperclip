#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// CreateVolumeBody : Create Volume Body








/// Create Volume Body

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct CreateVolumeBody {

    /// Volume policy used to determine if and how to replace a replica
    #[serde(default, rename = "policy")]
    pub policy: crate::models::VolumePolicy,

    /// number of storage replicas
    #[serde(default, rename = "replicas")]
    pub replicas: u8,

    /// size of the volume in bytes
    #[serde(default, rename = "size")]
    pub size: u64,

    /// flag indicating whether or not the volume is thin provisioned
    #[serde(default, rename = "thin")]
    pub thin: bool,

    /// node and pool topology for volumes
    #[serde(default, rename = "topology", skip_serializing_if = "Option::is_none")]
    pub topology: Option<crate::models::Topology>,

    /// Optionally used to store custom volume information
    #[serde(default, rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,

    /// Affinity Group related information.
    #[serde(default, rename = "affinity_group", skip_serializing_if = "Option::is_none")]
    pub affinity_group: Option<crate::models::AffinityGroup>,

    /// Max Snapshots limit per volume.
    #[serde(default, rename = "max_snapshots", skip_serializing_if = "Option::is_none")]
    pub max_snapshots: Option<u32>,

}

impl CreateVolumeBody {
    /// CreateVolumeBody using only the required fields
    pub fn new(policy: impl Into<crate::models::VolumePolicy>, replicas: impl Into<u8>, size: impl Into<u64>, thin: impl Into<bool>) -> CreateVolumeBody {
        CreateVolumeBody {
            policy: policy.into(),
            replicas: replicas.into(),
            size: size.into(),
            thin: thin.into(),
            topology: None,
            labels: None,
            affinity_group: None,
            max_snapshots: None,
            
        }
    }
    /// CreateVolumeBody using all fields
    pub fn new_all(policy: impl Into<crate::models::VolumePolicy>, replicas: impl Into<u8>, size: impl Into<u64>, thin: impl Into<bool>, topology: impl Into<Option<crate::models::Topology>>, labels: impl Into<Option<::std::collections::HashMap<String, String>>>, affinity_group: impl Into<Option<crate::models::AffinityGroup>>, max_snapshots: impl Into<Option<u32>>) -> CreateVolumeBody {
        CreateVolumeBody {
            policy: policy.into(),
            replicas: replicas.into(),
            size: size.into(),
            thin: thin.into(),
            topology: topology.into(),
            labels: labels.into(),
            affinity_group: affinity_group.into(),
            max_snapshots: max_snapshots.into(),
            
        }
    }
}
























