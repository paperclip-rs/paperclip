#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// CreateNexusBody : Create Nexus Body








/// Create Nexus Body

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct CreateNexusBody {

    /// replica can be iscsi and nvmf remote targets or a local spdk bdev  (i.e. bdev:///name-of-the-bdev).   uris to the targets we connect to
    #[serde(default, rename = "children")]
    pub children: Vec<String>,

    /// size of the device in bytes
    #[serde(default, rename = "size")]
    pub size: u64,

}

impl CreateNexusBody {
    /// CreateNexusBody using only the required fields
    pub fn new(children: impl IntoVec<String>, size: impl Into<u64>) -> CreateNexusBody {
        CreateNexusBody {
            children: children.into_vec(),
            size: size.into(),
            
        }
    }
    /// CreateNexusBody using all fields
    pub fn new_all(children: impl IntoVec<String>, size: impl Into<u64>) -> CreateNexusBody {
        CreateNexusBody {
            children: children.into_vec(),
            size: size.into(),
            
        }
    }
}












