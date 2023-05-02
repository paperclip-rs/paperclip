#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// AffinityGroup : Affinity Group related information.








/// Affinity Group related information.

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct AffinityGroup {

    /// Identification of the Affinity Group.
    #[serde(default, rename = "id")]
    pub id: String,

}

impl AffinityGroup {
    /// AffinityGroup using only the required fields
    pub fn new(id: impl Into<String>) -> AffinityGroup {
        AffinityGroup {
            id: id.into(),
            
        }
    }
    /// AffinityGroup using all fields
    pub fn new_all(id: impl Into<String>) -> AffinityGroup {
        AffinityGroup {
            id: id.into(),
            
        }
    }
}










