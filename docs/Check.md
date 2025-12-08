# Check

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | A unique ID that can be used to identify and reference the check. | [optional][readonly]
**name** | Option<**String**> | A human-friendly display name. | [optional]
**r#type** | Option<**String**> | The type of health check to perform. | [optional]
**target** | Option<**String**> | The endpoint to perform healthchecks on. | [optional]
**regions** | Option<**Vec<String>**> | An array containing the selected regions to perform healthchecks from. | [optional]
**enabled** | Option<**bool**> | A boolean value indicating whether the check is enabled/disabled. | [optional][default to true]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


