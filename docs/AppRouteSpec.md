# AppRouteSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**path** | Option<**String**> | (Deprecated - Use Ingress Rules instead). An HTTP path prefix. Paths must start with / and must be unique across all components within an app. | [optional]
**preserve_path_prefix** | Option<**bool**> | An optional flag to preserve the path that is forwarded to the backend service. By default, the HTTP request path will be trimmed from the left when forwarded to the component. For example, a component with `path=/api` will have requests to `/api/list` trimmed to `/list`. If this value is `true`, the path will remain `/api/list`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


