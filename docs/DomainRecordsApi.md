# \DomainRecordsApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**domains_create_record**](DomainRecordsApi.md#domains_create_record) | **POST** /v2/domains/{domain_name}/records | Create a New Domain Record
[**domains_delete_record**](DomainRecordsApi.md#domains_delete_record) | **DELETE** /v2/domains/{domain_name}/records/{domain_record_id} | Delete a Domain Record
[**domains_get_record**](DomainRecordsApi.md#domains_get_record) | **GET** /v2/domains/{domain_name}/records/{domain_record_id} | Retrieve an Existing Domain Record
[**domains_list_records**](DomainRecordsApi.md#domains_list_records) | **GET** /v2/domains/{domain_name}/records | List All Domain Records
[**domains_patch_record**](DomainRecordsApi.md#domains_patch_record) | **PATCH** /v2/domains/{domain_name}/records/{domain_record_id} | Update a Domain Record
[**domains_update_record**](DomainRecordsApi.md#domains_update_record) | **PUT** /v2/domains/{domain_name}/records/{domain_record_id} | Update a Domain Record



## domains_create_record

> models::DomainsCreateRecord201Response domains_create_record(domain_name, domains_create_record_request)
Create a New Domain Record

To create a new record to a domain, send a POST request to `/v2/domains/$DOMAIN_NAME/records`.  The request must include all of the required fields for the domain record type being added.  See the [attribute table](#tag/Domain-Records) for details regarding record types and their respective required attributes. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_name** | **String** | The name of the domain itself. | [required] |
**domains_create_record_request** | Option<[**DomainsCreateRecordRequest**](DomainsCreateRecordRequest.md)> |  |  |

### Return type

[**models::DomainsCreateRecord201Response**](domains_create_record_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domains_delete_record

> domains_delete_record(domain_name, domain_record_id)
Delete a Domain Record

To delete a record for a domain, send a DELETE request to `/v2/domains/$DOMAIN_NAME/records/$DOMAIN_RECORD_ID`.  The record will be deleted and the response status will be a 204. This indicates a successful request with no body returned. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_name** | **String** | The name of the domain itself. | [required] |
**domain_record_id** | **i32** | The unique identifier of the domain record. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domains_get_record

> models::DomainsGetRecord200Response domains_get_record(domain_name, domain_record_id)
Retrieve an Existing Domain Record

To retrieve a specific domain record, send a GET request to `/v2/domains/$DOMAIN_NAME/records/$RECORD_ID`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_name** | **String** | The name of the domain itself. | [required] |
**domain_record_id** | **i32** | The unique identifier of the domain record. | [required] |

### Return type

[**models::DomainsGetRecord200Response**](domains_get_record_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domains_list_records

> models::DomainsListRecords200Response domains_list_records(domain_name, name, r#type, per_page, page)
List All Domain Records

To get a listing of all records configured for a domain, send a GET request to `/v2/domains/$DOMAIN_NAME/records`. The list of records returned can be filtered by using the `name` and `type` query parameters. For example, to only include A records for a domain, send a GET request to `/v2/domains/$DOMAIN_NAME/records?type=A`. `name` must be a fully qualified record name. For example, to only include records matching `sub.example.com`, send a GET request to `/v2/domains/$DOMAIN_NAME/records?name=sub.example.com`. Both name and type may be used together.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_name** | **String** | The name of the domain itself. | [required] |
**name** | Option<**String**> | A fully qualified record name. For example, to only include records matching sub.example.com, send a GET request to `/v2/domains/$DOMAIN_NAME/records?name=sub.example.com`. |  |
**r#type** | Option<**String**> | The type of the DNS record. For example: A, CNAME, TXT, ... |  |
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**models::DomainsListRecords200Response**](domains_list_records_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domains_patch_record

> models::DomainsGetRecord200Response domains_patch_record(domain_name, domain_record_id, domain_record)
Update a Domain Record

To update an existing record, send a PATCH request to `/v2/domains/$DOMAIN_NAME/records/$DOMAIN_RECORD_ID`. Any attribute valid for the record type can be set to a new value for the record.  See the [attribute table](#tag/Domain-Records) for details regarding record types and their respective attributes. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_name** | **String** | The name of the domain itself. | [required] |
**domain_record_id** | **i32** | The unique identifier of the domain record. | [required] |
**domain_record** | Option<[**DomainRecord**](DomainRecord.md)> |  |  |

### Return type

[**models::DomainsGetRecord200Response**](domains_get_record_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domains_update_record

> models::DomainsGetRecord200Response domains_update_record(domain_name, domain_record_id, domain_record)
Update a Domain Record

To update an existing record, send a PUT request to `/v2/domains/$DOMAIN_NAME/records/$DOMAIN_RECORD_ID`. Any attribute valid for the record type can be set to a new value for the record.  See the [attribute table](#tag/Domain-Records) for details regarding record types and their respective attributes. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_name** | **String** | The name of the domain itself. | [required] |
**domain_record_id** | **i32** | The unique identifier of the domain record. | [required] |
**domain_record** | Option<[**DomainRecord**](DomainRecord.md)> |  |  |

### Return type

[**models::DomainsGetRecord200Response**](domains_get_record_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

