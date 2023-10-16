# VolumeState

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**target** | Option<[**crate::models::Nexus**](.md)> | target exposed via a Nexus | [optional]
**size** | **u64** | size of the volume in bytes | 
**status** | [**crate::models::VolumeStatus**](.md) | current volume status | 
**uuid** | **uuid::Uuid** | name of the volume | 
**replica_topology** | [**::std::collections::HashMap<String, crate::models::ReplicaTopology>**](.md) | replica topology information | 
**usage** | Option<[**crate::models::VolumeUsage**](.md)> | Volume space usage | [optional]


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

