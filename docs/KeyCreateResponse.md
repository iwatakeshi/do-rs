# KeyCreateResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The access key's name. | [optional]
**grants** | Option<[**Vec<models::Grant>**](grant.md)> | The list of permissions for the access key. | [optional][default to []]
**access_key** | Option<**String**> | The Access Key ID used to access a bucket. | [optional][readonly]
**created_at** | Option<**String**> | The date and time the key was created. | [optional][readonly]
**secret_key** | Option<**String**> | The secret key used to access the bucket. We return secret keys only once upon creation. Make sure to copy the key and securely store it. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


