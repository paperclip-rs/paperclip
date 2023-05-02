# Children

All URIs are relative to **

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_nexus_children**](Children.md#get_nexus_children) | **Get** /nexuses/{nexus_id}/children | 
[**get_nexus_child**](Children.md#get_nexus_child) | **Get** /nexuses/{nexus_id}/children/{child_id} | 
[**put_nexus_child**](Children.md#put_nexus_child) | **Put** /nexuses/{nexus_id}/children/{child_id} | 
[**del_nexus_child**](Children.md#del_nexus_child) | **Delete** /nexuses/{nexus_id}/children/{child_id} | 
[**get_node_nexus_children**](Children.md#get_node_nexus_children) | **Get** /nodes/{node_id}/nexuses/{nexus_id}/children | 
[**get_node_nexus_child**](Children.md#get_node_nexus_child) | **Get** /nodes/{node_id}/nexuses/{nexus_id}/children/{child_id} | 
[**put_node_nexus_child**](Children.md#put_node_nexus_child) | **Put** /nodes/{node_id}/nexuses/{nexus_id}/children/{child_id} | 
[**del_node_nexus_child**](Children.md#del_node_nexus_child) | **Delete** /nodes/{node_id}/nexuses/{nexus_id}/children/{child_id} | 





## get_nexus_children

> Vec<crate::models::Child> get_nexus_children(nexus_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------

**nexus_id** | **uuid::Uuid** |  | [required] |


### Return type

[**Vec<crate::models::Child>**](.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)



## get_nexus_child

> crate::models::Child get_nexus_child(nexus_id, child_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------

**nexus_id** | **uuid::Uuid** |  | [required] |

**child_id** | **String** |  | [required] |


### Return type

[**crate::models::Child**](.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)



## put_nexus_child

> crate::models::Child put_nexus_child(nexus_id, child_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------

**nexus_id** | **uuid::Uuid** |  | [required] |

**child_id** | **String** |  | [required] |


### Return type

[**crate::models::Child**](.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)



## del_nexus_child

> () del_nexus_child(nexus_id, child_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------

**nexus_id** | **uuid::Uuid** |  | [required] |

**child_id** | **String** |  | [required] |


### Return type

[**()**](.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)



## get_node_nexus_children

> Vec<crate::models::Child> get_node_nexus_children(node_id, nexus_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------

**node_id** | **String** |  | [required] |

**nexus_id** | **uuid::Uuid** |  | [required] |


### Return type

[**Vec<crate::models::Child>**](.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)



## get_node_nexus_child

> crate::models::Child get_node_nexus_child(node_id, nexus_id, child_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------

**node_id** | **String** |  | [required] |

**nexus_id** | **uuid::Uuid** |  | [required] |

**child_id** | **String** |  | [required] |


### Return type

[**crate::models::Child**](.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)



## put_node_nexus_child

> crate::models::Child put_node_nexus_child(node_id, nexus_id, child_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------

**node_id** | **String** |  | [required] |

**nexus_id** | **uuid::Uuid** |  | [required] |

**child_id** | **String** |  | [required] |


### Return type

[**crate::models::Child**](.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)



## del_node_nexus_child

> () del_node_nexus_child(node_id, nexus_id, child_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------

**node_id** | **String** |  | [required] |

**nexus_id** | **uuid::Uuid** |  | [required] |

**child_id** | **String** |  | [required] |


### Return type

[**()**](.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


