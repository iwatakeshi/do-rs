# NfsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique identifier of the NFS share. | [readonly]
**name** | **String** | The human-readable name of the share. | 
**size_gib** | **i32** | The desired/provisioned size of the share in GiB (Gibibytes). Must be >= 50. | 
**region** | **String** | The DigitalOcean region slug (e.g., nyc2, atl1) where the NFS share resides. | 
**status** | **String** | The current status of the share. | [readonly]
**created_at** | **String** | Timestamp for when the NFS share was created. | [readonly]
**vpc_ids** | Option<**Vec<String>**> | List of VPC IDs that should be able to access the share. | [optional]
**mount_path** | Option<**String**> | Path at which the share will be available, to be mounted at a target of the user's choice within the client | [optional]
**host** | Option<**String**> | The host IP of the NFS server that will be accessible from the associated VPC | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


