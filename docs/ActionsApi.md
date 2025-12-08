# \ActionsApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**actions_get**](ActionsApi.md#actions_get) | **GET** /v2/actions/{action_id} | Retrieve an Existing Action
[**actions_list**](ActionsApi.md#actions_list) | **GET** /v2/actions | List All Actions



## actions_get

> models::ActionsGet200Response actions_get(action_id)
Retrieve an Existing Action

To retrieve a specific action object, send a GET request to `/v2/actions/$ACTION_ID`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_id** | **i32** | A unique numeric ID that can be used to identify and reference an action. | [required] |

### Return type

[**models::ActionsGet200Response**](actions_get_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## actions_list

> models::ActionsList200Response actions_list(per_page, page)
List All Actions

This will be the entire list of actions taken on your account, so it will be quite large. As with any large collection returned by the API, the results will be paginated with only 20 on each page by default.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**models::ActionsList200Response**](actions_list_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

