# \DropletActionsApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**droplet_actions_get**](DropletActionsApi.md#droplet_actions_get) | **GET** /v2/droplets/{droplet_id}/actions/{action_id} | Retrieve a Droplet Action
[**droplet_actions_list**](DropletActionsApi.md#droplet_actions_list) | **GET** /v2/droplets/{droplet_id}/actions | List Actions for a Droplet
[**droplet_actions_post**](DropletActionsApi.md#droplet_actions_post) | **POST** /v2/droplets/{droplet_id}/actions | Initiate a Droplet Action
[**droplet_actions_post_by_tag**](DropletActionsApi.md#droplet_actions_post_by_tag) | **POST** /v2/droplets/actions | Acting on Tagged Droplets



## droplet_actions_get

> models::ActionsGet200Response droplet_actions_get(droplet_id, action_id)
Retrieve a Droplet Action

To retrieve a Droplet action, send a GET request to `/v2/droplets/$DROPLET_ID/actions/$ACTION_ID`.  The response will be a JSON object with a key called `action`. The value will be a Droplet action object. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**droplet_id** | **i32** | A unique identifier for a Droplet instance. | [required] |
**action_id** | **i32** | A unique numeric ID that can be used to identify and reference an action. | [required] |

### Return type

[**models::ActionsGet200Response**](actions_get_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## droplet_actions_list

> models::ActionsList200Response droplet_actions_list(droplet_id, per_page, page)
List Actions for a Droplet

To retrieve a list of all actions that have been executed for a Droplet, send a GET request to `/v2/droplets/$DROPLET_ID/actions`.  The results will be returned as a JSON object with an `actions` key. This will be set to an array filled with `action` objects containing the standard `action` attributes. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**droplet_id** | **i32** | A unique identifier for a Droplet instance. | [required] |
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


## droplet_actions_post

> models::ActionsGet200Response droplet_actions_post(droplet_id, droplet_actions_post_request)
Initiate a Droplet Action

To initiate an action on a Droplet send a POST request to `/v2/droplets/$DROPLET_ID/actions`. In the JSON body to the request, set the `type` attribute to on of the supported action types:  | Action                                   | Details | Additionally Required Permission | | ---------------------------------------- | ----------- | ----------- | | <nobr>`enable_backups`</nobr>            | Enables backups for a Droplet | | | <nobr>`disable_backups`</nobr>           | Disables backups for a Droplet | | | <nobr>`change_backup_policy`</nobr>      | Update the backup policy for a Droplet | | | <nobr>`reboot`</nobr>                    | Reboots a Droplet. A `reboot` action is an attempt to reboot the Droplet in a graceful way, similar to using the `reboot` command from the console. | | | <nobr>`power_cycle`</nobr>               | Power cycles a Droplet. A `powercycle` action is similar to pushing the reset button on a physical machine, it's similar to booting from scratch. | | | <nobr>`shutdown`</nobr>                  | Shutsdown a Droplet. A shutdown action is an attempt to shutdown the Droplet in a graceful way, similar to using the `shutdown` command from the console. Since a `shutdown` command can fail, this action guarantees that the command is issued, not that it succeeds. The preferred way to turn off a Droplet is to attempt a shutdown, with a reasonable timeout, followed by a `power_off` action to ensure the Droplet is off. | | | <nobr>`power_off`</nobr>                 | Powers off a Droplet. A `power_off` event is a hard shutdown and should only be used if the `shutdown` action is not successful. It is similar to cutting the power on a server and could lead to complications. | | | <nobr>`power_on`</nobr>                  | Powers on a Droplet. | | | <nobr>`restore`</nobr>                   | Restore a Droplet using a backup image. The image ID that is passed in must be a backup of the current Droplet instance. The operation will leave any embedded SSH keys intact. | droplet:admin | | <nobr>`password_reset`</nobr>            | Resets the root password for a Droplet. A new password will be provided via email. It must be changed after first use. | droplet:admin | | <nobr>`resize`</nobr>                    | Resizes a Droplet. Set the `size` attribute to a size slug. If a permanent resize with disk changes included is desired, set the `disk` attribute to `true`. | droplet:create | | <nobr>`rebuild`</nobr>                   | Rebuilds a Droplet from a new base image. Set the `image` attribute to an image ID or slug. | droplet:admin | | <nobr>`rename`</nobr>                    | Renames a Droplet. | | | <nobr>`change_kernel`</nobr>             | Changes a Droplet's kernel. Only applies to Droplets with externally managed kernels. All Droplets created after March 2017 use internal kernels by default. | | | <nobr>`enable_ipv6`</nobr>               | Enables IPv6 for a Droplet. Once enabled for a Droplet, IPv6 can not be disabled. When enabling IPv6 on an existing Droplet, [additional OS-level configuration](https://docs.digitalocean.com/products/networking/ipv6/how-to/enable/#on-existing-droplets) is required. | | | <nobr>`snapshot`</nobr>                  | Takes a snapshot of a Droplet. | image:create | 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**droplet_id** | **i32** | A unique identifier for a Droplet instance. | [required] |
**droplet_actions_post_request** | Option<[**DropletActionsPostRequest**](DropletActionsPostRequest.md)> | The `type` attribute set in the request body will specify the  action that will be taken on the Droplet. Some actions will require additional attributes to be set as well.  |  |

### Return type

[**models::ActionsGet200Response**](actions_get_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## droplet_actions_post_by_tag

> models::DropletActionsPostByTag201Response droplet_actions_post_by_tag(tag_name, droplet_actions_post_by_tag_request)
Acting on Tagged Droplets

Some actions can be performed in bulk on tagged Droplets. The actions can be initiated by sending a POST to `/v2/droplets/actions?tag_name=$TAG_NAME` with the action arguments.  Only a sub-set of action types are supported:  - `power_cycle` - `power_on` - `power_off` - `shutdown` - `enable_ipv6` - `enable_backups` - `disable_backups` - `snapshot` (also requires `image:create` permission) 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_name** | Option<**String**> | Used to filter Droplets by a specific tag. Can not be combined with `name` or `type`.<br>Requires `tag:read` scope. |  |
**droplet_actions_post_by_tag_request** | Option<[**DropletActionsPostByTagRequest**](DropletActionsPostByTagRequest.md)> | The `type` attribute set in the request body will specify the action that will be taken on the Droplet. Some actions will require additional attributes to be set as well.  |  |

### Return type

[**models::DropletActionsPostByTag201Response**](dropletActions_post_byTag_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

