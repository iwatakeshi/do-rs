# ApiIndexedDataSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**completed_at** | Option<**String**> | Timestamp when data source completed indexing | [optional]
**data_source_uuid** | Option<**String**> | Uuid of the indexed data source | [optional]
**error_details** | Option<**String**> | A detailed error description | [optional]
**error_msg** | Option<**String**> | A string code provinding a hint which part of the system experienced an error | [optional]
**failed_item_count** | Option<**String**> | Total count of files that have failed | [optional]
**indexed_file_count** | Option<**String**> | Total count of files that have been indexed | [optional]
**indexed_item_count** | Option<**String**> | Total count of files that have been indexed | [optional]
**removed_item_count** | Option<**String**> | Total count of files that have been removed | [optional]
**skipped_item_count** | Option<**String**> | Total count of files that have been skipped | [optional]
**started_at** | Option<**String**> | Timestamp when data source started indexing | [optional]
**status** | Option<[**models::ApiIndexedDataSourceStatus**](apiIndexedDataSourceStatus.md)> |  | [optional]
**total_bytes** | Option<**String**> | Total size of files in data source in bytes | [optional]
**total_bytes_indexed** | Option<**String**> | Total size of files in data source in bytes that have been indexed | [optional]
**total_file_count** | Option<**String**> | Total file count in the data source | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


