# AppServiceSpecTermination

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**drain_seconds** | Option<**i32**> | The number of seconds to wait between selecting a container instance for termination and issuing the TERM signal. Selecting a container instance for termination begins an asynchronous drain of new requests on upstream load-balancers. (Default 15) | [optional]
**grace_period_seconds** | Option<**i32**> | The number of seconds to wait between sending a TERM signal to a container and issuing a KILL which causes immediate shutdown. (Default 120) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


