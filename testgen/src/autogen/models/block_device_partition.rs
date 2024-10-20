#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// BlockDevicePartition : partition information in case where device represents a partition








/// partition information in case where device represents a partition

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct BlockDevicePartition {

    /// partition name
    #[serde(default, rename = "name")]
    pub name: String,

    /// partition number
    #[serde(default, rename = "number")]
    pub number: i32,

    /// devname of parent device to which this partition belongs
    #[serde(default, rename = "parent")]
    pub parent: String,

    /// partition scheme: gpt, dos, ...
    #[serde(default, rename = "scheme")]
    pub scheme: String,

    /// partition type identifier
    #[serde(default, rename = "typeid")]
    pub typeid: String,

    /// UUID identifying partition
    #[serde(default, rename = "uuid")]
    pub uuid: String,

}

impl BlockDevicePartition {
    /// BlockDevicePartition using only the required fields
    pub fn new(name: impl Into<String>, number: impl Into<i32>, parent: impl Into<String>, scheme: impl Into<String>, typeid: impl Into<String>, uuid: impl Into<String>) -> BlockDevicePartition {
        BlockDevicePartition {
            name: name.into(),
            number: number.into(),
            parent: parent.into(),
            scheme: scheme.into(),
            typeid: typeid.into(),
            uuid: uuid.into(),
            
        }
    }
    /// BlockDevicePartition using all fields
    pub fn new_all(name: impl Into<String>, number: impl Into<i32>, parent: impl Into<String>, scheme: impl Into<String>, typeid: impl Into<String>, uuid: impl Into<String>) -> BlockDevicePartition {
        BlockDevicePartition {
            name: name.into(),
            number: number.into(),
            parent: parent.into(),
            scheme: scheme.into(),
            typeid: typeid.into(),
            uuid: uuid.into(),
            
        }
    }
}




















