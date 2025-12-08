# UpdateEndpoint

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ttl** | Option<**i32**> | The amount of time the content is cached by the CDN's edge servers in seconds. TTL must be one of 60, 600, 3600, 86400, or 604800. Defaults to 3600 (one hour) when excluded. | [optional][default to Variant3600]
**certificate_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The ID of a DigitalOcean managed TLS certificate used for SSL when a custom subdomain is provided. | [optional]
**custom_domain** | Option<**String**> | The fully qualified domain name (FQDN) of the custom subdomain used with the CDN endpoint. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


