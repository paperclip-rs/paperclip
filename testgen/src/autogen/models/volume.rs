#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// Volume : Volumes Volume information








/// Volumes Volume information

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Volume {

    /// User specification of a volume.
    #[serde(default, rename = "spec")]
    pub spec: crate::models::VolumeSpec,

    /// Runtime state of the volume
    #[serde(default, rename = "state")]
    pub state: crate::models::VolumeState,

}

impl Volume {
    /// Volume using only the required fields
    pub fn new(spec: impl Into<crate::models::VolumeSpec>, state: impl Into<crate::models::VolumeState>) -> Volume {
        Volume {
            spec: spec.into(),
            state: state.into(),
            
        }
    }
    /// Volume using all fields
    pub fn new_all(spec: impl Into<crate::models::VolumeSpec>, state: impl Into<crate::models::VolumeState>) -> Volume {
        Volume {
            spec: spec.into(),
            state: state.into(),
            
        }
    }
}












