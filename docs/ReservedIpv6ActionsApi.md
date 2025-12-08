# \ReservedIpv6ActionsApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**reserved_ipv6_actions_post**](ReservedIpv6ActionsApi.md#reserved_ipv6_actions_post) | **POST** /v2/reserved_ipv6/{reserved_ipv6}/actions | Initiate a Reserved IPv6 Action



## reserved_ipv6_actions_post

> models::ReservedIpv6ActionsPost201Response reserved_ipv6_actions_post(reserved_ipv6, reserved_ipv6_actions_post_request)
Initiate a Reserved IPv6 Action

To initiate an action on a reserved IPv6 send a POST request to `/v2/reserved_ipv6/$RESERVED_IPV6/actions`. In the JSON body to the request, set the `type` attribute to on of the supported action types:  | Action     | Details |------------|-------- | `assign`   | Assigns a reserved IPv6 to a Droplet | `unassign` | Unassign a reserved IPv6 from a Droplet 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reserved_ipv6** | **String** | A reserved IPv6 address. | [required] |
**reserved_ipv6_actions_post_request** | Option<[**ReservedIpv6ActionsPostRequest**](ReservedIpv6ActionsPostRequest.md)> | The `type` attribute set in the request body will specify the action that will be taken on the reserved IPv6.  |  |

### Return type

[**models::ReservedIpv6ActionsPost201Response**](reservedIPv6Actions_post_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

