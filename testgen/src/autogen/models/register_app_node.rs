#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};














#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct RegisterAppNode {

    /// gRPC server endpoint of the app node.
    #[serde(default, rename = "endpoint")]
    pub endpoint: String,

    /// Labels to be set on the app node.
    #[serde(default, rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,

}

impl RegisterAppNode {
    /// RegisterAppNode using only the required fields
    pub fn new(endpoint: impl Into<String>) -> RegisterAppNode {
        RegisterAppNode {
            endpoint: endpoint.into(),
            labels: None,
            
        }
    }
    /// RegisterAppNode using all fields
    pub fn new_all(endpoint: impl Into<String>, labels: impl Into<Option<::std::collections::HashMap<String, String>>>) -> RegisterAppNode {
        RegisterAppNode {
            endpoint: endpoint.into(),
            labels: labels.into(),
            
        }
    }
}












