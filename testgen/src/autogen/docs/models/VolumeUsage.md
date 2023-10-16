# VolumeUsage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**capacity** | **u64** | Capacity of the volume in bytes. | 
**allocated** | **u64** | -| Allocated size in bytes, related to a single healthy replica. For example, if a volume has 2 replicas, each with 1MiB allocated space, then this field will be 1MiB. | 
**total_allocated** | **u64** | -| Allocated size in bytes, accrued from all the replica. For example, if a volume has 2 replicas, each with 1MiB allocated space, then this field will be 2MiB. | 


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

