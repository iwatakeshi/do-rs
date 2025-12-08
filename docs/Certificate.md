# Certificate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | A unique ID that can be used to identify and reference a certificate. | [optional][readonly]
**name** | Option<**String**> | A unique human-readable name referring to a certificate. | [optional]
**not_after** | Option<**String**> | A time value given in ISO8601 combined date and time format that represents the certificate's expiration date. | [optional][readonly]
**sha1_fingerprint** | Option<**String**> | A unique identifier generated from the SHA-1 fingerprint of the certificate. | [optional][readonly]
**created_at** | Option<**String**> | A time value given in ISO8601 combined date and time format that represents when the certificate was created. | [optional][readonly]
**dns_names** | Option<**Vec<String>**> | An array of fully qualified domain names (FQDNs) for which the certificate was issued. | [optional]
**state** | Option<**String**> | A string representing the current state of the certificate. It may be `pending`, `verified`, or `error`. | [optional][readonly]
**r#type** | Option<**String**> | A string representing the type of the certificate. The value will be `custom` for a user-uploaded certificate or `lets_encrypt` for one automatically generated with Let's Encrypt. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


