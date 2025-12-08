# \FirewallsApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**firewalls_add_rules**](FirewallsApi.md#firewalls_add_rules) | **POST** /v2/firewalls/{firewall_id}/rules | Add Rules to a Firewall
[**firewalls_add_tags**](FirewallsApi.md#firewalls_add_tags) | **POST** /v2/firewalls/{firewall_id}/tags | Add Tags to a Firewall
[**firewalls_assign_droplets**](FirewallsApi.md#firewalls_assign_droplets) | **POST** /v2/firewalls/{firewall_id}/droplets | Add Droplets to a Firewall
[**firewalls_create**](FirewallsApi.md#firewalls_create) | **POST** /v2/firewalls | Create a New Firewall
[**firewalls_delete**](FirewallsApi.md#firewalls_delete) | **DELETE** /v2/firewalls/{firewall_id} | Delete a Firewall
[**firewalls_delete_droplets**](FirewallsApi.md#firewalls_delete_droplets) | **DELETE** /v2/firewalls/{firewall_id}/droplets | Remove Droplets from a Firewall
[**firewalls_delete_rules**](FirewallsApi.md#firewalls_delete_rules) | **DELETE** /v2/firewalls/{firewall_id}/rules | Remove Rules from a Firewall
[**firewalls_delete_tags**](FirewallsApi.md#firewalls_delete_tags) | **DELETE** /v2/firewalls/{firewall_id}/tags | Remove Tags from a Firewall
[**firewalls_get**](FirewallsApi.md#firewalls_get) | **GET** /v2/firewalls/{firewall_id} | Retrieve an Existing Firewall
[**firewalls_list**](FirewallsApi.md#firewalls_list) | **GET** /v2/firewalls | List All Firewalls
[**firewalls_update**](FirewallsApi.md#firewalls_update) | **PUT** /v2/firewalls/{firewall_id} | Update a Firewall



## firewalls_add_rules

> firewalls_add_rules(firewall_id, firewalls_add_rules_request)
Add Rules to a Firewall

To add additional access rules to a firewall, send a POST request to `/v2/firewalls/$FIREWALL_ID/rules`. The body of the request may include an inbound_rules and/or outbound_rules attribute containing an array of rules to be added.  No response body will be sent back, but the response code will indicate success. Specifically, the response code will be a 204, which means that the action was successful with no returned body data. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**firewall_id** | **uuid::Uuid** | A unique ID that can be used to identify and reference a firewall. | [required] |
**firewalls_add_rules_request** | Option<[**FirewallsAddRulesRequest**](FirewallsAddRulesRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## firewalls_add_tags

> firewalls_add_tags(firewall_id, firewalls_add_tags_request)
Add Tags to a Firewall

To assign a tag representing a group of Droplets to a firewall, send a POST request to `/v2/firewalls/$FIREWALL_ID/tags`. In the body of the request, there should be a `tags` attribute containing a list of tag names.  No response body will be sent back, but the response code will indicate success. Specifically, the response code will be a 204, which means that the action was successful with no returned body data. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**firewall_id** | **uuid::Uuid** | A unique ID that can be used to identify and reference a firewall. | [required] |
**firewalls_add_tags_request** | Option<[**FirewallsAddTagsRequest**](FirewallsAddTagsRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## firewalls_assign_droplets

> firewalls_assign_droplets(firewall_id, firewalls_assign_droplets_request)
Add Droplets to a Firewall

To assign a Droplet to a firewall, send a POST request to `/v2/firewalls/$FIREWALL_ID/droplets`. In the body of the request, there should be a `droplet_ids` attribute containing a list of Droplet IDs.  No response body will be sent back, but the response code will indicate success. Specifically, the response code will be a 204, which means that the action was successful with no returned body data. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**firewall_id** | **uuid::Uuid** | A unique ID that can be used to identify and reference a firewall. | [required] |
**firewalls_assign_droplets_request** | Option<[**FirewallsAssignDropletsRequest**](FirewallsAssignDropletsRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## firewalls_create

> models::FirewallsCreate202Response firewalls_create(firewalls_create_request)
Create a New Firewall

To create a new firewall, send a POST request to `/v2/firewalls`. The request must contain at least one inbound or outbound access rule. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**firewalls_create_request** | Option<[**FirewallsCreateRequest**](FirewallsCreateRequest.md)> |  |  |

### Return type

[**models::FirewallsCreate202Response**](firewalls_create_202_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## firewalls_delete

> firewalls_delete(firewall_id)
Delete a Firewall

To delete a firewall send a DELETE request to `/v2/firewalls/$FIREWALL_ID`.  No response body will be sent back, but the response code will indicate success. Specifically, the response code will be a 204, which means that the action was successful with no returned body data. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**firewall_id** | **uuid::Uuid** | A unique ID that can be used to identify and reference a firewall. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## firewalls_delete_droplets

> firewalls_delete_droplets(firewall_id, firewalls_delete_droplets_request)
Remove Droplets from a Firewall

To remove a Droplet from a firewall, send a DELETE request to `/v2/firewalls/$FIREWALL_ID/droplets`. In the body of the request, there should be a `droplet_ids` attribute containing a list of Droplet IDs.  No response body will be sent back, but the response code will indicate success. Specifically, the response code will be a 204, which means that the action was successful with no returned body data. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**firewall_id** | **uuid::Uuid** | A unique ID that can be used to identify and reference a firewall. | [required] |
**firewalls_delete_droplets_request** | Option<[**FirewallsDeleteDropletsRequest**](FirewallsDeleteDropletsRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## firewalls_delete_rules

> firewalls_delete_rules(firewall_id, firewalls_add_rules_request)
Remove Rules from a Firewall

To remove access rules from a firewall, send a DELETE request to `/v2/firewalls/$FIREWALL_ID/rules`. The body of the request may include an `inbound_rules` and/or `outbound_rules` attribute containing an array of rules to be removed.  No response body will be sent back, but the response code will indicate success. Specifically, the response code will be a 204, which means that the action was successful with no returned body data. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**firewall_id** | **uuid::Uuid** | A unique ID that can be used to identify and reference a firewall. | [required] |
**firewalls_add_rules_request** | Option<[**FirewallsAddRulesRequest**](FirewallsAddRulesRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## firewalls_delete_tags

> firewalls_delete_tags(firewall_id, firewalls_delete_tags_request)
Remove Tags from a Firewall

To remove a tag representing a group of Droplets from a firewall, send a DELETE request to `/v2/firewalls/$FIREWALL_ID/tags`. In the body of the request, there should be a `tags` attribute containing a list of tag names.  No response body will be sent back, but the response code will indicate success. Specifically, the response code will be a 204, which means that the action was successful with no returned body data. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**firewall_id** | **uuid::Uuid** | A unique ID that can be used to identify and reference a firewall. | [required] |
**firewalls_delete_tags_request** | Option<[**FirewallsDeleteTagsRequest**](FirewallsDeleteTagsRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## firewalls_get

> models::FirewallsCreate202Response firewalls_get(firewall_id)
Retrieve an Existing Firewall

To show information about an existing firewall, send a GET request to `/v2/firewalls/$FIREWALL_ID`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**firewall_id** | **uuid::Uuid** | A unique ID that can be used to identify and reference a firewall. | [required] |

### Return type

[**models::FirewallsCreate202Response**](firewalls_create_202_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## firewalls_list

> models::DropletsListFirewalls200Response firewalls_list(per_page, page)
List All Firewalls

To list all of the firewalls available on your account, send a GET request to `/v2/firewalls`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**models::DropletsListFirewalls200Response**](droplets_list_firewalls_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## firewalls_update

> models::FirewallsCreate202Response firewalls_update(firewall_id, firewalls_update_request)
Update a Firewall

To update the configuration of an existing firewall, send a PUT request to `/v2/firewalls/$FIREWALL_ID`. The request should contain a full representation of the firewall including existing attributes. **Note that any attributes that are not provided will be reset to their default values.** <br><br>You must have read access (e.g. `droplet:read`) to all resources attached to the firewall to successfully update the firewall. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**firewall_id** | **uuid::Uuid** | A unique ID that can be used to identify and reference a firewall. | [required] |
**firewalls_update_request** | Option<[**FirewallsUpdateRequest**](FirewallsUpdateRequest.md)> |  |  |

### Return type

[**models::FirewallsCreate202Response**](firewalls_create_202_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

