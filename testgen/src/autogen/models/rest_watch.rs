#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// RestWatch : Watch Resource in the store








/// Watch Resource in the store

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct RestWatch {

    /// callback used to notify the watch of a change
    #[serde(default, rename = "callback")]
    pub callback: String,

    /// id of the resource to watch on
    #[serde(default, rename = "resource")]
    pub resource: String,

}

impl RestWatch {
    /// RestWatch using only the required fields
    pub fn new(callback: impl Into<String>, resource: impl Into<String>) -> RestWatch {
        RestWatch {
            callback: callback.into(),
            resource: resource.into(),
            
        }
    }
    /// RestWatch using all fields
    pub fn new_all(callback: impl Into<String>, resource: impl Into<String>) -> RestWatch {
        RestWatch {
            callback: callback.into(),
            resource: resource.into(),
            
        }
    }
}












