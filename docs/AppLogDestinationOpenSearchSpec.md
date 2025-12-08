# AppLogDestinationOpenSearchSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**endpoint** | Option<**String**> | OpenSearch API Endpoint. Only HTTPS is supported. Format: https://<host>:<port>. Cannot be specified if `cluster_name` is also specified. | [optional]
**basic_auth** | Option<[**models::AppLogDestinationOpenSearchSpecBasicAuth**](app_log_destination_open_search_spec_basic_auth.md)> |  | [optional]
**index_name** | Option<**String**> | The index name to use for the logs. If not set, the default index name is \"logs\". | [optional][default to logs]
**cluster_name** | Option<**String**> | The name of a DigitalOcean DBaaS OpenSearch cluster to use as a log forwarding destination. Cannot be specified if `endpoint` is also specified. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


