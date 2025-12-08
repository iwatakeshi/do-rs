# NamespaceInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_host** | Option<**String**> | The namespace's API hostname. Each function in a namespace is provided an endpoint at the namespace's hostname. | [optional]
**namespace** | Option<**String**> | A unique string format of UUID with a prefix fn-. | [optional]
**created_at** | Option<**String**> | UTC time string. | [optional]
**updated_at** | Option<**String**> | UTC time string. | [optional]
**label** | Option<**String**> | The namespace's unique name. | [optional]
**region** | Option<**String**> | The namespace's datacenter region. | [optional]
**uuid** | Option<**String**> | The namespace's Universally Unique Identifier. | [optional]
**key** | Option<**String**> | A random alpha numeric string. This key is used in conjunction with the namespace's UUID to authenticate  a user to use the namespace via `doctl`, DigitalOcean's official CLI. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


