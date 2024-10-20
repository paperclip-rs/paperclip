#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// VolumeSnapshotSpec : Volume Snapshot Spec information.








/// Volume Snapshot Spec information.

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct VolumeSnapshotSpec {

    
    #[serde(default, rename = "uuid")]
    pub uuid: uuid::Uuid,

    
    #[serde(default, rename = "source_volume")]
    pub source_volume: uuid::Uuid,

}

impl VolumeSnapshotSpec {
    /// VolumeSnapshotSpec using only the required fields
    pub fn new(uuid: impl Into<uuid::Uuid>, source_volume: impl Into<uuid::Uuid>) -> VolumeSnapshotSpec {
        VolumeSnapshotSpec {
            uuid: uuid.into(),
            source_volume: source_volume.into(),
            
        }
    }
    /// VolumeSnapshotSpec using all fields
    pub fn new_all(uuid: impl Into<uuid::Uuid>, source_volume: impl Into<uuid::Uuid>) -> VolumeSnapshotSpec {
        VolumeSnapshotSpec {
            uuid: uuid.into(),
            source_volume: source_volume.into(),
            
        }
    }
}












