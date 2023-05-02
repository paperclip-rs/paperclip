# VolumeUsage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**capacity** | **u64** | Capacity of the volume in bytes. | 
**allocated** | **u64** | -| Allocated size in bytes, related the largest healthy replica, including snapshots. For example, if a volume has 2 replicas, each with 1MiB allocated space, then this field will be 1MiB. | 
**allocated_replica** | **u64** | -| Allocated size in bytes, related to the largest healthy replica, excluding snapshots. | 
**allocated_snapshots** | **u64** | -| Allocated size in bytes, related the healthy replica with the highest snapshot usage. | 
**allocated_all_snapshots** | **u64** | -| For a restored/cloned volume, allocated size in bytes, related to the healthy replica with largest parent snapshot allocation. | 
**total_allocated** | **u64** | -| Allocated size in bytes, accrued from all the replicas, including snapshots. For example, if a volume has 2 replicas, each with 1MiB allocated space, then this field will be 2MiB. | 
**total_allocated_replicas** | [**serde_json::Value**](.md) | -| Allocated size in bytes, accrued from all the replicas, excluding snapshots. | 
**total_allocated_snapshots** | **u64** | -| Allocated size in bytes, accrued from all the replica\'s snapshots. | 


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

