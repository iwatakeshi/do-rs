# \BlockStorageActionsApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**volume_actions_get**](BlockStorageActionsApi.md#volume_actions_get) | **GET** /v2/volumes/{volume_id}/actions/{action_id} | Retrieve an Existing Volume Action
[**volume_actions_list**](BlockStorageActionsApi.md#volume_actions_list) | **GET** /v2/volumes/{volume_id}/actions | List All Actions for a Volume
[**volume_actions_post**](BlockStorageActionsApi.md#volume_actions_post) | **POST** /v2/volumes/actions | Initiate A Block Storage Action By Volume Name
[**volume_actions_post_by_id**](BlockStorageActionsApi.md#volume_actions_post_by_id) | **POST** /v2/volumes/{volume_id}/actions | Initiate A Block Storage Action By Volume Id



## volume_actions_get

> models::VolumeActionsPost202Response volume_actions_get(volume_id, action_id, per_page, page)
Retrieve an Existing Volume Action

To retrieve the status of a volume action, send a GET request to `/v2/volumes/$VOLUME_ID/actions/$ACTION_ID`.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**volume_id** | **uuid::Uuid** | The ID of the block storage volume. | [required] |
**action_id** | **i32** | A unique numeric ID that can be used to identify and reference an action. | [required] |
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**models::VolumeActionsPost202Response**](volumeActions_post_202_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## volume_actions_list

> models::VolumeActionsList200Response volume_actions_list(volume_id, per_page, page)
List All Actions for a Volume

To retrieve all actions that have been executed on a volume, send a GET request to `/v2/volumes/$VOLUME_ID/actions`.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**volume_id** | **uuid::Uuid** | The ID of the block storage volume. | [required] |
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**models::VolumeActionsList200Response**](volumeActions_list_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## volume_actions_post

> models::VolumeActionsPost202Response volume_actions_post(volume_actions_post_request, per_page, page)
Initiate A Block Storage Action By Volume Name

To initiate an action on a block storage volume by Name, send a POST request to `~/v2/volumes/actions`. The body should contain the appropriate attributes for the respective action.  ## Attach a Block Storage Volume to a Droplet  | Attribute   | Details                                                             | | ----------- | ------------------------------------------------------------------- | | type        | This must be `attach`                                               | | volume_name | The name of the block storage volume                                | | droplet_id  | Set to the Droplet's ID                                             | | region      | Set to the slug representing the region where the volume is located |  Each volume may only be attached to a single Droplet. However, up to fifteen volumes may be attached to a Droplet at a time. Pre-formatted volumes will be automatically mounted to Ubuntu, Debian, Fedora, Fedora Atomic, and CentOS Droplets created on or after April 26, 2018 when attached. On older Droplets, [additional configuration](https://docs.digitalocean.com/products/volumes/how-to/mount/) is required.  ## Remove a Block Storage Volume from a Droplet  | Attribute   | Details                                                             | | ----------- | ------------------------------------------------------------------- | | type        | This must be `detach`                                               | | volume_name | The name of the block storage volume                                | | droplet_id  | Set to the Droplet's ID                                             | | region      | Set to the slug representing the region where the volume is located | 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**volume_actions_post_request** | [**VolumeActionsPostRequest**](VolumeActionsPostRequest.md) |  | [required] |
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**models::VolumeActionsPost202Response**](volumeActions_post_202_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## volume_actions_post_by_id

> models::VolumeActionsPost202Response volume_actions_post_by_id(volume_id, volume_actions_post_by_id_request, per_page, page)
Initiate A Block Storage Action By Volume Id

To initiate an action on a block storage volume by Id, send a POST request to `~/v2/volumes/$VOLUME_ID/actions`. The body should contain the appropriate attributes for the respective action.  ## Attach a Block Storage Volume to a Droplet  | Attribute  | Details                                                             | | ---------- | ------------------------------------------------------------------- | | type       | This must be `attach`                                               | | droplet_id | Set to the Droplet's ID                                             | | region     | Set to the slug representing the region where the volume is located |  Each volume may only be attached to a single Droplet. However, up to fifteen volumes may be attached to a Droplet at a time. Pre-formatted volumes will be automatically mounted to Ubuntu, Debian, Fedora, Fedora Atomic, and CentOS Droplets created on or after April 26, 2018 when attached. On older Droplets, [additional configuration](https://docs.digitalocean.com/products/volumes/how-to/mount/) is required.  ## Remove a Block Storage Volume from a Droplet  | Attribute  | Details                                                             | | ---------- | ------------------------------------------------------------------- | | type       | This must be `detach`                                               | | droplet_id | Set to the Droplet's ID                                             | | region     | Set to the slug representing the region where the volume is located |  ## Resize a Volume  | Attribute      | Details                                                             | | -------------- | ------------------------------------------------------------------- | | type           | This must be `resize`                                               | | size_gigabytes | The new size of the block storage volume in GiB (1024^3)            | | region         | Set to the slug representing the region where the volume is located |  Volumes may only be resized upwards. The maximum size for a volume is 16TiB. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**volume_id** | **uuid::Uuid** | The ID of the block storage volume. | [required] |
**volume_actions_post_by_id_request** | [**VolumeActionsPostByIdRequest**](VolumeActionsPostByIdRequest.md) |  | [required] |
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**models::VolumeActionsPost202Response**](volumeActions_post_202_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

