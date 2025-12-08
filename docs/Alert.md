# Alert

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | A unique ID that can be used to identify and reference the alert. | [optional][readonly]
**name** | Option<**String**> | A human-friendly display name. | [optional]
**r#type** | Option<**String**> | The type of alert. | [optional]
**threshold** | Option<**i32**> | The threshold at which the alert will enter a trigger state. The specific threshold is dependent on the alert type. | [optional]
**comparison** | Option<**String**> | The comparison operator used against the alert's threshold. | [optional]
**notifications** | Option<[**models::Notification**](notification.md)> |  | [optional]
**period** | Option<**String**> | Period of time the threshold must be exceeded to trigger the alert. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


