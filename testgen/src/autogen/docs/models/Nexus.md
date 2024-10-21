# Nexus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**children** | [**Vec<crate::models::Child>**](.md) | Array of Nexus Children | 
**device_uri** | **String** | URI of the device for the volume (missing if not published).  Missing property and empty string are treated the same. | 
**node** | **String** | id of the io-engine instance | 
**rebuilds** | **u32** | total number of rebuild tasks | 
**protocol** | [**crate::models::Protocol**](.md) | Common Protocol | 
**size** | **u64** | size of the volume in bytes | 
**state** | [**crate::models::NexusState**](.md) | State of the Nexus | 
**uuid** | **uuid::Uuid** | uuid of the nexus | 


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

