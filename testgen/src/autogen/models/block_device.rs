#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// BlockDevice : Block device information








/// Block device information

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct BlockDevice {

    /// identifies if device is available for use (ie. is not \"currently\" in  use)
    #[serde(default, rename = "available")]
    pub available: bool,

    /// the type of bus through which the device is connected to the system
    #[serde(default, rename = "connection_type")]
    pub connection_type: String,

    /// list of udev generated symlinks by which device may be identified
    #[serde(default, rename = "devlinks")]
    pub devlinks: Vec<String>,

    /// major device number
    #[serde(default, rename = "devmajor")]
    pub devmajor: i32,

    /// minor device number
    #[serde(default, rename = "devminor")]
    pub devminor: i32,

    /// entry in /dev associated with device
    #[serde(default, rename = "devname")]
    pub devname: String,

    /// official device path
    #[serde(default, rename = "devpath")]
    pub devpath: String,

    /// currently \"disk\" or \"partition\"
    #[serde(default, rename = "devtype")]
    pub devtype: String,

    /// filesystem information in case where a filesystem is present
    #[serde(default, rename = "filesystem", skip_serializing_if = "Option::is_none")]
    pub filesystem: Option<crate::models::BlockDeviceFilesystem>,

    /// indicates whether the device is rotational or non-rotational
    #[serde(default, rename = "is_rotational", skip_serializing_if = "Option::is_none")]
    pub is_rotational: Option<bool>,

    /// device model - useful for identifying devices
    #[serde(default, rename = "model")]
    pub model: String,

    /// partition information in case where device represents a partition
    #[serde(default, rename = "partition", skip_serializing_if = "Option::is_none")]
    pub partition: Option<crate::models::BlockDevicePartition>,

    /// size of device in (512 byte) blocks
    #[serde(default, rename = "size")]
    pub size: u64,

}

impl BlockDevice {
    /// BlockDevice using only the required fields
    pub fn new(available: impl Into<bool>, connection_type: impl Into<String>, devlinks: impl IntoVec<String>, devmajor: impl Into<i32>, devminor: impl Into<i32>, devname: impl Into<String>, devpath: impl Into<String>, devtype: impl Into<String>, model: impl Into<String>, size: impl Into<u64>) -> BlockDevice {
        BlockDevice {
            available: available.into(),
            connection_type: connection_type.into(),
            devlinks: devlinks.into_vec(),
            devmajor: devmajor.into(),
            devminor: devminor.into(),
            devname: devname.into(),
            devpath: devpath.into(),
            devtype: devtype.into(),
            filesystem: None,
            is_rotational: None,
            model: model.into(),
            partition: None,
            size: size.into(),
            
        }
    }
    /// BlockDevice using all fields
    pub fn new_all(available: impl Into<bool>, connection_type: impl Into<String>, devlinks: impl IntoVec<String>, devmajor: impl Into<i32>, devminor: impl Into<i32>, devname: impl Into<String>, devpath: impl Into<String>, devtype: impl Into<String>, filesystem: impl Into<Option<crate::models::BlockDeviceFilesystem>>, is_rotational: impl Into<Option<bool>>, model: impl Into<String>, partition: impl Into<Option<crate::models::BlockDevicePartition>>, size: impl Into<u64>) -> BlockDevice {
        BlockDevice {
            available: available.into(),
            connection_type: connection_type.into(),
            devlinks: devlinks.into_vec(),
            devmajor: devmajor.into(),
            devminor: devminor.into(),
            devname: devname.into(),
            devpath: devpath.into(),
            devtype: devtype.into(),
            filesystem: filesystem.into(),
            is_rotational: is_rotational.into(),
            model: model.into(),
            partition: partition.into(),
            size: size.into(),
            
        }
    }
}


































