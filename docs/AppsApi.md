# \AppsApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**apps_assign_alert_destinations**](AppsApi.md#apps_assign_alert_destinations) | **POST** /v2/apps/{app_id}/alerts/{alert_id}/destinations | Update destinations for alerts
[**apps_cancel_deployment**](AppsApi.md#apps_cancel_deployment) | **POST** /v2/apps/{app_id}/deployments/{deployment_id}/cancel | Cancel a Deployment
[**apps_commit_rollback**](AppsApi.md#apps_commit_rollback) | **POST** /v2/apps/{app_id}/rollback/commit | Commit App Rollback
[**apps_create**](AppsApi.md#apps_create) | **POST** /v2/apps | Create a New App
[**apps_create_deployment**](AppsApi.md#apps_create_deployment) | **POST** /v2/apps/{app_id}/deployments | Create an App Deployment
[**apps_create_rollback**](AppsApi.md#apps_create_rollback) | **POST** /v2/apps/{app_id}/rollback | Rollback App
[**apps_delete**](AppsApi.md#apps_delete) | **DELETE** /v2/apps/{id} | Delete an App
[**apps_get**](AppsApi.md#apps_get) | **GET** /v2/apps/{id} | Retrieve an Existing App
[**apps_get_deployment**](AppsApi.md#apps_get_deployment) | **GET** /v2/apps/{app_id}/deployments/{deployment_id} | Retrieve an App Deployment
[**apps_get_exec**](AppsApi.md#apps_get_exec) | **GET** /v2/apps/{app_id}/deployments/{deployment_id}/components/{component_name}/exec | Retrieve Exec URL for Deployment
[**apps_get_exec_active_deployment**](AppsApi.md#apps_get_exec_active_deployment) | **GET** /v2/apps/{app_id}/components/{component_name}/exec | Retrieve Exec URL
[**apps_get_health**](AppsApi.md#apps_get_health) | **GET** /v2/apps/{app_id}/health | Retrieve App Health
[**apps_get_instance_size**](AppsApi.md#apps_get_instance_size) | **GET** /v2/apps/tiers/instance_sizes/{slug} | Retrieve an Instance Size
[**apps_get_instances**](AppsApi.md#apps_get_instances) | **GET** /v2/apps/{app_id}/instances | Retrieve App Instances
[**apps_get_job_invocation**](AppsApi.md#apps_get_job_invocation) | **GET** /v2/apps/{app_id}/job-invocations/{job_invocation_id} | Get Job Invocations
[**apps_get_job_invocation_logs**](AppsApi.md#apps_get_job_invocation_logs) | **GET** /v2/apps/{app_id}/jobs/{job_name}/invocations/{job_invocation_id}/logs | Retrieve Job Invocation Logs
[**apps_get_logs**](AppsApi.md#apps_get_logs) | **GET** /v2/apps/{app_id}/deployments/{deployment_id}/components/{component_name}/logs | Retrieve Deployment Logs
[**apps_get_logs_active_deployment**](AppsApi.md#apps_get_logs_active_deployment) | **GET** /v2/apps/{app_id}/components/{component_name}/logs | Retrieve Active Deployment Logs
[**apps_get_logs_active_deployment_aggregate**](AppsApi.md#apps_get_logs_active_deployment_aggregate) | **GET** /v2/apps/{app_id}/logs | Retrieve Active Deployment Aggregate Logs
[**apps_get_logs_aggregate**](AppsApi.md#apps_get_logs_aggregate) | **GET** /v2/apps/{app_id}/deployments/{deployment_id}/logs | Retrieve Aggregate Deployment Logs
[**apps_get_metrics_bandwidth_daily**](AppsApi.md#apps_get_metrics_bandwidth_daily) | **GET** /v2/apps/{app_id}/metrics/bandwidth_daily | Retrieve App Daily Bandwidth Metrics
[**apps_list**](AppsApi.md#apps_list) | **GET** /v2/apps | List All Apps
[**apps_list_alerts**](AppsApi.md#apps_list_alerts) | **GET** /v2/apps/{app_id}/alerts | List all app alerts
[**apps_list_deployments**](AppsApi.md#apps_list_deployments) | **GET** /v2/apps/{app_id}/deployments | List App Deployments
[**apps_list_instance_sizes**](AppsApi.md#apps_list_instance_sizes) | **GET** /v2/apps/tiers/instance_sizes | List Instance Sizes
[**apps_list_job_invocations**](AppsApi.md#apps_list_job_invocations) | **GET** /v2/apps/{app_id}/job-invocations | List Job Invocations
[**apps_list_metrics_bandwidth_daily**](AppsApi.md#apps_list_metrics_bandwidth_daily) | **POST** /v2/apps/metrics/bandwidth_daily | Retrieve Multiple Apps' Daily Bandwidth Metrics
[**apps_list_regions**](AppsApi.md#apps_list_regions) | **GET** /v2/apps/regions | List App Regions
[**apps_restart**](AppsApi.md#apps_restart) | **POST** /v2/apps/{app_id}/restart | Restart an App
[**apps_revert_rollback**](AppsApi.md#apps_revert_rollback) | **POST** /v2/apps/{app_id}/rollback/revert | Revert App Rollback
[**apps_update**](AppsApi.md#apps_update) | **PUT** /v2/apps/{id} | Update an App
[**apps_validate_app_spec**](AppsApi.md#apps_validate_app_spec) | **POST** /v2/apps/propose | Propose an App Spec
[**apps_validate_rollback**](AppsApi.md#apps_validate_rollback) | **POST** /v2/apps/{app_id}/rollback/validate | Validate App Rollback



## apps_assign_alert_destinations

> models::AppsAlertResponse apps_assign_alert_destinations(app_id, alert_id, apps_assign_app_alert_destinations_request)
Update destinations for alerts

Updates the emails and slack webhook destinations for app alerts. Emails must be associated to a user with access to the app.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app ID | [required] |
**alert_id** | **String** | The alert ID | [required] |
**apps_assign_app_alert_destinations_request** | [**AppsAssignAppAlertDestinationsRequest**](AppsAssignAppAlertDestinationsRequest.md) |  | [required] |

### Return type

[**models::AppsAlertResponse**](apps_alert_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_cancel_deployment

> models::AppsDeploymentResponse apps_cancel_deployment(app_id, deployment_id)
Cancel a Deployment

Immediately cancel an in-progress deployment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app ID | [required] |
**deployment_id** | **String** | The deployment ID | [required] |

### Return type

[**models::AppsDeploymentResponse**](apps_deployment_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_commit_rollback

> apps_commit_rollback(app_id)
Commit App Rollback

Commit an app rollback. This action permanently applies the rollback and unpins the app to resume new deployments. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app ID | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_create

> models::AppResponse apps_create(apps_create_app_request, accept, content_type)
Create a New App

Create a new app by submitting an app specification. For documentation on app specifications (`AppSpec` objects), please refer to [the product documentation](https://docs.digitalocean.com/products/app-platform/reference/app-spec/).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**apps_create_app_request** | [**AppsCreateAppRequest**](AppsCreateAppRequest.md) |  | [required] |
**accept** | Option<**String**> | The content-type that should be used by the response. By default, the response will be `application/json`. `application/yaml` is also supported. |  |
**content_type** | Option<**String**> | The content-type used for the request. By default, the requests are assumed to use `application/json`. `application/yaml` is also supported. |  |

### Return type

[**models::AppResponse**](app_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_create_deployment

> models::AppsDeploymentResponse apps_create_deployment(app_id, apps_create_deployment_request)
Create an App Deployment

Creating an app deployment will pull the latest changes from your repository and schedule a new deployment for your app.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app ID | [required] |
**apps_create_deployment_request** | [**AppsCreateDeploymentRequest**](AppsCreateDeploymentRequest.md) |  | [required] |

### Return type

[**models::AppsDeploymentResponse**](apps_deployment_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_create_rollback

> models::AppsDeploymentResponse apps_create_rollback(app_id, apps_rollback_app_request)
Rollback App

Rollback an app to a previous deployment. A new deployment will be created to perform the rollback. The app will be pinned to the rollback deployment preventing any new deployments from being created, either manually or through Auto Deploy on Push webhooks. To resume deployments, the rollback must be either committed or reverted.  It is recommended to use the Validate App Rollback endpoint to double check if the rollback is valid and if there are any warnings. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app ID | [required] |
**apps_rollback_app_request** | [**AppsRollbackAppRequest**](AppsRollbackAppRequest.md) |  | [required] |

### Return type

[**models::AppsDeploymentResponse**](apps_deployment_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_delete

> models::AppsDeleteAppResponse apps_delete(id)
Delete an App

Delete an existing app. Once deleted, all active deployments will be permanently shut down and the app deleted. If needed, be sure to back up your app specification so that you may re-create it at a later time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the app | [required] |

### Return type

[**models::AppsDeleteAppResponse**](apps_delete_app_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_get

> models::AppResponse apps_get(id, name)
Retrieve an Existing App

Retrieve details about an existing app by either its ID or name. To retrieve an app by its name, do not include an ID in the request path. Information about the current active deployment as well as any in progress ones will also be included in the response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the app | [required] |
**name** | Option<**String**> | The name of the app to retrieve. |  |

### Return type

[**models::AppResponse**](app_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_get_deployment

> models::AppsDeploymentResponse apps_get_deployment(app_id, deployment_id)
Retrieve an App Deployment

Retrieve information about an app deployment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app ID | [required] |
**deployment_id** | **String** | The deployment ID | [required] |

### Return type

[**models::AppsDeploymentResponse**](apps_deployment_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_get_exec

> models::AppsGetExecResponse apps_get_exec(app_id, deployment_id, component_name, instance_name)
Retrieve Exec URL for Deployment

Returns a websocket URL that allows sending/receiving console input and output to a component of the specified deployment if one exists. Optionally, the instance_name parameter can be provided to retrieve the exec URL for a specific instance. Note that instances are ephemeral; therefore, we recommended to avoid making persistent changes or such scripting around them.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app ID | [required] |
**deployment_id** | **String** | The deployment ID | [required] |
**component_name** | **String** | An optional component name. If set, logs will be limited to this component only. | [required] |
**instance_name** | Option<**String**> | The name of the actively running ephemeral compute instance |  |

### Return type

[**models::AppsGetExecResponse**](apps_get_exec_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_get_exec_active_deployment

> models::AppsGetExecResponse apps_get_exec_active_deployment(app_id, component_name, instance_name)
Retrieve Exec URL

Returns a websocket URL that allows sending/receiving console input and output to a component of the active deployment if one exists.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app ID | [required] |
**component_name** | **String** | An optional component name. If set, logs will be limited to this component only. | [required] |
**instance_name** | Option<**String**> | The name of the actively running ephemeral compute instance |  |

### Return type

[**models::AppsGetExecResponse**](apps_get_exec_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_get_health

> models::AppHealthResponse apps_get_health(app_id)
Retrieve App Health

Retrieve information like health status, cpu and memory utilization of app components.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app ID | [required] |

### Return type

[**models::AppHealthResponse**](app_health_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_get_instance_size

> models::AppsGetInstanceSizeResponse apps_get_instance_size(slug)
Retrieve an Instance Size

Retrieve information about a specific instance size for `service`, `worker`, and `job` components.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | The slug of the instance size | [required] |

### Return type

[**models::AppsGetInstanceSizeResponse**](apps_get_instance_size_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_get_instances

> models::AppInstances apps_get_instances(app_id)
Retrieve App Instances

Retrieve the list of running instances for a given application, including instance names and component types. Please note that these instances are ephemeral and may change over time. It is recommended not to make persistent changes or develop scripts that rely on their persistence.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app ID | [required] |

### Return type

[**models::AppInstances**](app_instances.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_get_job_invocation

> models::AppJobInvocation apps_get_job_invocation(app_id, job_invocation_id, job_name)
Get Job Invocations

Get a specific job invocation for an app.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app ID | [required] |
**job_invocation_id** | **String** | The ID of the job invocation to retrieve. | [required] |
**job_name** | Option<**String**> | The job name to list job invocations for. |  |

### Return type

[**models::AppJobInvocation**](app_job_invocation.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_get_job_invocation_logs

> models::AppsGetLogsResponse apps_get_job_invocation_logs(app_id, job_name, job_invocation_id, r#type, deployment_id, follow, pod_connection_timeout, tail_lines)
Retrieve Job Invocation Logs

Retrieve the logs of a past, in-progress, or active deployment. If a component name is specified, the logs will be limited to only that component. If deployment is omitted the active deployment will be selected (if available). The response will include links to either real-time logs of an in-progress or active deployment or archived logs of a past deployment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app ID | [required] |
**job_name** | **String** | The job name to list job invocations for. | [required] |
**job_invocation_id** | **String** | The ID of the job invocation to retrieve. | [required] |
**r#type** | **String** | The type of logs to retrieve | [required] |
**deployment_id** | Option<**String**> | The deployment ID |  |
**follow** | Option<**bool**> | Whether the logs should follow live updates. |  |
**pod_connection_timeout** | Option<**String**> | An optional time duration to wait if the underlying component instance is not immediately available. Default: `3m`. |  |
**tail_lines** | Option<**String**> | The number of lines from the end of the logs to retrieve. |  |

### Return type

[**models::AppsGetLogsResponse**](apps_get_logs_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_get_logs

> models::AppsGetLogsResponse apps_get_logs(app_id, deployment_id, component_name, r#type, follow, pod_connection_timeout)
Retrieve Deployment Logs

Retrieve the logs of a past, in-progress, or active deployment. The response will include links to either real-time logs of an in-progress or active deployment or archived logs of a past deployment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app ID | [required] |
**deployment_id** | **String** | The deployment ID | [required] |
**component_name** | **String** | An optional component name. If set, logs will be limited to this component only. | [required] |
**r#type** | **String** | The type of logs to retrieve - BUILD: Build-time logs - DEPLOY: Deploy-time logs - RUN: Live run-time logs - RUN_RESTARTED: Logs of crashed/restarted instances during runtime | [required] |[default to UNSPECIFIED]
**follow** | Option<**bool**> | Whether the logs should follow live updates. |  |
**pod_connection_timeout** | Option<**String**> | An optional time duration to wait if the underlying component instance is not immediately available. Default: `3m`. |  |

### Return type

[**models::AppsGetLogsResponse**](apps_get_logs_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_get_logs_active_deployment

> models::AppsGetLogsResponse apps_get_logs_active_deployment(app_id, component_name, r#type, follow, pod_connection_timeout)
Retrieve Active Deployment Logs

Retrieve the logs of the active deployment if one exists. The response will include links to either real-time logs of an in-progress or active deployment or archived logs of a past deployment. Note log_type=BUILD logs will return logs associated with the current active deployment (being served). To view build logs associated with in-progress build, the query must explicitly reference the deployment id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app ID | [required] |
**component_name** | **String** | An optional component name. If set, logs will be limited to this component only. | [required] |
**r#type** | **String** | The type of logs to retrieve - BUILD: Build-time logs - DEPLOY: Deploy-time logs - RUN: Live run-time logs - RUN_RESTARTED: Logs of crashed/restarted instances during runtime | [required] |[default to UNSPECIFIED]
**follow** | Option<**bool**> | Whether the logs should follow live updates. |  |
**pod_connection_timeout** | Option<**String**> | An optional time duration to wait if the underlying component instance is not immediately available. Default: `3m`. |  |

### Return type

[**models::AppsGetLogsResponse**](apps_get_logs_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_get_logs_active_deployment_aggregate

> models::AppsGetLogsResponse apps_get_logs_active_deployment_aggregate(app_id, r#type, follow, pod_connection_timeout)
Retrieve Active Deployment Aggregate Logs

Retrieve the logs of the active deployment if one exists. The response will include links to either real-time logs of an in-progress or active deployment or archived logs of a past deployment. Note log_type=BUILD logs will return logs associated with the current active deployment (being served). To view build logs associated with in-progress build, the query must explicitly reference the deployment id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app ID | [required] |
**r#type** | **String** | The type of logs to retrieve - BUILD: Build-time logs - DEPLOY: Deploy-time logs - RUN: Live run-time logs - RUN_RESTARTED: Logs of crashed/restarted instances during runtime | [required] |[default to UNSPECIFIED]
**follow** | Option<**bool**> | Whether the logs should follow live updates. |  |
**pod_connection_timeout** | Option<**String**> | An optional time duration to wait if the underlying component instance is not immediately available. Default: `3m`. |  |

### Return type

[**models::AppsGetLogsResponse**](apps_get_logs_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_get_logs_aggregate

> models::AppsGetLogsResponse apps_get_logs_aggregate(app_id, deployment_id, r#type, follow, pod_connection_timeout)
Retrieve Aggregate Deployment Logs

Retrieve the logs of a past, in-progress, or active deployment. If a component name is specified, the logs will be limited to only that component. The response will include links to either real-time logs of an in-progress or active deployment or archived logs of a past deployment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app ID | [required] |
**deployment_id** | **String** | The deployment ID | [required] |
**r#type** | **String** | The type of logs to retrieve - BUILD: Build-time logs - DEPLOY: Deploy-time logs - RUN: Live run-time logs - RUN_RESTARTED: Logs of crashed/restarted instances during runtime | [required] |[default to UNSPECIFIED]
**follow** | Option<**bool**> | Whether the logs should follow live updates. |  |
**pod_connection_timeout** | Option<**String**> | An optional time duration to wait if the underlying component instance is not immediately available. Default: `3m`. |  |

### Return type

[**models::AppsGetLogsResponse**](apps_get_logs_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_get_metrics_bandwidth_daily

> models::AppMetricsBandwidthUsage apps_get_metrics_bandwidth_daily(app_id, date)
Retrieve App Daily Bandwidth Metrics

Retrieve daily bandwidth usage metrics for a single app.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app ID | [required] |
**date** | Option<**String**> | Optional day to query. Only the date component of the timestamp will be considered. Default: yesterday. |  |

### Return type

[**models::AppMetricsBandwidthUsage**](app_metrics_bandwidth_usage.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_list

> models::AppsResponse apps_list(page, per_page, with_projects)
List All Apps

List all apps on your account. Information about the current active deployment as well as any in progress ones will also be included for each app.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**with_projects** | Option<**bool**> | Whether the project_id of listed apps should be fetched and included. |  |

### Return type

[**models::AppsResponse**](apps_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_list_alerts

> models::AppsListAlertsResponse apps_list_alerts(app_id)
List all app alerts

List alerts associated to the app and any components. This includes configuration information about the alerts including emails, slack webhooks, and triggering events or conditions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app ID | [required] |

### Return type

[**models::AppsListAlertsResponse**](apps_list_alerts_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_list_deployments

> models::AppsDeploymentsResponse apps_list_deployments(app_id, page, per_page, deployment_types)
List App Deployments

List all deployments of an app.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app ID | [required] |
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**deployment_types** | Option<[**Vec<String>**](String.md)> | Optional. Filter deployments by deployment_type   - MANUAL: manual deployment   - DEPLOY_ON_PUSH: deployment triggered by a push to the app's repository   - MAINTENANCE: deployment for maintenance purposes   - MANUAL_ROLLBACK: manual revert to a previous deployment   - AUTO_ROLLBACK: automatic revert to a previous deployment   - UPDATE_DATABASE_TRUSTED_SOURCES: update database trusted sources   - AUTOSCALED: deployment that has been autoscaled |  |

### Return type

[**models::AppsDeploymentsResponse**](apps_deployments_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_list_instance_sizes

> models::AppsListInstanceSizesResponse apps_list_instance_sizes()
List Instance Sizes

List all instance sizes for `service`, `worker`, and `job` components.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AppsListInstanceSizesResponse**](apps_list_instance_sizes_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_list_job_invocations

> models::AppJobInvocations apps_list_job_invocations(app_id, job_names, deployment_id, page, per_page)
List Job Invocations

List all job invocations for an app.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app ID | [required] |
**job_names** | Option<[**Vec<String>**](String.md)> | The job names to list job invocations for. |  |
**deployment_id** | Option<**String**> | The deployment ID |  |
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]

### Return type

[**models::AppJobInvocations**](app_job_invocations.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_list_metrics_bandwidth_daily

> models::AppMetricsBandwidthUsage apps_list_metrics_bandwidth_daily(app_metrics_bandwidth_usage_request)
Retrieve Multiple Apps' Daily Bandwidth Metrics

Retrieve daily bandwidth usage metrics for multiple apps.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_metrics_bandwidth_usage_request** | [**AppMetricsBandwidthUsageRequest**](AppMetricsBandwidthUsageRequest.md) |  | [required] |

### Return type

[**models::AppMetricsBandwidthUsage**](app_metrics_bandwidth_usage.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_list_regions

> models::AppsListRegionsResponse apps_list_regions()
List App Regions

List all regions supported by App Platform.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AppsListRegionsResponse**](apps_list_regions_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_restart

> models::AppsDeploymentResponse apps_restart(app_id, apps_restart_request)
Restart an App

Perform a rolling restart of all or specific components in an app.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app ID | [required] |
**apps_restart_request** | Option<[**AppsRestartRequest**](AppsRestartRequest.md)> |  |  |

### Return type

[**models::AppsDeploymentResponse**](apps_deployment_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_revert_rollback

> models::AppsDeploymentResponse apps_revert_rollback(app_id)
Revert App Rollback

Revert an app rollback. This action reverts the active rollback by creating a new deployment from the latest app spec prior to the rollback and unpins the app to resume new deployments. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app ID | [required] |

### Return type

[**models::AppsDeploymentResponse**](apps_deployment_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_update

> models::AppResponse apps_update(id, apps_update_app_request)
Update an App

Update an existing app by submitting a new app specification. For documentation on app specifications (`AppSpec` objects), please refer to [the product documentation](https://docs.digitalocean.com/products/app-platform/reference/app-spec/).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the app | [required] |
**apps_update_app_request** | [**AppsUpdateAppRequest**](AppsUpdateAppRequest.md) |  | [required] |

### Return type

[**models::AppResponse**](app_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_validate_app_spec

> models::AppProposeResponse apps_validate_app_spec(app_propose)
Propose an App Spec

To propose and validate a spec for a new or existing app, send a POST request to the `/v2/apps/propose` endpoint. The request returns some information about the proposed app, including app cost and upgrade cost. If an existing app ID is specified, the app spec is treated as a proposed update to the existing app.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_propose** | [**AppPropose**](AppPropose.md) |  | [required] |

### Return type

[**models::AppProposeResponse**](app_propose_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_validate_rollback

> models::AppsValidateRollback200Response apps_validate_rollback(app_id, apps_rollback_app_request)
Validate App Rollback

Check whether an app can be rolled back to a specific deployment. This endpoint can also be used to check if there are any warnings or validation conditions that will cause the rollback to proceed under unideal circumstances. For example, if a component must be rebuilt as part of the rollback causing it to take longer than usual. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app ID | [required] |
**apps_rollback_app_request** | [**AppsRollbackAppRequest**](AppsRollbackAppRequest.md) |  | [required] |

### Return type

[**models::AppsValidateRollback200Response**](apps_validate_rollback_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

