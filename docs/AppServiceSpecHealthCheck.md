# AppServiceSpecHealthCheck

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**failure_threshold** | Option<**i32**> | The number of failed health checks before considered unhealthy. | [optional]
**port** | Option<**i64**> | The port on which the health check will be performed. If not set, the health check will be performed on the component's http_port. | [optional]
**http_path** | Option<**String**> | The route path used for the HTTP health check ping. If not set, the HTTP health check will be disabled and a TCP health check used instead. | [optional]
**initial_delay_seconds** | Option<**i32**> | The number of seconds to wait before beginning health checks. | [optional]
**period_seconds** | Option<**i32**> | The number of seconds to wait between health checks. | [optional]
**success_threshold** | Option<**i32**> | The number of successful health checks before considered healthy. | [optional]
**timeout_seconds** | Option<**i32**> | The number of seconds after which the check times out. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


