# AppsDeploymentProgressStep

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**component_name** | Option<**String**> |  | [optional]
**ended_at** | Option<**String**> |  | [optional]
**message_base** | Option<**String**> | The base of a human-readable description of the step intended to be combined with the component name for presentation. For example:  `message_base` = \"Building service\" `component_name` = \"api\" | [optional]
**name** | Option<**String**> |  | [optional]
**reason** | Option<[**models::AppsDeploymentProgressStepReason**](apps_deployment_progress_step_reason.md)> |  | [optional]
**started_at** | Option<**String**> |  | [optional]
**status** | Option<[**models::AppsDeploymentProgressStepStatus**](apps_deployment_progress_step_status.md)> |  | [optional]
**steps** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


