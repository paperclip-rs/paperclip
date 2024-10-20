#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// PoolState : State of a pool, as reported by io-engine








/// State of a pool, as reported by io-engine

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct PoolState {

    /// size of the pool in bytes
    #[serde(default, rename = "capacity")]
    pub capacity: u64,

    /// absolute disk paths claimed by the pool
    #[serde(default, rename = "disks")]
    pub disks: Vec<String>,

    /// storage pool identifier
    #[serde(default, rename = "id")]
    pub id: String,

    /// storage node identifier
    #[serde(default, rename = "node")]
    pub node: String,

    /// current status of the pool
    #[serde(default, rename = "status")]
    pub status: crate::models::PoolStatus,

    /// used bytes from the pool
    #[serde(default, rename = "used")]
    pub used: u64,

    /// accrued size of all replicas contained in this pool
    #[serde(default, rename = "committed", skip_serializing_if = "Option::is_none")]
    pub committed: Option<u64>,

}

impl PoolState {
    /// PoolState using only the required fields
    pub fn new(capacity: impl Into<u64>, disks: impl IntoVec<String>, id: impl Into<String>, node: impl Into<String>, status: impl Into<crate::models::PoolStatus>, used: impl Into<u64>) -> PoolState {
        PoolState {
            capacity: capacity.into(),
            disks: disks.into_vec(),
            id: id.into(),
            node: node.into(),
            status: status.into(),
            used: used.into(),
            committed: None,
            
        }
    }
    /// PoolState using all fields
    pub fn new_all(capacity: impl Into<u64>, disks: impl IntoVec<String>, id: impl Into<String>, node: impl Into<String>, status: impl Into<crate::models::PoolStatus>, used: impl Into<u64>, committed: impl Into<Option<u64>>) -> PoolState {
        PoolState {
            capacity: capacity.into(),
            disks: disks.into_vec(),
            id: id.into(),
            node: node.into(),
            status: status.into(),
            used: used.into(),
            committed: committed.into(),
            
        }
    }
}






















