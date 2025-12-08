# \ImageActionsApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**image_actions_get**](ImageActionsApi.md#image_actions_get) | **GET** /v2/images/{image_id}/actions/{action_id} | Retrieve an Existing Action
[**image_actions_list**](ImageActionsApi.md#image_actions_list) | **GET** /v2/images/{image_id}/actions | List All Actions for an Image
[**image_actions_post**](ImageActionsApi.md#image_actions_post) | **POST** /v2/images/{image_id}/actions | Initiate an Image Action



## image_actions_get

> models::Action image_actions_get(image_id, action_id)
Retrieve an Existing Action

To retrieve the status of an image action, send a GET request to `/v2/images/$IMAGE_ID/actions/$IMAGE_ACTION_ID`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**image_id** | **i32** | A unique number that can be used to identify and reference a specific image. | [required] |
**action_id** | **i32** | A unique numeric ID that can be used to identify and reference an action. | [required] |

### Return type

[**models::Action**](action.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_actions_list

> models::ActionsList200Response image_actions_list(image_id)
List All Actions for an Image

To retrieve all actions that have been executed on an image, send a GET request to `/v2/images/$IMAGE_ID/actions`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**image_id** | **i32** | A unique number that can be used to identify and reference a specific image. | [required] |

### Return type

[**models::ActionsList200Response**](actions_list_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## image_actions_post

> models::Action image_actions_post(image_id, image_actions_post_request)
Initiate an Image Action

The following actions are available on an Image.  ## Convert an Image to a Snapshot  To convert an image, for example, a backup to a snapshot, send a POST request to `/v2/images/$IMAGE_ID/actions`. Set the `type` attribute to `convert`.  ## Transfer an Image  To transfer an image to another region, send a POST request to `/v2/images/$IMAGE_ID/actions`. Set the `type` attribute to `transfer` and set `region` attribute to the slug identifier of the region you wish to transfer to. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**image_id** | **i32** | A unique number that can be used to identify and reference a specific image. | [required] |
**image_actions_post_request** | Option<[**ImageActionsPostRequest**](ImageActionsPostRequest.md)> |  |  |

### Return type

[**models::Action**](action.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

