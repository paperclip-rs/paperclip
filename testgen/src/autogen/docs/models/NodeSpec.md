# NodeSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**grpc_endpoint** | **String** | gRPC endpoint of the io-engine instance | 
**id** | **String** | storage node identifier | 
**labels** | Option<[**::std::collections::HashMap<String, String>**](.md)> | labels to be set on the node | [optional]
**cordondrainstate** | Option<[**crate::models::CordonDrainState**](.md)> | the drain state | [optional]
**node_nqn** | Option<**String**> | NVMe Qualified Names (NQNs) are used to uniquely describe a host or NVM subsystem for the purposes of identification and authentication | [optional]


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

