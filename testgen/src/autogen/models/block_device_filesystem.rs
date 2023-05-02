#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// BlockDeviceFilesystem : filesystem information in case where a filesystem is present








/// filesystem information in case where a filesystem is present

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct BlockDeviceFilesystem {

    /// filesystem type: ext3, ntfs, ...
    #[serde(default, rename = "fstype")]
    pub fstype: String,

    /// volume label
    #[serde(default, rename = "label")]
    pub label: String,

    /// path where filesystem is currently mounted
    #[serde(default, rename = "mountpoint")]
    pub mountpoint: String,

    /// UUID identifying the volume (filesystem)
    #[serde(default, rename = "uuid")]
    pub uuid: String,

}

impl BlockDeviceFilesystem {
    /// BlockDeviceFilesystem using only the required fields
    pub fn new(fstype: impl Into<String>, label: impl Into<String>, mountpoint: impl Into<String>, uuid: impl Into<String>) -> BlockDeviceFilesystem {
        BlockDeviceFilesystem {
            fstype: fstype.into(),
            label: label.into(),
            mountpoint: mountpoint.into(),
            uuid: uuid.into(),
            
        }
    }
    /// BlockDeviceFilesystem using all fields
    pub fn new_all(fstype: impl Into<String>, label: impl Into<String>, mountpoint: impl Into<String>, uuid: impl Into<String>) -> BlockDeviceFilesystem {
        BlockDeviceFilesystem {
            fstype: fstype.into(),
            label: label.into(),
            mountpoint: mountpoint.into(),
            uuid: uuid.into(),
            
        }
    }
}
















