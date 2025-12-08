# \Class1ClickApplicationsApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**one_clicks_install_kubernetes**](Class1ClickApplicationsApi.md#one_clicks_install_kubernetes) | **POST** /v2/1-clicks/kubernetes | Install Kubernetes 1-Click Applications
[**one_clicks_list**](Class1ClickApplicationsApi.md#one_clicks_list) | **GET** /v2/1-clicks | List 1-Click Applications



## one_clicks_install_kubernetes

> models::OneClicksInstallKubernetes200Response one_clicks_install_kubernetes(one_clicks_create)
Install Kubernetes 1-Click Applications

To install a Kubernetes 1-Click application on a cluster, send a POST request to `/v2/1-clicks/kubernetes`. The `addon_slugs` and `cluster_uuid` must be provided as body parameter in order to specify which 1-Click application(s) to install. To list all available 1-Click Kubernetes applications, send a request to `/v2/1-clicks?type=kubernetes`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**one_clicks_create** | [**OneClicksCreate**](OneClicksCreate.md) |  | [required] |

### Return type

[**models::OneClicksInstallKubernetes200Response**](oneClicks_install_kubernetes_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## one_clicks_list

> models::OneClicksList200Response one_clicks_list(r#type)
List 1-Click Applications

To list all available 1-Click applications, send a GET request to `/v2/1-clicks`. The `type` may be provided as query paramater in order to restrict results to a certain type of 1-Click, for example: `/v2/1-clicks?type=droplet`. Current supported types are `kubernetes` and `droplet`.  The response will be a JSON object with a key called `1_clicks`. This will be set to an array of 1-Click application data, each of which will contain the the slug and type for the 1-Click. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | Option<**String**> | Restrict results to a certain type of 1-Click. |  |

### Return type

[**models::OneClicksList200Response**](oneClicks_list_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

