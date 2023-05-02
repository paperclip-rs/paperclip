# NexusSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**children** | [**Vec<String>**](.md) | List of children. | 
**managed** | **bool** | Managed by our control plane | 
**node** | **String** | Node where the nexus should live. | 
**operation** | Option<[**crate::models::NexusSpecOperation**](.md)> | Record of the operation in progress | [optional]
**owner** | Option<**uuid::Uuid**> | Volume which owns this nexus, if any | [optional]
**share** | [**crate::models::Protocol**](.md) | Common Protocol | 
**size** | **u64** | Size of the nexus. | 
**status** | [**crate::models::SpecStatus**](.md) | Common base state for a resource | 
**uuid** | **uuid::Uuid** | Nexus Id | 


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

