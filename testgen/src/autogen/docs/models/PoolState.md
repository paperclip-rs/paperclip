# PoolState

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**capacity** | **u64** | size of the pool in bytes | 
**disks** | [**Vec<String>**](.md) | absolute disk paths claimed by the pool | 
**id** | **String** | storage pool identifier | 
**node** | **String** | storage node identifier | 
**status** | [**crate::models::PoolStatus**](.md) | current status of the pool | 
**used** | **u64** | used bytes from the pool | 
**committed** | Option<**u64**> | accrued size of all replicas contained in this pool | [optional]


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

