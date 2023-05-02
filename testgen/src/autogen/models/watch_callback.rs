#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// WatchCallback : Watch Callbacks








/// Watch Callbacks



#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum WatchCallback {

    
    #[serde(rename = "uri")]
    uri(String),

}










