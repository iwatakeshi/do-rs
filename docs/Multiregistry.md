# Multiregistry

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | A globally unique name for the container registry. Must be lowercase and be composed only of numbers, letters and `-`, up to a limit of 63 characters. | [optional]
**created_at** | Option<**String**> | A time value given in ISO8601 combined date and time format that represents when the registry was created. | [optional][readonly]
**region** | Option<**String**> | Slug of the region where registry data is stored | [optional]
**storage_usage_bytes** | Option<**i32**> | The amount of storage used in the registry in bytes. | [optional][readonly]
**storage_usage_bytes_updated_at** | Option<**String**> | The time at which the storage usage was updated. Storage usage is calculated asynchronously, and may not immediately reflect pushes to the registry. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


