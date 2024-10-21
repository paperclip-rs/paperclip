# Snapshots

All URIs are relative to **

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_volume_snapshot**](Snapshots.md#get_volume_snapshot) | **Get** /volumes/{volume_id}/snapshots/{snapshot_id} | 
[**put_volume_snapshot**](Snapshots.md#put_volume_snapshot) | **Put** /volumes/{volume_id}/snapshots/{snapshot_id} | 
[**del_volume_snapshot**](Snapshots.md#del_volume_snapshot) | **Delete** /volumes/{volume_id}/snapshots/{snapshot_id} | 
[**get_volume_snapshots**](Snapshots.md#get_volume_snapshots) | **Get** /volumes/{volume_id}/snapshots | 
[**get_volumes_snapshot**](Snapshots.md#get_volumes_snapshot) | **Get** /volumes/snapshots/{snapshot_id} | 
[**del_snapshot**](Snapshots.md#del_snapshot) | **Delete** /volumes/snapshots/{snapshot_id} | 
[**get_volumes_snapshots**](Snapshots.md#get_volumes_snapshots) | **Get** /volumes/snapshots | 





## get_volume_snapshot

> crate::models::VolumeSnapshot get_volume_snapshot(volume_id, snapshot_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------

**volume_id** | **uuid::Uuid** |  | [required] |

**snapshot_id** | **uuid::Uuid** |  | [required] |


### Return type

[**crate::models::VolumeSnapshot**](.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)



## put_volume_snapshot

> crate::models::VolumeSnapshot put_volume_snapshot(volume_id, snapshot_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------

**volume_id** | **uuid::Uuid** |  | [required] |

**snapshot_id** | **uuid::Uuid** |  | [required] |


### Return type

[**crate::models::VolumeSnapshot**](.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)



## del_volume_snapshot

> () del_volume_snapshot(volume_id, snapshot_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------

**volume_id** | **uuid::Uuid** |  | [required] |

**snapshot_id** | **uuid::Uuid** |  | [required] |


### Return type

[**()**](.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)



## get_volume_snapshots

> crate::models::VolumeSnapshots get_volume_snapshots(max_entries, starting_token, volume_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------

**max_entries** | **isize** |  | [required] |

**starting_token** | Option<**isize**> |  |  |

**volume_id** | **uuid::Uuid** |  | [required] |


### Return type

[**crate::models::VolumeSnapshots**](.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)



## get_volumes_snapshot

> crate::models::VolumeSnapshot get_volumes_snapshot(snapshot_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------

**snapshot_id** | **uuid::Uuid** |  | [required] |


### Return type

[**crate::models::VolumeSnapshot**](.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)



## del_snapshot

> () del_snapshot(snapshot_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------

**snapshot_id** | **uuid::Uuid** |  | [required] |


### Return type

[**()**](.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)



## get_volumes_snapshots

> crate::models::VolumeSnapshots get_volumes_snapshots(snapshot_id, volume_id, max_entries, starting_token)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------

**snapshot_id** | Option<**uuid::Uuid**> |  |  |

**volume_id** | Option<**uuid::Uuid**> |  |  |

**max_entries** | **isize** |  | [required] |

**starting_token** | Option<**isize**> |  |  |


### Return type

[**crate::models::VolumeSnapshots**](.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


