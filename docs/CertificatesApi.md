# \CertificatesApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**certificates_create**](CertificatesApi.md#certificates_create) | **POST** /v2/certificates | Create a New Certificate
[**certificates_delete**](CertificatesApi.md#certificates_delete) | **DELETE** /v2/certificates/{certificate_id} | Delete a Certificate
[**certificates_get**](CertificatesApi.md#certificates_get) | **GET** /v2/certificates/{certificate_id} | Retrieve an Existing Certificate
[**certificates_list**](CertificatesApi.md#certificates_list) | **GET** /v2/certificates | List All Certificates



## certificates_create

> models::CertificatesCreate201Response certificates_create(certificates_create_request)
Create a New Certificate

To upload new SSL certificate which you have previously generated, send a POST request to `/v2/certificates`.  When uploading a user-generated certificate, the `private_key`, `leaf_certificate`, and optionally the `certificate_chain` attributes should be provided. The type must be set to `custom`.  When using Let's Encrypt to create a certificate, the `dns_names` attribute must be provided, and the type must be set to `lets_encrypt`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificates_create_request** | [**CertificatesCreateRequest**](CertificatesCreateRequest.md) |  | [required] |

### Return type

[**models::CertificatesCreate201Response**](certificates_create_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## certificates_delete

> certificates_delete(certificate_id)
Delete a Certificate

To delete a specific certificate, send a DELETE request to `/v2/certificates/$CERTIFICATE_ID`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificate_id** | **uuid::Uuid** | A unique identifier for a certificate. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## certificates_get

> models::CertificatesCreate201Response certificates_get(certificate_id)
Retrieve an Existing Certificate

To show information about an existing certificate, send a GET request to `/v2/certificates/$CERTIFICATE_ID`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificate_id** | **uuid::Uuid** | A unique identifier for a certificate. | [required] |

### Return type

[**models::CertificatesCreate201Response**](certificates_create_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## certificates_list

> models::CertificatesList200Response certificates_list(per_page, page, name)
List All Certificates

To list all of the certificates available on your account, send a GET request to `/v2/certificates`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]
**name** | Option<**String**> | Name of expected certificate |  |[default to ]

### Return type

[**models::CertificatesList200Response**](certificates_list_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

