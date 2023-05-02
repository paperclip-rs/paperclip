#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// LabelledTopology : labelled topology








/// labelled topology

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct LabelledTopology {

    /// Excludes resources with the same $label name, eg:  \"Zone\" would not allow for resources with the same \"Zone\" value  to be used for a certain operation, eg:  A node with \"Zone: A\" would not be paired up with a node with \"Zone: A\",  but it could be paired up with a node with \"Zone: B\"  exclusive label NAME in the form \"NAME\", and not \"NAME: VALUE\"
    #[serde(default, rename = "exclusion")]
    pub exclusion: ::std::collections::HashMap<String, String>,

    /// Includes resources with the same $label or $label:$value eg:  if label is \"Zone: A\":  A resource with \"Zone: A\" would be paired up with a resource with \"Zone: A\",  but not with a resource with \"Zone: B\"  if label is \"Zone\":  A resource with \"Zone: A\" would be paired up with a resource with \"Zone: B\",  but not with a resource with \"OtherLabel: B\"  inclusive label key value in the form \"NAME: VALUE\"
    #[serde(default, rename = "inclusion")]
    pub inclusion: ::std::collections::HashMap<String, String>,

    /// This feature includes resources with identical $label keys. For example,  if the affinity key is set to \"Zone\":  Initially, a resource that matches the label is selected, example \"Zone: A\".  Subsequently, all other resources must match the given label \"Zone: A\",  effectively adding this requirement as an inclusion label.
    #[serde(default, rename = "affinitykey")]
    pub affinitykey: Vec<String>,

}

impl LabelledTopology {
    /// LabelledTopology using only the required fields
    pub fn new(exclusion: impl Into<::std::collections::HashMap<String, String>>, inclusion: impl Into<::std::collections::HashMap<String, String>>, affinitykey: impl IntoVec<String>) -> LabelledTopology {
        LabelledTopology {
            exclusion: exclusion.into(),
            inclusion: inclusion.into(),
            affinitykey: affinitykey.into_vec(),
            
        }
    }
    /// LabelledTopology using all fields
    pub fn new_all(exclusion: impl Into<::std::collections::HashMap<String, String>>, inclusion: impl Into<::std::collections::HashMap<String, String>>, affinitykey: impl IntoVec<String>) -> LabelledTopology {
        LabelledTopology {
            exclusion: exclusion.into(),
            inclusion: inclusion.into(),
            affinitykey: affinitykey.into_vec(),
            
        }
    }
}














