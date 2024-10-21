# OnlineReplicaSnapshotState

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** |  | 
**source_id** | **uuid::Uuid** |  | 
**pool_id** | **String** | storage pool identifier | 
**pool_uuid** | **uuid::Uuid** | storage pool unique identifier | 
**timestamp** | **String** | Timestamp when the replica snapshot is taken on the storage system. | 
**size** | **u64** | Replica snapshot size. | 
**allocated_size** | **u64** | Runtime size in bytes of the snapshot. Equal to the volume allocation at the time of the snapshot creation. It may grow larger if any of its predecessors are deleted. | 
**predecessor_alloc_size** | **u64** | Total allocated size of all the snapshot predecessors. | 


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

