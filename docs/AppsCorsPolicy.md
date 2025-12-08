# AppsCorsPolicy

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allow_origins** | Option<[**Vec<models::AppsStringMatch>**](apps_string_match.md)> | The set of allowed CORS origins. | [optional]
**allow_methods** | Option<**Vec<String>**> | The set of allowed HTTP methods. This configures the `Access-Control-Allow-Methods` header. | [optional]
**allow_headers** | Option<**Vec<String>**> | The set of allowed HTTP request headers. This configures the `Access-Control-Allow-Headers` header. | [optional]
**expose_headers** | Option<**Vec<String>**> | The set of HTTP response headers that browsers are allowed to access. This configures the `Access-Control-Expose-Headers` header. | [optional]
**max_age** | Option<**String**> | An optional duration specifying how long browsers can cache the results of a preflight request. This configures the `Access-Control-Max-Age` header. | [optional]
**allow_credentials** | Option<**bool**> | Whether browsers should expose the response to the client-side JavaScript code when the request’s credentials mode is include. This configures the `Access-Control-Allow-Credentials` header. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


