# \ByoipPrefixesApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**byoip_prefixes_create**](ByoipPrefixesApi.md#byoip_prefixes_create) | **POST** /v2/byoip_prefixes | Create a BYOIP Prefix
[**byoip_prefixes_delete**](ByoipPrefixesApi.md#byoip_prefixes_delete) | **DELETE** /v2/byoip_prefixes/{byoip_prefix_uuid} | Delete a BYOIP Prefix
[**byoip_prefixes_get**](ByoipPrefixesApi.md#byoip_prefixes_get) | **GET** /v2/byoip_prefixes/{byoip_prefix_uuid} | Get a BYOIP Prefix
[**byoip_prefixes_list**](ByoipPrefixesApi.md#byoip_prefixes_list) | **GET** /v2/byoip_prefixes | List BYOIP Prefixes
[**byoip_prefixes_list_resources**](ByoipPrefixesApi.md#byoip_prefixes_list_resources) | **GET** /v2/byoip_prefixes/{byoip_prefix_uuid}/ips | List BYOIP Prefix Resources
[**byoip_prefixes_patch**](ByoipPrefixesApi.md#byoip_prefixes_patch) | **PATCH** /v2/byoip_prefixes/{byoip_prefix_uuid} | Update a BYOIP Prefix



## byoip_prefixes_create

> models::ByoipPrefixesCreate202Response byoip_prefixes_create(byoip_prefix_create)
Create a BYOIP Prefix

To create a BYOIP prefix, send a POST request to `/v2/byoip_prefixes`.  A successful request will initiate the process of bringing your BYOIP Prefix into your account. The response will include the details of the created prefix, including its UUID and status. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**byoip_prefix_create** | [**ByoipPrefixCreate**](ByoipPrefixCreate.md) |  | [required] |

### Return type

[**models::ByoipPrefixesCreate202Response**](byoipPrefixes_create_202_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## byoip_prefixes_delete

> byoip_prefixes_delete(byoip_prefix_uuid)
Delete a BYOIP Prefix

To delete a BYOIP prefix and remove it from your account, send a DELETE request to `/v2/byoip_prefixes/$byoip_prefix_uuid`.  A successful request will receive a 202 status code with no body in response. This indicates that the request was accepted and the prefix is being deleted. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**byoip_prefix_uuid** | **uuid::Uuid** | The unique identifier for the BYOIP Prefix. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## byoip_prefixes_get

> models::ByoipPrefixesGet200Response byoip_prefixes_get(byoip_prefix_uuid)
Get a BYOIP Prefix

To get a BYOIP prefix, send a GET request to `/v2/byoip_prefixes/$byoip_prefix_uuid`.   A successful response will return the details of the specified BYOIP prefix. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**byoip_prefix_uuid** | **uuid::Uuid** | The unique identifier for the BYOIP Prefix. | [required] |

### Return type

[**models::ByoipPrefixesGet200Response**](byoipPrefixes_get_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## byoip_prefixes_list

> models::ByoipPrefixesList200Response byoip_prefixes_list(per_page, page)
List BYOIP Prefixes

To list all BYOIP prefixes, send a GET request to `/v2/byoip_prefixes`. A successful response will return a list of all BYOIP prefixes associated with the account. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**models::ByoipPrefixesList200Response**](byoipPrefixes_list_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## byoip_prefixes_list_resources

> models::ByoipPrefixesListResources200Response byoip_prefixes_list_resources(byoip_prefix_uuid, per_page, page)
List BYOIP Prefix Resources

To list resources associated with BYOIP prefixes, send a GET request to `/v2/byoip_prefixes/{byoip_prefix_uuid}/ips`.  A successful response will return a list of resources associated with the specified BYOIP prefix. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**byoip_prefix_uuid** | **uuid::Uuid** | The unique identifier for the BYOIP Prefix. | [required] |
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**models::ByoipPrefixesListResources200Response**](byoipPrefixes_list_resources_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## byoip_prefixes_patch

> models::ByoipPrefixesGet200Response byoip_prefixes_patch(byoip_prefix_uuid, byoip_prefix_update)
Update a BYOIP Prefix

To update a BYOIP prefix, send a PATCH request to `/v2/byoip_prefixes/$byoip_prefix_uuid`.  Currently, you can update the advertisement status of the prefix. The response will include the updated details of the prefix. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**byoip_prefix_uuid** | **uuid::Uuid** | A unique identifier for a BYOIP prefix. | [required] |
**byoip_prefix_update** | [**ByoipPrefixUpdate**](ByoipPrefixUpdate.md) |  | [required] |

### Return type

[**models::ByoipPrefixesGet200Response**](byoipPrefixes_get_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

