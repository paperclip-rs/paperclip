#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// VolumePolicy : Volume policy used to determine if and how to replace a replica








/// Volume policy used to determine if and how to replace a replica

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct VolumePolicy {

    /// If true the control plane will attempt to heal the volume by itself
    #[serde(default, rename = "self_heal")]
    pub self_heal: bool,

}

impl VolumePolicy {
    /// VolumePolicy using only the required fields
    pub fn new(self_heal: impl Into<bool>) -> VolumePolicy {
        VolumePolicy {
            self_heal: self_heal.into(),
            
        }
    }
    /// VolumePolicy using all fields
    pub fn new_all(self_heal: impl Into<bool>) -> VolumePolicy {
        VolumePolicy {
            self_heal: self_heal.into(),
            
        }
    }
}










