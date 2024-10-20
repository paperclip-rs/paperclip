#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// ReplicaTopology : Volume Replica information.








/// Volume Replica information.

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReplicaTopology {

    /// storage node identifier
    #[serde(default, rename = "node", skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,

    /// storage pool identifier
    #[serde(default, rename = "pool", skip_serializing_if = "Option::is_none")]
    pub pool: Option<String>,

    /// state of the replica
    #[serde(default, rename = "state")]
    pub state: crate::models::ReplicaState,

    /// State of a Nexus Child
    #[serde(default, rename = "child-status", skip_serializing_if = "Option::is_none")]
    pub child_status: Option<crate::models::ChildState>,

    /// Reason for the state of a Nexus Child
    #[serde(default, rename = "child-status-reason", skip_serializing_if = "Option::is_none")]
    pub child_status_reason: Option<crate::models::ChildStateReason>,

    /// Replica space usage information. Useful for capacity management, eg: figure out how much of a thin-provisioned replica is allocated. 
    #[serde(default, rename = "usage", skip_serializing_if = "Option::is_none")]
    pub usage: Option<crate::models::ReplicaUsage>,

    /// current rebuild progress (%)
    #[serde(default, rename = "rebuild-progress", skip_serializing_if = "Option::is_none")]
    pub rebuild_progress: Option<usize>,

}

impl ReplicaTopology {
    /// ReplicaTopology using only the required fields
    pub fn new(state: impl Into<crate::models::ReplicaState>) -> ReplicaTopology {
        ReplicaTopology {
            node: None,
            pool: None,
            state: state.into(),
            child_status: None,
            child_status_reason: None,
            usage: None,
            rebuild_progress: None,
            
        }
    }
    /// ReplicaTopology using all fields
    pub fn new_all(node: impl Into<Option<String>>, pool: impl Into<Option<String>>, state: impl Into<crate::models::ReplicaState>, child_status: impl Into<Option<crate::models::ChildState>>, child_status_reason: impl Into<Option<crate::models::ChildStateReason>>, usage: impl Into<Option<crate::models::ReplicaUsage>>, rebuild_progress: impl Into<Option<usize>>) -> ReplicaTopology {
        ReplicaTopology {
            node: node.into(),
            pool: pool.into(),
            state: state.into(),
            child_status: child_status.into(),
            child_status_reason: child_status_reason.into(),
            usage: usage.into(),
            rebuild_progress: rebuild_progress.into(),
            
        }
    }
}






















