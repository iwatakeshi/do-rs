# \ImagesApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**images_create_custom**](ImagesApi.md#images_create_custom) | **POST** /v2/images | Create a Custom Image
[**images_delete**](ImagesApi.md#images_delete) | **DELETE** /v2/images/{image_id} | Delete an Image
[**images_get**](ImagesApi.md#images_get) | **GET** /v2/images/{image_id} | Retrieve an Existing Image
[**images_list**](ImagesApi.md#images_list) | **GET** /v2/images | List All Images
[**images_update**](ImagesApi.md#images_update) | **PUT** /v2/images/{image_id} | Update an Image



## images_create_custom

> models::ImagesCreateCustom202Response images_create_custom(image_new_custom)
Create a Custom Image

To create a new custom image, send a POST request to /v2/images. The body must contain a url attribute pointing to a Linux virtual machine image to be imported into DigitalOcean. The image must be in the raw, qcow2, vhdx, vdi, or vmdk format. It may be compressed using gzip or bzip2 and must be smaller than 100 GB after  being decompressed. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**image_new_custom** | [**ImageNewCustom**](ImageNewCustom.md) |  | [required] |

### Return type

[**models::ImagesCreateCustom202Response**](images_create_custom_202_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## images_delete

> images_delete(image_id)
Delete an Image

To delete a snapshot or custom image, send a `DELETE` request to `/v2/images/$IMAGE_ID`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**image_id** | **i32** | A unique number that can be used to identify and reference a specific image. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## images_get

> models::ImagesGet200Response images_get(image_id)
Retrieve an Existing Image

To retrieve information about an image, send a `GET` request to `/v2/images/$IDENTIFIER`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**image_id** | **String** | A unique number (id) or string (slug) used to identify and reference a specific image.  **Public** images can be identified by image `id` or `slug`.  **Private** images *must* be identified by image `id`.  | [required] |

### Return type

[**models::ImagesGet200Response**](images_get_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## images_list

> models::ImagesList200Response images_list(r#type, private, tag_name, per_page, page)
List All Images

To list all of the images available on your account, send a GET request to /v2/images.  ## Filtering Results -----  It's possible to request filtered results by including certain query parameters.  **Image Type**  Either 1-Click Application or OS Distribution images can be filtered by using the `type` query parameter.  > Important: The `type` query parameter does not directly relate to the `type` attribute.  To retrieve only ***distribution*** images, include the `type` query parameter set to distribution, `/v2/images?type=distribution`.  To retrieve only ***application*** images, include the `type` query parameter set to application, `/v2/images?type=application`.  **User Images**  To retrieve only the private images of a user, include the `private` query parameter set to true, `/v2/images?private=true`.  **Tags**  To list all images assigned to a specific tag, include the `tag_name` query parameter set to the name of the tag in your GET request. For example, `/v2/images?tag_name=$TAG_NAME`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | Option<**String**> | Filters results based on image type which can be either `application` or `distribution`. |  |
**private** | Option<**bool**> | Used to filter only user images. |  |
**tag_name** | Option<**String**> | Used to filter images by a specific tag. |  |
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**models::ImagesList200Response**](images_list_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## images_update

> models::ImagesGet200Response images_update(image_id, image_update)
Update an Image

To update an image, send a `PUT` request to `/v2/images/$IMAGE_ID`. Set the `name` attribute to the new value you would like to use. For custom images, the `description` and `distribution` attributes may also be updated. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**image_id** | **i32** | A unique number that can be used to identify and reference a specific image. | [required] |
**image_update** | [**ImageUpdate**](ImageUpdate.md) |  | [required] |

### Return type

[**models::ImagesGet200Response**](images_get_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

