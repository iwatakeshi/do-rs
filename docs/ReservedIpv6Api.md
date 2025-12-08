# \ReservedIpv6Api

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**reserved_ipv6_create**](ReservedIpv6Api.md#reserved_ipv6_create) | **POST** /v2/reserved_ipv6 | Create a New Reserved IPv6
[**reserved_ipv6_delete**](ReservedIpv6Api.md#reserved_ipv6_delete) | **DELETE** /v2/reserved_ipv6/{reserved_ipv6} | Delete a Reserved IPv6
[**reserved_ipv6_get**](ReservedIpv6Api.md#reserved_ipv6_get) | **GET** /v2/reserved_ipv6/{reserved_ipv6} | Retrieve an Existing Reserved IPv6
[**reserved_ipv6_list**](ReservedIpv6Api.md#reserved_ipv6_list) | **GET** /v2/reserved_ipv6 | List All Reserved IPv6s



## reserved_ipv6_create

> models::ReservedIpv6Create201Response reserved_ipv6_create(reserved_ipv6_create)
Create a New Reserved IPv6

On creation, a reserved IPv6 must be reserved to a region. * To create a new reserved IPv6 reserved to a region, send a POST request to   `/v2/reserved_ipv6` with the `region_slug` attribute.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reserved_ipv6_create** | [**ReservedIpv6Create**](ReservedIpv6Create.md) |  | [required] |

### Return type

[**models::ReservedIpv6Create201Response**](reservedIPv6_create_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reserved_ipv6_delete

> reserved_ipv6_delete(reserved_ipv6)
Delete a Reserved IPv6

To delete a reserved IP and remove it from your account, send a DELETE request to `/v2/reserved_ipv6/$RESERVED_IPV6`.  A successful request will receive a 204 status code with no body in response. This indicates that the request was processed successfully. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reserved_ipv6** | **String** | A reserved IPv6 address. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reserved_ipv6_get

> models::ReservedIpv6Get200Response reserved_ipv6_get(reserved_ipv6)
Retrieve an Existing Reserved IPv6

To show information about a reserved IPv6, send a GET request to `/v2/reserved_ipv6/$RESERVED_IPV6`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reserved_ipv6** | **String** | A reserved IPv6 address. | [required] |

### Return type

[**models::ReservedIpv6Get200Response**](reservedIPv6_get_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reserved_ipv6_list

> models::ReservedIpv6List200Response reserved_ipv6_list(per_page, page)
List All Reserved IPv6s

To list all of the reserved IPv6s available on your account, send a GET request to `/v2/reserved_ipv6`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**models::ReservedIpv6List200Response**](reservedIPv6_list_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

