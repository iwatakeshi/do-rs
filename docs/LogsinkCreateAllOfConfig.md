# LogsinkCreateAllOfConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**server** | **String** | DNS name or IPv4 address of the rsyslog server | 
**port** | **i32** | The internal port on which the rsyslog server is listening | 
**tls** | **bool** | Use TLS (as the messages are not filtered and may contain sensitive information, it is highly recommended to set this to true if the remote server supports it) | 
**format** | **String** | Message format used by the server, this can be either rfc3164 (the old BSD style message format), `rfc5424` (current syslog message format) or custom | 
**logline** | Option<**String**> | Conditional (required if `format` == `custom`).  Syslog log line template for a custom format, supporting limited rsyslog style templating (using `%tag%`). Supported tags are: `HOSTNAME`, `app-name`, `msg`, `msgid`, `pri`, `procid`, `structured-data`, `timestamp` and `timestamp:::date-rfc3339`.  --- **Datadog Integration Example for Non-Mongo clusters**: ``` DD_KEY <%pri%>1 %timestamp:::date-rfc3339% %HOSTNAME%.DB_NAME %app-name% - - - %msg% ``` - Replace `DD_KEY` with your actual Datadog API key. - Replace `DB_NAME` with the actual name of your database cluster. - Configure the Server:   - US Region: Use `intake.logs.datadoghq.com`   - EU Region: Use `tcp-intake.logs.datadoghq.eu` - Configure the Port:   - US Region: Use port `10516`   - EU Region: Use port `443` - Enable TLS:   - Ensure the TLS checkbox is enabled. - Note: This configuration applies to **non-Mongo clusters only**. For **Mongo clusters**, use the `datadog_logsink` integration instead.  | [optional]
**sd** | Option<**String**> | content of the structured data block of rfc5424 message | [optional]
**ca** | Option<**String**> | PEM encoded CA certificate | [optional]
**key** | Option<**String**> | (PEM format) client key if the server requires client authentication | [optional]
**cert** | Option<**String**> | (PEM format) client cert to use | [optional]
**url** | **String** | Opensearch connection URL | 
**index_prefix** | **String** | Opensearch index prefix | 
**index_days_max** | Option<**i32**> | Maximum number of days of logs to keep | [optional][default to 7]
**timeout** | Option<**f32**> | Opensearch request timeout limit | [optional][default to 10]
**site** | **String** | Datadog connection URL | 
**datadog_api_key** | **String** | Datadog API key | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


