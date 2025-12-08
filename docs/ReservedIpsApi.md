# \ReservedIpsApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**reserved_ips_create**](ReservedIpsApi.md#reserved_ips_create) | **POST** /v2/reserved_ips | Create a New Reserved IP
[**reserved_ips_delete**](ReservedIpsApi.md#reserved_ips_delete) | **DELETE** /v2/reserved_ips/{reserved_ip} | Delete a Reserved IP
[**reserved_ips_get**](ReservedIpsApi.md#reserved_ips_get) | **GET** /v2/reserved_ips/{reserved_ip} | Retrieve an Existing Reserved IP
[**reserved_ips_list**](ReservedIpsApi.md#reserved_ips_list) | **GET** /v2/reserved_ips | List All Reserved IPs



## reserved_ips_create

> models::ReservedIpsCreate202Response reserved_ips_create(reserved_ip_create)
Create a New Reserved IP

On creation, a reserved IP must be either assigned to a Droplet or reserved to a region. * To create a new reserved IP assigned to a Droplet, send a POST   request to `/v2/reserved_ips` with the `droplet_id` attribute.  * To create a new reserved IP reserved to a region, send a POST request to   `/v2/reserved_ips` with the `region` attribute.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reserved_ip_create** | [**ReservedIpCreate**](ReservedIpCreate.md) |  | [required] |

### Return type

[**models::ReservedIpsCreate202Response**](reservedIPs_create_202_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reserved_ips_delete

> reserved_ips_delete(reserved_ip)
Delete a Reserved IP

To delete a reserved IP and remove it from your account, send a DELETE request to `/v2/reserved_ips/$RESERVED_IP_ADDR`.  A successful request will receive a 204 status code with no body in response. This indicates that the request was processed successfully. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reserved_ip** | **String** | A reserved IP address. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reserved_ips_get

> models::ReservedIpsGet200Response reserved_ips_get(reserved_ip)
Retrieve an Existing Reserved IP

To show information about a reserved IP, send a GET request to `/v2/reserved_ips/$RESERVED_IP_ADDR`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reserved_ip** | **String** | A reserved IP address. | [required] |

### Return type

[**models::ReservedIpsGet200Response**](reservedIPs_get_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reserved_ips_list

> models::ReservedIpsList200Response reserved_ips_list(per_page, page)
List All Reserved IPs

To list all of the reserved IPs available on your account, send a GET request to `/v2/reserved_ips`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**models::ReservedIpsList200Response**](reservedIPs_list_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

