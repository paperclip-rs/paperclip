# CreateVolumeBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**policy** | [**crate::models::VolumePolicy**](.md) | Volume policy used to determine if and how to replace a replica | 
**replicas** | **u8** | number of storage replicas | 
**size** | **u64** | size of the volume in bytes | 
**thin** | **bool** | flag indicating whether or not the volume is thin provisioned | 
**topology** | Option<[**crate::models::Topology**](.md)> | node and pool topology for volumes | [optional]
**labels** | Option<[**::std::collections::HashMap<String, String>**](.md)> | Optionally used to store custom volume information | [optional]
**affinity_group** | Option<[**crate::models::AffinityGroup**](.md)> | Affinity Group related information. | [optional]
**max_snapshots** | Option<**u32**> | Max Snapshots limit per volume. | [optional]


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

