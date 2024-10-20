#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// CreatePoolBody : Create Pool Body








/// Create Pool Body

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct CreatePoolBody {

    /// disk device paths or URIs to be claimed by the pool
    #[serde(default, rename = "disks")]
    pub disks: Vec<String>,

    /// labels to be set on the pools
    #[serde(default, rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,

}

impl CreatePoolBody {
    /// CreatePoolBody using only the required fields
    pub fn new(disks: impl IntoVec<String>) -> CreatePoolBody {
        CreatePoolBody {
            disks: disks.into_vec(),
            labels: None,
            
        }
    }
    /// CreatePoolBody using all fields
    pub fn new_all(disks: impl IntoVec<String>, labels: impl Into<Option<::std::collections::HashMap<String, String>>>) -> CreatePoolBody {
        CreatePoolBody {
            disks: disks.into_vec(),
            labels: labels.into(),
            
        }
    }
}












