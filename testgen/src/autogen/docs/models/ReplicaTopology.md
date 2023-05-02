# ReplicaTopology

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**node** | Option<**String**> | storage node identifier | [optional]
**pool** | Option<**String**> | storage pool identifier | [optional]
**state** | [**crate::models::ReplicaState**](.md) | state of the replica | 
**child_status** | Option<[**crate::models::ChildState**](.md)> | State of a Nexus Child | [optional]
**child_status_reason** | Option<[**crate::models::ChildStateReason**](.md)> | Reason for the state of a Nexus Child | [optional]
**usage** | Option<[**crate::models::ReplicaUsage**](.md)> | Replica space usage information. Useful for capacity management, eg: figure out how much of a thin-provisioned replica is allocated.  | [optional]
**rebuild_progress** | Option<**usize**> | current rebuild progress (%) | [optional]


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

