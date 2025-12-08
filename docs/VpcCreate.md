# VpcCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**region** | Option<**String**> | The slug identifier for the region where the VPC will be created. | [optional]
**ip_range** | Option<**String**> | The range of IP addresses in the VPC in CIDR notation. Network ranges cannot overlap with other networks in the same account and must be in range of private addresses as defined in RFC1918. It may not be smaller than `/28` nor larger than `/16`. If no IP range is specified, a `/20` network range is generated that won't conflict with other VPC networks in your account. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


