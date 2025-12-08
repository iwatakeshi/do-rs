# \FunctionsApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**functions_create_namespace**](FunctionsApi.md#functions_create_namespace) | **POST** /v2/functions/namespaces | Create Namespace
[**functions_create_trigger**](FunctionsApi.md#functions_create_trigger) | **POST** /v2/functions/namespaces/{namespace_id}/triggers | Create Trigger
[**functions_delete_namespace**](FunctionsApi.md#functions_delete_namespace) | **DELETE** /v2/functions/namespaces/{namespace_id} | Delete Namespace
[**functions_delete_trigger**](FunctionsApi.md#functions_delete_trigger) | **DELETE** /v2/functions/namespaces/{namespace_id}/triggers/{trigger_name} | Delete Trigger
[**functions_get_namespace**](FunctionsApi.md#functions_get_namespace) | **GET** /v2/functions/namespaces/{namespace_id} | Get Namespace
[**functions_get_trigger**](FunctionsApi.md#functions_get_trigger) | **GET** /v2/functions/namespaces/{namespace_id}/triggers/{trigger_name} | Get Trigger
[**functions_list_namespaces**](FunctionsApi.md#functions_list_namespaces) | **GET** /v2/functions/namespaces | List Namespaces
[**functions_list_triggers**](FunctionsApi.md#functions_list_triggers) | **GET** /v2/functions/namespaces/{namespace_id}/triggers | List Triggers
[**functions_update_trigger**](FunctionsApi.md#functions_update_trigger) | **PUT** /v2/functions/namespaces/{namespace_id}/triggers/{trigger_name} | Update Trigger



## functions_create_namespace

> models::FunctionsCreateNamespace200Response functions_create_namespace(create_namespace)
Create Namespace

Creates a new serverless functions namespace in the desired region and associates it with the provided label. A namespace is a collection of functions and their associated packages, triggers, and project specifications. To create a namespace, send a POST request to `/v2/functions/namespaces` with the `region` and `label` properties.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_namespace** | [**CreateNamespace**](CreateNamespace.md) |  | [required] |

### Return type

[**models::FunctionsCreateNamespace200Response**](functions_create_namespace_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## functions_create_trigger

> models::FunctionsCreateTrigger200Response functions_create_trigger(namespace_id, create_trigger)
Create Trigger

Creates a new trigger for a given function in a namespace. To create a trigger, send a POST request to `/v2/functions/namespaces/$NAMESPACE_ID/triggers` with the `name`, `function`, `type`, `is_enabled` and `scheduled_details` properties.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace_id** | **String** | The ID of the namespace to be managed. | [required] |
**create_trigger** | [**CreateTrigger**](CreateTrigger.md) |  | [required] |

### Return type

[**models::FunctionsCreateTrigger200Response**](functions_create_trigger_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## functions_delete_namespace

> functions_delete_namespace(namespace_id)
Delete Namespace

Deletes the given namespace.  When a namespace is deleted all assets, in the namespace are deleted, this includes packages, functions and triggers. Deleting a namespace is a destructive operation and assets in the namespace are not recoverable after deletion. Some metadata is retained, such as activations, or soft deleted for reporting purposes. To delete namespace, send a DELETE request to `/v2/functions/namespaces/$NAMESPACE_ID`. A successful deletion returns a 204 response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace_id** | **String** | The ID of the namespace to be managed. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## functions_delete_trigger

> functions_delete_trigger(namespace_id, trigger_name)
Delete Trigger

Deletes the given trigger. To delete trigger, send a DELETE request to `/v2/functions/namespaces/$NAMESPACE_ID/triggers/$TRIGGER_NAME`. A successful deletion returns a 204 response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace_id** | **String** | The ID of the namespace to be managed. | [required] |
**trigger_name** | **String** | The name of the trigger to be managed. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## functions_get_namespace

> models::FunctionsCreateNamespace200Response functions_get_namespace(namespace_id)
Get Namespace

Gets the namespace details for the given namespace UUID. To get namespace details, send a GET request to `/v2/functions/namespaces/$NAMESPACE_ID` with no parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace_id** | **String** | The ID of the namespace to be managed. | [required] |

### Return type

[**models::FunctionsCreateNamespace200Response**](functions_create_namespace_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## functions_get_trigger

> models::FunctionsCreateTrigger200Response functions_get_trigger(namespace_id, trigger_name)
Get Trigger

Gets the trigger details. To get the trigger details, send a GET request to `/v2/functions/namespaces/$NAMESPACE_ID/triggers/$TRIGGER_NAME`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace_id** | **String** | The ID of the namespace to be managed. | [required] |
**trigger_name** | **String** | The name of the trigger to be managed. | [required] |

### Return type

[**models::FunctionsCreateTrigger200Response**](functions_create_trigger_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## functions_list_namespaces

> models::FunctionsListNamespaces200Response functions_list_namespaces()
List Namespaces

Returns a list of namespaces associated with the current user. To get all namespaces, send a GET request to `/v2/functions/namespaces`.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::FunctionsListNamespaces200Response**](functions_list_namespaces_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## functions_list_triggers

> models::FunctionsListTriggers200Response functions_list_triggers(namespace_id)
List Triggers

Returns a list of triggers associated with the current user and namespace. To get all triggers, send a GET request to `/v2/functions/namespaces/$NAMESPACE_ID/triggers`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace_id** | **String** | The ID of the namespace to be managed. | [required] |

### Return type

[**models::FunctionsListTriggers200Response**](functions_list_triggers_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## functions_update_trigger

> models::FunctionsCreateTrigger200Response functions_update_trigger(namespace_id, trigger_name, update_trigger)
Update Trigger

Updates the details of the given trigger. To update a trigger, send a PUT request to `/v2/functions/namespaces/$NAMESPACE_ID/triggers/$TRIGGER_NAME` with new values for the `is_enabled ` or `scheduled_details` properties.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace_id** | **String** | The ID of the namespace to be managed. | [required] |
**trigger_name** | **String** | The name of the trigger to be managed. | [required] |
**update_trigger** | [**UpdateTrigger**](UpdateTrigger.md) |  | [required] |

### Return type

[**models::FunctionsCreateTrigger200Response**](functions_create_trigger_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

