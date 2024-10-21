#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// ReplicaSnapshotState : Replica Snapshot state information.








/// Replica Snapshot state information.



#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ReplicaSnapshotState {

    /// Online ReplicaSnapshotState representation.
    #[serde(rename = "online")]
    online(crate::models::OnlineReplicaSnapshotState),

    /// Offline ReplicaSnapshotState representation.
    #[serde(rename = "offline")]
    offline(crate::models::OfflineReplicaSnapshotState),

}












