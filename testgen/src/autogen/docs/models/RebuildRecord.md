# RebuildRecord

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**child_uri** | **String** | Uri of the rebuilding child | 
**src_uri** | **String** | Uri of source child for rebuild job | 
**rebuild_job_state** | [**crate::models::RebuildJobState**](.md) | State of the rebuild job | 
**blocks_total** | **u64** | Total blocks to rebuild | 
**blocks_recovered** | **u64** | Number of blocks processed | 
**blocks_transferred** | **u64** | Number of blocks to transferred | 
**blocks_remaining** | **u64** | Number of blocks remaining | 
**block_size** | **u64** | Size of each block in the task | 
**is_partial** | **bool** | True means its Partial rebuild job. If false, its Full rebuild job | 
**start_time** | **String** | Start time of the rebuild job (UTC) | 
**end_time** | **String** | End time of the rebuild job (UTC) | 


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

