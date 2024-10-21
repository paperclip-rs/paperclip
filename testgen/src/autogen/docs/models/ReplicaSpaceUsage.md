# ReplicaSpaceUsage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**capacity_bytes** | **u64** | Replica capacity in bytes. | 
**allocated_bytes** | **u64** | Amount of actually allocated disk space for this replica in bytes. | 
**allocated_bytes_snapshots** | **u64** | Amount of actually allocated disk space for this replica\'s snapshots in bytes. | 
**allocated_bytes_all_snapshots** | Option<**u64**> | Amount of actually allocated disk space for this replica\'s snapshots and its predecessors in bytes. For a restored/cloned replica this includes snapshots from the parent source.  | [optional]
**cluster_size** | **u64** | Cluster size in bytes. | 
**clusters** | **u64** | Total number of clusters. | 
**allocated_clusters** | **u64** | Number of actually used clusters. | 


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

