# ReplicaSnapshotState

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** |  | 
**source_id** | **uuid::Uuid** |  | 
**creation_timestamp** | **String** | Timestamp when replica snapshot is taken on the storage system. | 
**size** | **i64** | Replica snapshot size. | 
**referenced_size** | **i64** | Replica snapshot referenced size. | 
**state** | [**crate::models::ReplicaSnapshotStatus**](.md) | Current ReplicaSnapshot status | 


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

