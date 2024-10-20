#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// SpecStatus : Common base state for a resource



/// Common base state for a resource
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SpecStatus {


    #[serde(rename = "Creating")]
    Creating,

    #[serde(rename = "Created")]
    Created,

    #[serde(rename = "Deleting")]
    Deleting,

    #[serde(rename = "Deleted")]
    Deleted,

}

impl ToString for SpecStatus {
    fn to_string(&self) -> String {
        match self {
            
            
            Self::Creating => String::from("Creating"),
            
            Self::Created => String::from("Created"),
            
            Self::Deleting => String::from("Deleting"),
            
            Self::Deleted => String::from("Deleted"),
            
            
        }
    }
}

impl Default for SpecStatus {
    fn default() -> Self {
        Self::Creating
    }
}









