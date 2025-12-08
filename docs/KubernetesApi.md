# \KubernetesApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**kubernetes_add_node_pool**](KubernetesApi.md#kubernetes_add_node_pool) | **POST** /v2/kubernetes/clusters/{cluster_id}/node_pools | Add a Node Pool to a Kubernetes Cluster
[**kubernetes_add_registries**](KubernetesApi.md#kubernetes_add_registries) | **POST** /v2/kubernetes/registries | Add Container Registries to Kubernetes Clusters
[**kubernetes_add_registry**](KubernetesApi.md#kubernetes_add_registry) | **POST** /v2/kubernetes/registry | Add Container Registry to Kubernetes Clusters
[**kubernetes_create_cluster**](KubernetesApi.md#kubernetes_create_cluster) | **POST** /v2/kubernetes/clusters | Create a New Kubernetes Cluster
[**kubernetes_delete_cluster**](KubernetesApi.md#kubernetes_delete_cluster) | **DELETE** /v2/kubernetes/clusters/{cluster_id} | Delete a Kubernetes Cluster
[**kubernetes_delete_node**](KubernetesApi.md#kubernetes_delete_node) | **DELETE** /v2/kubernetes/clusters/{cluster_id}/node_pools/{node_pool_id}/nodes/{node_id} | Delete a Node in a Kubernetes Cluster
[**kubernetes_delete_node_pool**](KubernetesApi.md#kubernetes_delete_node_pool) | **DELETE** /v2/kubernetes/clusters/{cluster_id}/node_pools/{node_pool_id} | Delete a Node Pool in a Kubernetes Cluster
[**kubernetes_destroy_associated_resources_dangerous**](KubernetesApi.md#kubernetes_destroy_associated_resources_dangerous) | **DELETE** /v2/kubernetes/clusters/{cluster_id}/destroy_with_associated_resources/dangerous | Delete a Cluster and All of its Associated Resources (Dangerous)
[**kubernetes_destroy_associated_resources_selective**](KubernetesApi.md#kubernetes_destroy_associated_resources_selective) | **DELETE** /v2/kubernetes/clusters/{cluster_id}/destroy_with_associated_resources/selective | Selectively Delete a Cluster and its Associated Resources
[**kubernetes_get_available_upgrades**](KubernetesApi.md#kubernetes_get_available_upgrades) | **GET** /v2/kubernetes/clusters/{cluster_id}/upgrades | Retrieve Available Upgrades for an Existing Kubernetes Cluster
[**kubernetes_get_cluster**](KubernetesApi.md#kubernetes_get_cluster) | **GET** /v2/kubernetes/clusters/{cluster_id} | Retrieve an Existing Kubernetes Cluster
[**kubernetes_get_cluster_lint_results**](KubernetesApi.md#kubernetes_get_cluster_lint_results) | **GET** /v2/kubernetes/clusters/{cluster_id}/clusterlint | Fetch Clusterlint Diagnostics for a Kubernetes Cluster
[**kubernetes_get_cluster_user**](KubernetesApi.md#kubernetes_get_cluster_user) | **GET** /v2/kubernetes/clusters/{cluster_id}/user | Retrieve User Information for a Kubernetes Cluster
[**kubernetes_get_credentials**](KubernetesApi.md#kubernetes_get_credentials) | **GET** /v2/kubernetes/clusters/{cluster_id}/credentials | Retrieve Credentials for a Kubernetes Cluster
[**kubernetes_get_kubeconfig**](KubernetesApi.md#kubernetes_get_kubeconfig) | **GET** /v2/kubernetes/clusters/{cluster_id}/kubeconfig | Retrieve the kubeconfig for a Kubernetes Cluster
[**kubernetes_get_node_pool**](KubernetesApi.md#kubernetes_get_node_pool) | **GET** /v2/kubernetes/clusters/{cluster_id}/node_pools/{node_pool_id} | Retrieve a Node Pool for a Kubernetes Cluster
[**kubernetes_get_status_messages**](KubernetesApi.md#kubernetes_get_status_messages) | **GET** /v2/kubernetes/clusters/{cluster_id}/status_messages | Fetch Status Messages for a Kubernetes Cluster
[**kubernetes_list_associated_resources**](KubernetesApi.md#kubernetes_list_associated_resources) | **GET** /v2/kubernetes/clusters/{cluster_id}/destroy_with_associated_resources | List Associated Resources for Cluster Deletion
[**kubernetes_list_clusters**](KubernetesApi.md#kubernetes_list_clusters) | **GET** /v2/kubernetes/clusters | List All Kubernetes Clusters
[**kubernetes_list_node_pools**](KubernetesApi.md#kubernetes_list_node_pools) | **GET** /v2/kubernetes/clusters/{cluster_id}/node_pools | List All Node Pools in a Kubernetes Clusters
[**kubernetes_list_options**](KubernetesApi.md#kubernetes_list_options) | **GET** /v2/kubernetes/options | List Available Regions, Node Sizes, and Versions of Kubernetes
[**kubernetes_recycle_node_pool**](KubernetesApi.md#kubernetes_recycle_node_pool) | **POST** /v2/kubernetes/clusters/{cluster_id}/node_pools/{node_pool_id}/recycle | Recycle a Kubernetes Node Pool
[**kubernetes_remove_registries**](KubernetesApi.md#kubernetes_remove_registries) | **DELETE** /v2/kubernetes/registries | Remove Container Registries from Kubernetes Clusters
[**kubernetes_remove_registry**](KubernetesApi.md#kubernetes_remove_registry) | **DELETE** /v2/kubernetes/registry | Remove Container Registry from Kubernetes Clusters
[**kubernetes_run_cluster_lint**](KubernetesApi.md#kubernetes_run_cluster_lint) | **POST** /v2/kubernetes/clusters/{cluster_id}/clusterlint | Run Clusterlint Checks on a Kubernetes Cluster
[**kubernetes_update_cluster**](KubernetesApi.md#kubernetes_update_cluster) | **PUT** /v2/kubernetes/clusters/{cluster_id} | Update a Kubernetes Cluster
[**kubernetes_update_node_pool**](KubernetesApi.md#kubernetes_update_node_pool) | **PUT** /v2/kubernetes/clusters/{cluster_id}/node_pools/{node_pool_id} | Update a Node Pool in a Kubernetes Cluster
[**kubernetes_upgrade_cluster**](KubernetesApi.md#kubernetes_upgrade_cluster) | **POST** /v2/kubernetes/clusters/{cluster_id}/upgrade | Upgrade a Kubernetes Cluster



## kubernetes_add_node_pool

> models::KubernetesAddNodePool201Response kubernetes_add_node_pool(cluster_id, kubernetes_node_pool)
Add a Node Pool to a Kubernetes Cluster

To add an additional node pool to a Kubernetes clusters, send a POST request to `/v2/kubernetes/clusters/$K8S_CLUSTER_ID/node_pools` with the following attributes. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **uuid::Uuid** | A unique ID that can be used to reference a Kubernetes cluster. | [required] |
**kubernetes_node_pool** | [**KubernetesNodePool**](KubernetesNodePool.md) |  | [required] |

### Return type

[**models::KubernetesAddNodePool201Response**](kubernetes_add_nodePool_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_add_registries

> kubernetes_add_registries(cluster_registries)
Add Container Registries to Kubernetes Clusters

To integrate the container registries with Kubernetes clusters, send a POST request to `/v2/kubernetes/registries`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_registries** | Option<[**ClusterRegistries**](ClusterRegistries.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_add_registry

> kubernetes_add_registry(cluster_registry)
Add Container Registry to Kubernetes Clusters

To integrate the container registry with Kubernetes clusters, send a POST request to `/v2/kubernetes/registry`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_registry** | Option<[**ClusterRegistry**](ClusterRegistry.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_create_cluster

> models::KubernetesCreateCluster201Response kubernetes_create_cluster(cluster)
Create a New Kubernetes Cluster

To create a new Kubernetes cluster, send a POST request to `/v2/kubernetes/clusters`. The request must contain at least one node pool with at least one worker.  The request may contain a maintenance window policy describing a time period when disruptive maintenance tasks may be carried out. Omitting the policy implies that a window will be chosen automatically. See [here](https://docs.digitalocean.com/products/kubernetes/how-to/upgrade-cluster/) for details. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | [**Cluster**](Cluster.md) |  | [required] |

### Return type

[**models::KubernetesCreateCluster201Response**](kubernetes_create_cluster_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_delete_cluster

> kubernetes_delete_cluster(cluster_id)
Delete a Kubernetes Cluster

To delete a Kubernetes cluster and all services deployed to it, send a DELETE request to `/v2/kubernetes/clusters/$K8S_CLUSTER_ID`.  A 204 status code with no body will be returned in response to a successful request. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **uuid::Uuid** | A unique ID that can be used to reference a Kubernetes cluster. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_delete_node

> kubernetes_delete_node(cluster_id, node_pool_id, node_id, skip_drain, replace)
Delete a Node in a Kubernetes Cluster

To delete a single node in a pool, send a DELETE request to `/v2/kubernetes/clusters/$K8S_CLUSTER_ID/node_pools/$NODE_POOL_ID/nodes/$NODE_ID`.  Appending the `skip_drain=1` query parameter to the request causes node draining to be skipped. Omitting the query parameter or setting its value to `0` carries out draining prior to deletion.  Appending the `replace=1` query parameter to the request causes the node to be replaced by a new one after deletion. Omitting the query parameter or setting its value to `0` deletes without replacement. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **uuid::Uuid** | A unique ID that can be used to reference a Kubernetes cluster. | [required] |
**node_pool_id** | **uuid::Uuid** | A unique ID that can be used to reference a Kubernetes node pool. | [required] |
**node_id** | **uuid::Uuid** | A unique ID that can be used to reference a node in a Kubernetes node pool. | [required] |
**skip_drain** | Option<**i32**> | Specifies whether or not to drain workloads from a node before it is deleted. Setting it to `1` causes node draining to be skipped. Omitting the query parameter or setting its value to `0` carries out draining prior to deletion. |  |[default to 0]
**replace** | Option<**i32**> | Specifies whether or not to replace a node after it has been deleted. Setting it to `1` causes the node to be replaced by a new one after deletion. Omitting the query parameter or setting its value to `0` deletes without replacement. |  |[default to 0]

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_delete_node_pool

> kubernetes_delete_node_pool(cluster_id, node_pool_id)
Delete a Node Pool in a Kubernetes Cluster

To delete a node pool, send a DELETE request to `/v2/kubernetes/clusters/$K8S_CLUSTER_ID/node_pools/$NODE_POOL_ID`.  A 204 status code with no body will be returned in response to a successful request. Nodes in the pool will subsequently be drained and deleted. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **uuid::Uuid** | A unique ID that can be used to reference a Kubernetes cluster. | [required] |
**node_pool_id** | **uuid::Uuid** | A unique ID that can be used to reference a Kubernetes node pool. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_destroy_associated_resources_dangerous

> kubernetes_destroy_associated_resources_dangerous(cluster_id)
Delete a Cluster and All of its Associated Resources (Dangerous)

To delete a Kubernetes cluster with all of its associated resources, send a DELETE request to `/v2/kubernetes/clusters/$K8S_CLUSTER_ID/destroy_with_associated_resources/dangerous`. A 204 status code with no body will be returned in response to a successful request. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **uuid::Uuid** | A unique ID that can be used to reference a Kubernetes cluster. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_destroy_associated_resources_selective

> kubernetes_destroy_associated_resources_selective(cluster_id, destroy_associated_kubernetes_resources)
Selectively Delete a Cluster and its Associated Resources

To delete a Kubernetes cluster along with a subset of its associated resources, send a DELETE request to `/v2/kubernetes/clusters/$K8S_CLUSTER_ID/destroy_with_associated_resources/selective`.  The JSON body of the request should include `load_balancers`, `volumes`, or `volume_snapshots` keys each set to an array of IDs for the associated resources to be destroyed.  The IDs can be found by querying the cluster's associated resources endpoint. Any associated resource not included in the request will remain and continue to accrue changes on your account. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **uuid::Uuid** | A unique ID that can be used to reference a Kubernetes cluster. | [required] |
**destroy_associated_kubernetes_resources** | [**DestroyAssociatedKubernetesResources**](DestroyAssociatedKubernetesResources.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_get_available_upgrades

> models::KubernetesGetAvailableUpgrades200Response kubernetes_get_available_upgrades(cluster_id)
Retrieve Available Upgrades for an Existing Kubernetes Cluster

To determine whether a cluster can be upgraded, and the versions to which it can be upgraded, send a GET request to `/v2/kubernetes/clusters/$K8S_CLUSTER_ID/upgrades`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **uuid::Uuid** | A unique ID that can be used to reference a Kubernetes cluster. | [required] |

### Return type

[**models::KubernetesGetAvailableUpgrades200Response**](kubernetes_get_availableUpgrades_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_get_cluster

> models::KubernetesGetCluster200Response kubernetes_get_cluster(cluster_id)
Retrieve an Existing Kubernetes Cluster

To show information about an existing Kubernetes cluster, send a GET request to `/v2/kubernetes/clusters/$K8S_CLUSTER_ID`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **uuid::Uuid** | A unique ID that can be used to reference a Kubernetes cluster. | [required] |

### Return type

[**models::KubernetesGetCluster200Response**](kubernetes_get_cluster_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_get_cluster_lint_results

> models::ClusterlintResults kubernetes_get_cluster_lint_results(cluster_id, run_id)
Fetch Clusterlint Diagnostics for a Kubernetes Cluster

To request clusterlint diagnostics for your cluster, send a GET request to `/v2/kubernetes/clusters/$K8S_CLUSTER_ID/clusterlint`. If the `run_id` query parameter is provided, then the diagnostics for the specific run is fetched. By default, the latest results are shown.  To find out how to address clusterlint feedback, please refer to [the clusterlint check documentation](https://github.com/digitalocean/clusterlint/blob/master/checks.md). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **uuid::Uuid** | A unique ID that can be used to reference a Kubernetes cluster. | [required] |
**run_id** | Option<**uuid::Uuid**> | Specifies the clusterlint run whose results will be retrieved. |  |

### Return type

[**models::ClusterlintResults**](clusterlint_results.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_get_cluster_user

> models::User kubernetes_get_cluster_user(cluster_id)
Retrieve User Information for a Kubernetes Cluster

To show information the user associated with a Kubernetes cluster, send a GET request to `/v2/kubernetes/clusters/$K8S_CLUSTER_ID/user`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **uuid::Uuid** | A unique ID that can be used to reference a Kubernetes cluster. | [required] |

### Return type

[**models::User**](user.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_get_credentials

> models::Credentials kubernetes_get_credentials(cluster_id, expiry_seconds)
Retrieve Credentials for a Kubernetes Cluster

This endpoint returns a JSON object . It can be used to programmatically construct Kubernetes clients which cannot parse kubeconfig files.  The resulting JSON object contains token-based authentication for clusters supporting it, and certificate-based authentication otherwise. For a list of supported versions and more information, see \"[How to Connect to a DigitalOcean Kubernetes Cluster](https://docs.digitalocean.com/products/kubernetes/how-to/connect-to-cluster/)\".  To retrieve credentials for accessing a Kubernetes cluster, send a GET request to `/v2/kubernetes/clusters/$K8S_CLUSTER_ID/credentials`.  Clusters supporting token-based authentication may define an expiration by passing a duration in seconds as a query parameter to `/v2/kubernetes/clusters/$K8S_CLUSTER_ID/credentials?expiry_seconds=$DURATION_IN_SECONDS`. If not set or 0, then the token will have a 7 day expiry. The query parameter has no impact in certificate-based authentication. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **uuid::Uuid** | A unique ID that can be used to reference a Kubernetes cluster. | [required] |
**expiry_seconds** | Option<**i32**> | The duration in seconds that the returned Kubernetes credentials will be valid. If not set or 0, the credentials will have a 7 day expiry. |  |[default to 0]

### Return type

[**models::Credentials**](credentials.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_get_kubeconfig

> kubernetes_get_kubeconfig(cluster_id, expiry_seconds)
Retrieve the kubeconfig for a Kubernetes Cluster

This endpoint returns a kubeconfig file in YAML format. It can be used to connect to and administer the cluster using the Kubernetes command line tool, `kubectl`, or other programs supporting kubeconfig files (e.g., client libraries).  The resulting kubeconfig file uses token-based authentication for clusters supporting it, and certificate-based authentication otherwise. For a list of supported versions and more information, see \"[How to Connect to a DigitalOcean Kubernetes Cluster](https://docs.digitalocean.com/products/kubernetes/how-to/connect-to-cluster/)\".  To retrieve a kubeconfig file for use with a Kubernetes cluster, send a GET request to `/v2/kubernetes/clusters/$K8S_CLUSTER_ID/kubeconfig`.  Clusters supporting token-based authentication may define an expiration by passing a duration in seconds as a query parameter to `/v2/kubernetes/clusters/$K8S_CLUSTER_ID/kubeconfig?expiry_seconds=$DURATION_IN_SECONDS`. If not set or 0, then the token will have a 7 day expiry. The query parameter has no impact in certificate-based authentication.  Kubernetes Roles granted to a user with a token-based kubeconfig are derived from that user's DigitalOcean role. Predefined roles (Owner, Member, Modifier etc.) have an automatic mapping to Kubernetes roles. Custom roles are not automatically mapped to any Kubernetes roles, and require [additional configuration](https://docs.digitalocean.com/products/kubernetes/how-to/set-up-custom-rolebindings/) by a cluster administrator. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **uuid::Uuid** | A unique ID that can be used to reference a Kubernetes cluster. | [required] |
**expiry_seconds** | Option<**i32**> | The duration in seconds that the returned Kubernetes credentials will be valid. If not set or 0, the credentials will have a 7 day expiry. |  |[default to 0]

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/yaml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_get_node_pool

> models::KubernetesGetNodePool200Response kubernetes_get_node_pool(cluster_id, node_pool_id)
Retrieve a Node Pool for a Kubernetes Cluster

To show information about a specific node pool in a Kubernetes cluster, send a GET request to `/v2/kubernetes/clusters/$K8S_CLUSTER_ID/node_pools/$NODE_POOL_ID`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **uuid::Uuid** | A unique ID that can be used to reference a Kubernetes cluster. | [required] |
**node_pool_id** | **uuid::Uuid** | A unique ID that can be used to reference a Kubernetes node pool. | [required] |

### Return type

[**models::KubernetesGetNodePool200Response**](kubernetes_get_nodePool_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_get_status_messages

> models::KubernetesGetStatusMessages200Response kubernetes_get_status_messages(cluster_id, since)
Fetch Status Messages for a Kubernetes Cluster

To retrieve status messages for a Kubernetes cluster, send a GET request to `/v2/kubernetes/clusters/$K8S_CLUSTER_ID/status_messages`. Status messages inform users of any issues that come up during the cluster lifecycle. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **uuid::Uuid** | A unique ID that can be used to reference a Kubernetes cluster. | [required] |
**since** | Option<**String**> | A timestamp used to return status messages emitted since the specified time. The timestamp should be in ISO8601 format. |  |

### Return type

[**models::KubernetesGetStatusMessages200Response**](kubernetes_get_status_messages_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_list_associated_resources

> models::AssociatedKubernetesResources kubernetes_list_associated_resources(cluster_id)
List Associated Resources for Cluster Deletion

To list the associated billable resources that can be destroyed along with a cluster, send a GET request to the `/v2/kubernetes/clusters/$K8S_CLUSTER_ID/destroy_with_associated_resources` endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **uuid::Uuid** | A unique ID that can be used to reference a Kubernetes cluster. | [required] |

### Return type

[**models::AssociatedKubernetesResources**](associated_kubernetes_resources.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_list_clusters

> models::KubernetesListClusters200Response kubernetes_list_clusters(per_page, page)
List All Kubernetes Clusters

To list all of the Kubernetes clusters on your account, send a GET request to `/v2/kubernetes/clusters`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**models::KubernetesListClusters200Response**](kubernetes_list_clusters_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_list_node_pools

> models::KubernetesListNodePools200Response kubernetes_list_node_pools(cluster_id)
List All Node Pools in a Kubernetes Clusters

To list all of the node pools in a Kubernetes clusters, send a GET request to `/v2/kubernetes/clusters/$K8S_CLUSTER_ID/node_pools`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **uuid::Uuid** | A unique ID that can be used to reference a Kubernetes cluster. | [required] |

### Return type

[**models::KubernetesListNodePools200Response**](kubernetes_list_nodePools_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_list_options

> models::KubernetesOptions kubernetes_list_options()
List Available Regions, Node Sizes, and Versions of Kubernetes

To list the versions of Kubernetes available for use, the regions that support Kubernetes, and the available node sizes, send a GET request to `/v2/kubernetes/options`.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::KubernetesOptions**](kubernetes_options.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_recycle_node_pool

> kubernetes_recycle_node_pool(cluster_id, node_pool_id, kubernetes_recycle_node_pool_request)
Recycle a Kubernetes Node Pool

The endpoint has been deprecated. Please use the DELETE `/v2/kubernetes/clusters/$K8S_CLUSTER_ID/node_pools/$NODE_POOL_ID/nodes/$NODE_ID` method instead. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **uuid::Uuid** | A unique ID that can be used to reference a Kubernetes cluster. | [required] |
**node_pool_id** | **uuid::Uuid** | A unique ID that can be used to reference a Kubernetes node pool. | [required] |
**kubernetes_recycle_node_pool_request** | [**KubernetesRecycleNodePoolRequest**](KubernetesRecycleNodePoolRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_remove_registries

> kubernetes_remove_registries(cluster_registries)
Remove Container Registries from Kubernetes Clusters

To remove the container registries from Kubernetes clusters, send a DELETE request to `/v2/kubernetes/registries`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_registries** | Option<[**ClusterRegistries**](ClusterRegistries.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_remove_registry

> kubernetes_remove_registry(cluster_registry)
Remove Container Registry from Kubernetes Clusters

To remove the container registry from Kubernetes clusters, send a DELETE request to `/v2/kubernetes/registry`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_registry** | Option<[**ClusterRegistry**](ClusterRegistry.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_run_cluster_lint

> models::KubernetesRunClusterLint202Response kubernetes_run_cluster_lint(cluster_id, clusterlint_request)
Run Clusterlint Checks on a Kubernetes Cluster

Clusterlint helps operators conform to Kubernetes best practices around resources, security and reliability to avoid common problems while operating or upgrading the clusters.  To request a clusterlint run on your cluster, send a POST request to `/v2/kubernetes/clusters/$K8S_CLUSTER_ID/clusterlint`. This will run all checks present in the `doks` group by default, if a request body is not specified. Optionally specify the below attributes.  For information about the available checks, please refer to [the clusterlint check documentation](https://github.com/digitalocean/clusterlint/blob/master/checks.md). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **uuid::Uuid** | A unique ID that can be used to reference a Kubernetes cluster. | [required] |
**clusterlint_request** | Option<[**ClusterlintRequest**](ClusterlintRequest.md)> |  |  |

### Return type

[**models::KubernetesRunClusterLint202Response**](kubernetes_run_clusterLint_202_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_update_cluster

> models::KubernetesCreateCluster201Response kubernetes_update_cluster(cluster_id, cluster_update)
Update a Kubernetes Cluster

To update a Kubernetes cluster, send a PUT request to `/v2/kubernetes/clusters/$K8S_CLUSTER_ID` and specify one or more of the attributes below. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **uuid::Uuid** | A unique ID that can be used to reference a Kubernetes cluster. | [required] |
**cluster_update** | [**ClusterUpdate**](ClusterUpdate.md) |  | [required] |

### Return type

[**models::KubernetesCreateCluster201Response**](kubernetes_create_cluster_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_update_node_pool

> models::KubernetesUpdateNodePool202Response kubernetes_update_node_pool(cluster_id, node_pool_id, kubernetes_node_pool_update)
Update a Node Pool in a Kubernetes Cluster

To update the name of a node pool, edit the tags applied to it, or adjust its number of nodes, send a PUT request to `/v2/kubernetes/clusters/$K8S_CLUSTER_ID/node_pools/$NODE_POOL_ID` with the following attributes. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **uuid::Uuid** | A unique ID that can be used to reference a Kubernetes cluster. | [required] |
**node_pool_id** | **uuid::Uuid** | A unique ID that can be used to reference a Kubernetes node pool. | [required] |
**kubernetes_node_pool_update** | [**KubernetesNodePoolUpdate**](KubernetesNodePoolUpdate.md) |  | [required] |

### Return type

[**models::KubernetesUpdateNodePool202Response**](kubernetes_update_nodePool_202_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kubernetes_upgrade_cluster

> kubernetes_upgrade_cluster(cluster_id, kubernetes_upgrade_cluster_request)
Upgrade a Kubernetes Cluster

To immediately upgrade a Kubernetes cluster to a newer patch release of Kubernetes, send a POST request to `/v2/kubernetes/clusters/$K8S_CLUSTER_ID/upgrade`. The body of the request must specify a version attribute.  Available upgrade versions for a cluster can be fetched from `/v2/kubernetes/clusters/$K8S_CLUSTER_ID/upgrades`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **uuid::Uuid** | A unique ID that can be used to reference a Kubernetes cluster. | [required] |
**kubernetes_upgrade_cluster_request** | [**KubernetesUpgradeClusterRequest**](KubernetesUpgradeClusterRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

