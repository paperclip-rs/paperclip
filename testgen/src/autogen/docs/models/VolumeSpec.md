# VolumeSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**labels** | Option<[**::std::collections::HashMap<String, String>**](.md)> | Optionally used to store custom volume information | [optional]
**num_replicas** | **u8** | Number of children the volume should have. | 
**operation** | Option<[**crate::models::VolumeSpecOperation**](.md)> | Record of the operation in progress | [optional]
**size** | **u64** | Size that the volume should be. | 
**status** | [**crate::models::SpecStatus**](.md) | Common base state for a resource | 
**target** | Option<[**crate::models::VolumeTarget**](.md)> | Specification of a volume target | [optional]
**uuid** | **uuid::Uuid** | Volume Id | 
**topology** | Option<[**crate::models::Topology**](.md)> | node and pool topology for volumes | [optional]
**policy** | [**crate::models::VolumePolicy**](.md) | Volume policy used to determine if and how to replace a replica | 
**thin** | **bool** | Thin provisioning flag | 
**affinity_group** | Option<[**crate::models::AffinityGroup**](.md)> | Affinity Group related information. | [optional]


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

