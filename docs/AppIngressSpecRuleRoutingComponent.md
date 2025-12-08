# AppIngressSpecRuleRoutingComponent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the component to route to. | 
**preserve_path_prefix** | Option<**String**> | An optional flag to preserve the path that is forwarded to the backend service. By default, the HTTP request path will be trimmed from the left when forwarded to the component. For example, a component with `path=/api` will have requests to `/api/list` trimmed to `/list`. If this value is `true`, the path will remain `/api/list`. Note: this is not applicable for Functions Components and is mutually exclusive with `rewrite`. | [optional]
**rewrite** | Option<**String**> | An optional field that will rewrite the path of the component to be what is specified here. By default, the HTTP request path will be trimmed from the left when forwarded to the component. For example, a component with `path=/api` will have requests to `/api/list` trimmed to `/list`. If you specified the rewrite to be `/v1/`, requests to `/api/list` would be rewritten to `/v1/list`. Note: this is mutually exclusive with `preserve_path_prefix`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


