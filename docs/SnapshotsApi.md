# \SnapshotsApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**snapshots_delete**](SnapshotsApi.md#snapshots_delete) | **DELETE** /v2/snapshots/{snapshot_id} | Delete a Snapshot
[**snapshots_get**](SnapshotsApi.md#snapshots_get) | **GET** /v2/snapshots/{snapshot_id} | Retrieve an Existing Snapshot
[**snapshots_list**](SnapshotsApi.md#snapshots_list) | **GET** /v2/snapshots | List All Snapshots



## snapshots_delete

> snapshots_delete(snapshot_id)
Delete a Snapshot

Both Droplet and volume snapshots are managed through the `/v2/snapshots/` endpoint. To delete a snapshot, send a DELETE request to `/v2/snapshots/$SNAPSHOT_ID`.  A status of 204 will be given. This indicates that the request was processed successfully, but that no response body is needed. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**snapshot_id** | **String** | Either the ID of an existing snapshot. This will be an integer for a Droplet snapshot or a string for a volume snapshot. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## snapshots_get

> models::SnapshotsGet200Response snapshots_get(snapshot_id)
Retrieve an Existing Snapshot

To retrieve information about a snapshot, send a GET request to `/v2/snapshots/$SNAPSHOT_ID`.  The response will be a JSON object with a key called `snapshot`. The value of this will be an snapshot object containing the standard snapshot attributes. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**snapshot_id** | **String** | Either the ID of an existing snapshot. This will be an integer for a Droplet snapshot or a string for a volume snapshot. | [required] |

### Return type

[**models::SnapshotsGet200Response**](snapshots_get_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## snapshots_list

> models::SnapshotsList200Response snapshots_list(per_page, page, resource_type)
List All Snapshots

To list all of the snapshots available on your account, send a GET request to `/v2/snapshots`.  The response will be a JSON object with a key called `snapshots`. This will be set to an array of `snapshot` objects, each of which will contain the standard snapshot attributes.  ### Filtering Results by Resource Type  It's possible to request filtered results by including certain query parameters.  #### List Droplet Snapshots  To retrieve only snapshots based on Droplets, include the `resource_type` query parameter set to `droplet`. For example, `/v2/snapshots?resource_type=droplet`.  #### List Volume Snapshots  To retrieve only snapshots based on volumes, include the `resource_type` query parameter set to `volume`. For example, `/v2/snapshots?resource_type=volume`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]
**resource_type** | Option<**String**> | Used to filter snapshots by a resource type. |  |

### Return type

[**models::SnapshotsList200Response**](snapshots_list_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

