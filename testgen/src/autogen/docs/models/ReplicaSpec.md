# ReplicaSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**managed** | **bool** | Managed by our control plane | 
**operation** | Option<[**crate::models::ReplicaSpecOperation**](.md)> | Record of the operation in progress | [optional]
**owners** | [**crate::models::ReplicaSpecOwners**](.md) | Owner Resource | 
**pool** | **String** | The pool that the replica should live on. | 
**pool_uuid** | Option<**uuid::Uuid**> | storage pool unique identifier | [optional]
**share** | [**crate::models::Protocol**](.md) | Common Protocol | 
**size** | **u64** | The size that the replica should be. | 
**status** | [**crate::models::SpecStatus**](.md) | Common base state for a resource | 
**thin** | **bool** | Thin provisioning. | 
**uuid** | **uuid::Uuid** | uuid of the replica | 
**kind** | Option<[**crate::models::ReplicaKind**](.md)> | Type of replica, example regular or snapshot. | [optional]


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

