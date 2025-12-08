# ElasticsearchLogsink

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | **String** | Elasticsearch connection URL | 
**index_prefix** | **String** | Elasticsearch index prefix | 
**index_days_max** | Option<**i32**> | Maximum number of days of logs to keep | [optional][default to 7]
**timeout** | Option<**f32**> | Elasticsearch request timeout limit | [optional][default to 10]
**ca** | Option<**String**> | PEM encoded CA certificate | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


