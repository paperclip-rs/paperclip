# Volumes

All URIs are relative to **

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_volumes**](Volumes.md#get_volumes) | **Get** /volumes | 
[**get_volume**](Volumes.md#get_volume) | **Get** /volumes/{volume_id} | 
[**put_volume**](Volumes.md#put_volume) | **Put** /volumes/{volume_id} | 
[**del_volume**](Volumes.md#del_volume) | **Delete** /volumes/{volume_id} | 
[**get_rebuild_history**](Volumes.md#get_rebuild_history) | **Get** /volumes/{volume_id}/rebuild-history | 
[**put_volume_replica_count**](Volumes.md#put_volume_replica_count) | **Put** /volumes/{volume_id}/replica_count/{replica_count} | 
[**put_volume_property**](Volumes.md#put_volume_property) | **Put** /volumes/{volume_id}/property | 
[**put_volume_target**](Volumes.md#put_volume_target) | **Put** /volumes/{volume_id}/target | 
[**del_volume_target**](Volumes.md#del_volume_target) | **Delete** /volumes/{volume_id}/target | 
[**put_volume_size**](Volumes.md#put_volume_size) | **Put** /volumes/{volume_id}/size | 
[**del_volume_shutdown_targets**](Volumes.md#del_volume_shutdown_targets) | **Delete** /volumes/{volume_id}/shutdown_targets | 
[**put_volume_share**](Volumes.md#put_volume_share) | **Put** /volumes/{volume_id}/share/{protocol} | 
[**del_share**](Volumes.md#del_share) | **Delete** /volumes{volume_id}/share | 
[**put_snapshot_volume**](Volumes.md#put_snapshot_volume) | **Put** /snapshots/{snapshot_id}/volumes/{volume_id} | 





## get_volumes

> crate::models::Volumes get_volumes(volume_id, max_entries, starting_token)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------

**volume_id** | Option<**uuid::Uuid**> |  |  |

**max_entries** | **isize** |  | [required] |

**starting_token** | Option<**isize**> |  |  |


### Return type

[**crate::models::Volumes**](.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)



## get_volume

> crate::models::Volume get_volume(volume_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------

**volume_id** | **uuid::Uuid** |  | [required] |


### Return type

[**crate::models::Volume**](.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)



## put_volume

> crate::models::Volume put_volume(volume_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------

**volume_id** | **uuid::Uuid** |  | [required] |


### Return type

[**crate::models::Volume**](.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)



## del_volume

> () del_volume(volume_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------

**volume_id** | **uuid::Uuid** |  | [required] |


### Return type

[**()**](.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)



## get_rebuild_history

> serde_json::Value get_rebuild_history(volume_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------

**volume_id** | **uuid::Uuid** |  | [required] |


### Return type

[**serde_json::Value**](.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)



## put_volume_replica_count

> crate::models::Volume put_volume_replica_count(volume_id, replica_count)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------

**volume_id** | **uuid::Uuid** |  | [required] |

**replica_count** | **u8** |  | [required] |


### Return type

[**crate::models::Volume**](.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)



## put_volume_property

> crate::models::Volume put_volume_property(volume_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------

**volume_id** | **uuid::Uuid** |  | [required] |


### Return type

[**crate::models::Volume**](.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)



## put_volume_target

> crate::models::Volume put_volume_target(volume_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------

**volume_id** | **uuid::Uuid** |  | [required] |


### Return type

[**crate::models::Volume**](.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)



## del_volume_target

> crate::models::Volume del_volume_target(force, volume_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------

**force** | Option<**bool**> |  |  |

**volume_id** | **uuid::Uuid** |  | [required] |


### Return type

[**crate::models::Volume**](.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)



## put_volume_size

> crate::models::Volume put_volume_size(volume_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------

**volume_id** | **uuid::Uuid** |  | [required] |


### Return type

[**crate::models::Volume**](.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)



## del_volume_shutdown_targets

> () del_volume_shutdown_targets(volume_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------

**volume_id** | **uuid::Uuid** |  | [required] |


### Return type

[**()**](.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)



## put_volume_share

> String put_volume_share(frontend_host, volume_id, protocol)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------

**frontend_host** | Option<**String**> |  |  |

**volume_id** | **uuid::Uuid** |  | [required] |

**protocol** | [**Protocol**](.md) |  | [required] |


### Return type

[**String**](.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)



## del_share

> () del_share(volume_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------

**volume_id** | **uuid::Uuid** |  | [required] |


### Return type

[**()**](.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)



## put_snapshot_volume

> crate::models::Volume put_snapshot_volume(snapshot_id, volume_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------

**snapshot_id** | **uuid::Uuid** |  | [required] |

**volume_id** | **uuid::Uuid** |  | [required] |


### Return type

[**crate::models::Volume**](.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


