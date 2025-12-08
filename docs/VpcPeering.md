# VpcPeering

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | A unique ID that can be used to identify and reference the VPC peering. | [optional][readonly]
**created_at** | Option<**String**> | A time value given in ISO8601 combined date and time format. | [optional][readonly]
**status** | Option<**String**> | The current status of the VPC peering. | [optional][readonly]
**vpc_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | An array of the two peered VPCs IDs. | [optional]
**name** | Option<**String**> | The name of the VPC peering. Must be unique within the team and may only contain alphanumeric characters and dashes. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


