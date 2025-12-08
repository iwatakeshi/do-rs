# \ContainerRegistryApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**registry_create**](ContainerRegistryApi.md#registry_create) | **POST** /v2/registry | Create Container Registry
[**registry_delete**](ContainerRegistryApi.md#registry_delete) | **DELETE** /v2/registry | Delete Container Registry
[**registry_delete_repository_manifest**](ContainerRegistryApi.md#registry_delete_repository_manifest) | **DELETE** /v2/registry/{registry_name}/repositories/{repository_name}/digests/{manifest_digest} | Delete Container Registry Repository Manifest
[**registry_delete_repository_tag**](ContainerRegistryApi.md#registry_delete_repository_tag) | **DELETE** /v2/registry/{registry_name}/repositories/{repository_name}/tags/{repository_tag} | Delete Container Registry Repository Tag
[**registry_get**](ContainerRegistryApi.md#registry_get) | **GET** /v2/registry | Get Container Registry Information
[**registry_get_docker_credentials**](ContainerRegistryApi.md#registry_get_docker_credentials) | **GET** /v2/registry/docker-credentials | Get Docker Credentials for Container Registry
[**registry_get_garbage_collection**](ContainerRegistryApi.md#registry_get_garbage_collection) | **GET** /v2/registry/{registry_name}/garbage-collection | Get Active Garbage Collection
[**registry_get_options**](ContainerRegistryApi.md#registry_get_options) | **GET** /v2/registry/options | List Registry Options (Subscription Tiers and Available Regions)
[**registry_get_subscription**](ContainerRegistryApi.md#registry_get_subscription) | **GET** /v2/registry/subscription | Get Subscription Information
[**registry_list_garbage_collections**](ContainerRegistryApi.md#registry_list_garbage_collections) | **GET** /v2/registry/{registry_name}/garbage-collections | List Garbage Collections
[**registry_list_repositories**](ContainerRegistryApi.md#registry_list_repositories) | **GET** /v2/registry/{registry_name}/repositories | List All Container Registry Repositories
[**registry_list_repositories_v2**](ContainerRegistryApi.md#registry_list_repositories_v2) | **GET** /v2/registry/{registry_name}/repositoriesV2 | List All Container Registry Repositories (V2)
[**registry_list_repository_manifests**](ContainerRegistryApi.md#registry_list_repository_manifests) | **GET** /v2/registry/{registry_name}/repositories/{repository_name}/digests | List All Container Registry Repository Manifests
[**registry_list_repository_tags**](ContainerRegistryApi.md#registry_list_repository_tags) | **GET** /v2/registry/{registry_name}/repositories/{repository_name}/tags | List All Container Registry Repository Tags
[**registry_run_garbage_collection**](ContainerRegistryApi.md#registry_run_garbage_collection) | **POST** /v2/registry/{registry_name}/garbage-collection | Start Garbage Collection
[**registry_update_garbage_collection**](ContainerRegistryApi.md#registry_update_garbage_collection) | **PUT** /v2/registry/{registry_name}/garbage-collection/{garbage_collection_uuid} | Update Garbage Collection
[**registry_update_subscription**](ContainerRegistryApi.md#registry_update_subscription) | **POST** /v2/registry/subscription | Update Subscription Tier
[**registry_validate_name**](ContainerRegistryApi.md#registry_validate_name) | **POST** /v2/registry/validate-name | Validate a Container Registry Name



## registry_create

> models::RegistryGet200Response registry_create(registry_create)
Create Container Registry

To create your container registry, send a POST request to `/v2/registry`.  The `name` becomes part of the URL for images stored in the registry. For example, if your registry is called `example`, an image in it will have the URL `registry.digitalocean.com/example/image:tag`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**registry_create** | [**RegistryCreate**](RegistryCreate.md) |  | [required] |

### Return type

[**models::RegistryGet200Response**](registry_get_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## registry_delete

> registry_delete()
Delete Container Registry

To delete your container registry, destroying all container image data stored in it, send a DELETE request to `/v2/registry`. This operation is not compatible with multiple registries in a DO account. You should use `/v2/registries/{registry_name}` instead.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## registry_delete_repository_manifest

> registry_delete_repository_manifest(registry_name, repository_name, manifest_digest)
Delete Container Registry Repository Manifest

To delete a container repository manifest by digest, send a DELETE request to `/v2/registry/$REGISTRY_NAME/repositories/$REPOSITORY_NAME/digests/$MANIFEST_DIGEST`.  Note that if your repository name contains `/` characters, it must be URL-encoded in the request URL. For example, to delete `registry.digitalocean.com/example/my/repo@sha256:abcd`, the path would be `/v2/registry/example/repositories/my%2Frepo/digests/sha256:abcd`.  A successful request will receive a 204 status code with no body in response. This indicates that the request was processed successfully. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**registry_name** | **String** | The name of a container registry. | [required] |
**repository_name** | **String** | The name of a container registry repository. If the name contains `/` characters, they must be URL-encoded, e.g. `%2F`. | [required] |
**manifest_digest** | **String** | The manifest digest of a container registry repository tag. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## registry_delete_repository_tag

> registry_delete_repository_tag(registry_name, repository_name, repository_tag)
Delete Container Registry Repository Tag

To delete a container repository tag, send a DELETE request to `/v2/registry/$REGISTRY_NAME/repositories/$REPOSITORY_NAME/tags/$TAG`.  Note that if your repository name contains `/` characters, it must be URL-encoded in the request URL. For example, to delete `registry.digitalocean.com/example/my/repo:mytag`, the path would be `/v2/registry/example/repositories/my%2Frepo/tags/mytag`.  A successful request will receive a 204 status code with no body in response. This indicates that the request was processed successfully. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**registry_name** | **String** | The name of a container registry. | [required] |
**repository_name** | **String** | The name of a container registry repository. If the name contains `/` characters, they must be URL-encoded, e.g. `%2F`. | [required] |
**repository_tag** | **String** | The name of a container registry repository tag. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## registry_get

> models::RegistryGet200Response registry_get()
Get Container Registry Information

To get information about your container registry, send a GET request to `/v2/registry`. This operation is not compatible with multiple registries in a DO account. You should use `/v2/registries/{registry_name}` instead.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RegistryGet200Response**](registry_get_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## registry_get_docker_credentials

> models::DockerCredentials registry_get_docker_credentials(expiry_seconds, read_write)
Get Docker Credentials for Container Registry

In order to access your container registry with the Docker client or from a Kubernetes cluster, you will need to configure authentication. The necessary JSON configuration can be retrieved by sending a GET request to `/v2/registry/docker-credentials`.  The response will be in the format of a Docker `config.json` file. To use the config in your Kubernetes cluster, create a Secret with:      kubectl create secret generic docr \\       --from-file=.dockerconfigjson=config.json \\       --type=kubernetes.io/dockerconfigjson  By default, the returned credentials have read-only access to your registry and cannot be used to push images. This is appropriate for most Kubernetes clusters. To retrieve read/write credentials, suitable for use with the Docker client or in a CI system, read_write may be provided as query parameter. For example: `/v2/registry/docker-credentials?read_write=true`  By default, the returned credentials will not expire. To retrieve credentials with an expiry set, expiry_seconds may be provided as a query parameter. For example: `/v2/registry/docker-credentials?expiry_seconds=3600` will return credentials that expire after one hour. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**expiry_seconds** | Option<**i32**> | The duration in seconds that the returned registry credentials will be valid. If not set or 0, the credentials will not expire. |  |[default to 0]
**read_write** | Option<**bool**> | By default, the registry credentials allow for read-only access. Set this query parameter to `true` to obtain read-write credentials. |  |[default to false]

### Return type

[**models::DockerCredentials**](docker_credentials.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## registry_get_garbage_collection

> models::RegistriesGetGarbageCollection200Response registry_get_garbage_collection(registry_name)
Get Active Garbage Collection

To get information about the currently-active garbage collection for a registry, send a GET request to `/v2/registry/$REGISTRY_NAME/garbage-collection`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**registry_name** | **String** | The name of a container registry. | [required] |

### Return type

[**models::RegistriesGetGarbageCollection200Response**](registries_get_garbageCollection_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## registry_get_options

> models::RegistriesGetOptions200Response registry_get_options()
List Registry Options (Subscription Tiers and Available Regions)

This endpoint serves to provide additional information as to which option values are available when creating a container registry. There are multiple subscription tiers available for container registry. Each tier allows a different number of image repositories to be created in your registry, and has a different amount of storage and transfer included. There are multiple regions available for container registry and controls where your data is stored. To list the available options, send a GET request to `/v2/registry/options`.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RegistriesGetOptions200Response**](registries_get_options_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## registry_get_subscription

> models::RegistriesGetSubscription200Response registry_get_subscription()
Get Subscription Information

A subscription is automatically created when you configure your container registry. To get information about your subscription, send a GET request to `/v2/registry/subscription`.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RegistriesGetSubscription200Response**](registries_get_subscription_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## registry_list_garbage_collections

> models::RegistriesListGarbageCollections200Response registry_list_garbage_collections(registry_name, per_page, page)
List Garbage Collections

To get information about past garbage collections for a registry, send a GET request to `/v2/registry/$REGISTRY_NAME/garbage-collections`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**registry_name** | **String** | The name of a container registry. | [required] |
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**models::RegistriesListGarbageCollections200Response**](registries_list_garbageCollections_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## registry_list_repositories

> models::RegistryListRepositories200Response registry_list_repositories(registry_name, per_page, page)
List All Container Registry Repositories

This endpoint has been deprecated in favor of the _List All Container Registry Repositories [V2]_ endpoint.  To list all repositories in your container registry, send a GET request to `/v2/registry/$REGISTRY_NAME/repositories`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**registry_name** | **String** | The name of a container registry. | [required] |
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**models::RegistryListRepositories200Response**](registry_list_repositories_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## registry_list_repositories_v2

> models::RegistriesListRepositoriesV2200Response registry_list_repositories_v2(registry_name, per_page, page, page_token)
List All Container Registry Repositories (V2)

To list all repositories in your container registry, send a GET request to `/v2/registry/$REGISTRY_NAME/repositoriesV2`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**registry_name** | **String** | The name of a container registry. | [required] |
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. Ignored when 'page_token' is provided. |  |[default to 1]
**page_token** | Option<**String**> | Token to retrieve of the next or previous set of results more quickly than using 'page'. |  |

### Return type

[**models::RegistriesListRepositoriesV2200Response**](registries_list_repositoriesV2_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## registry_list_repository_manifests

> models::RegistriesListRepositoryManifests200Response registry_list_repository_manifests(registry_name, repository_name, per_page, page)
List All Container Registry Repository Manifests

To list all manifests in your container registry repository, send a GET request to `/v2/registry/$REGISTRY_NAME/repositories/$REPOSITORY_NAME/digests`.  Note that if your repository name contains `/` characters, it must be URL-encoded in the request URL. For example, to list manifests for `registry.digitalocean.com/example/my/repo`, the path would be `/v2/registry/example/repositories/my%2Frepo/digests`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**registry_name** | **String** | The name of a container registry. | [required] |
**repository_name** | **String** | The name of a container registry repository. If the name contains `/` characters, they must be URL-encoded, e.g. `%2F`. | [required] |
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**models::RegistriesListRepositoryManifests200Response**](registries_list_repositoryManifests_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## registry_list_repository_tags

> models::RegistriesListRepositoryTags200Response registry_list_repository_tags(registry_name, repository_name, per_page, page)
List All Container Registry Repository Tags

To list all tags in your container registry repository, send a GET request to `/v2/registry/$REGISTRY_NAME/repositories/$REPOSITORY_NAME/tags`.  Note that if your repository name contains `/` characters, it must be URL-encoded in the request URL. For example, to list tags for `registry.digitalocean.com/example/my/repo`, the path would be `/v2/registry/example/repositories/my%2Frepo/tags`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**registry_name** | **String** | The name of a container registry. | [required] |
**repository_name** | **String** | The name of a container registry repository. If the name contains `/` characters, they must be URL-encoded, e.g. `%2F`. | [required] |
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**models::RegistriesListRepositoryTags200Response**](registries_list_repositoryTags_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## registry_run_garbage_collection

> models::RegistriesGetGarbageCollection200Response registry_run_garbage_collection(registry_name, registry_run_gc)
Start Garbage Collection

Garbage collection enables users to clear out unreferenced blobs (layer & manifest data) after deleting one or more manifests from a repository. If there are no unreferenced blobs resulting from the deletion of one or more manifests, garbage collection is effectively a noop. [See here for more information](https://docs.digitalocean.com/products/container-registry/how-to/clean-up-container-registry/) about how and why you should clean up your container registry periodically.  To request a garbage collection run on your registry, send a POST request to `/v2/registry/$REGISTRY_NAME/garbage-collection`. This will initiate the following sequence of events on your registry.  * Set the registry to read-only mode, meaning no further write-scoped   JWTs will be issued to registry clients. Existing write-scoped JWTs will   continue to work until they expire which can take up to 15 minutes. * Wait until all existing write-scoped JWTs have expired. * Scan all registry manifests to determine which blobs are unreferenced. * Delete all unreferenced blobs from the registry. * Record the number of blobs deleted and bytes freed, mark the garbage   collection status as `success`. * Remove the read-only mode restriction from the registry, meaning write-scoped   JWTs will once again be issued to registry clients. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**registry_name** | **String** | The name of a container registry. | [required] |
**registry_run_gc** | Option<[**RegistryRunGc**](RegistryRunGc.md)> |  |  |

### Return type

[**models::RegistriesGetGarbageCollection200Response**](registries_get_garbageCollection_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## registry_update_garbage_collection

> models::RegistriesGetGarbageCollection200Response registry_update_garbage_collection(registry_name, garbage_collection_uuid, update_registry)
Update Garbage Collection

To cancel the currently-active garbage collection for a registry, send a PUT request to `/v2/registry/$REGISTRY_NAME/garbage-collection/$GC_UUID` and specify one or more of the attributes below.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**registry_name** | **String** | The name of a container registry. | [required] |
**garbage_collection_uuid** | **String** | The UUID of a garbage collection run. | [required] |
**update_registry** | [**UpdateRegistry**](UpdateRegistry.md) |  | [required] |

### Return type

[**models::RegistriesGetGarbageCollection200Response**](registries_get_garbageCollection_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## registry_update_subscription

> models::RegistriesGetSubscription200Response registry_update_subscription(registries_update_subscription_request)
Update Subscription Tier

After creating your registry, you can switch to a different subscription tier to better suit your needs. To do this, send a POST request to `/v2/registry/subscription`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**registries_update_subscription_request** | Option<[**RegistriesUpdateSubscriptionRequest**](RegistriesUpdateSubscriptionRequest.md)> |  |  |

### Return type

[**models::RegistriesGetSubscription200Response**](registries_get_subscription_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## registry_validate_name

> registry_validate_name(validate_registry)
Validate a Container Registry Name

To validate that a container registry name is available for use, send a POST request to `/v2/registry/validate-name`.  If the name is both formatted correctly and available, the response code will be 204 and contain no body. If the name is already in use, the response will be a 409 Conflict. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**validate_registry** | [**ValidateRegistry**](ValidateRegistry.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

