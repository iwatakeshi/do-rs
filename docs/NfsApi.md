# \NfsApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**nfs_create**](NfsApi.md#nfs_create) | **POST** /v2/nfs | Create a new NFS share
[**nfs_delete**](NfsApi.md#nfs_delete) | **DELETE** /v2/nfs/{nfs_id} | Delete an NFS share
[**nfs_delete_snapshot**](NfsApi.md#nfs_delete_snapshot) | **DELETE** /v2/nfs/snapshots/{nfs_snapshot_id} | Delete an NFS snapshot
[**nfs_get**](NfsApi.md#nfs_get) | **GET** /v2/nfs/{nfs_id} | Get an NFS share
[**nfs_get_snapshot**](NfsApi.md#nfs_get_snapshot) | **GET** /v2/nfs/snapshots/{nfs_snapshot_id} | Get an NFS snapshot by ID
[**nfs_list**](NfsApi.md#nfs_list) | **GET** /v2/nfs | List NFS shares per region
[**nfs_list_snapshot**](NfsApi.md#nfs_list_snapshot) | **GET** /v2/nfs/snapshots | List NFS snapshots per region



## nfs_create

> models::NfsCreateResponse nfs_create(nfs_request)
Create a new NFS share

To create a new NFS share, send a POST request to `/v2/nfs`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**nfs_request** | [**NfsRequest**](NfsRequest.md) |  | [required] |

### Return type

[**models::NfsCreateResponse**](nfs_create_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## nfs_delete

> nfs_delete(nfs_id, region)
Delete an NFS share

To delete an NFS share, send a DELETE request to `/v2/nfs/{nfs_id}?region=${region}`.  A successful request will return a `204 No Content` status code. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**nfs_id** | **String** | The unique ID of the NFS share | [required] |
**region** | **String** | The DigitalOcean region slug (e.g., nyc2, atl1) where the NFS share resides. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## nfs_delete_snapshot

> nfs_delete_snapshot(nfs_snapshot_id, region)
Delete an NFS snapshot

To delete an NFS snapshot, send a DELETE request to `/v2/nfs/snapshots/{nfs_snapshot_id}?region=${region}`.  A successful request will return a `204 No Content` status code. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**nfs_snapshot_id** | **String** | The unique ID of the NFS snapshot | [required] |
**region** | **String** | The DigitalOcean region slug (e.g., nyc2, atl1) where the NFS share resides. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## nfs_get

> models::NfsGetResponse nfs_get(nfs_id, region)
Get an NFS share

To get an NFS share, send a GET request to `/v2/nfs/{nfs_id}?region=${region}`.  A successful request will return the NFS share. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**nfs_id** | **String** | The unique ID of the NFS share | [required] |
**region** | **String** | The DigitalOcean region slug (e.g., nyc2, atl1) where the NFS share resides. | [required] |

### Return type

[**models::NfsGetResponse**](nfs_get_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## nfs_get_snapshot

> models::NfsSnapshotGetResponse nfs_get_snapshot(nfs_snapshot_id, region)
Get an NFS snapshot by ID

To get an NFS snapshot, send a GET request to `/v2/nfs/snapshots/{nfs_snapshot_id}?region=${region}`.  A successful request will return the NFS snapshot. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**nfs_snapshot_id** | **String** | The unique ID of the NFS snapshot | [required] |
**region** | **String** | The DigitalOcean region slug (e.g., nyc2, atl1) where the NFS share resides. | [required] |

### Return type

[**models::NfsSnapshotGetResponse**](nfs_snapshot_get_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## nfs_list

> models::NfsListResponse nfs_list(region)
List NFS shares per region

To list NFS shares, send a GET request to `/v2/nfs?region=${region}`.  A successful request will return all NFS shares belonging to the authenticated user. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The DigitalOcean region slug (e.g., nyc2, atl1) where the NFS share resides. | [required] |

### Return type

[**models::NfsListResponse**](nfs_list_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## nfs_list_snapshot

> models::NfsSnapshotListResponse nfs_list_snapshot(region, share_id)
List NFS snapshots per region

To list all NFS snapshots, send a GET request to `/v2/nfs/snapshots?region=${region}&share_id={share_id}`.  A successful request will return all NFS snapshots belonging to the authenticated user in the specified region.  Optionally, you can filter snapshots by a specific NFS share by including the `share_id` query parameter. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | The DigitalOcean region slug (e.g., nyc2, atl1) where the NFS share resides. | [required] |
**share_id** | Option<**String**> | The unique ID of an NFS share. If provided, only snapshots of this specific share will be returned. |  |

### Return type

[**models::NfsSnapshotListResponse**](nfs_snapshot_list_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

