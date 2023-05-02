# ReplicaUsage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**capacity** | **u64** | Replica capacity in bytes. | 
**allocated** | **u64** | Amount of actually allocated disk space for this replica in bytes. | 
**allocated_snapshots** | **u64** | Amount of actually allocated disk space for this replica\'s snapshots in bytes. | 
**allocated_all_snapshots** | **u64** | Amount of actually allocated disk space for this replica\'s snapshots and its predecessors in bytes. For a restored/cloned replica this includes snapshots from the parent source.  | 


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

