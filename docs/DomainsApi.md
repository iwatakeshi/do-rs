# \DomainsApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**domains_create**](DomainsApi.md#domains_create) | **POST** /v2/domains | Create a New Domain
[**domains_delete**](DomainsApi.md#domains_delete) | **DELETE** /v2/domains/{domain_name} | Delete a Domain
[**domains_get**](DomainsApi.md#domains_get) | **GET** /v2/domains/{domain_name} | Retrieve an Existing Domain
[**domains_list**](DomainsApi.md#domains_list) | **GET** /v2/domains | List All Domains



## domains_create

> models::DomainsCreate201Response domains_create(domain)
Create a New Domain

To create a new domain, send a POST request to `/v2/domains`. Set the \"name\" attribute to the domain name you are adding. Optionally, you may set the \"ip_address\" attribute, and an A record will be automatically created pointing to the apex domain. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | Option<[**Domain**](Domain.md)> |  |  |

### Return type

[**models::DomainsCreate201Response**](domains_create_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domains_delete

> domains_delete(domain_name)
Delete a Domain

To delete a domain, send a DELETE request to `/v2/domains/$DOMAIN_NAME`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_name** | **String** | The name of the domain itself. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domains_get

> models::DomainsGet200Response domains_get(domain_name)
Retrieve an Existing Domain

To get details about a specific domain, send a GET request to `/v2/domains/$DOMAIN_NAME`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_name** | **String** | The name of the domain itself. | [required] |

### Return type

[**models::DomainsGet200Response**](domains_get_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domains_list

> models::DomainsList200Response domains_list(per_page, page)
List All Domains

To retrieve a list of all of the domains in your account, send a GET request to `/v2/domains`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**models::DomainsList200Response**](domains_list_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

