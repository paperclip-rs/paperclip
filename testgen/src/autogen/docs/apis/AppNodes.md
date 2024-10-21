# AppNodes

All URIs are relative to **

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_app_nodes**](AppNodes.md#get_app_nodes) | **Get** /app-nodes | 
[**get_app_node**](AppNodes.md#get_app_node) | **Get** /app-nodes/{app_node_id} | 
[**register_app_node**](AppNodes.md#register_app_node) | **Put** /app-nodes/{app_node_id} | 
[**deregister_app_node**](AppNodes.md#deregister_app_node) | **Delete** /app-nodes/{app_node_id} | 





## get_app_nodes

> crate::models::AppNodes get_app_nodes(max_entries, starting_token)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------

**max_entries** | **isize** |  | [required] |

**starting_token** | Option<**isize**> |  |  |


### Return type

[**crate::models::AppNodes**](.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)



## get_app_node

> crate::models::AppNode get_app_node(app_node_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------

**app_node_id** | **String** |  | [required] |


### Return type

[**crate::models::AppNode**](.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)



## register_app_node

> () register_app_node(app_node_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------

**app_node_id** | **String** |  | [required] |


### Return type

[**()**](.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)



## deregister_app_node

> () deregister_app_node(app_node_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------

**app_node_id** | **String** |  | [required] |


### Return type

[**()**](.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


