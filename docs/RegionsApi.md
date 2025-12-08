# \RegionsApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**regions_list**](RegionsApi.md#regions_list) | **GET** /v2/regions | List All Data Center Regions



## regions_list

> models::RegionsList200Response regions_list(per_page, page)
List All Data Center Regions

To list all of the regions that are available, send a GET request to `/v2/regions`. The response will be a JSON object with a key called `regions`. The value of this will be an array of `region` objects, each of which will contain the standard region attributes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**models::RegionsList200Response**](regions_list_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

