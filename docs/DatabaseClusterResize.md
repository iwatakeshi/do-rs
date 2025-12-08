# DatabaseClusterResize

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**size** | **String** | A slug identifier representing desired the size of the nodes in the database cluster. | 
**num_nodes** | **i32** | The number of nodes in the database cluster. Valid values are are 1-3. In addition to the primary node, up to two standby nodes may be added for highly available configurations. | 
**storage_size_mib** | Option<**i32**> | Additional storage added to the cluster, in MiB. If null, no additional storage is added to the cluster, beyond what is provided as a base amount from the 'size' and any previously added additional storage. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


