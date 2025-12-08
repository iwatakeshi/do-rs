# OpensearchConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | A unique identifier for a configuration. | [optional]
**credentials** | Option<[**models::OpensearchConfigRequestCredentials**](opensearch_config_request_credentials.md)> |  | [optional]
**endpoint** | **String** | host of the OpenSearch cluster | 
**cluster_uuid** | Option<**String**> | A unique identifier for a managed OpenSearch cluster. | [optional]
**cluster_name** | Option<**String**> | Name of a managed OpenSearch cluster. | [optional]
**index_name** | Option<**String**> | OpenSearch index to send logs to. | [optional]
**retention_days** | Option<**i32**> | Number of days to retain logs in OpenSearch (default: 14) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


