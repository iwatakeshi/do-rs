# \AddOnsApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**addons_create**](AddOnsApi.md#addons_create) | **POST** /v2/add-ons/saas | Create/Provision a New Add-on Resource
[**addons_delete**](AddOnsApi.md#addons_delete) | **DELETE** /v2/add-ons/saas/{resource_uuid} | Delete/Deprovision an Add-on Resource
[**addons_get**](AddOnsApi.md#addons_get) | **GET** /v2/add-ons/saas/{resource_uuid} | Get details on an Add-On Resource
[**addons_get_app**](AddOnsApi.md#addons_get_app) | **GET** /v2/add-ons/apps | List Available Add-On Applications
[**addons_get_app_metadata**](AddOnsApi.md#addons_get_app_metadata) | **GET** /v2/add-ons/apps/{app_slug}/metadata | Get Metadata for an Add-On Application
[**addons_list**](AddOnsApi.md#addons_list) | **GET** /v2/add-ons/saas | List all Add-On Resources
[**addons_patch**](AddOnsApi.md#addons_patch) | **PATCH** /v2/add-ons/saas/{resource_uuid} | Update the name for an Add-On Resource
[**addons_patch_plan**](AddOnsApi.md#addons_patch_plan) | **PATCH** /v2/add-ons/saas/{resource_uuid}/plan | Update the plan for an Add-On Resource



## addons_create

> models::AddonsCreate200Response addons_create(addons_resource_new)
Create/Provision a New Add-on Resource

To create an add-on resource, send a POST request to `/v2/add-ons/saas` with required parameters. Some add-ons require additional metadata to be provided in the request body. To find out what metadata is required for a specific add-on, send a GET request to `/v2/add-ons/apps/{app_slug}/metadata`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**addons_resource_new** | [**AddonsResourceNew**](AddonsResourceNew.md) |  | [required] |

### Return type

[**models::AddonsCreate200Response**](addons_create_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## addons_delete

> addons_delete(resource_uuid)
Delete/Deprovision an Add-on Resource

To delete an add-on resource, send a DELETE request to `/v2/add-ons/saas/{resource_uuid}` with the UUID of the resource to delete.  You cannot retrieve the resource after it has been deleted. The response indicates a request was sent to the 3rd party add-on provider to delete the resource. You will no longer be billed for this resource. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**resource_uuid** | **uuid::Uuid** | A unique identifier for the add-on resource. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## addons_get

> models::AddonsCreate200Response addons_get(resource_uuid)
Get details on an Add-On Resource

To fetch details of a specific Add-On Resource, send a GET request to `/v2/add-ons/saas/{resource_uuid}`. Replace `{resource_uuid}` with the UUID of the resource you want to retrieve. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**resource_uuid** | **String** | The UUID of the add-on resource to retrieve. | [required] |

### Return type

[**models::AddonsCreate200Response**](addons_create_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## addons_get_app

> models::AddonsGetApp200Response addons_get_app()
List Available Add-On Applications

To fetch details of all available Add-On Applications, send a GET request to `/v2/add-ons/apps`. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AddonsGetApp200Response**](addons_get_app_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## addons_get_app_metadata

> models::AddonsGetAppMetadata200Response addons_get_app_metadata(app_slug)
Get Metadata for an Add-On Application

To find out what metadata is required for a specific add-on, send a GET request to `/v2/add-ons/apps/{app_slug}/metadata`. Metadata varies by application. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_slug** | **String** | The slug identifier for the application whose metadata is being requested. | [required] |

### Return type

[**models::AddonsGetAppMetadata200Response**](addons_get_app_metadata_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## addons_list

> models::AddonsList200Response addons_list()
List all Add-On Resources

To fetch all Add-On Resources under your team, send a GET request to `/v2/add-ons/saas`. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AddonsList200Response**](addons_list_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## addons_patch

> models::AddonsCreate200Response addons_patch(resource_uuid, addons_patch_request)
Update the name for an Add-On Resource

To change the name of an Add-On Resource, send a PATCH request to `/v2/add-ons/saas/{resource_uuid}`. Replace `{resource_uuid}` with the UUID of the resource for which you want to change the name. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**resource_uuid** | **String** | The UUID of the add-on resource to rename. | [required] |
**addons_patch_request** | [**AddonsPatchRequest**](AddonsPatchRequest.md) |  | [required] |

### Return type

[**models::AddonsCreate200Response**](addons_create_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## addons_patch_plan

> models::AddonsCreate200Response addons_patch_plan(resource_uuid, addons_patch_plan_request)
Update the plan for an Add-On Resource

To change the plan associated with an Add-On Resource, send a PATCH request to `/v2/add-ons/saas/{resource_uuid}/plan`. Replace `{resource_uuid}` with the UUID of the resource for which you want to change the plan. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**resource_uuid** | **String** | The UUID of the add-on resource to update. | [required] |
**addons_patch_plan_request** | [**AddonsPatchPlanRequest**](AddonsPatchPlanRequest.md) |  | [required] |

### Return type

[**models::AddonsCreate200Response**](addons_create_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

