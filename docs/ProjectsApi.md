# \ProjectsApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**projects_create**](ProjectsApi.md#projects_create) | **POST** /v2/projects | Create a Project
[**projects_delete**](ProjectsApi.md#projects_delete) | **DELETE** /v2/projects/{project_id} | Delete an Existing Project
[**projects_get**](ProjectsApi.md#projects_get) | **GET** /v2/projects/{project_id} | Retrieve an Existing Project
[**projects_get_default**](ProjectsApi.md#projects_get_default) | **GET** /v2/projects/default | Retrieve the Default Project
[**projects_list**](ProjectsApi.md#projects_list) | **GET** /v2/projects | List All Projects
[**projects_patch**](ProjectsApi.md#projects_patch) | **PATCH** /v2/projects/{project_id} | Patch a Project
[**projects_patch_default**](ProjectsApi.md#projects_patch_default) | **PATCH** /v2/projects/default | Patch the Default Project
[**projects_update**](ProjectsApi.md#projects_update) | **PUT** /v2/projects/{project_id} | Update a Project
[**projects_update_default**](ProjectsApi.md#projects_update_default) | **PUT** /v2/projects/default | Update the Default Project



## projects_create

> models::ProjectsCreate201Response projects_create(project_base)
Create a Project

To create a project, send a POST request to `/v2/projects`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_base** | [**ProjectBase**](ProjectBase.md) |  | [required] |

### Return type

[**models::ProjectsCreate201Response**](projects_create_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_delete

> projects_delete(project_id)
Delete an Existing Project

To delete a project, send a DELETE request to `/v2/projects/$PROJECT_ID`. To be deleted, a project must not have any resources assigned to it. Any existing resources must first be reassigned or destroyed, or you will receive a 412 error.  A successful request will receive a 204 status code with no body in response. This indicates that the request was processed successfully. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | A unique identifier for a project. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_get

> models::ProjectsCreate201Response projects_get(project_id)
Retrieve an Existing Project

To get a project, send a GET request to `/v2/projects/$PROJECT_ID`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | A unique identifier for a project. | [required] |

### Return type

[**models::ProjectsCreate201Response**](projects_create_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_get_default

> models::ProjectsGetDefault200Response projects_get_default()
Retrieve the Default Project

To get your default project, send a GET request to `/v2/projects/default`.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ProjectsGetDefault200Response**](projects_get_default_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_list

> models::ProjectsList200Response projects_list(per_page, page)
List All Projects

To list all your projects, send a GET request to `/v2/projects`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**models::ProjectsList200Response**](projects_list_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_patch

> models::ProjectsCreate201Response projects_patch(project_id, project)
Patch a Project

To update only specific attributes of a project, send a PATCH request to `/v2/projects/$PROJECT_ID`. At least one of the following attributes needs to be sent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | A unique identifier for a project. | [required] |
**project** | [**Project**](Project.md) |  | [required] |

### Return type

[**models::ProjectsCreate201Response**](projects_create_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_patch_default

> models::ProjectsCreate201Response projects_patch_default(project)
Patch the Default Project

To update only specific attributes of your default project, send a PATCH request to `/v2/projects/default`. At least one of the following attributes needs to be sent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | [**Project**](Project.md) |  | [required] |

### Return type

[**models::ProjectsCreate201Response**](projects_create_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_update

> models::ProjectsCreate201Response projects_update(project_id, project)
Update a Project

To update a project, send a PUT request to `/v2/projects/$PROJECT_ID`. All of the following attributes must be sent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | A unique identifier for a project. | [required] |
**project** | [**Project**](Project.md) |  | [required] |

### Return type

[**models::ProjectsCreate201Response**](projects_create_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_update_default

> models::ProjectsCreate201Response projects_update_default(project)
Update the Default Project

To update you default project, send a PUT request to `/v2/projects/default`. All of the following attributes must be sent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project** | [**Project**](Project.md) |  | [required] |

### Return type

[**models::ProjectsCreate201Response**](projects_create_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

