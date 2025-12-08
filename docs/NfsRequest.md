# NfsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The human-readable name of the share. | 
**size_gib** | **i32** | The desired/provisioned size of the share in GiB (Gibibytes). Must be >= 50. | 
**region** | **String** | The DigitalOcean region slug (e.g., nyc2, atl1) where the NFS share resides. | 
**vpc_ids** | **Vec<String>** | List of VPC IDs that should be able to access the share. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


