# \UptimeApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**uptime_create_alert**](UptimeApi.md#uptime_create_alert) | **POST** /v2/uptime/checks/{check_id}/alerts | Create a New Alert
[**uptime_create_check**](UptimeApi.md#uptime_create_check) | **POST** /v2/uptime/checks | Create a New Check
[**uptime_delete_alert**](UptimeApi.md#uptime_delete_alert) | **DELETE** /v2/uptime/checks/{check_id}/alerts/{alert_id} | Delete an Alert
[**uptime_delete_check**](UptimeApi.md#uptime_delete_check) | **DELETE** /v2/uptime/checks/{check_id} | Delete a Check
[**uptime_get_alert**](UptimeApi.md#uptime_get_alert) | **GET** /v2/uptime/checks/{check_id}/alerts/{alert_id} | Retrieve an Existing Alert
[**uptime_get_check**](UptimeApi.md#uptime_get_check) | **GET** /v2/uptime/checks/{check_id} | Retrieve an Existing Check
[**uptime_get_check_state**](UptimeApi.md#uptime_get_check_state) | **GET** /v2/uptime/checks/{check_id}/state | Retrieve Check State
[**uptime_list_alerts**](UptimeApi.md#uptime_list_alerts) | **GET** /v2/uptime/checks/{check_id}/alerts | List All Alerts
[**uptime_list_checks**](UptimeApi.md#uptime_list_checks) | **GET** /v2/uptime/checks | List All Checks
[**uptime_update_alert**](UptimeApi.md#uptime_update_alert) | **PUT** /v2/uptime/checks/{check_id}/alerts/{alert_id} | Update an Alert
[**uptime_update_check**](UptimeApi.md#uptime_update_check) | **PUT** /v2/uptime/checks/{check_id} | Update a Check



## uptime_create_alert

> models::UptimeCreateAlert201Response uptime_create_alert(check_id, alert)
Create a New Alert

To create an Uptime alert, send a POST request to `/v2/uptime/checks/$CHECK_ID/alerts` specifying the attributes in the table below in the JSON body. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**check_id** | **uuid::Uuid** | A unique identifier for a check. | [required] |
**alert** | [**Alert**](Alert.md) | The ''type'' field dictates the type of alert, and hence what type of value to pass into the threshold property. Type | Description | Threshold Value -----|-------------|-------------------- `latency` | alerts on the response latency | milliseconds `down` | alerts on a target registering as down in any region | N/A (Not required) `down_global` | alerts on a target registering as down globally | N/A (Not required) `ssl_expiry` | alerts on a SSL certificate expiring within $threshold days | days  | [required] |

### Return type

[**models::UptimeCreateAlert201Response**](uptime_create_alert_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## uptime_create_check

> models::UptimeCreateCheck201Response uptime_create_check(check_updatable)
Create a New Check

To create an Uptime check, send a POST request to `/v2/uptime/checks` specifying the attributes in the table below in the JSON body. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**check_updatable** | [**CheckUpdatable**](CheckUpdatable.md) |  | [required] |

### Return type

[**models::UptimeCreateCheck201Response**](uptime_create_check_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## uptime_delete_alert

> uptime_delete_alert(check_id, alert_id)
Delete an Alert

To delete an Uptime alert, send a DELETE request to `/v2/uptime/checks/$CHECK_ID/alerts/$ALERT_ID`. A 204 status code with no body will be returned in response to a successful request. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**check_id** | **uuid::Uuid** | A unique identifier for a check. | [required] |
**alert_id** | **uuid::Uuid** | A unique identifier for an alert. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## uptime_delete_check

> uptime_delete_check(check_id)
Delete a Check

To delete an Uptime check, send a DELETE request to `/v2/uptime/checks/$CHECK_ID`. A 204 status code with no body will be returned in response to a successful request.   Deleting a check will also delete alerts associated with the check. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**check_id** | **uuid::Uuid** | A unique identifier for a check. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## uptime_get_alert

> models::UptimeCreateAlert201Response uptime_get_alert(check_id, alert_id)
Retrieve an Existing Alert

To show information about an existing alert, send a GET request to `/v2/uptime/checks/$CHECK_ID/alerts/$ALERT_ID`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**check_id** | **uuid::Uuid** | A unique identifier for a check. | [required] |
**alert_id** | **uuid::Uuid** | A unique identifier for an alert. | [required] |

### Return type

[**models::UptimeCreateAlert201Response**](uptime_create_alert_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## uptime_get_check

> models::UptimeCreateCheck201Response uptime_get_check(check_id)
Retrieve an Existing Check

To show information about an existing check, send a GET request to `/v2/uptime/checks/$CHECK_ID`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**check_id** | **uuid::Uuid** | A unique identifier for a check. | [required] |

### Return type

[**models::UptimeCreateCheck201Response**](uptime_create_check_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## uptime_get_check_state

> models::UptimeGetCheckState200Response uptime_get_check_state(check_id)
Retrieve Check State

To show information about an existing check's state, send a GET request to `/v2/uptime/checks/$CHECK_ID/state`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**check_id** | **uuid::Uuid** | A unique identifier for a check. | [required] |

### Return type

[**models::UptimeGetCheckState200Response**](uptime_get_checkState_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## uptime_list_alerts

> models::UptimeListAlerts200Response uptime_list_alerts(check_id, per_page, page)
List All Alerts

To list all of the alerts for an Uptime check, send a GET request to `/v2/uptime/checks/$CHECK_ID/alerts`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**check_id** | **uuid::Uuid** | A unique identifier for a check. | [required] |
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**models::UptimeListAlerts200Response**](uptime_list_alerts_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## uptime_list_checks

> models::UptimeListChecks200Response uptime_list_checks(per_page, page)
List All Checks

To list all of the Uptime checks on your account, send a GET request to `/v2/uptime/checks`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**models::UptimeListChecks200Response**](uptime_list_checks_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## uptime_update_alert

> models::UptimeCreateAlert201Response uptime_update_alert(check_id, alert_id, alert_updatable)
Update an Alert

To update the settings of an Uptime alert, send a PUT request to `/v2/uptime/checks/$CHECK_ID/alerts/$ALERT_ID`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**check_id** | **uuid::Uuid** | A unique identifier for a check. | [required] |
**alert_id** | **uuid::Uuid** | A unique identifier for an alert. | [required] |
**alert_updatable** | [**AlertUpdatable**](AlertUpdatable.md) |  | [required] |

### Return type

[**models::UptimeCreateAlert201Response**](uptime_create_alert_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## uptime_update_check

> models::UptimeCreateCheck201Response uptime_update_check(check_id, check_updatable)
Update a Check

To update the settings of an Uptime check, send a PUT request to `/v2/uptime/checks/$CHECK_ID`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**check_id** | **uuid::Uuid** | A unique identifier for a check. | [required] |
**check_updatable** | [**CheckUpdatable**](CheckUpdatable.md) |  | [required] |

### Return type

[**models::UptimeCreateCheck201Response**](uptime_create_check_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

