# ApiIndexingJob

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**completed_datasources** | Option<**i64**> | Number of datasources indexed completed | [optional]
**created_at** | Option<**String**> | Creation date / time | [optional]
**data_source_jobs** | Option<[**Vec<models::ApiIndexedDataSource>**](apiIndexedDataSource.md)> | Details on Data Sources included in the Indexing Job | [optional]
**data_source_uuids** | Option<**Vec<String>**> |  | [optional]
**finished_at** | Option<**String**> |  | [optional]
**is_report_available** | Option<**bool**> | Boolean value to determine if the indexing job details are available | [optional]
**knowledge_base_uuid** | Option<**String**> | Knowledge base id | [optional]
**phase** | Option<[**models::ApiBatchJobPhase**](apiBatchJobPhase.md)> |  | [optional]
**started_at** | Option<**String**> |  | [optional]
**status** | Option<[**models::ApiIndexJobStatus**](apiIndexJobStatus.md)> |  | [optional]
**tokens** | Option<**i64**> | Number of tokens [This field is deprecated] | [optional]
**total_datasources** | Option<**i64**> | Number of datasources being indexed | [optional]
**total_tokens** | Option<**String**> | Total Tokens Consumed By the Indexing Job | [optional]
**updated_at** | Option<**String**> | Last modified | [optional]
**uuid** | Option<**String**> | Unique id | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


