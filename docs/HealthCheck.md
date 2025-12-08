# HealthCheck

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**protocol** | Option<**String**> | The protocol used for health checks sent to the backend Droplets. The possible values are `http`, `https`, or `tcp`. | [optional][default to Http]
**port** | Option<**i32**> | An integer representing the port on the backend Droplets on which the health check will attempt a connection. | [optional][default to 80]
**path** | Option<**String**> | The path on the backend Droplets to which the load balancer instance will send a request. | [optional][default to /]
**check_interval_seconds** | Option<**i32**> | The number of seconds between between two consecutive health checks. | [optional][default to 10]
**response_timeout_seconds** | Option<**i32**> | The number of seconds the load balancer instance will wait for a response until marking a health check as failed. | [optional][default to 5]
**unhealthy_threshold** | Option<**i32**> | The number of times a health check must fail for a backend Droplet to be marked \"unhealthy\" and be removed from the pool. | [optional][default to 5]
**healthy_threshold** | Option<**i32**> | The number of times a health check must pass for a backend Droplet to be marked \"healthy\" and be re-added to the pool. | [optional][default to 3]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


