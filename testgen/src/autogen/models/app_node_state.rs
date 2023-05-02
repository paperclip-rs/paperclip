#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// AppNodeState : Deemed state of the app node.








/// Deemed state of the app node.

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct AppNodeState {

    /// App node identifier.
    #[serde(default, rename = "id")]
    pub id: String,

    /// gRPC server endpoint of the app node.
    #[serde(default, rename = "endpoint")]
    pub endpoint: String,

    /// Deemed Status of the app node.
    #[serde(default, rename = "status")]
    pub status: Status,

}

impl AppNodeState {
    /// AppNodeState using only the required fields
    pub fn new(id: impl Into<String>, endpoint: impl Into<String>, status: impl Into<Status>) -> AppNodeState {
        AppNodeState {
            id: id.into(),
            endpoint: endpoint.into(),
            status: status.into(),
            
        }
    }
    /// AppNodeState using all fields
    pub fn new_all(id: impl Into<String>, endpoint: impl Into<String>, status: impl Into<Status>) -> AppNodeState {
        AppNodeState {
            id: id.into(),
            endpoint: endpoint.into(),
            status: status.into(),
            
        }
    }
}












/// Deemed Status of the app node.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {


    #[serde(rename = "Online")]
    Online,

    #[serde(rename = "Offline")]
    Offline,


}

impl Default for Status {
    fn default() -> Self {
        Self::Online
    }
}




