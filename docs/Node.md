# Node

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | A unique ID that can be used to identify and reference the node. | [optional]
**name** | Option<**String**> | An automatically generated, human-readable name for the node. | [optional]
**status** | Option<[**models::NodeStatus**](node_status.md)> |  | [optional]
**droplet_id** | Option<**String**> | The ID of the Droplet used for the worker node. | [optional]
**created_at** | Option<**String**> | A time value given in ISO8601 combined date and time format that represents when the node was created. | [optional]
**updated_at** | Option<**String**> | A time value given in ISO8601 combined date and time format that represents when the node was last updated. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


