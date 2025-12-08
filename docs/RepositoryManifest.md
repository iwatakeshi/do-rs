# RepositoryManifest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**registry_name** | Option<**String**> | The name of the container registry. | [optional]
**repository** | Option<**String**> | The name of the repository. | [optional]
**digest** | Option<**String**> | The manifest digest | [optional]
**compressed_size_bytes** | Option<**i32**> | The compressed size of the manifest in bytes. | [optional]
**size_bytes** | Option<**i32**> | The uncompressed size of the manifest in bytes (this size is calculated asynchronously so it may not be immediately available). | [optional]
**updated_at** | Option<**String**> | The time the manifest was last updated. | [optional]
**tags** | Option<**Vec<String>**> | All tags associated with this manifest | [optional]
**blobs** | Option<[**Vec<models::RepositoryBlob>**](repository_blob.md)> | All blobs associated with this manifest | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


