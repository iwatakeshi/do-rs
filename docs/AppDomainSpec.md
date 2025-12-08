# AppDomainSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**domain** | **String** | The hostname for the domain | 
**r#type** | Option<**String**> | - DEFAULT: The default `.ondigitalocean.app` domain assigned to this app - PRIMARY: The primary domain for this app that is displayed as the default in the control panel, used in bindable environment variables, and any other places that reference an app's live URL. Only one domain may be set as primary. - ALIAS: A non-primary domain | [optional][default to Unspecified]
**wildcard** | Option<**bool**> | Indicates whether the domain includes all sub-domains, in addition to the given domain | [optional]
**zone** | Option<**String**> | Optional. If the domain uses DigitalOcean DNS and you would like App Platform to automatically manage it for you, set this to the name of the domain on your account.  For example, If the domain you are adding is `app.domain.com`, the zone could be `domain.com`. | [optional]
**minimum_tls_version** | Option<**String**> | The minimum version of TLS a client application can use to access resources for the domain.  Must be one of the following values wrapped within quotations: `\"1.2\"` or `\"1.3\"`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


