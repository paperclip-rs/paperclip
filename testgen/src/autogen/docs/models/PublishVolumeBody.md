# PublishVolumeBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**publish_context** | [**::std::collections::HashMap<String, String>**](.md) | Controller Volume Publish context | 
**reuse_existing** | Option<**bool**> | Allows reusing of the current target. | [optional]
**node** | Option<[**String**](.md)> | The node where the target will reside in. It may be moved elsewhere during volume republish. | [optional]
**protocol** | [**crate::models::VolumeShareProtocol**](.md) | The protocol used to connect to the front-end node. | 
**republish** | Option<**bool**> | Allows republishing the volume on the node by shutting down the existing target first. | [optional]
**frontend_node** | Option<**String**> | The node where the front-end workload resides. If the workload moves then the volume must be republished. | [optional]


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

