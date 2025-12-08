# \DropletAutoscalePoolsApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**autoscalepools_create**](DropletAutoscalePoolsApi.md#autoscalepools_create) | **POST** /v2/droplets/autoscale | Create a New Autoscale Pool
[**autoscalepools_delete**](DropletAutoscalePoolsApi.md#autoscalepools_delete) | **DELETE** /v2/droplets/autoscale/{autoscale_pool_id} | Delete autoscale pool
[**autoscalepools_delete_dangerous**](DropletAutoscalePoolsApi.md#autoscalepools_delete_dangerous) | **DELETE** /v2/droplets/autoscale/{autoscale_pool_id}/dangerous | Delete autoscale pool and resources
[**autoscalepools_get**](DropletAutoscalePoolsApi.md#autoscalepools_get) | **GET** /v2/droplets/autoscale/{autoscale_pool_id} | Retrieve an Existing Autoscale Pool
[**autoscalepools_list**](DropletAutoscalePoolsApi.md#autoscalepools_list) | **GET** /v2/droplets/autoscale | List All Autoscale Pools
[**autoscalepools_list_history**](DropletAutoscalePoolsApi.md#autoscalepools_list_history) | **GET** /v2/droplets/autoscale/{autoscale_pool_id}/history | List history events
[**autoscalepools_list_members**](DropletAutoscalePoolsApi.md#autoscalepools_list_members) | **GET** /v2/droplets/autoscale/{autoscale_pool_id}/members | List members
[**autoscalepools_update**](DropletAutoscalePoolsApi.md#autoscalepools_update) | **PUT** /v2/droplets/autoscale/{autoscale_pool_id} | Update Autoscale Pool



## autoscalepools_create

> models::AutoscalepoolsCreate202Response autoscalepools_create(autoscale_pool_create)
Create a New Autoscale Pool

To create a new autoscale pool, send a POST request to `/v2/droplets/autoscale` setting the required attributes.  The response body will contain a JSON object with a key called `autoscale_pool` containing the standard attributes for the new autoscale pool. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**autoscale_pool_create** | Option<[**AutoscalePoolCreate**](AutoscalePoolCreate.md)> |  |  |

### Return type

[**models::AutoscalepoolsCreate202Response**](autoscalepools_create_202_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## autoscalepools_delete

> autoscalepools_delete(autoscale_pool_id)
Delete autoscale pool

To destroy an autoscale pool, send a DELETE request to the `/v2/droplets/autoscale/$AUTOSCALE_POOL_ID` endpoint.  A successful response will include a 202 response code and no content.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**autoscale_pool_id** | **String** | A unique identifier for an autoscale pool. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## autoscalepools_delete_dangerous

> autoscalepools_delete_dangerous(autoscale_pool_id, x_dangerous)
Delete autoscale pool and resources

To destroy an autoscale pool and its associated resources (Droplets), send a DELETE request to the `/v2/droplets/autoscale/$AUTOSCALE_POOL_ID/dangerous` endpoint. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**autoscale_pool_id** | **String** | A unique identifier for an autoscale pool. | [required] |
**x_dangerous** | **bool** | Acknowledge this action will destroy the autoscale pool and its associated resources and _can not_ be reversed. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## autoscalepools_get

> models::AutoscalepoolsCreate202Response autoscalepools_get(autoscale_pool_id)
Retrieve an Existing Autoscale Pool

To show information about an individual autoscale pool, send a GET request to `/v2/droplets/autoscale/$AUTOSCALE_POOL_ID`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**autoscale_pool_id** | **String** | A unique identifier for an autoscale pool. | [required] |

### Return type

[**models::AutoscalepoolsCreate202Response**](autoscalepools_create_202_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## autoscalepools_list

> models::AutoscalepoolsList200Response autoscalepools_list(per_page, page, name)
List All Autoscale Pools

To list all autoscale pools in your team, send a GET request to `/v2/droplets/autoscale`. The response body will be a JSON object with a key of `autoscale_pools` containing an array of autoscale pool objects. These each contain the standard autoscale pool attributes. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]
**name** | Option<**String**> | The name of the autoscale pool |  |

### Return type

[**models::AutoscalepoolsList200Response**](autoscalepools_list_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## autoscalepools_list_history

> models::AutoscalepoolsListHistory200Response autoscalepools_list_history(autoscale_pool_id, per_page, page)
List history events

To list all of the scaling history events of an autoscale pool, send a GET request to `/v2/droplets/autoscale/$AUTOSCALE_POOL_ID/history`.  The response body will be a JSON object with a key of `history`. This will be set to an array containing objects each representing a history event.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**autoscale_pool_id** | **String** | A unique identifier for an autoscale pool. | [required] |
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**models::AutoscalepoolsListHistory200Response**](autoscalepools_list_history_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## autoscalepools_list_members

> models::AutoscalepoolsListMembers200Response autoscalepools_list_members(autoscale_pool_id, per_page, page)
List members

To list the Droplets in an autoscale pool, send a GET request to `/v2/droplets/autoscale/$AUTOSCALE_POOL_ID/members`.  The response body will be a JSON object with a key of `droplets`. This will be set to an array containing information about each of the Droplets in the autoscale pool. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**autoscale_pool_id** | **String** | A unique identifier for an autoscale pool. | [required] |
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**models::AutoscalepoolsListMembers200Response**](autoscalepools_list_members_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## autoscalepools_update

> models::AutoscalepoolsCreate202Response autoscalepools_update(autoscale_pool_id, autoscale_pool_create)
Update Autoscale Pool

To update the configuration of an existing autoscale pool, send a PUT request to `/v2/droplets/autoscale/$AUTOSCALE_POOL_ID`. The request must contain a full representation of the autoscale pool including existing attributes.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**autoscale_pool_id** | **String** | A unique identifier for an autoscale pool. | [required] |
**autoscale_pool_create** | Option<[**AutoscalePoolCreate**](AutoscalePoolCreate.md)> |  |  |

### Return type

[**models::AutoscalepoolsCreate202Response**](autoscalepools_create_202_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

