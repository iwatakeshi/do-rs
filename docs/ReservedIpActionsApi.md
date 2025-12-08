# \ReservedIpActionsApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**reserved_ips_actions_get**](ReservedIpActionsApi.md#reserved_ips_actions_get) | **GET** /v2/reserved_ips/{reserved_ip}/actions/{action_id} | Retrieve an Existing Reserved IP Action
[**reserved_ips_actions_list**](ReservedIpActionsApi.md#reserved_ips_actions_list) | **GET** /v2/reserved_ips/{reserved_ip}/actions | List All Actions for a Reserved IP
[**reserved_ips_actions_post**](ReservedIpActionsApi.md#reserved_ips_actions_post) | **POST** /v2/reserved_ips/{reserved_ip}/actions | Initiate a Reserved IP Action



## reserved_ips_actions_get

> models::ReservedIpsActionsPost201Response reserved_ips_actions_get(reserved_ip, action_id)
Retrieve an Existing Reserved IP Action

To retrieve the status of a reserved IP action, send a GET request to `/v2/reserved_ips/$RESERVED_IP/actions/$ACTION_ID`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reserved_ip** | **String** | A reserved IP address. | [required] |
**action_id** | **i32** | A unique numeric ID that can be used to identify and reference an action. | [required] |

### Return type

[**models::ReservedIpsActionsPost201Response**](reservedIPsActions_post_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reserved_ips_actions_list

> models::ReservedIpsActionsList200Response reserved_ips_actions_list(reserved_ip)
List All Actions for a Reserved IP

To retrieve all actions that have been executed on a reserved IP, send a GET request to `/v2/reserved_ips/$RESERVED_IP/actions`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reserved_ip** | **String** | A reserved IP address. | [required] |

### Return type

[**models::ReservedIpsActionsList200Response**](reservedIPsActions_list_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reserved_ips_actions_post

> models::ReservedIpsActionsPost201Response reserved_ips_actions_post(reserved_ip, reserved_ips_actions_post_request)
Initiate a Reserved IP Action

To initiate an action on a reserved IP send a POST request to `/v2/reserved_ips/$RESERVED_IP/actions`. In the JSON body to the request, set the `type` attribute to on of the supported action types:  | Action     | Details |------------|-------- | `assign`   | Assigns a reserved IP to a Droplet | `unassign` | Unassign a reserved IP from a Droplet 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reserved_ip** | **String** | A reserved IP address. | [required] |
**reserved_ips_actions_post_request** | Option<[**ReservedIpsActionsPostRequest**](ReservedIpsActionsPostRequest.md)> | The `type` attribute set in the request body will specify the action that will be taken on the reserved IP.  |  |

### Return type

[**models::ReservedIpsActionsPost201Response**](reservedIPsActions_post_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

