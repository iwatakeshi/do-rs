# VolumeAction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | A unique numeric ID that can be used to identify and reference an action. | [optional]
**status** | Option<**String**> | The current status of the action. This can be \"in-progress\", \"completed\", or \"errored\". | [optional][default to InProgress]
**r#type** | Option<**String**> | This is the type of action that the object represents. For example, this could be \"attach_volume\" to represent the state of a volume attach action. | [optional]
**started_at** | Option<**String**> | A time value given in ISO8601 combined date and time format that represents when the action was initiated. | [optional]
**completed_at** | Option<**String**> | A time value given in ISO8601 combined date and time format that represents when the action was completed. | [optional]
**resource_id** | Option<**i32**> |  | [optional]
**resource_type** | Option<**String**> | The type of resource that the action is associated with. | [optional]
**region** | Option<[**models::Region**](region.md)> |  | [optional]
**region_slug** | Option<**String**> | A human-readable string that is used as a unique identifier for each region. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


