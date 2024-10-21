#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// VolumeSnapshotDefinition : Volume Snapshot Metadata and Spec information.








/// Volume Snapshot Metadata and Spec information.

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct VolumeSnapshotDefinition {

    /// Volume Snapshot Metadata information.
    #[serde(default, rename = "metadata")]
    pub metadata: crate::models::VolumeSnapshotMetadata,

    /// Volume Snapshot Spec information.
    #[serde(default, rename = "spec")]
    pub spec: crate::models::VolumeSnapshotSpec,

}

impl VolumeSnapshotDefinition {
    /// VolumeSnapshotDefinition using only the required fields
    pub fn new(metadata: impl Into<crate::models::VolumeSnapshotMetadata>, spec: impl Into<crate::models::VolumeSnapshotSpec>) -> VolumeSnapshotDefinition {
        VolumeSnapshotDefinition {
            metadata: metadata.into(),
            spec: spec.into(),
            
        }
    }
    /// VolumeSnapshotDefinition using all fields
    pub fn new_all(metadata: impl Into<crate::models::VolumeSnapshotMetadata>, spec: impl Into<crate::models::VolumeSnapshotSpec>) -> VolumeSnapshotDefinition {
        VolumeSnapshotDefinition {
            metadata: metadata.into(),
            spec: spec.into(),
            
        }
    }
}












