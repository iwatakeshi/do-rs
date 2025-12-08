# StickySessions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> | An attribute indicating how and if requests from a client will be persistently served by the same backend Droplet. The possible values are `cookies` or `none`. | [optional][default to None]
**cookie_name** | Option<**String**> | The name of the cookie sent to the client. This attribute is only returned when using `cookies` for the sticky sessions type. | [optional]
**cookie_ttl_seconds** | Option<**i32**> | The number of seconds until the cookie set by the load balancer expires. This attribute is only returned when using `cookies` for the sticky sessions type. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


