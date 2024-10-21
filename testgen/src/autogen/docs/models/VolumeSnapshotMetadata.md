# VolumeSnapshotMetadata

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | [**crate::models::SpecStatus**](.md) | Common base state for a resource | 
**timestamp** | Option<**String**> | Timestamp when snapshot is taken on the storage system. | [optional]
**size** | **u64** | Size in bytes of the snapshot (which is equivalent to its source size). | 
**spec_size** | **u64** | Spec size in bytes of the snapshot (which is equivalent to its source spec size). | 
**total_allocated_size** | **u64** | Size in bytes taken by the snapshot and its predecessors. | 
**txn_id** | **String** |  | 
**transactions** | [**::std::collections::HashMap<String, Vec<crate::models::ReplicaSnapshot>>**](.md) |  | 
**num_restores** | **u32** | Number of restores done from this snapshot. | 
**num_snapshot_replicas** | **u32** | Number of snapshot replicas for a volumesnapshot. | 


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

