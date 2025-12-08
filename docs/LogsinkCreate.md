# LogsinkCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sink_name** | Option<**String**> | The name of the Logsink | [optional]
**sink_type** | Option<**String**> | Type of logsink integration.  - Use `datadog` for Datadog integration **only with MongoDB clusters**. - For non-MongoDB clusters, use `rsyslog` for general syslog forwarding. - Other supported types include `elasticsearch` and `opensearch`.  More details about the configuration can be found in the `config` property.  | [optional]
**config** | Option<[**models::LogsinkCreateAllOfConfig**](logsink_create_allOf_config.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


