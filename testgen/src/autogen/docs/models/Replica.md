# Replica

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**node** | **String** | storage node identifier | 
**pool** | **String** | storage pool identifier | 
**pool_uuid** | Option<**uuid::Uuid**> | storage pool unique identifier | [optional]
**share** | [**crate::models::Protocol**](.md) | Common Protocol | 
**size** | **u64** | size of the replica in bytes | 
**space** | Option<[**crate::models::ReplicaSpaceUsage**](.md)> | Replica space usage information. Useful for capacity management, eg: figure out how much of a thin-provisioned replica is allocated.  | [optional]
**state** | [**crate::models::ReplicaState**](.md) | state of the replica | 
**thin** | **bool** | thin provisioning | 
**uri** | **String** | uri usable by nexus to access it | 
**uuid** | **uuid::Uuid** | uuid of the replica | 
**allowed_hosts** | Option<[**Vec<String>**](.md)> | NQNs of hosts allowed to connect to this replica | [optional]
**kind** | [**crate::models::ReplicaKind**](.md) | Type of replica, example regular or snapshot. | 


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

