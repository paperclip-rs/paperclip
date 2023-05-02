#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// RebuildJobState : State of the rebuild job



/// State of the rebuild job
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RebuildJobState {


    #[serde(rename = "Init")]
    Init,

    #[serde(rename = "Rebuilding")]
    Rebuilding,

    #[serde(rename = "Stopped")]
    Stopped,

    #[serde(rename = "Paused")]
    Paused,

    #[serde(rename = "Failed")]
    Failed,

    #[serde(rename = "Completed")]
    Completed,

}

impl ToString for RebuildJobState {
    fn to_string(&self) -> String {
        match self {
            
            
            Self::Init => String::from("Init"),
            
            Self::Rebuilding => String::from("Rebuilding"),
            
            Self::Stopped => String::from("Stopped"),
            
            Self::Paused => String::from("Paused"),
            
            Self::Failed => String::from("Failed"),
            
            Self::Completed => String::from("Completed"),
            
            
        }
    }
}

impl Default for RebuildJobState {
    fn default() -> Self {
        Self::Init
    }
}









