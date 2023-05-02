#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// PoolSpec : User specification of a pool.








/// User specification of a pool.

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct PoolSpec {

    /// absolute disk paths claimed by the pool
    #[serde(default, rename = "disks")]
    pub disks: Vec<String>,

    /// storage pool identifier
    #[serde(default, rename = "id")]
    pub id: String,

    /// labels to be set on the pools
    #[serde(default, rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,

    /// storage node identifier
    #[serde(default, rename = "node")]
    pub node: String,

    /// Common base state for a resource
    #[serde(default, rename = "status")]
    pub status: crate::models::SpecStatus,

}

impl PoolSpec {
    /// PoolSpec using only the required fields
    pub fn new(disks: impl IntoVec<String>, id: impl Into<String>, node: impl Into<String>, status: impl Into<crate::models::SpecStatus>) -> PoolSpec {
        PoolSpec {
            disks: disks.into_vec(),
            id: id.into(),
            labels: None,
            node: node.into(),
            status: status.into(),
            
        }
    }
    /// PoolSpec using all fields
    pub fn new_all(disks: impl IntoVec<String>, id: impl Into<String>, labels: impl Into<Option<::std::collections::HashMap<String, String>>>, node: impl Into<String>, status: impl Into<crate::models::SpecStatus>) -> PoolSpec {
        PoolSpec {
            disks: disks.into_vec(),
            id: id.into(),
            labels: labels.into(),
            node: node.into(),
            status: status.into(),
            
        }
    }
}


















