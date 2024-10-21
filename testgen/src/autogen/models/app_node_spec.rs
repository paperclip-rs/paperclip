#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// AppNodeSpec : App node attributes.








/// App node attributes.

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct AppNodeSpec {

    /// App node identifier.
    #[serde(default, rename = "id")]
    pub id: String,

    /// gRPC server endpoint of the app node.
    #[serde(default, rename = "endpoint")]
    pub endpoint: String,

    /// Labels to be set on the app node.
    #[serde(default, rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,

}

impl AppNodeSpec {
    /// AppNodeSpec using only the required fields
    pub fn new(id: impl Into<String>, endpoint: impl Into<String>) -> AppNodeSpec {
        AppNodeSpec {
            id: id.into(),
            endpoint: endpoint.into(),
            labels: None,
            
        }
    }
    /// AppNodeSpec using all fields
    pub fn new_all(id: impl Into<String>, endpoint: impl Into<String>, labels: impl Into<Option<::std::collections::HashMap<String, String>>>) -> AppNodeSpec {
        AppNodeSpec {
            id: id.into(),
            endpoint: endpoint.into(),
            labels: labels.into(),
            
        }
    }
}














