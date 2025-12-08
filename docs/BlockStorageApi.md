# \BlockStorageApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**volume_snapshots_create**](BlockStorageApi.md#volume_snapshots_create) | **POST** /v2/volumes/{volume_id}/snapshots | Create Snapshot from a Volume
[**volume_snapshots_delete_by_id**](BlockStorageApi.md#volume_snapshots_delete_by_id) | **DELETE** /v2/volumes/snapshots/{snapshot_id} | Delete a Volume Snapshot
[**volume_snapshots_get_by_id**](BlockStorageApi.md#volume_snapshots_get_by_id) | **GET** /v2/volumes/snapshots/{snapshot_id} | Retrieve an Existing Volume Snapshot
[**volume_snapshots_list**](BlockStorageApi.md#volume_snapshots_list) | **GET** /v2/volumes/{volume_id}/snapshots | List Snapshots for a Volume
[**volumes_create**](BlockStorageApi.md#volumes_create) | **POST** /v2/volumes | Create a New Block Storage Volume
[**volumes_delete**](BlockStorageApi.md#volumes_delete) | **DELETE** /v2/volumes/{volume_id} | Delete a Block Storage Volume
[**volumes_delete_by_name**](BlockStorageApi.md#volumes_delete_by_name) | **DELETE** /v2/volumes | Delete a Block Storage Volume by Name
[**volumes_get**](BlockStorageApi.md#volumes_get) | **GET** /v2/volumes/{volume_id} | Retrieve an Existing Block Storage Volume
[**volumes_list**](BlockStorageApi.md#volumes_list) | **GET** /v2/volumes | List All Block Storage Volumes



## volume_snapshots_create

> models::VolumeSnapshotsGetById200Response volume_snapshots_create(volume_id, volume_snapshots_create_request)
Create Snapshot from a Volume

To create a snapshot from a volume, sent a POST request to `/v2/volumes/$VOLUME_ID/snapshots`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**volume_id** | **uuid::Uuid** | The ID of the block storage volume. | [required] |
**volume_snapshots_create_request** | [**VolumeSnapshotsCreateRequest**](VolumeSnapshotsCreateRequest.md) |  | [required] |

### Return type

[**models::VolumeSnapshotsGetById200Response**](volumeSnapshots_get_byId_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## volume_snapshots_delete_by_id

> volume_snapshots_delete_by_id(snapshot_id)
Delete a Volume Snapshot

To delete a volume snapshot, send a DELETE request to `/v2/volumes/snapshots/$VOLUME_SNAPSHOT_ID`.  A status of 204 will be given. This indicates that the request was processed successfully, but that no response body is needed. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**snapshot_id** | **String** | The unique identifier for the snapshot. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## volume_snapshots_get_by_id

> models::VolumeSnapshotsGetById200Response volume_snapshots_get_by_id(snapshot_id)
Retrieve an Existing Volume Snapshot

To retrieve the details of a snapshot that has been created from a volume, send a GET request to `/v2/volumes/snapshots/$VOLUME_SNAPSHOT_ID`.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**snapshot_id** | **String** | The unique identifier for the snapshot. | [required] |

### Return type

[**models::VolumeSnapshotsGetById200Response**](volumeSnapshots_get_byId_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## volume_snapshots_list

> models::VolumeSnapshotsList200Response volume_snapshots_list(volume_id, per_page, page)
List Snapshots for a Volume

To retrieve the snapshots that have been created from a volume, send a GET request to `/v2/volumes/$VOLUME_ID/snapshots`.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**volume_id** | **uuid::Uuid** | The ID of the block storage volume. | [required] |
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**models::VolumeSnapshotsList200Response**](volumeSnapshots_list_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## volumes_create

> models::VolumesCreate201Response volumes_create(volumes_create_request)
Create a New Block Storage Volume

To create a new volume, send a POST request to `/v2/volumes`. Optionally, a `filesystem_type` attribute may be provided in order to automatically format the volume's filesystem. Pre-formatted volumes are automatically mounted when attached to Ubuntu, Debian, Fedora, Fedora Atomic, and CentOS Droplets created on or after April 26, 2018. Attaching pre-formatted volumes to Droplets without support for auto-mounting is not recommended.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**volumes_create_request** | [**VolumesCreateRequest**](VolumesCreateRequest.md) |  | [required] |

### Return type

[**models::VolumesCreate201Response**](volumes_create_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## volumes_delete

> volumes_delete(volume_id)
Delete a Block Storage Volume

To delete a block storage volume, destroying all data and removing it from your account, send a DELETE request to `/v2/volumes/$VOLUME_ID`. No response body will be sent back, but the response code will indicate success. Specifically, the response code will be a 204, which means that the action was successful with no returned body data.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**volume_id** | **uuid::Uuid** | The ID of the block storage volume. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## volumes_delete_by_name

> volumes_delete_by_name(name, region)
Delete a Block Storage Volume by Name

Block storage volumes may also be deleted by name by sending a DELETE request with the volume's **name** and the **region slug** for the region it is located in as query parameters to `/v2/volumes?name=$VOLUME_NAME&region=nyc1`. No response body will be sent back, but the response code will indicate success. Specifically, the response code will be a 204, which means that the action was successful with no returned body data.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | The block storage volume's name. |  |
**region** | Option<[**RegionSlug**](.md)> | The slug identifier for the region where the resource is available. |  |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## volumes_get

> models::VolumesCreate201Response volumes_get(volume_id)
Retrieve an Existing Block Storage Volume

To show information about a block storage volume, send a GET request to `/v2/volumes/$VOLUME_ID`.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**volume_id** | **uuid::Uuid** | The ID of the block storage volume. | [required] |

### Return type

[**models::VolumesCreate201Response**](volumes_create_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## volumes_list

> models::VolumesList200Response volumes_list(name, region, per_page, page)
List All Block Storage Volumes

To list all of the block storage volumes available on your account, send a GET request to `/v2/volumes`. ## Filtering Results ### By Region The `region` may be provided as query parameter in order to restrict results to volumes available in a specific region. For example: `/v2/volumes?region=nyc1` ### By Name It is also possible to list volumes on your account that match a specified name. To do so, send a GET request with the volume's name as a query parameter to `/v2/volumes?name=$VOLUME_NAME`. **Note:** You can only create one volume per region with the same name. ### By Name and Region It is also possible to retrieve information about a block storage volume by name. To do so, send a GET request with the volume's name and the region slug for the region it is located in as query parameters to `/v2/volumes?name=$VOLUME_NAME&region=nyc1`.   

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | The block storage volume's name. |  |
**region** | Option<[**RegionSlug**](.md)> | The slug identifier for the region where the resource is available. |  |
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**models::VolumesList200Response**](volumes_list_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

