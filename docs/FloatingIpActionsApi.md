# \FloatingIpActionsApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**floating_ips_action_get**](FloatingIpActionsApi.md#floating_ips_action_get) | **GET** /v2/floating_ips/{floating_ip}/actions/{action_id} | Retrieve an Existing Floating IP Action
[**floating_ips_action_list**](FloatingIpActionsApi.md#floating_ips_action_list) | **GET** /v2/floating_ips/{floating_ip}/actions | List All Actions for a Floating IP
[**floating_ips_action_post**](FloatingIpActionsApi.md#floating_ips_action_post) | **POST** /v2/floating_ips/{floating_ip}/actions | Initiate a Floating IP Action



## floating_ips_action_get

> models::FloatingIpsActionPost201Response floating_ips_action_get(floating_ip, action_id)
Retrieve an Existing Floating IP Action

To retrieve the status of a floating IP action, send a GET request to `/v2/floating_ips/$FLOATING_IP/actions/$ACTION_ID`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**floating_ip** | **String** | A floating IP address. | [required] |
**action_id** | **i32** | A unique numeric ID that can be used to identify and reference an action. | [required] |

### Return type

[**models::FloatingIpsActionPost201Response**](floatingIPsAction_post_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## floating_ips_action_list

> models::FloatingIpsActionList200Response floating_ips_action_list(floating_ip)
List All Actions for a Floating IP

To retrieve all actions that have been executed on a floating IP, send a GET request to `/v2/floating_ips/$FLOATING_IP/actions`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**floating_ip** | **String** | A floating IP address. | [required] |

### Return type

[**models::FloatingIpsActionList200Response**](floatingIPsAction_list_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## floating_ips_action_post

> models::FloatingIpsActionPost201Response floating_ips_action_post(floating_ip, floating_ips_action_post_request)
Initiate a Floating IP Action

To initiate an action on a floating IP send a POST request to `/v2/floating_ips/$FLOATING_IP/actions`. In the JSON body to the request, set the `type` attribute to on of the supported action types:  | Action     | Details |------------|-------- | `assign`   | Assigns a floating IP to a Droplet | `unassign` | Unassign a floating IP from a Droplet 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**floating_ip** | **String** | A floating IP address. | [required] |
**floating_ips_action_post_request** | Option<[**FloatingIpsActionPostRequest**](FloatingIpsActionPostRequest.md)> | The `type` attribute set in the request body will specify the action that will be taken on the floating IP.  |  |

### Return type

[**models::FloatingIpsActionPost201Response**](floatingIPsAction_post_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

