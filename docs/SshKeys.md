# SshKeys

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | A unique identification number for this key. Can be used to embed a  specific SSH key into a Droplet. | [optional][readonly]
**fingerprint** | Option<**String**> | A unique identifier that differentiates this key from other keys using  a format that SSH recognizes. The fingerprint is created when the key is added to your account. | [optional][readonly]
**public_key** | **String** | The entire public key string that was uploaded. Embedded into the root user's `authorized_keys` file if you include this key during Droplet creation. | 
**name** | **String** | A human-readable display name for this key, used to easily identify the SSH keys when they are displayed. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


