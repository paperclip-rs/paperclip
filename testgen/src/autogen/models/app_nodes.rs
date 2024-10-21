#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// AppNodes : Array of app nodes plus the next token for subsequent get requests when using pagination.








/// Array of app nodes plus the next token for subsequent get requests when using pagination.

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct AppNodes {

    
    #[serde(default, rename = "entries")]
    pub entries: Vec<crate::models::AppNode>,

    
    #[serde(default, rename = "next_token", skip_serializing_if = "Option::is_none")]
    pub next_token: Option<isize>,

}

impl AppNodes {
    /// AppNodes using only the required fields
    pub fn new(entries: impl IntoVec<crate::models::AppNode>) -> AppNodes {
        AppNodes {
            entries: entries.into_vec(),
            next_token: None,
            
        }
    }
    /// AppNodes using all fields
    pub fn new_all(entries: impl IntoVec<crate::models::AppNode>, next_token: impl Into<Option<isize>>) -> AppNodes {
        AppNodes {
            entries: entries.into_vec(),
            next_token: next_token.into(),
            
        }
    }
}












