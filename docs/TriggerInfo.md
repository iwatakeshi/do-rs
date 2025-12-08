# TriggerInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**namespace** | Option<**String**> | A unique string format of UUID with a prefix fn-. | [optional]
**name** | Option<**String**> | The trigger's unique name within the namespace. | [optional]
**function** | Option<**String**> | Name of function(action) that exists in the given namespace. | [optional]
**r#type** | Option<**String**> | String which indicates the type of trigger source like SCHEDULED. | [optional]
**is_enabled** | Option<**bool**> | Indicates weather the trigger is paused or unpaused. | [optional]
**created_at** | Option<**String**> | UTC time string. | [optional]
**updated_at** | Option<**String**> | UTC time string. | [optional]
**scheduled_details** | Option<[**models::ScheduledDetails**](scheduled_details.md)> |  | [optional]
**scheduled_runs** | Option<[**models::TriggerInfoScheduledRuns**](trigger_info_scheduled_runs.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


