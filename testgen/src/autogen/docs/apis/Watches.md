# Watches

All URIs are relative to **

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_watch_volume**](Watches.md#get_watch_volume) | **Get** /watches/volumes/{volume_id} | 
[**put_watch_volume**](Watches.md#put_watch_volume) | **Put** /watches/volumes/{volume_id} | 
[**del_watch_volume**](Watches.md#del_watch_volume) | **Delete** /watches/volumes/{volume_id} | 





## get_watch_volume

> Vec<crate::models::RestWatch> get_watch_volume(volume_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------

**volume_id** | **uuid::Uuid** |  | [required] |


### Return type

[**Vec<crate::models::RestWatch>**](.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)



## put_watch_volume

> () put_watch_volume(callback, volume_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------

**callback** | **String** |  | [required] |

**volume_id** | **uuid::Uuid** |  | [required] |


### Return type

[**()**](.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)



## del_watch_volume

> () del_watch_volume(callback, volume_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------

**callback** | **String** |  | [required] |

**volume_id** | **uuid::Uuid** |  | [required] |


### Return type

[**()**](.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


