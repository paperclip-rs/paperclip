#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// DrainState : The item is draining








/// The item is draining

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct DrainState {

    
    #[serde(default, rename = "cordonlabels")]
    pub cordonlabels: Vec<String>,

    
    #[serde(default, rename = "drainlabels")]
    pub drainlabels: Vec<String>,

}

impl DrainState {
    /// DrainState using only the required fields
    pub fn new(cordonlabels: impl IntoVec<String>, drainlabels: impl IntoVec<String>) -> DrainState {
        DrainState {
            cordonlabels: cordonlabels.into_vec(),
            drainlabels: drainlabels.into_vec(),
            
        }
    }
    /// DrainState using all fields
    pub fn new_all(cordonlabels: impl IntoVec<String>, drainlabels: impl IntoVec<String>) -> DrainState {
        DrainState {
            cordonlabels: cordonlabels.into_vec(),
            drainlabels: drainlabels.into_vec(),
            
        }
    }
}












