#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// VolumeSnapshot : Volume Snapshot Information.








/// Volume Snapshot Information.

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct VolumeSnapshot {

    /// Volume Snapshot Metadata and Spec information.
    #[serde(default, rename = "definition")]
    pub definition: crate::models::VolumeSnapshotDefinition,

    /// Volume Snapshot State information.
    #[serde(default, rename = "state")]
    pub state: crate::models::VolumeSnapshotState,

}

impl VolumeSnapshot {
    /// VolumeSnapshot using only the required fields
    pub fn new(definition: impl Into<crate::models::VolumeSnapshotDefinition>, state: impl Into<crate::models::VolumeSnapshotState>) -> VolumeSnapshot {
        VolumeSnapshot {
            definition: definition.into(),
            state: state.into(),
            
        }
    }
    /// VolumeSnapshot using all fields
    pub fn new_all(definition: impl Into<crate::models::VolumeSnapshotDefinition>, state: impl Into<crate::models::VolumeSnapshotState>) -> VolumeSnapshot {
        VolumeSnapshot {
            definition: definition.into(),
            state: state.into(),
            
        }
    }
}












