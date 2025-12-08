# \ProjectResourcesApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**projects_assign_resources**](ProjectResourcesApi.md#projects_assign_resources) | **POST** /v2/projects/{project_id}/resources | Assign Resources to a Project
[**projects_assign_resources_default**](ProjectResourcesApi.md#projects_assign_resources_default) | **POST** /v2/projects/default/resources | Assign Resources to Default Project
[**projects_list_resources**](ProjectResourcesApi.md#projects_list_resources) | **GET** /v2/projects/{project_id}/resources | List Project Resources
[**projects_list_resources_default**](ProjectResourcesApi.md#projects_list_resources_default) | **GET** /v2/projects/default/resources | List Default Project Resources



## projects_assign_resources

> models::ProjectsAssignResources200Response projects_assign_resources(project_id, project_assignment)
Assign Resources to a Project

To assign resources to a project, send a POST request to `/v2/projects/$PROJECT_ID/resources`.  You must have both `project:update` and `<resource>:read` scopes to assign new resources. For example, to assign a Droplet to a project, include both the `project:update` and `droplet:read` scopes. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | A unique identifier for a project. | [required] |
**project_assignment** | [**ProjectAssignment**](ProjectAssignment.md) |  | [required] |

### Return type

[**models::ProjectsAssignResources200Response**](projects_assign_resources_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_assign_resources_default

> models::ProjectsAssignResources200Response projects_assign_resources_default(project_assignment)
Assign Resources to Default Project

To assign resources to your default project, send a POST request to `/v2/projects/default/resources`.  You must have both project:update and <resource>:read scopes to assign new resources. For example, to assign a Droplet to the default project, include both the `project:update` and `droplet:read` scopes. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_assignment** | [**ProjectAssignment**](ProjectAssignment.md) |  | [required] |

### Return type

[**models::ProjectsAssignResources200Response**](projects_assign_resources_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_list_resources

> models::ProjectsListResources200Response projects_list_resources(project_id, per_page, page)
List Project Resources

To list all your resources in a project, send a GET request to `/v2/projects/$PROJECT_ID/resources`.  This endpoint will only return resources that you are authorized to see. For example, to see Droplets in a project, include the `droplet:read` scope. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **uuid::Uuid** | A unique identifier for a project. | [required] |
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**models::ProjectsListResources200Response**](projects_list_resources_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_list_resources_default

> models::ProjectsListResources200Response projects_list_resources_default()
List Default Project Resources

To list all your resources in your default project, send a GET request to `/v2/projects/default/resources`.  Only resources that you are authorized to see will be returned. For example, to see Droplets in a project, include the `droplet:read` scope. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ProjectsListResources200Response**](projects_list_resources_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

