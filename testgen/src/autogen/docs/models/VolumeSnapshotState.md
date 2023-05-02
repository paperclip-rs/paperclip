# VolumeSnapshotState

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** |  | 
**allocated_size** | **u64** | Runtime size in bytes of the snapshot. Equal to the volume allocation at the time of the snapshot creation. It may grow larger if any of its predecessors are deleted. | 
**source_volume** | **uuid::Uuid** |  | 
**timestamp** | Option<**String**> | Timestamp when snapshot is taken on the storage system. | [optional]
**ready_as_source** | **bool** | Indicates if a snapshot is ready to be used as a new volume source. | 
**replica_snapshots** | [**Vec<crate::models::ReplicaSnapshotState>**](.md) | List of individual ReplicaSnapshotStates. | 


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

