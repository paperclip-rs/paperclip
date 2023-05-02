#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// SnapshotAsSource : The snapshot source for the volume content.








/// The snapshot source for the volume content.

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct SnapshotAsSource {

    
    #[serde(default, rename = "snapshot")]
    pub snapshot: uuid::Uuid,

    
    #[serde(default, rename = "volume")]
    pub volume: uuid::Uuid,

}

impl SnapshotAsSource {
    /// SnapshotAsSource using only the required fields
    pub fn new(snapshot: impl Into<uuid::Uuid>, volume: impl Into<uuid::Uuid>) -> SnapshotAsSource {
        SnapshotAsSource {
            snapshot: snapshot.into(),
            volume: volume.into(),
            
        }
    }
    /// SnapshotAsSource using all fields
    pub fn new_all(snapshot: impl Into<uuid::Uuid>, volume: impl Into<uuid::Uuid>) -> SnapshotAsSource {
        SnapshotAsSource {
            snapshot: snapshot.into(),
            volume: volume.into(),
            
        }
    }
}












