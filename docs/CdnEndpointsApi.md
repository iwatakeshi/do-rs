# \CdnEndpointsApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cdn_create_endpoint**](CdnEndpointsApi.md#cdn_create_endpoint) | **POST** /v2/cdn/endpoints | Create a New CDN Endpoint
[**cdn_delete_endpoint**](CdnEndpointsApi.md#cdn_delete_endpoint) | **DELETE** /v2/cdn/endpoints/{cdn_id} | Delete a CDN Endpoint
[**cdn_get_endpoint**](CdnEndpointsApi.md#cdn_get_endpoint) | **GET** /v2/cdn/endpoints/{cdn_id} | Retrieve an Existing CDN Endpoint
[**cdn_list_endpoints**](CdnEndpointsApi.md#cdn_list_endpoints) | **GET** /v2/cdn/endpoints | List All CDN Endpoints
[**cdn_purge_cache**](CdnEndpointsApi.md#cdn_purge_cache) | **DELETE** /v2/cdn/endpoints/{cdn_id}/cache | Purge the Cache for an Existing CDN Endpoint
[**cdn_update_endpoints**](CdnEndpointsApi.md#cdn_update_endpoints) | **PUT** /v2/cdn/endpoints/{cdn_id} | Update a CDN Endpoint



## cdn_create_endpoint

> models::CdnCreateEndpoint201Response cdn_create_endpoint(cdn_endpoint)
Create a New CDN Endpoint

To create a new CDN endpoint, send a POST request to `/v2/cdn/endpoints`. The origin attribute must be set to the fully qualified domain name (FQDN) of a DigitalOcean Space. Optionally, the TTL may be configured by setting the `ttl` attribute.  A custom subdomain may be configured by specifying the `custom_domain` and `certificate_id` attributes. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cdn_endpoint** | [**CdnEndpoint**](CdnEndpoint.md) |  | [required] |

### Return type

[**models::CdnCreateEndpoint201Response**](cdn_create_endpoint_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cdn_delete_endpoint

> cdn_delete_endpoint(cdn_id)
Delete a CDN Endpoint

To delete a specific CDN endpoint, send a DELETE request to `/v2/cdn/endpoints/$ENDPOINT_ID`.  A status of 204 will be given. This indicates that the request was processed successfully, but that no response body is needed. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cdn_id** | **uuid::Uuid** | A unique identifier for a CDN endpoint. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cdn_get_endpoint

> models::CdnCreateEndpoint201Response cdn_get_endpoint(cdn_id)
Retrieve an Existing CDN Endpoint

To show information about an existing CDN endpoint, send a GET request to `/v2/cdn/endpoints/$ENDPOINT_ID`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cdn_id** | **uuid::Uuid** | A unique identifier for a CDN endpoint. | [required] |

### Return type

[**models::CdnCreateEndpoint201Response**](cdn_create_endpoint_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cdn_list_endpoints

> models::CdnListEndpoints200Response cdn_list_endpoints(per_page, page)
List All CDN Endpoints

To list all of the CDN endpoints available on your account, send a GET request to `/v2/cdn/endpoints`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**models::CdnListEndpoints200Response**](cdn_list_endpoints_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cdn_purge_cache

> cdn_purge_cache(cdn_id, purge_cache)
Purge the Cache for an Existing CDN Endpoint

To purge cached content from a CDN endpoint, send a DELETE request to `/v2/cdn/endpoints/$ENDPOINT_ID/cache`. The body of the request should include a `files` attribute containing a list of cached file paths to be purged. A path may be for a single file or may contain a wildcard (`*`) to recursively purge all files under a directory. When only a wildcard is provided, all cached  files will be purged. There is a rate limit of 50 files per 20 seconds that can  be purged. CDN endpoints have a rate limit of 5 requests per 10 seconds.  Purging files using a wildcard path counts as a single request against the API's  rate limit. Two identical purge requests cannot be sent at the same time. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cdn_id** | **uuid::Uuid** | A unique identifier for a CDN endpoint. | [required] |
**purge_cache** | [**PurgeCache**](PurgeCache.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cdn_update_endpoints

> models::CdnCreateEndpoint201Response cdn_update_endpoints(cdn_id, update_endpoint)
Update a CDN Endpoint

To update the TTL, certificate ID, or the FQDN of the custom subdomain for an existing CDN endpoint, send a PUT request to `/v2/cdn/endpoints/$ENDPOINT_ID`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cdn_id** | **uuid::Uuid** | A unique identifier for a CDN endpoint. | [required] |
**update_endpoint** | [**UpdateEndpoint**](UpdateEndpoint.md) |  | [required] |

### Return type

[**models::CdnCreateEndpoint201Response**](cdn_create_endpoint_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

