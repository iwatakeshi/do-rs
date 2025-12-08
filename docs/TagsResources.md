# TagsResources

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**count** | Option<**i32**> | The number of tagged objects for this type of resource. | [optional]
**last_tagged_uri** | Option<**String**> | The URI for the last tagged object for this type of resource. | [optional]
**droplets** | Option<[**models::TagsMetadata**](tags_metadata.md)> | Droplets that are tagged with the specified tag.<br>Requires `droplet:read` scope. | [optional]
**imgages** | Option<[**models::TagsMetadata**](tags_metadata.md)> | Images that are tagged with the specified tag.<br>Requires `image:read` scope. | [optional]
**volumes** | Option<[**models::TagsMetadata**](tags_metadata.md)> | Volumes that are tagged with the specified tag.<br>Requires `block_storage:read` scope. | [optional]
**volume_snapshots** | Option<[**models::TagsMetadata**](tags_metadata.md)> | Volume Snapshots that are tagged with the specified tag.<br>Requires `block_storage_snapshot:read` scope. | [optional]
**databases** | Option<[**models::TagsMetadata**](tags_metadata.md)> | Databases that are tagged with the specified tag.<br>Requires `database:read` scope. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


