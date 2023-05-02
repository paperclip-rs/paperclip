# VolumeTarget

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**node** | **String** | The node where front-end IO will be sent to | 
**protocol** | Option<[**crate::models::VolumeShareProtocol**](.md)> | Volume Share Protocol | [optional]
**frontend_nodes** | Option<[**Vec<crate::models::NodeAccessInfo>**](.md)> | The nodes where the front-end workload resides. If the workload moves then the volume must be republished. | [optional]


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

