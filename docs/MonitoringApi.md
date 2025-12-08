# \MonitoringApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**monitoring_create_alert_policy**](MonitoringApi.md#monitoring_create_alert_policy) | **POST** /v2/monitoring/alerts | Create Alert Policy
[**monitoring_create_destination**](MonitoringApi.md#monitoring_create_destination) | **POST** /v2/monitoring/sinks/destinations | Create Logging Destination
[**monitoring_create_sink**](MonitoringApi.md#monitoring_create_sink) | **POST** /v2/monitoring/sinks | Create Sink
[**monitoring_delete_alert_policy**](MonitoringApi.md#monitoring_delete_alert_policy) | **DELETE** /v2/monitoring/alerts/{alert_uuid} | Delete an Alert Policy
[**monitoring_delete_destination**](MonitoringApi.md#monitoring_delete_destination) | **DELETE** /v2/monitoring/sinks/destinations/{destination_uuid} | Delete Logging Destination
[**monitoring_delete_sink**](MonitoringApi.md#monitoring_delete_sink) | **DELETE** /v2/monitoring/sinks/{sink_uuid} | Delete Sink
[**monitoring_get_alert_policy**](MonitoringApi.md#monitoring_get_alert_policy) | **GET** /v2/monitoring/alerts/{alert_uuid} | Retrieve an Existing Alert Policy
[**monitoring_get_app_cpu_percentage_metrics**](MonitoringApi.md#monitoring_get_app_cpu_percentage_metrics) | **GET** /v2/monitoring/metrics/apps/cpu_percentage | Get App CPU Percentage Metrics
[**monitoring_get_app_memory_percentage_metrics**](MonitoringApi.md#monitoring_get_app_memory_percentage_metrics) | **GET** /v2/monitoring/metrics/apps/memory_percentage | Get App Memory Percentage Metrics
[**monitoring_get_app_restart_count_metrics_yml**](MonitoringApi.md#monitoring_get_app_restart_count_metrics_yml) | **GET** /v2/monitoring/metrics/apps/restart_count | Get App Restart Count Metrics
[**monitoring_get_destination**](MonitoringApi.md#monitoring_get_destination) | **GET** /v2/monitoring/sinks/destinations/{destination_uuid} | Get Logging Destination
[**monitoring_get_droplet_autoscale_current_cpu_utilization_yml**](MonitoringApi.md#monitoring_get_droplet_autoscale_current_cpu_utilization_yml) | **GET** /v2/monitoring/metrics/droplet_autoscale/current_cpu_utilization | Get Droplet Autoscale Pool Current Average CPU utilization
[**monitoring_get_droplet_autoscale_current_instances**](MonitoringApi.md#monitoring_get_droplet_autoscale_current_instances) | **GET** /v2/monitoring/metrics/droplet_autoscale/current_instances | Get Droplet Autoscale Pool Current Size
[**monitoring_get_droplet_autoscale_current_memory_utilization**](MonitoringApi.md#monitoring_get_droplet_autoscale_current_memory_utilization) | **GET** /v2/monitoring/metrics/droplet_autoscale/current_memory_utilization | Get Droplet Autoscale Pool Current Average Memory utilization
[**monitoring_get_droplet_autoscale_target_cpu_utilization**](MonitoringApi.md#monitoring_get_droplet_autoscale_target_cpu_utilization) | **GET** /v2/monitoring/metrics/droplet_autoscale/target_cpu_utilization | Get Droplet Autoscale Pool Target Average CPU utilization
[**monitoring_get_droplet_autoscale_target_instances**](MonitoringApi.md#monitoring_get_droplet_autoscale_target_instances) | **GET** /v2/monitoring/metrics/droplet_autoscale/target_instances | Get Droplet Autoscale Pool Target Size
[**monitoring_get_droplet_autoscale_target_memory_utilization**](MonitoringApi.md#monitoring_get_droplet_autoscale_target_memory_utilization) | **GET** /v2/monitoring/metrics/droplet_autoscale/target_memory_utilization | Get Droplet Autoscale Pool Target Average Memory utilization
[**monitoring_get_droplet_bandwidth_metrics**](MonitoringApi.md#monitoring_get_droplet_bandwidth_metrics) | **GET** /v2/monitoring/metrics/droplet/bandwidth | Get Droplet Bandwidth Metrics
[**monitoring_get_droplet_cpu_metrics**](MonitoringApi.md#monitoring_get_droplet_cpu_metrics) | **GET** /v2/monitoring/metrics/droplet/cpu | Get Droplet CPU Metrics
[**monitoring_get_droplet_filesystem_free_metrics**](MonitoringApi.md#monitoring_get_droplet_filesystem_free_metrics) | **GET** /v2/monitoring/metrics/droplet/filesystem_free | Get Droplet Filesystem Free Metrics
[**monitoring_get_droplet_filesystem_size_metrics**](MonitoringApi.md#monitoring_get_droplet_filesystem_size_metrics) | **GET** /v2/monitoring/metrics/droplet/filesystem_size | Get Droplet Filesystem Size Metrics
[**monitoring_get_droplet_load15_metrics**](MonitoringApi.md#monitoring_get_droplet_load15_metrics) | **GET** /v2/monitoring/metrics/droplet/load_15 | Get Droplet Load15 Metrics
[**monitoring_get_droplet_load1_metrics**](MonitoringApi.md#monitoring_get_droplet_load1_metrics) | **GET** /v2/monitoring/metrics/droplet/load_1 | Get Droplet Load1 Metrics
[**monitoring_get_droplet_load5_metrics**](MonitoringApi.md#monitoring_get_droplet_load5_metrics) | **GET** /v2/monitoring/metrics/droplet/load_5 | Get Droplet Load5 Metrics
[**monitoring_get_droplet_memory_available_metrics**](MonitoringApi.md#monitoring_get_droplet_memory_available_metrics) | **GET** /v2/monitoring/metrics/droplet/memory_available | Get Droplet Available Memory Metrics
[**monitoring_get_droplet_memory_cached_metrics**](MonitoringApi.md#monitoring_get_droplet_memory_cached_metrics) | **GET** /v2/monitoring/metrics/droplet/memory_cached | Get Droplet Cached Memory Metrics
[**monitoring_get_droplet_memory_free_metrics**](MonitoringApi.md#monitoring_get_droplet_memory_free_metrics) | **GET** /v2/monitoring/metrics/droplet/memory_free | Get Droplet Free Memory Metrics
[**monitoring_get_droplet_memory_total_metrics**](MonitoringApi.md#monitoring_get_droplet_memory_total_metrics) | **GET** /v2/monitoring/metrics/droplet/memory_total | Get Droplet Total Memory Metrics
[**monitoring_get_lb_droplets_connections**](MonitoringApi.md#monitoring_get_lb_droplets_connections) | **GET** /v2/monitoring/metrics/load_balancer/droplets_connections | Get Load Balancer Droplets Active Connections Metrics
[**monitoring_get_lb_droplets_downtime**](MonitoringApi.md#monitoring_get_lb_droplets_downtime) | **GET** /v2/monitoring/metrics/load_balancer/droplets_downtime | Get Load Balancer Droplets Downtime Status Metrics
[**monitoring_get_lb_droplets_health_checks**](MonitoringApi.md#monitoring_get_lb_droplets_health_checks) | **GET** /v2/monitoring/metrics/load_balancer/droplets_health_checks | Get Load Balancer Droplets Health Check Status Metrics
[**monitoring_get_lb_droplets_http_response_time50p**](MonitoringApi.md#monitoring_get_lb_droplets_http_response_time50p) | **GET** /v2/monitoring/metrics/load_balancer/droplets_http_response_time_50p | Get Load Balancer Droplets 50th Percentile HTTP Response Time Metrics
[**monitoring_get_lb_droplets_http_response_time95p**](MonitoringApi.md#monitoring_get_lb_droplets_http_response_time95p) | **GET** /v2/monitoring/metrics/load_balancer/droplets_http_response_time_95p | Get Load Balancer Droplets 95th Percentile HTTP Response Time Metrics
[**monitoring_get_lb_droplets_http_response_time99p**](MonitoringApi.md#monitoring_get_lb_droplets_http_response_time99p) | **GET** /v2/monitoring/metrics/load_balancer/droplets_http_response_time_99p | Get Load Balancer Droplets 99th Percentile HTTP Response Time Metrics
[**monitoring_get_lb_droplets_http_response_time_avg**](MonitoringApi.md#monitoring_get_lb_droplets_http_response_time_avg) | **GET** /v2/monitoring/metrics/load_balancer/droplets_http_response_time_avg | Get Load Balancer Droplets Average HTTP Response Time Metrics
[**monitoring_get_lb_droplets_http_responses**](MonitoringApi.md#monitoring_get_lb_droplets_http_responses) | **GET** /v2/monitoring/metrics/load_balancer/droplets_http_responses | Get Load Balancer Droplets HTTP Rate Of Response Code Metrics
[**monitoring_get_lb_droplets_http_session_duration50p**](MonitoringApi.md#monitoring_get_lb_droplets_http_session_duration50p) | **GET** /v2/monitoring/metrics/load_balancer/droplets_http_session_duration_50p | Get Load Balancer Droplets 50th Percentile HTTP Session Duration Metrics
[**monitoring_get_lb_droplets_http_session_duration95p**](MonitoringApi.md#monitoring_get_lb_droplets_http_session_duration95p) | **GET** /v2/monitoring/metrics/load_balancer/droplets_http_session_duration_95p | Get Load Balancer Droplets 95th Percentile HTTP Session Duration Metrics
[**monitoring_get_lb_droplets_http_session_duration_avg**](MonitoringApi.md#monitoring_get_lb_droplets_http_session_duration_avg) | **GET** /v2/monitoring/metrics/load_balancer/droplets_http_session_duration_avg | Get Load Balancer Droplets Average HTTP Session Duration Metrics
[**monitoring_get_lb_droplets_queue_size**](MonitoringApi.md#monitoring_get_lb_droplets_queue_size) | **GET** /v2/monitoring/metrics/load_balancer/droplets_queue_size | Get Load Balancer Droplets Queue Size Metrics
[**monitoring_get_lb_frontend_connections_current**](MonitoringApi.md#monitoring_get_lb_frontend_connections_current) | **GET** /v2/monitoring/metrics/load_balancer/frontend_connections_current | Get Load Balancer Frontend Total Current Active Connections Metrics
[**monitoring_get_lb_frontend_connections_limit**](MonitoringApi.md#monitoring_get_lb_frontend_connections_limit) | **GET** /v2/monitoring/metrics/load_balancer/frontend_connections_limit | Get Load Balancer Frontend Max Connections Limit Metrics
[**monitoring_get_lb_frontend_cpu_utilization**](MonitoringApi.md#monitoring_get_lb_frontend_cpu_utilization) | **GET** /v2/monitoring/metrics/load_balancer/frontend_cpu_utilization | Get Load Balancer Frontend Average Percentage CPU Utilization Metrics
[**monitoring_get_lb_frontend_firewall_dropped_bytes**](MonitoringApi.md#monitoring_get_lb_frontend_firewall_dropped_bytes) | **GET** /v2/monitoring/metrics/load_balancer/frontend_firewall_dropped_bytes | Get Load Balancer Frontend Firewall Dropped Bytes Metrics
[**monitoring_get_lb_frontend_firewall_dropped_packets**](MonitoringApi.md#monitoring_get_lb_frontend_firewall_dropped_packets) | **GET** /v2/monitoring/metrics/load_balancer/frontend_firewall_dropped_packets | Get Load Balancer Frontend Firewall Dropped Packets Metrics
[**monitoring_get_lb_frontend_http_requests_per_second**](MonitoringApi.md#monitoring_get_lb_frontend_http_requests_per_second) | **GET** /v2/monitoring/metrics/load_balancer/frontend_http_requests_per_second | Get Load Balancer Frontend HTTP Requests Metrics
[**monitoring_get_lb_frontend_http_responses**](MonitoringApi.md#monitoring_get_lb_frontend_http_responses) | **GET** /v2/monitoring/metrics/load_balancer/frontend_http_responses | Get Load Balancer Frontend HTTP Rate Of Response Code Metrics
[**monitoring_get_lb_frontend_network_throughput_http**](MonitoringApi.md#monitoring_get_lb_frontend_network_throughput_http) | **GET** /v2/monitoring/metrics/load_balancer/frontend_network_throughput_http | Get Load Balancer Frontend HTTP Throughput Metrics
[**monitoring_get_lb_frontend_network_throughput_tcp**](MonitoringApi.md#monitoring_get_lb_frontend_network_throughput_tcp) | **GET** /v2/monitoring/metrics/load_balancer/frontend_network_throughput_tcp | Get Load Balancer Frontend TCP Throughput Metrics
[**monitoring_get_lb_frontend_network_throughput_udp**](MonitoringApi.md#monitoring_get_lb_frontend_network_throughput_udp) | **GET** /v2/monitoring/metrics/load_balancer/frontend_network_throughput_udp | Get Load Balancer Frontend UDP Throughput Metrics
[**monitoring_get_lb_frontend_nlb_tcp_network_throughput**](MonitoringApi.md#monitoring_get_lb_frontend_nlb_tcp_network_throughput) | **GET** /v2/monitoring/metrics/load_balancer/frontend_nlb_tcp_network_throughput | Get Network Load Balancer Frontend TCP Throughput Metrics
[**monitoring_get_lb_frontend_nlb_udp_network_throughput**](MonitoringApi.md#monitoring_get_lb_frontend_nlb_udp_network_throughput) | **GET** /v2/monitoring/metrics/load_balancer/frontend_nlb_udp_network_throughput | Get Network Load Balancer Frontend UDP Throughput Metrics
[**monitoring_get_lb_frontend_tls_connections_current**](MonitoringApi.md#monitoring_get_lb_frontend_tls_connections_current) | **GET** /v2/monitoring/metrics/load_balancer/frontend_tls_connections_current | Get Load Balancer Frontend Current TLS Connections Rate Metrics
[**monitoring_get_lb_frontend_tls_connections_exceeding_rate_limit**](MonitoringApi.md#monitoring_get_lb_frontend_tls_connections_exceeding_rate_limit) | **GET** /v2/monitoring/metrics/load_balancer/frontend_tls_connections_exceeding_rate_limit | Get Load Balancer Frontend Closed TLS Connections For Exceeded Rate Limit Metrics
[**monitoring_get_lb_frontend_tls_connections_limit**](MonitoringApi.md#monitoring_get_lb_frontend_tls_connections_limit) | **GET** /v2/monitoring/metrics/load_balancer/frontend_tls_connections_limit | Get Load Balancer Frontend Max TLS Connections Limit Metrics
[**monitoring_get_sink**](MonitoringApi.md#monitoring_get_sink) | **GET** /v2/monitoring/sinks/{sink_uuid} | Get Sink
[**monitoring_list_alert_policy**](MonitoringApi.md#monitoring_list_alert_policy) | **GET** /v2/monitoring/alerts | List Alert Policies
[**monitoring_list_destinations**](MonitoringApi.md#monitoring_list_destinations) | **GET** /v2/monitoring/sinks/destinations | List Logging Destinations
[**monitoring_list_sinks**](MonitoringApi.md#monitoring_list_sinks) | **GET** /v2/monitoring/sinks | Lists all sinks
[**monitoring_update_alert_policy**](MonitoringApi.md#monitoring_update_alert_policy) | **PUT** /v2/monitoring/alerts/{alert_uuid} | Update an Alert Policy
[**monitoring_update_destination**](MonitoringApi.md#monitoring_update_destination) | **POST** /v2/monitoring/sinks/destinations/{destination_uuid} | Update Logging Destination



## monitoring_create_alert_policy

> models::MonitoringCreateAlertPolicy200Response monitoring_create_alert_policy(alert_policy_request)
Create Alert Policy

To create a new alert, send a POST request to `/v2/monitoring/alerts`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alert_policy_request** | [**AlertPolicyRequest**](AlertPolicyRequest.md) | The `type` field dictates what type of entity that the alert policy applies to and hence what type of entity is passed in the `entities` array. If both the `tags` array and `entities` array are empty the alert policy applies to all entities of the relevant type that are owned by the user account. Otherwise the following table shows the valid entity types for each type of alert policy:  Type | Description | Valid Entity Type -----|-------------|-------------------- `v1/insights/droplet/memory_utilization_percent` | alert on the percent of memory utilization | Droplet ID `v1/insights/droplet/disk_read` | alert on the rate of disk read I/O in MBps | Droplet ID `v1/insights/droplet/load_5` | alert on the 5 minute load average | Droplet ID `v1/insights/droplet/load_15` | alert on the 15 minute load average | Droplet ID `v1/insights/droplet/disk_utilization_percent` | alert on the percent of disk utilization | Droplet ID `v1/insights/droplet/cpu` | alert on the percent of CPU utilization | Droplet ID `v1/insights/droplet/disk_write` | alert on the rate of disk write I/O in MBps | Droplet ID `v1/insights/droplet/public_outbound_bandwidth` | alert on the rate of public outbound bandwidth in Mbps | Droplet ID `v1/insights/droplet/public_inbound_bandwidth` | alert on the rate of public inbound bandwidth in Mbps | Droplet ID `v1/insights/droplet/private_outbound_bandwidth` | alert on the rate of private outbound bandwidth in Mbps | Droplet ID `v1/insights/droplet/private_inbound_bandwidth` | alert on the rate of private inbound bandwidth in Mbps | Droplet ID `v1/insights/droplet/load_1` | alert on the 1 minute load average | Droplet ID `v1/insights/lbaas/avg_cpu_utilization_percent`|alert on the percent of CPU utilization|load balancer ID `v1/insights/lbaas/connection_utilization_percent`|alert on the percent of connection utilization|load balancer ID `v1/insights/lbaas/droplet_health`|alert on Droplet health status changes|load balancer ID `v1/insights/lbaas/tls_connections_per_second_utilization_percent`|alert on the percent of TLS connections per second utilization (requires at least one HTTPS forwarding rule)|load balancer ID `v1/insights/lbaas/increase_in_http_error_rate_percentage_5xx`|alert on the percent increase of 5xx level http errors over 5m|load balancer ID `v1/insights/lbaas/increase_in_http_error_rate_percentage_4xx`|alert on the percent increase of 4xx level http errors over 5m|load balancer ID `v1/insights/lbaas/increase_in_http_error_rate_count_5xx`|alert on the count of 5xx level http errors over 5m|load balancer ID `v1/insights/lbaas/increase_in_http_error_rate_count_4xx`|alert on the count of 4xx level http errors over 5m|load balancer ID `v1/insights/lbaas/high_http_request_response_time`|alert on high average http response time|load balancer ID `v1/insights/lbaas/high_http_request_response_time_50p`|alert on high 50th percentile http response time|load balancer ID `v1/insights/lbaas/high_http_request_response_time_95p`|alert on high 95th percentile http response time|load balancer ID `v1/insights/lbaas/high_http_request_response_time_99p`|alert on high 99th percentile http response time|load balancer ID `v1/dbaas/alerts/load_15_alerts` | alert on 15 minute load average across the database cluster | database cluster UUID `v1/dbaas/alerts/memory_utilization_alerts` | alert on the percent memory utilization average across the database cluster | database cluster UUID `v1/dbaas/alerts/disk_utilization_alerts` | alert on the percent disk utilization average across the database cluster | database cluster UUID `v1/dbaas/alerts/cpu_alerts` | alert on the percent CPU usage average across the database cluster | database cluster UUID `v1/droplet/autoscale_alerts/current_instances` | alert on current pool size | autoscale pool ID `v1/droplet/autoscale_alerts/target_instances` | alert on target pool size | autoscale pool ID `v1/droplet/autoscale_alerts/current_cpu_utilization` | alert on current average CPU utilization | autoscale pool ID `v1/droplet/autoscale_alerts/target_cpu_utilization` | alert on target average CPU utilization | autoscale pool ID `v1/droplet/autoscale_alerts/current_memory_utilization` | alert on current average memory utilization | autoscale pool ID `v1/droplet/autoscale_alerts/target_memory_utilization` | alert on target average memory utilization | autoscale pool ID `v1/droplet/autoscale_alerts/scale_up` | alert on scale up event | autoscale pool ID `v1/droplet/autoscale_alerts/scale_down` | alert on scale down event | autoscale pool ID  | [required] |

### Return type

[**models::MonitoringCreateAlertPolicy200Response**](monitoring_create_alertPolicy_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_create_destination

> models::MonitoringCreateDestination200Response monitoring_create_destination(destination_request)
Create Logging Destination

To create a new destination, send a POST request to `/v2/monitoring/sinks/destinations`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**destination_request** | [**DestinationRequest**](DestinationRequest.md) |  | [required] |

### Return type

[**models::MonitoringCreateDestination200Response**](monitoring_create_destination_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_create_sink

> monitoring_create_sink(monitoring_create_sink_request)
Create Sink

To create a new sink, send a POST request to `/v2/monitoring/sinks`. Forwards logs from the  resources identified in `resources` to the specified pre-existing destination. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**monitoring_create_sink_request** | [**MonitoringCreateSinkRequest**](MonitoringCreateSinkRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_delete_alert_policy

> monitoring_delete_alert_policy(alert_uuid)
Delete an Alert Policy

To delete an alert policy, send a DELETE request to `/v2/monitoring/alerts/{alert_uuid}`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alert_uuid** | **String** | A unique identifier for an alert policy. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_delete_destination

> monitoring_delete_destination(destination_uuid)
Delete Logging Destination

To delete a destination and all associated sinks, send a DELETE request to `/v2/monitoring/sinks/destinations/${destination_uuid}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**destination_uuid** | **String** | A unique identifier for a destination. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_delete_sink

> monitoring_delete_sink(sink_uuid)
Delete Sink

To delete a sink, send a DELETE request to `/v2/monitoring/sinks/${sink_uuid}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sink_uuid** | **String** | A unique identifier for a sink. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_alert_policy

> models::MonitoringCreateAlertPolicy200Response monitoring_get_alert_policy(alert_uuid)
Retrieve an Existing Alert Policy

To retrieve a given alert policy, send a GET request to `/v2/monitoring/alerts/{alert_uuid}`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alert_uuid** | **String** | A unique identifier for an alert policy. | [required] |

### Return type

[**models::MonitoringCreateAlertPolicy200Response**](monitoring_create_alertPolicy_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_app_cpu_percentage_metrics

> models::Metrics monitoring_get_app_cpu_percentage_metrics(app_id, start, end, app_component)
Get App CPU Percentage Metrics

To retrieve cpu percentage metrics for a given app, send a GET request to `/v2/monitoring/metrics/apps/cpu_percentage`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app UUID. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |
**app_component** | Option<**String**> | The app component name. |  |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_app_memory_percentage_metrics

> models::Metrics monitoring_get_app_memory_percentage_metrics(app_id, start, end, app_component)
Get App Memory Percentage Metrics

To retrieve memory percentage metrics for a given app, send a GET request to `/v2/monitoring/metrics/apps/memory_percentage`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app UUID. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |
**app_component** | Option<**String**> | The app component name. |  |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_app_restart_count_metrics_yml

> models::Metrics monitoring_get_app_restart_count_metrics_yml(app_id, start, end, app_component)
Get App Restart Count Metrics

To retrieve restart count metrics for a given app, send a GET request to `/v2/monitoring/metrics/apps/restart_count`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** | The app UUID. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |
**app_component** | Option<**String**> | The app component name. |  |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_destination

> models::MonitoringCreateDestination200Response monitoring_get_destination(destination_uuid)
Get Logging Destination

To get the details of a destination, send a GET request to `/v2/monitoring/sinks/destinations/${destination_uuid}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**destination_uuid** | **String** | A unique identifier for a destination. | [required] |

### Return type

[**models::MonitoringCreateDestination200Response**](monitoring_create_destination_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_droplet_autoscale_current_cpu_utilization_yml

> models::Metrics monitoring_get_droplet_autoscale_current_cpu_utilization_yml(autoscale_pool_id, start, end)
Get Droplet Autoscale Pool Current Average CPU utilization

To retrieve the current average CPU utilization for a given Droplet Autoscale Pool, send a GET request to `/v2/monitoring/metrics/droplet_autoscale/current_cpu_utilization`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**autoscale_pool_id** | **String** | A unique identifier for an autoscale pool. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_droplet_autoscale_current_instances

> models::Metrics monitoring_get_droplet_autoscale_current_instances(autoscale_pool_id, start, end)
Get Droplet Autoscale Pool Current Size

To retrieve the current size for a given Droplet Autoscale Pool, send a GET request to `/v2/monitoring/metrics/droplet_autoscale/current_instances`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**autoscale_pool_id** | **String** | A unique identifier for an autoscale pool. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_droplet_autoscale_current_memory_utilization

> models::Metrics monitoring_get_droplet_autoscale_current_memory_utilization(autoscale_pool_id, start, end)
Get Droplet Autoscale Pool Current Average Memory utilization

To retrieve the current average memory utilization for a given Droplet Autoscale Pool, send a GET request to `/v2/monitoring/metrics/droplet_autoscale/current_memory_utilization`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**autoscale_pool_id** | **String** | A unique identifier for an autoscale pool. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_droplet_autoscale_target_cpu_utilization

> models::Metrics monitoring_get_droplet_autoscale_target_cpu_utilization(autoscale_pool_id, start, end)
Get Droplet Autoscale Pool Target Average CPU utilization

To retrieve the target average CPU utilization for a given Droplet Autoscale Pool, send a GET request to `/v2/monitoring/metrics/droplet_autoscale/target_cpu_utilization`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**autoscale_pool_id** | **String** | A unique identifier for an autoscale pool. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_droplet_autoscale_target_instances

> models::Metrics monitoring_get_droplet_autoscale_target_instances(autoscale_pool_id, start, end)
Get Droplet Autoscale Pool Target Size

To retrieve the target size for a given Droplet Autoscale Pool, send a GET request to `/v2/monitoring/metrics/droplet_autoscale/target_instances`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**autoscale_pool_id** | **String** | A unique identifier for an autoscale pool. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_droplet_autoscale_target_memory_utilization

> models::Metrics monitoring_get_droplet_autoscale_target_memory_utilization(autoscale_pool_id, start, end)
Get Droplet Autoscale Pool Target Average Memory utilization

To retrieve the target average memory utilization for a given Droplet Autoscale Pool, send a GET request to `/v2/monitoring/metrics/droplet_autoscale/target_memory_utilization`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**autoscale_pool_id** | **String** | A unique identifier for an autoscale pool. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_droplet_bandwidth_metrics

> models::Metrics monitoring_get_droplet_bandwidth_metrics(host_id, interface, direction, start, end)
Get Droplet Bandwidth Metrics

To retrieve bandwidth metrics for a given Droplet, send a GET request to `/v2/monitoring/metrics/droplet/bandwidth`. Use the `interface` query parameter to specify if the results should be for the `private` or `public` interface. Use the `direction` query parameter to specify if the results should be for `inbound` or `outbound` traffic. The metrics in the response body are in megabits per second (Mbps).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**host_id** | **String** | The droplet ID. | [required] |
**interface** | **String** | The network interface. | [required] |
**direction** | **String** | The traffic direction. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_droplet_cpu_metrics

> models::Metrics monitoring_get_droplet_cpu_metrics(host_id, start, end)
Get Droplet CPU Metrics

To retrieve CPU metrics for a given droplet, send a GET request to `/v2/monitoring/metrics/droplet/cpu`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**host_id** | **String** | The droplet ID. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_droplet_filesystem_free_metrics

> models::Metrics monitoring_get_droplet_filesystem_free_metrics(host_id, start, end)
Get Droplet Filesystem Free Metrics

To retrieve filesystem free metrics for a given droplet, send a GET request to `/v2/monitoring/metrics/droplet/filesystem_free`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**host_id** | **String** | The droplet ID. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_droplet_filesystem_size_metrics

> models::Metrics monitoring_get_droplet_filesystem_size_metrics(host_id, start, end)
Get Droplet Filesystem Size Metrics

To retrieve filesystem size metrics for a given droplet, send a GET request to `/v2/monitoring/metrics/droplet/filesystem_size`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**host_id** | **String** | The droplet ID. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_droplet_load15_metrics

> models::Metrics monitoring_get_droplet_load15_metrics(host_id, start, end)
Get Droplet Load15 Metrics

To retrieve 15 minute load average metrics for a given droplet, send a GET request to `/v2/monitoring/metrics/droplet/load_15`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**host_id** | **String** | The droplet ID. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_droplet_load1_metrics

> models::Metrics monitoring_get_droplet_load1_metrics(host_id, start, end)
Get Droplet Load1 Metrics

To retrieve 1 minute load average metrics for a given droplet, send a GET request to `/v2/monitoring/metrics/droplet/load_1`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**host_id** | **String** | The droplet ID. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_droplet_load5_metrics

> models::Metrics monitoring_get_droplet_load5_metrics(host_id, start, end)
Get Droplet Load5 Metrics

To retrieve 5 minute load average metrics for a given droplet, send a GET request to `/v2/monitoring/metrics/droplet/load_5`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**host_id** | **String** | The droplet ID. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_droplet_memory_available_metrics

> models::Metrics monitoring_get_droplet_memory_available_metrics(host_id, start, end)
Get Droplet Available Memory Metrics

To retrieve available memory metrics for a given droplet, send a GET request to `/v2/monitoring/metrics/droplet/memory_available`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**host_id** | **String** | The droplet ID. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_droplet_memory_cached_metrics

> models::Metrics monitoring_get_droplet_memory_cached_metrics(host_id, start, end)
Get Droplet Cached Memory Metrics

To retrieve cached memory metrics for a given droplet, send a GET request to `/v2/monitoring/metrics/droplet/memory_cached`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**host_id** | **String** | The droplet ID. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_droplet_memory_free_metrics

> models::Metrics monitoring_get_droplet_memory_free_metrics(host_id, start, end)
Get Droplet Free Memory Metrics

To retrieve free memory metrics for a given droplet, send a GET request to `/v2/monitoring/metrics/droplet/memory_free`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**host_id** | **String** | The droplet ID. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_droplet_memory_total_metrics

> models::Metrics monitoring_get_droplet_memory_total_metrics(host_id, start, end)
Get Droplet Total Memory Metrics

To retrieve total memory metrics for a given droplet, send a GET request to `/v2/monitoring/metrics/droplet/memory_total`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**host_id** | **String** | The droplet ID. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_lb_droplets_connections

> models::Metrics monitoring_get_lb_droplets_connections(lb_id, start, end)
Get Load Balancer Droplets Active Connections Metrics

To retrieve Droplets active connections for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/droplets_connections`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lb_id** | **String** | A unique identifier for a load balancer. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_lb_droplets_downtime

> models::Metrics monitoring_get_lb_droplets_downtime(lb_id, start, end)
Get Load Balancer Droplets Downtime Status Metrics

To retrieve Droplets downtime status for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/droplets_downtime`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lb_id** | **String** | A unique identifier for a load balancer. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_lb_droplets_health_checks

> models::Metrics monitoring_get_lb_droplets_health_checks(lb_id, start, end)
Get Load Balancer Droplets Health Check Status Metrics

To retrieve Droplets health check status for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/droplets_health_checks`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lb_id** | **String** | A unique identifier for a load balancer. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_lb_droplets_http_response_time50p

> models::Metrics monitoring_get_lb_droplets_http_response_time50p(lb_id, start, end)
Get Load Balancer Droplets 50th Percentile HTTP Response Time Metrics

To retrieve Droplets 50th percentile HTTP response time in seconds for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/droplets_http_response_time_50p`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lb_id** | **String** | A unique identifier for a load balancer. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_lb_droplets_http_response_time95p

> models::Metrics monitoring_get_lb_droplets_http_response_time95p(lb_id, start, end)
Get Load Balancer Droplets 95th Percentile HTTP Response Time Metrics

To retrieve Droplets 95th percentile HTTP response time in seconds for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/droplets_http_response_time_95p`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lb_id** | **String** | A unique identifier for a load balancer. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_lb_droplets_http_response_time99p

> models::Metrics monitoring_get_lb_droplets_http_response_time99p(lb_id, start, end)
Get Load Balancer Droplets 99th Percentile HTTP Response Time Metrics

To retrieve Droplets 99th percentile HTTP response time in seconds for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/droplets_http_response_time_99p`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lb_id** | **String** | A unique identifier for a load balancer. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_lb_droplets_http_response_time_avg

> models::Metrics monitoring_get_lb_droplets_http_response_time_avg(lb_id, start, end)
Get Load Balancer Droplets Average HTTP Response Time Metrics

To retrieve Droplets average HTTP response time in seconds for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/droplets_http_response_time_avg`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lb_id** | **String** | A unique identifier for a load balancer. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_lb_droplets_http_responses

> models::Metrics monitoring_get_lb_droplets_http_responses(lb_id, start, end)
Get Load Balancer Droplets HTTP Rate Of Response Code Metrics

To retrieve Droplets HTTP rate of response code for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/droplets_http_responses`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lb_id** | **String** | A unique identifier for a load balancer. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_lb_droplets_http_session_duration50p

> models::Metrics monitoring_get_lb_droplets_http_session_duration50p(lb_id, start, end)
Get Load Balancer Droplets 50th Percentile HTTP Session Duration Metrics

To retrieve Droplets 50th percentile HTTP session duration in seconds for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/droplets_http_session_duration_50p`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lb_id** | **String** | A unique identifier for a load balancer. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_lb_droplets_http_session_duration95p

> models::Metrics monitoring_get_lb_droplets_http_session_duration95p(lb_id, start, end)
Get Load Balancer Droplets 95th Percentile HTTP Session Duration Metrics

To retrieve Droplets 95th percentile HTTP session duration in seconds for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/droplets_http_session_duration_95p`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lb_id** | **String** | A unique identifier for a load balancer. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_lb_droplets_http_session_duration_avg

> models::Metrics monitoring_get_lb_droplets_http_session_duration_avg(lb_id, start, end)
Get Load Balancer Droplets Average HTTP Session Duration Metrics

To retrieve Droplets average HTTP session duration in seconds for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/droplets_http_session_duration_avg`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lb_id** | **String** | A unique identifier for a load balancer. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_lb_droplets_queue_size

> models::Metrics monitoring_get_lb_droplets_queue_size(lb_id, start, end)
Get Load Balancer Droplets Queue Size Metrics

To retrieve Droplets queue size for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/droplets_queue_size`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lb_id** | **String** | A unique identifier for a load balancer. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_lb_frontend_connections_current

> models::Metrics monitoring_get_lb_frontend_connections_current(lb_id, start, end)
Get Load Balancer Frontend Total Current Active Connections Metrics

To retrieve frontend total current active connections for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/frontend_connections_current`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lb_id** | **String** | A unique identifier for a load balancer. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_lb_frontend_connections_limit

> models::Metrics monitoring_get_lb_frontend_connections_limit(lb_id, start, end)
Get Load Balancer Frontend Max Connections Limit Metrics

To retrieve frontend max connections limit for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/frontend_connections_limit`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lb_id** | **String** | A unique identifier for a load balancer. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_lb_frontend_cpu_utilization

> models::Metrics monitoring_get_lb_frontend_cpu_utilization(lb_id, start, end)
Get Load Balancer Frontend Average Percentage CPU Utilization Metrics

To retrieve frontend average percentage CPU utilization for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/frontend_cpu_utilization`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lb_id** | **String** | A unique identifier for a load balancer. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_lb_frontend_firewall_dropped_bytes

> models::Metrics monitoring_get_lb_frontend_firewall_dropped_bytes(lb_id, start, end)
Get Load Balancer Frontend Firewall Dropped Bytes Metrics

To retrieve firewall dropped bytes for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/frontend_firewall_dropped_bytes`. This is currently only supported for network load balancers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lb_id** | **String** | A unique identifier for a load balancer. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_lb_frontend_firewall_dropped_packets

> models::Metrics monitoring_get_lb_frontend_firewall_dropped_packets(lb_id, start, end)
Get Load Balancer Frontend Firewall Dropped Packets Metrics

To retrieve firewall dropped packets per second for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/frontend_firewall_dropped_packets`. This is currently only supported for network load balancers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lb_id** | **String** | A unique identifier for a load balancer. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_lb_frontend_http_requests_per_second

> models::Metrics monitoring_get_lb_frontend_http_requests_per_second(lb_id, start, end)
Get Load Balancer Frontend HTTP Requests Metrics

To retrieve frontend HTTP requests per second for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/frontend_http_requests_per_second`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lb_id** | **String** | A unique identifier for a load balancer. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_lb_frontend_http_responses

> models::Metrics monitoring_get_lb_frontend_http_responses(lb_id, start, end)
Get Load Balancer Frontend HTTP Rate Of Response Code Metrics

To retrieve frontend HTTP rate of response code for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/frontend_http_responses`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lb_id** | **String** | A unique identifier for a load balancer. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_lb_frontend_network_throughput_http

> models::Metrics monitoring_get_lb_frontend_network_throughput_http(lb_id, start, end)
Get Load Balancer Frontend HTTP Throughput Metrics

To retrieve frontend HTTP throughput in bytes per second for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/frontend_network_throughput_http`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lb_id** | **String** | A unique identifier for a load balancer. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_lb_frontend_network_throughput_tcp

> models::Metrics monitoring_get_lb_frontend_network_throughput_tcp(lb_id, start, end)
Get Load Balancer Frontend TCP Throughput Metrics

To retrieve frontend TCP throughput in bytes per second for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/frontend_network_throughput_tcp`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lb_id** | **String** | A unique identifier for a load balancer. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_lb_frontend_network_throughput_udp

> models::Metrics monitoring_get_lb_frontend_network_throughput_udp(lb_id, start, end)
Get Load Balancer Frontend UDP Throughput Metrics

To retrieve frontend UDP throughput in bytes per second for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/frontend_network_throughput_udp`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lb_id** | **String** | A unique identifier for a load balancer. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_lb_frontend_nlb_tcp_network_throughput

> models::Metrics monitoring_get_lb_frontend_nlb_tcp_network_throughput(lb_id, start, end)
Get Network Load Balancer Frontend TCP Throughput Metrics

To retrieve frontend TCP throughput in bytes per second for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/frontend_nlb_tcp_network_throughput`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lb_id** | **String** | A unique identifier for a load balancer. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_lb_frontend_nlb_udp_network_throughput

> models::Metrics monitoring_get_lb_frontend_nlb_udp_network_throughput(lb_id, start, end)
Get Network Load Balancer Frontend UDP Throughput Metrics

To retrieve frontend UDP throughput in bytes per second for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/frontend_nlb_udp_network_throughput`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lb_id** | **String** | A unique identifier for a load balancer. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_lb_frontend_tls_connections_current

> models::Metrics monitoring_get_lb_frontend_tls_connections_current(lb_id, start, end)
Get Load Balancer Frontend Current TLS Connections Rate Metrics

To retrieve frontend current TLS connections rate for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/frontend_tls_connections_current`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lb_id** | **String** | A unique identifier for a load balancer. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_lb_frontend_tls_connections_exceeding_rate_limit

> models::Metrics monitoring_get_lb_frontend_tls_connections_exceeding_rate_limit(lb_id, start, end)
Get Load Balancer Frontend Closed TLS Connections For Exceeded Rate Limit Metrics

To retrieve frontend closed TLS connections for exceeded rate limit for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/frontend_tls_connections_exceeding_rate_limit`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lb_id** | **String** | A unique identifier for a load balancer. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_lb_frontend_tls_connections_limit

> models::Metrics monitoring_get_lb_frontend_tls_connections_limit(lb_id, start, end)
Get Load Balancer Frontend Max TLS Connections Limit Metrics

To retrieve frontend max TLS connections limit for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/frontend_tls_connections_limit`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lb_id** | **String** | A unique identifier for a load balancer. | [required] |
**start** | **String** | UNIX timestamp to start metric window. | [required] |
**end** | **String** | UNIX timestamp to end metric window. | [required] |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_get_sink

> models::MonitoringGetSink200Response monitoring_get_sink(sink_uuid)
Get Sink

To get the details of a sink (resources and destination), send a GET request to `/v2/monitoring/sinks/${sink_uuid}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sink_uuid** | **String** | A unique identifier for a sink. | [required] |

### Return type

[**models::MonitoringGetSink200Response**](monitoring_get_sink_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_list_alert_policy

> models::MonitoringListAlertPolicy200Response monitoring_list_alert_policy(per_page, page)
List Alert Policies

Returns all alert policies that are configured for the given account. To List all alert policies, send a GET request to `/v2/monitoring/alerts`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**models::MonitoringListAlertPolicy200Response**](monitoring_list_alertPolicy_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_list_destinations

> models::MonitoringListDestinations200Response monitoring_list_destinations()
List Logging Destinations

To list all logging destinations, send a GET request to `/v2/monitoring/sinks/destinations`.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::MonitoringListDestinations200Response**](monitoring_list_destinations_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_list_sinks

> models::MonitoringListSinks200Response monitoring_list_sinks(resource_id)
Lists all sinks

To list all sinks, send a GET request to `/v2/monitoring/sinks`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**resource_id** | Option<**String**> | A unique URN for a resource. |  |

### Return type

[**models::MonitoringListSinks200Response**](monitoring_list_sinks_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_update_alert_policy

> models::MonitoringCreateAlertPolicy200Response monitoring_update_alert_policy(alert_uuid, alert_policy_request)
Update an Alert Policy

To update en existing policy, send a PUT request to `v2/monitoring/alerts/{alert_uuid}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alert_uuid** | **String** | A unique identifier for an alert policy. | [required] |
**alert_policy_request** | [**AlertPolicyRequest**](AlertPolicyRequest.md) | The `type` field dictates what type of entity that the alert policy applies to and hence what type of entity is passed in the `entities` array. If both the `tags` array and `entities` array are empty the alert policy applies to all entities of the relevant type that are owned by the user account. Otherwise the following table shows the valid entity types for each type of alert policy:  Type | Description | Valid Entity Type -----|-------------|-------------------- `v1/insights/droplet/memory_utilization_percent` | alert on the percent of memory utilization | Droplet ID `v1/insights/droplet/disk_read` | alert on the rate of disk read I/O in MBps | Droplet ID `v1/insights/droplet/load_5` | alert on the 5 minute load average | Droplet ID `v1/insights/droplet/load_15` | alert on the 15 minute load average | Droplet ID `v1/insights/droplet/disk_utilization_percent` | alert on the percent of disk utilization | Droplet ID `v1/insights/droplet/cpu` | alert on the percent of CPU utilization | Droplet ID `v1/insights/droplet/disk_write` | alert on the rate of disk write I/O in MBps | Droplet ID `v1/insights/droplet/public_outbound_bandwidth` | alert on the rate of public outbound bandwidth in Mbps | Droplet ID `v1/insights/droplet/public_inbound_bandwidth` | alert on the rate of public inbound bandwidth in Mbps | Droplet ID `v1/insights/droplet/private_outbound_bandwidth` | alert on the rate of private outbound bandwidth in Mbps | Droplet ID `v1/insights/droplet/private_inbound_bandwidth` | alert on the rate of private inbound bandwidth in Mbps | Droplet ID `v1/insights/droplet/load_1` | alert on the 1 minute load average | Droplet ID `v1/insights/lbaas/avg_cpu_utilization_percent`|alert on the percent of CPU utilization|load balancer ID `v1/insights/lbaas/connection_utilization_percent`|alert on the percent of connection utilization|load balancer ID `v1/insights/lbaas/droplet_health`|alert on Droplet health status changes|load balancer ID `v1/insights/lbaas/tls_connections_per_second_utilization_percent`|alert on the percent of TLS connections per second utilization (requires at least one HTTPS forwarding rule)|load balancer ID `v1/insights/lbaas/increase_in_http_error_rate_percentage_5xx`|alert on the percent increase of 5xx level http errors over 5m|load balancer ID `v1/insights/lbaas/increase_in_http_error_rate_percentage_4xx`|alert on the percent increase of 4xx level http errors over 5m|load balancer ID `v1/insights/lbaas/increase_in_http_error_rate_count_5xx`|alert on the count of 5xx level http errors over 5m|load balancer ID `v1/insights/lbaas/increase_in_http_error_rate_count_4xx`|alert on the count of 4xx level http errors over 5m|load balancer ID `v1/insights/lbaas/high_http_request_response_time`|alert on high average http response time|load balancer ID `v1/insights/lbaas/high_http_request_response_time_50p`|alert on high 50th percentile http response time|load balancer ID `v1/insights/lbaas/high_http_request_response_time_95p`|alert on high 95th percentile http response time|load balancer ID `v1/insights/lbaas/high_http_request_response_time_99p`|alert on high 99th percentile http response time|load balancer ID `v1/dbaas/alerts/load_15_alerts` | alert on 15 minute load average across the database cluster | database cluster UUID `v1/dbaas/alerts/memory_utilization_alerts` | alert on the percent memory utilization average across the database cluster | database cluster UUID `v1/dbaas/alerts/disk_utilization_alerts` | alert on the percent disk utilization average across the database cluster | database cluster UUID `v1/dbaas/alerts/cpu_alerts` | alert on the percent CPU usage average across the database cluster | database cluster UUID `v1/droplet/autoscale_alerts/current_instances` | alert on current pool size | autoscale pool ID `v1/droplet/autoscale_alerts/target_instances` | alert on target pool size | autoscale pool ID `v1/droplet/autoscale_alerts/current_cpu_utilization` | alert on current average CPU utilization | autoscale pool ID `v1/droplet/autoscale_alerts/target_cpu_utilization` | alert on target average CPU utilization | autoscale pool ID `v1/droplet/autoscale_alerts/current_memory_utilization` | alert on current average memory utilization | autoscale pool ID `v1/droplet/autoscale_alerts/target_memory_utilization` | alert on target average memory utilization | autoscale pool ID `v1/droplet/autoscale_alerts/scale_up` | alert on scale up event | autoscale pool ID `v1/droplet/autoscale_alerts/scale_down` | alert on scale down event | autoscale pool ID  | [required] |

### Return type

[**models::MonitoringCreateAlertPolicy200Response**](monitoring_create_alertPolicy_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## monitoring_update_destination

> monitoring_update_destination(destination_uuid, destination_request)
Update Logging Destination

To update the details of a destination, send a PATCH request to `/v2/monitoring/sinks/destinations/${destination_uuid}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**destination_uuid** | **String** | A unique identifier for a destination. | [required] |
**destination_request** | [**DestinationRequest**](DestinationRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

