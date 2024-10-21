#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// RebuildRecord : Rebuild record of a Child








/// Rebuild record of a Child

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct RebuildRecord {

    /// Uri of the rebuilding child
    #[serde(default, rename = "childUri")]
    pub child_uri: String,

    /// Uri of source child for rebuild job
    #[serde(default, rename = "srcUri")]
    pub src_uri: String,

    /// State of the rebuild job
    #[serde(default, rename = "rebuildJobState")]
    pub rebuild_job_state: crate::models::RebuildJobState,

    /// Total blocks to rebuild
    #[serde(default, rename = "blocksTotal")]
    pub blocks_total: u64,

    /// Number of blocks processed
    #[serde(default, rename = "blocksRecovered")]
    pub blocks_recovered: u64,

    /// Number of blocks to transferred
    #[serde(default, rename = "blocksTransferred")]
    pub blocks_transferred: u64,

    /// Number of blocks remaining
    #[serde(default, rename = "blocksRemaining")]
    pub blocks_remaining: u64,

    /// Size of each block in the task
    #[serde(default, rename = "blockSize")]
    pub block_size: u64,

    /// True means its Partial rebuild job. If false, its Full rebuild job
    #[serde(default, rename = "isPartial")]
    pub is_partial: bool,

    /// Start time of the rebuild job (UTC)
    #[serde(default, rename = "startTime")]
    pub start_time: String,

    /// End time of the rebuild job (UTC)
    #[serde(default, rename = "endTime")]
    pub end_time: String,

}

impl RebuildRecord {
    /// RebuildRecord using only the required fields
    pub fn new(child_uri: impl Into<String>, src_uri: impl Into<String>, rebuild_job_state: impl Into<crate::models::RebuildJobState>, blocks_total: impl Into<u64>, blocks_recovered: impl Into<u64>, blocks_transferred: impl Into<u64>, blocks_remaining: impl Into<u64>, block_size: impl Into<u64>, is_partial: impl Into<bool>, start_time: impl Into<String>, end_time: impl Into<String>) -> RebuildRecord {
        RebuildRecord {
            child_uri: child_uri.into(),
            src_uri: src_uri.into(),
            rebuild_job_state: rebuild_job_state.into(),
            blocks_total: blocks_total.into(),
            blocks_recovered: blocks_recovered.into(),
            blocks_transferred: blocks_transferred.into(),
            blocks_remaining: blocks_remaining.into(),
            block_size: block_size.into(),
            is_partial: is_partial.into(),
            start_time: start_time.into(),
            end_time: end_time.into(),
            
        }
    }
    /// RebuildRecord using all fields
    pub fn new_all(child_uri: impl Into<String>, src_uri: impl Into<String>, rebuild_job_state: impl Into<crate::models::RebuildJobState>, blocks_total: impl Into<u64>, blocks_recovered: impl Into<u64>, blocks_transferred: impl Into<u64>, blocks_remaining: impl Into<u64>, block_size: impl Into<u64>, is_partial: impl Into<bool>, start_time: impl Into<String>, end_time: impl Into<String>) -> RebuildRecord {
        RebuildRecord {
            child_uri: child_uri.into(),
            src_uri: src_uri.into(),
            rebuild_job_state: rebuild_job_state.into(),
            blocks_total: blocks_total.into(),
            blocks_recovered: blocks_recovered.into(),
            blocks_transferred: blocks_transferred.into(),
            blocks_remaining: blocks_remaining.into(),
            block_size: block_size.into(),
            is_partial: is_partial.into(),
            start_time: start_time.into(),
            end_time: end_time.into(),
            
        }
    }
}






























