# \SpacesKeysApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**spaces_key_create**](SpacesKeysApi.md#spaces_key_create) | **POST** /v2/spaces/keys | Create a New Spaces Access Key
[**spaces_key_delete**](SpacesKeysApi.md#spaces_key_delete) | **DELETE** /v2/spaces/keys/{access_key} | Delete a Spaces Access Key
[**spaces_key_get**](SpacesKeysApi.md#spaces_key_get) | **GET** /v2/spaces/keys/{access_key} | Get a Spaces Access Key
[**spaces_key_list**](SpacesKeysApi.md#spaces_key_list) | **GET** /v2/spaces/keys | List Spaces Access Keys
[**spaces_key_patch**](SpacesKeysApi.md#spaces_key_patch) | **PATCH** /v2/spaces/keys/{access_key} | Update Spaces Access Keys
[**spaces_key_update**](SpacesKeysApi.md#spaces_key_update) | **PUT** /v2/spaces/keys/{access_key} | Update Spaces Access Keys



## spaces_key_create

> models::SpacesKeyCreate201Response spaces_key_create(key)
Create a New Spaces Access Key

To create a new Spaces Access Key, send a POST request to `/v2/spaces/keys`. At the moment, you cannot mix a fullaccess permission with scoped permissions. A fullaccess permission will be prioritized if fullaccess and scoped permissions are both added. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | [**Key**](Key.md) |  | [required] |

### Return type

[**models::SpacesKeyCreate201Response**](spacesKey_create_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## spaces_key_delete

> spaces_key_delete(access_key)
Delete a Spaces Access Key

To delete a Spaces Access Key, send a DELETE request to `/v2/spaces/keys/$ACCESS_KEY`.  A successful request will return a `204 No Content` status code. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**access_key** | **String** | The access key's ID. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## spaces_key_get

> models::SpacesKeyGet200Response spaces_key_get(access_key)
Get a Spaces Access Key

To get a Spaces Access Key, send a GET request to `/v2/spaces/keys/$ACCESS_KEY`.  A successful request will return the Access Key. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**access_key** | **String** | The access key's ID. | [required] |

### Return type

[**models::SpacesKeyGet200Response**](spacesKey_get_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## spaces_key_list

> models::SpacesKeyList200Response spaces_key_list(per_page, page, sort, sort_direction, name, bucket, permission)
List Spaces Access Keys

To list Spaces Access Key, send a GET request to `/v2/spaces/keys`. Sort parameter must be used with Sort Direction. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]
**sort** | Option<**String**> | The field to sort by. |  |[default to created_at]
**sort_direction** | Option<**String**> | The direction to sort by. Possible values are `asc` or `desc`. |  |[default to desc]
**name** | Option<**String**> | The access key's name. |  |
**bucket** | Option<**String**> | The bucket's name. |  |
**permission** | Option<**String**> | The permission of the access key. Possible values are `read`, `readwrite`, `fullaccess`, or an empty string. |  |

### Return type

[**models::SpacesKeyList200Response**](spacesKey_list_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## spaces_key_patch

> models::SpacesKeyUpdate200Response spaces_key_patch(access_key, key)
Update Spaces Access Keys

To update Spaces Access Key, send a PUT or PATCH request to `/v2/spaces/keys/$ACCESS_KEY`. At the moment, you cannot convert a fullaccess key to a scoped key or vice versa. You can only update the name of the key. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**access_key** | **String** | The access key's ID. | [required] |
**key** | [**Key**](Key.md) |  | [required] |

### Return type

[**models::SpacesKeyUpdate200Response**](spacesKey_update_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## spaces_key_update

> models::SpacesKeyUpdate200Response spaces_key_update(access_key, key)
Update Spaces Access Keys

To update Spaces Access Key, send a PUT or PATCH request to `/v2/spaces/keys/$ACCESS_KEY`. At the moment, you cannot convert a fullaccess key to a scoped key or vice versa. You can only update the name of the key. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**access_key** | **String** | The access key's ID. | [required] |
**key** | [**Key**](Key.md) |  | [required] |

### Return type

[**models::SpacesKeyUpdate200Response**](spacesKey_update_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

