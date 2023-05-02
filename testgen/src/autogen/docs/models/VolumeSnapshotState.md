# VolumeSnapshotState

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** |  | 
**size** | **u64** | Size in bytes of the snapshot (which is equivalent to its source size). | 
**source_volume** | **uuid::Uuid** |  | 
**creation_timestamp** | **String** | Timestamp when snapshot is taken on the storage system. | 
**clone_ready** | Option<**bool**> | Indicates if a snapshot is ready to be used as a clone. | [optional]
**replica_snapshots** | [**Vec<crate::models::ReplicaSnapshotState>**](.md) | List of individual ReplicaSnapshotStates. | 


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

