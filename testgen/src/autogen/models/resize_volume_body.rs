#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// ResizeVolumeBody : Resize Volume Body








/// Resize Volume Body

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResizeVolumeBody {

    /// New required size of the volume in bytes
    #[serde(default, rename = "size")]
    pub size: u64,

}

impl ResizeVolumeBody {
    /// ResizeVolumeBody using only the required fields
    pub fn new(size: impl Into<u64>) -> ResizeVolumeBody {
        ResizeVolumeBody {
            size: size.into(),
            
        }
    }
    /// ResizeVolumeBody using all fields
    pub fn new_all(size: impl Into<u64>) -> ResizeVolumeBody {
        ResizeVolumeBody {
            size: size.into(),
            
        }
    }
}










