# NfsSnapshotResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique identifier of the snapshot. | 
**name** | **String** | The human-readable name of the snapshot. | 
**size_gib** | **i32** | The size of the snapshot in GiB. | 
**region** | **String** | The DigitalOcean region slug where the snapshot is located. | 
**status** | **String** | The current status of the snapshot. | 
**created_at** | **String** | The timestamp when the snapshot was created. | 
**share_id** | [**uuid::Uuid**](uuid::Uuid.md) | The unique identifier of the share from which this snapshot was created. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


