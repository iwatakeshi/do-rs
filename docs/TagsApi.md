# \TagsApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**tags_assign_resources**](TagsApi.md#tags_assign_resources) | **POST** /v2/tags/{tag_id}/resources | Tag a Resource
[**tags_create**](TagsApi.md#tags_create) | **POST** /v2/tags | Create a New Tag
[**tags_delete**](TagsApi.md#tags_delete) | **DELETE** /v2/tags/{tag_id} | Delete a Tag
[**tags_get**](TagsApi.md#tags_get) | **GET** /v2/tags/{tag_id} | Retrieve a Tag
[**tags_list**](TagsApi.md#tags_list) | **GET** /v2/tags | List All Tags
[**tags_unassign_resources**](TagsApi.md#tags_unassign_resources) | **DELETE** /v2/tags/{tag_id}/resources | Untag a Resource



## tags_assign_resources

> tags_assign_resources(tag_id, tags_resource)
Tag a Resource

Resources can be tagged by sending a POST request to `/v2/tags/$TAG_NAME/resources` with an array of json objects containing `resource_id` and `resource_type` attributes.  Currently only tagging of Droplets, Databases, Images, Volumes, and Volume Snapshots is supported. `resource_type` is expected to be the string `droplet`, `database`, `image`, `volume` or `volume_snapshot`. `resource_id` is expected to be the ID of the resource as a string.  In order to tag a resource, you must have both `tag:create` and `<resource type>:update` scopes. For example,  to tag a Droplet, you must have `tag:create` and `droplet:update`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_id** | **String** | The name of the tag. Tags may contain letters, numbers, colons, dashes, and underscores. There is a limit of 255 characters per tag. | [required] |
**tags_resource** | [**TagsResource**](TagsResource.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tags_create

> models::TagsCreate201Response tags_create(tags)
Create a New Tag

To create a tag you can send a POST request to `/v2/tags` with a `name` attribute.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tags** | [**Tags**](Tags.md) |  | [required] |

### Return type

[**models::TagsCreate201Response**](tags_create_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tags_delete

> tags_delete(tag_id)
Delete a Tag

A tag can be deleted by sending a `DELETE` request to `/v2/tags/$TAG_NAME`. Deleting a tag also untags all the resources that have previously been tagged by the Tag

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_id** | **String** | The name of the tag. Tags may contain letters, numbers, colons, dashes, and underscores. There is a limit of 255 characters per tag. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tags_get

> models::TagsGet200Response tags_get(tag_id)
Retrieve a Tag

To retrieve an individual tag, you can send a `GET` request to `/v2/tags/$TAG_NAME`.  This endpoint will only return tagged resources that you are authorized to see. For example, to see tagged Droplets, include the `droplet:read` scope. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_id** | **String** | The name of the tag. Tags may contain letters, numbers, colons, dashes, and underscores. There is a limit of 255 characters per tag. | [required] |

### Return type

[**models::TagsGet200Response**](tags_get_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tags_list

> models::TagsList200Response tags_list(per_page, page)
List All Tags

To list all of your tags, you can send a GET request to `/v2/tags`.  This endpoint will only return tagged resources that you are authorized to see (e.g. Droplets will only be returned if you have `droplet:read`). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**models::TagsList200Response**](tags_list_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tags_unassign_resources

> tags_unassign_resources(tag_id, tags_resource)
Untag a Resource

Resources can be untagged by sending a DELETE request to `/v2/tags/$TAG_NAME/resources` with an array of json objects containing `resource_id` and `resource_type` attributes.  Currently only untagging of Droplets, Databases, Images, Volumes, and Volume Snapshots is supported. `resource_type` is expected to be the string `droplet`, `database`, `image`, `volume` or `volume_snapshot`. `resource_id` is expected to be the ID of the resource as a string.  In order to untag a resource, you must have both `tag:delete` and `<resource type>:update` scopes. For example,  to untag a Droplet, you must have `tag:delete` and `droplet:update`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_id** | **String** | The name of the tag. Tags may contain letters, numbers, colons, dashes, and underscores. There is a limit of 255 characters per tag. | [required] |
**tags_resource** | [**TagsResource**](TagsResource.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

