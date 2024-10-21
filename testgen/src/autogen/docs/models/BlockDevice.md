# BlockDevice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**available** | **bool** | identifies if device is available for use (ie. is not \"currently\" in  use) | 
**connection_type** | **String** | the type of bus through which the device is connected to the system | 
**devlinks** | [**Vec<String>**](.md) | list of udev generated symlinks by which device may be identified | 
**devmajor** | **i32** | major device number | 
**devminor** | **i32** | minor device number | 
**devname** | **String** | entry in /dev associated with device | 
**devpath** | **String** | official device path | 
**devtype** | **String** | currently \"disk\" or \"partition\" | 
**filesystem** | Option<[**crate::models::BlockDeviceFilesystem**](.md)> | filesystem information in case where a filesystem is present | [optional]
**is_rotational** | Option<**bool**> | indicates whether the device is rotational or non-rotational | [optional]
**model** | **String** | device model - useful for identifying devices | 
**partition** | Option<[**crate::models::BlockDevicePartition**](.md)> | partition information in case where device represents a partition | [optional]
**size** | **u64** | size of device in (512 byte) blocks | 


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

