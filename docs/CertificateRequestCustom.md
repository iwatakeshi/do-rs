# CertificateRequestCustom

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | A unique human-readable name referring to a certificate. | 
**r#type** | Option<**String**> | A string representing the type of the certificate. The value will be `custom` for a user-uploaded certificate or `lets_encrypt` for one automatically generated with Let's Encrypt. | [optional]
**private_key** | **String** | The contents of a PEM-formatted private-key corresponding to the SSL certificate. | 
**leaf_certificate** | **String** | The contents of a PEM-formatted public SSL certificate. | 
**certificate_chain** | Option<**String**> | The full PEM-formatted trust chain between the certificate authority's certificate and your domain's SSL certificate. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


