# AppIngressSpecRuleRoutingRedirect

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uri** | Option<**String**> | An optional URI path to redirect to. Note: if this is specified the whole URI of the original request will be overwritten to this value, irrespective of the original request URI being matched. | [optional]
**authority** | Option<**String**> | The authority/host to redirect to. This can be a hostname or IP address. Note: use `port` to set the port. | [optional]
**port** | Option<**i64**> | The port to redirect to. | [optional]
**scheme** | Option<**String**> | The scheme to redirect to. Supported values are `http` or `https`. Default: `https`. | [optional]
**redirect_code** | Option<**i64**> | The redirect code to use. Defaults to `302`. Supported values are 300, 301, 302, 303, 304, 307, 308. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


