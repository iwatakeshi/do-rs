# \LoadBalancersApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**load_balancers_add_droplets**](LoadBalancersApi.md#load_balancers_add_droplets) | **POST** /v2/load_balancers/{lb_id}/droplets | Add Droplets to a Load Balancer
[**load_balancers_add_forwarding_rules**](LoadBalancersApi.md#load_balancers_add_forwarding_rules) | **POST** /v2/load_balancers/{lb_id}/forwarding_rules | Add Forwarding Rules to a Load Balancer
[**load_balancers_create**](LoadBalancersApi.md#load_balancers_create) | **POST** /v2/load_balancers | Create a New Load Balancer
[**load_balancers_delete**](LoadBalancersApi.md#load_balancers_delete) | **DELETE** /v2/load_balancers/{lb_id} | Delete a Load Balancer
[**load_balancers_delete_cache**](LoadBalancersApi.md#load_balancers_delete_cache) | **DELETE** /v2/load_balancers/{lb_id}/cache | Delete a Global Load Balancer CDN Cache
[**load_balancers_get**](LoadBalancersApi.md#load_balancers_get) | **GET** /v2/load_balancers/{lb_id} | Retrieve an Existing Load Balancer
[**load_balancers_list**](LoadBalancersApi.md#load_balancers_list) | **GET** /v2/load_balancers | List All Load Balancers
[**load_balancers_remove_droplets**](LoadBalancersApi.md#load_balancers_remove_droplets) | **DELETE** /v2/load_balancers/{lb_id}/droplets | Remove Droplets from a Load Balancer
[**load_balancers_remove_forwarding_rules**](LoadBalancersApi.md#load_balancers_remove_forwarding_rules) | **DELETE** /v2/load_balancers/{lb_id}/forwarding_rules | Remove Forwarding Rules from a Load Balancer
[**load_balancers_update**](LoadBalancersApi.md#load_balancers_update) | **PUT** /v2/load_balancers/{lb_id} | Update a Load Balancer



## load_balancers_add_droplets

> load_balancers_add_droplets(lb_id, load_balancers_add_droplets_request)
Add Droplets to a Load Balancer

To assign a Droplet to a load balancer instance, send a POST request to `/v2/load_balancers/$LOAD_BALANCER_ID/droplets`. In the body of the request, there should be a `droplet_ids` attribute containing a list of Droplet IDs. Individual Droplets can not be added to a load balancer configured with a Droplet tag. Attempting to do so will result in a \"422 Unprocessable Entity\" response from the API.  No response body will be sent back, but the response code will indicate success. Specifically, the response code will be a 204, which means that the action was successful with no returned body data. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lb_id** | **String** | A unique identifier for a load balancer. | [required] |
**load_balancers_add_droplets_request** | [**LoadBalancersAddDropletsRequest**](LoadBalancersAddDropletsRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## load_balancers_add_forwarding_rules

> load_balancers_add_forwarding_rules(lb_id, load_balancers_add_forwarding_rules_request)
Add Forwarding Rules to a Load Balancer

To add an additional forwarding rule to a load balancer instance, send a POST request to `/v2/load_balancers/$LOAD_BALANCER_ID/forwarding_rules`. In the body of the request, there should be a `forwarding_rules` attribute containing an array of rules to be added.  No response body will be sent back, but the response code will indicate success. Specifically, the response code will be a 204, which means that the action was successful with no returned body data. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lb_id** | **String** | A unique identifier for a load balancer. | [required] |
**load_balancers_add_forwarding_rules_request** | [**LoadBalancersAddForwardingRulesRequest**](LoadBalancersAddForwardingRulesRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## load_balancers_create

> models::LoadBalancersCreate202Response load_balancers_create(load_balancer_create)
Create a New Load Balancer

To create a new load balancer instance, send a POST request to `/v2/load_balancers`.  You can specify the Droplets that will sit behind the load balancer using one of two methods:  * Set `droplet_ids` to a list of specific Droplet IDs. * Set `tag` to the name of a tag. All Droplets with this tag applied will be   assigned to the load balancer. Additional Droplets will be automatically   assigned as they are tagged.  These methods are mutually exclusive. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**load_balancer_create** | [**LoadBalancerCreate**](LoadBalancerCreate.md) |  | [required] |

### Return type

[**models::LoadBalancersCreate202Response**](loadBalancers_create_202_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## load_balancers_delete

> load_balancers_delete(lb_id)
Delete a Load Balancer

To delete a load balancer instance, disassociating any Droplets assigned to it and removing it from your account, send a DELETE request to `/v2/load_balancers/$LOAD_BALANCER_ID`.  A successful request will receive a 204 status code with no body in response. This indicates that the request was processed successfully. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lb_id** | **String** | A unique identifier for a load balancer. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## load_balancers_delete_cache

> load_balancers_delete_cache(lb_id)
Delete a Global Load Balancer CDN Cache

To delete a Global load balancer CDN cache, send a DELETE request to `/v2/load_balancers/$LOAD_BALANCER_ID/cache`.  A successful request will receive a 204 status code with no body in response. This indicates that the request was processed successfully. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lb_id** | **String** | A unique identifier for a load balancer. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## load_balancers_get

> models::LoadBalancersCreate202Response load_balancers_get(lb_id)
Retrieve an Existing Load Balancer

To show information about a load balancer instance, send a GET request to `/v2/load_balancers/$LOAD_BALANCER_ID`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lb_id** | **String** | A unique identifier for a load balancer. | [required] |

### Return type

[**models::LoadBalancersCreate202Response**](loadBalancers_create_202_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## load_balancers_list

> models::LoadBalancersList200Response load_balancers_list(per_page, page)
List All Load Balancers

To list all of the load balancer instances on your account, send a GET request to `/v2/load_balancers`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**models::LoadBalancersList200Response**](loadBalancers_list_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## load_balancers_remove_droplets

> load_balancers_remove_droplets(lb_id, load_balancers_add_droplets_request)
Remove Droplets from a Load Balancer

To remove a Droplet from a load balancer instance, send a DELETE request to `/v2/load_balancers/$LOAD_BALANCER_ID/droplets`. In the body of the request, there should be a `droplet_ids` attribute containing a list of Droplet IDs.  No response body will be sent back, but the response code will indicate success. Specifically, the response code will be a 204, which means that the action was successful with no returned body data. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lb_id** | **String** | A unique identifier for a load balancer. | [required] |
**load_balancers_add_droplets_request** | [**LoadBalancersAddDropletsRequest**](LoadBalancersAddDropletsRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## load_balancers_remove_forwarding_rules

> load_balancers_remove_forwarding_rules(lb_id, load_balancers_add_forwarding_rules_request)
Remove Forwarding Rules from a Load Balancer

To remove forwarding rules from a load balancer instance, send a DELETE request to `/v2/load_balancers/$LOAD_BALANCER_ID/forwarding_rules`. In the body of the request, there should be a `forwarding_rules` attribute containing an array of rules to be removed.  No response body will be sent back, but the response code will indicate success. Specifically, the response code will be a 204, which means that the action was successful with no returned body data. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lb_id** | **String** | A unique identifier for a load balancer. | [required] |
**load_balancers_add_forwarding_rules_request** | [**LoadBalancersAddForwardingRulesRequest**](LoadBalancersAddForwardingRulesRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## load_balancers_update

> models::LoadBalancersCreate202Response load_balancers_update(lb_id, load_balancer_create)
Update a Load Balancer

To update a load balancer's settings, send a PUT request to `/v2/load_balancers/$LOAD_BALANCER_ID`. The request should contain a full representation of the load balancer including existing attributes. It may contain _one of_ the `droplets_ids` or `tag` attributes as they are mutually exclusive. **Note that any attribute that is not provided will be reset to its default value.** 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lb_id** | **String** | A unique identifier for a load balancer. | [required] |
**load_balancer_create** | [**LoadBalancerCreate**](LoadBalancerCreate.md) |  | [required] |

### Return type

[**models::LoadBalancersCreate202Response**](loadBalancers_create_202_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

