#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// VolumeSpec : User specification of a volume.








/// User specification of a volume.

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct VolumeSpec {

    /// Optionally used to store custom volume information
    #[serde(default, rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,

    /// Number of children the volume should have.
    #[serde(default, rename = "num_replicas")]
    pub num_replicas: u8,

    /// Record of the operation in progress
    #[serde(default, rename = "operation", skip_serializing_if = "Option::is_none")]
    pub operation: Option<crate::models::VolumeSpecOperation>,

    /// Size that the volume should be.
    #[serde(default, rename = "size")]
    pub size: u64,

    /// Common base state for a resource
    #[serde(default, rename = "status")]
    pub status: crate::models::SpecStatus,

    /// Specification of a volume target
    #[serde(default, rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: Option<crate::models::VolumeTarget>,

    /// Volume Id
    #[serde(default, rename = "uuid")]
    pub uuid: uuid::Uuid,

    /// node and pool topology for volumes
    #[serde(default, rename = "topology", skip_serializing_if = "Option::is_none")]
    pub topology: Option<crate::models::Topology>,

    /// Volume policy used to determine if and how to replace a replica
    #[serde(default, rename = "policy")]
    pub policy: crate::models::VolumePolicy,

    /// Thin provisioning flag.
    #[serde(default, rename = "thin")]
    pub thin: bool,

    /// Volume converted to thin provisioned.
    #[serde(default, rename = "as_thin", skip_serializing_if = "Option::is_none")]
    pub as_thin: Option<bool>,

    /// Affinity Group related information.
    #[serde(default, rename = "affinity_group", skip_serializing_if = "Option::is_none")]
    pub affinity_group: Option<crate::models::AffinityGroup>,

    /// Volume Content Source i.e the snapshot or the volume.
    #[serde(default, rename = "content_source", skip_serializing_if = "Option::is_none")]
    pub content_source: Option<crate::models::VolumeContentSource>,

    /// Number of snapshots taken on this volume.
    #[serde(default, rename = "num_snapshots")]
    pub num_snapshots: u32,

    /// Max snapshots to limit per volume.
    #[serde(default, rename = "max_snapshots", skip_serializing_if = "Option::is_none")]
    pub max_snapshots: Option<u32>,

}

impl VolumeSpec {
    /// VolumeSpec using only the required fields
    pub fn new(num_replicas: impl Into<u8>, size: impl Into<u64>, status: impl Into<crate::models::SpecStatus>, uuid: impl Into<uuid::Uuid>, policy: impl Into<crate::models::VolumePolicy>, thin: impl Into<bool>, num_snapshots: impl Into<u32>) -> VolumeSpec {
        VolumeSpec {
            labels: None,
            num_replicas: num_replicas.into(),
            operation: None,
            size: size.into(),
            status: status.into(),
            target: None,
            uuid: uuid.into(),
            topology: None,
            policy: policy.into(),
            thin: thin.into(),
            as_thin: None,
            affinity_group: None,
            content_source: None,
            num_snapshots: num_snapshots.into(),
            max_snapshots: None,
            
        }
    }
    /// VolumeSpec using all fields
    pub fn new_all(labels: impl Into<Option<::std::collections::HashMap<String, String>>>, num_replicas: impl Into<u8>, operation: impl Into<Option<crate::models::VolumeSpecOperation>>, size: impl Into<u64>, status: impl Into<crate::models::SpecStatus>, target: impl Into<Option<crate::models::VolumeTarget>>, uuid: impl Into<uuid::Uuid>, topology: impl Into<Option<crate::models::Topology>>, policy: impl Into<crate::models::VolumePolicy>, thin: impl Into<bool>, as_thin: impl Into<Option<bool>>, affinity_group: impl Into<Option<crate::models::AffinityGroup>>, content_source: impl Into<Option<crate::models::VolumeContentSource>>, num_snapshots: impl Into<u32>, max_snapshots: impl Into<Option<u32>>) -> VolumeSpec {
        VolumeSpec {
            labels: labels.into(),
            num_replicas: num_replicas.into(),
            operation: operation.into(),
            size: size.into(),
            status: status.into(),
            target: target.into(),
            uuid: uuid.into(),
            topology: topology.into(),
            policy: policy.into(),
            thin: thin.into(),
            as_thin: as_thin.into(),
            affinity_group: affinity_group.into(),
            content_source: content_source.into(),
            num_snapshots: num_snapshots.into(),
            max_snapshots: max_snapshots.into(),
            
        }
    }
}






































