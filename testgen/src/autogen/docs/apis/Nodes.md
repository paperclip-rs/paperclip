# Nodes

All URIs are relative to **

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_nodes**](Nodes.md#get_nodes) | **Get** /nodes | 
[**get_node**](Nodes.md#get_node) | **Get** /nodes/{id} | 
[**put_node_cordon**](Nodes.md#put_node_cordon) | **Put** /nodes/{id}/cordon/{label} | 
[**delete_node_cordon**](Nodes.md#delete_node_cordon) | **Delete** /nodes/{id}/cordon/{label} | 
[**put_node_drain**](Nodes.md#put_node_drain) | **Put** /nodes/{id}/drain/{label} | 
[**put_node_label**](Nodes.md#put_node_label) | **Put** /nodes/{id}/label/{key}={value} | 
[**delete_node_label**](Nodes.md#delete_node_label) | **Delete** /nodes/{id}/label/{key} | 





## get_nodes

> Vec<crate::models::Node> get_nodes(node_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------

**node_id** | Option<**String**> |  |  |


### Return type

[**Vec<crate::models::Node>**](.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)



## get_node

> crate::models::Node get_node(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------

**id** | **String** |  | [required] |


### Return type

[**crate::models::Node**](.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)



## put_node_cordon

> crate::models::Node put_node_cordon(id, label)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------

**id** | **String** |  | [required] |

**label** | **String** |  | [required] |


### Return type

[**crate::models::Node**](.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)



## delete_node_cordon

> crate::models::Node delete_node_cordon(id, label)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------

**id** | **String** |  | [required] |

**label** | **String** |  | [required] |


### Return type

[**crate::models::Node**](.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)



## put_node_drain

> crate::models::Node put_node_drain(id, label)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------

**id** | **String** |  | [required] |

**label** | **String** |  | [required] |


### Return type

[**crate::models::Node**](.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)



## put_node_label

> crate::models::Node put_node_label(overwrite, id, key, value)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------

**overwrite** | Option<**bool**> |  |  |

**id** | **String** |  | [required] |

**key** | **String** |  | [required] |

**value** | **String** |  | [required] |


### Return type

[**crate::models::Node**](.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)



## delete_node_label

> crate::models::Node delete_node_label(id, key)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------

**id** | **String** |  | [required] |

**key** | **String** |  | [required] |


### Return type

[**crate::models::Node**](.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


