#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// CordonedState : The item is cordoned








/// The item is cordoned

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct CordonedState {

    
    #[serde(default, rename = "cordonlabels")]
    pub cordonlabels: Vec<String>,

}

impl CordonedState {
    /// CordonedState using only the required fields
    pub fn new(cordonlabels: impl IntoVec<String>) -> CordonedState {
        CordonedState {
            cordonlabels: cordonlabels.into_vec(),
            
        }
    }
    /// CordonedState using all fields
    pub fn new_all(cordonlabels: impl IntoVec<String>) -> CordonedState {
        CordonedState {
            cordonlabels: cordonlabels.into_vec(),
            
        }
    }
}










