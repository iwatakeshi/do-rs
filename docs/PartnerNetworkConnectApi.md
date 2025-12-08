# \PartnerNetworkConnectApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**partner_attachments_create**](PartnerNetworkConnectApi.md#partner_attachments_create) | **POST** /v2/partner_network_connect/attachments | Create a new partner attachment
[**partner_attachments_create_service_key**](PartnerNetworkConnectApi.md#partner_attachments_create_service_key) | **POST** /v2/partner_network_connect/attachments/{pa_id}/service_key | Regenerate the service key for the partner attachment
[**partner_attachments_delete**](PartnerNetworkConnectApi.md#partner_attachments_delete) | **DELETE** /v2/partner_network_connect/attachments/{pa_id} | Delete an existing partner attachment
[**partner_attachments_get**](PartnerNetworkConnectApi.md#partner_attachments_get) | **GET** /v2/partner_network_connect/attachments/{pa_id} | Retrieve an existing partner attachment
[**partner_attachments_get_bgp_auth_key**](PartnerNetworkConnectApi.md#partner_attachments_get_bgp_auth_key) | **GET** /v2/partner_network_connect/attachments/{pa_id}/bgp_auth_key | Get current BGP auth key for the partner attachment
[**partner_attachments_get_service_key**](PartnerNetworkConnectApi.md#partner_attachments_get_service_key) | **GET** /v2/partner_network_connect/attachments/{pa_id}/service_key | Get the current service key for the partner attachment
[**partner_attachments_list**](PartnerNetworkConnectApi.md#partner_attachments_list) | **GET** /v2/partner_network_connect/attachments | List all partner attachments
[**partner_attachments_list_remote_routes**](PartnerNetworkConnectApi.md#partner_attachments_list_remote_routes) | **GET** /v2/partner_network_connect/attachments/{pa_id}/remote_routes | List remote routes for a partner attachment
[**partner_attachments_patch**](PartnerNetworkConnectApi.md#partner_attachments_patch) | **PATCH** /v2/partner_network_connect/attachments/{pa_id} | Update an existing partner attachment



## partner_attachments_create

> models::PartnerAttachmentsCreate202Response partner_attachments_create(partner_attachment_writable)
Create a new partner attachment

To create a new partner attachment, send a `POST` request to `/v2/partner_network_connect/attachments` with a JSON object containing the required configuration details. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**partner_attachment_writable** | Option<[**PartnerAttachmentWritable**](PartnerAttachmentWritable.md)> |  |  |

### Return type

[**models::PartnerAttachmentsCreate202Response**](partnerAttachments_create_202_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## partner_attachments_create_service_key

> serde_json::Value partner_attachments_create_service_key(pa_id)
Regenerate the service key for the partner attachment

This operation generates a new service key for the specified partner attachment. The operation is asynchronous, and the response is an empty JSON object returned with a 202 status code. To poll for the new service key, send a `GET` request to `/v2/partner_network_connect/attachments/{pa_id}/service_key`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pa_id** | **String** | A unique identifier for a partner attachment. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## partner_attachments_delete

> models::PartnerAttachmentsDelete202Response partner_attachments_delete(pa_id)
Delete an existing partner attachment

To delete an existing partner attachment, send a `DELETE` request to `/v2/partner_network_connect/attachments/{pa_id}`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pa_id** | **String** | A unique identifier for a partner attachment. | [required] |

### Return type

[**models::PartnerAttachmentsDelete202Response**](partnerAttachments_delete_202_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## partner_attachments_get

> models::PartnerAttachmentsCreate202Response partner_attachments_get(pa_id)
Retrieve an existing partner attachment

To get the details of a partner attachment, send a `GET` request to `/v2/partner_network_connect/attachments/{pa_id}`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pa_id** | **String** | A unique identifier for a partner attachment. | [required] |

### Return type

[**models::PartnerAttachmentsCreate202Response**](partnerAttachments_create_202_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## partner_attachments_get_bgp_auth_key

> models::PartnerAttachmentsGetBgpAuthKey200Response partner_attachments_get_bgp_auth_key(pa_id)
Get current BGP auth key for the partner attachment

To get the current BGP auth key for a partner attachment, send a `GET` request to `/v2/partner_network_connect/attachments/{pa_id}/bgp_auth_key`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pa_id** | **String** | A unique identifier for a partner attachment. | [required] |

### Return type

[**models::PartnerAttachmentsGetBgpAuthKey200Response**](partnerAttachments_get_bgp_auth_key_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## partner_attachments_get_service_key

> models::PartnerAttachmentsGetServiceKey200Response partner_attachments_get_service_key(pa_id)
Get the current service key for the partner attachment

To get the current service key for a partner attachment, send a `GET` request to `/v2/partner_network_connect/attachments/{pa_id}/service_key`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pa_id** | **String** | A unique identifier for a partner attachment. | [required] |

### Return type

[**models::PartnerAttachmentsGetServiceKey200Response**](partnerAttachments_get_service_key_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## partner_attachments_list

> models::PartnerAttachmentsList200Response partner_attachments_list(per_page, page)
List all partner attachments

To list all of the Partner Attachments on your account, send a `GET` request to `/v2/partner_network_connect/attachments`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**models::PartnerAttachmentsList200Response**](partnerAttachments_list_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## partner_attachments_list_remote_routes

> models::PartnerAttachmentsListRemoteRoutes200Response partner_attachments_list_remote_routes(pa_id, per_page, page)
List remote routes for a partner attachment

To list all remote routes associated with a partner attachment, send a `GET` request to `/v2/partner_network_connect/attachments/{pa_id}/remote_routes`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pa_id** | **String** | A unique identifier for a partner attachment. | [required] |
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**models::PartnerAttachmentsListRemoteRoutes200Response**](partnerAttachments_list_remote_routes_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## partner_attachments_patch

> models::PartnerAttachmentsCreate202Response partner_attachments_patch(pa_id, partner_attachment_updatable)
Update an existing partner attachment

To update an existing partner attachment, send a `PATCH` request to `/v2/partner_network_connect/attachments/{pa_id}` with a JSON object containing the fields to be updated. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pa_id** | **String** | A unique identifier for a partner attachment. | [required] |
**partner_attachment_updatable** | Option<[**PartnerAttachmentUpdatable**](PartnerAttachmentUpdatable.md)> |  |  |

### Return type

[**models::PartnerAttachmentsCreate202Response**](partnerAttachments_create_202_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

