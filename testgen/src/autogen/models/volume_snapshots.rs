#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// VolumeSnapshots : Array of volume snapshots plus the next token for subsequent get requests when using pagination.








/// Array of volume snapshots plus the next token for subsequent get requests when using pagination.

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct VolumeSnapshots {

    
    #[serde(default, rename = "entries")]
    pub entries: Vec<crate::models::VolumeSnapshot>,

    
    #[serde(default, rename = "next_token", skip_serializing_if = "Option::is_none")]
    pub next_token: Option<isize>,

}

impl VolumeSnapshots {
    /// VolumeSnapshots using only the required fields
    pub fn new(entries: impl IntoVec<crate::models::VolumeSnapshot>) -> VolumeSnapshots {
        VolumeSnapshots {
            entries: entries.into_vec(),
            next_token: None,
            
        }
    }
    /// VolumeSnapshots using all fields
    pub fn new_all(entries: impl IntoVec<crate::models::VolumeSnapshot>, next_token: impl Into<Option<isize>>) -> VolumeSnapshots {
        VolumeSnapshots {
            entries: entries.into_vec(),
            next_token: next_token.into(),
            
        }
    }
}












