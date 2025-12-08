# Vpc

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the VPC. Must be unique and may only contain alphanumeric characters, dashes, and periods. | [optional]
**description** | Option<**String**> | A free-form text field for describing the VPC's purpose. It may be a maximum of 255 characters. | [optional]
**region** | Option<**String**> | The slug identifier for the region where the VPC will be created. | [optional]
**ip_range** | Option<**String**> | The range of IP addresses in the VPC in CIDR notation. Network ranges cannot overlap with other networks in the same account and must be in range of private addresses as defined in RFC1918. It may not be smaller than `/28` nor larger than `/16`. If no IP range is specified, a `/20` network range is generated that won't conflict with other VPC networks in your account. | [optional]
**default** | Option<**bool**> | A boolean value indicating whether or not the VPC is the default network for the region. All applicable resources are placed into the default VPC network unless otherwise specified during their creation. The `default` field cannot be unset from `true`. If you want to set a new default VPC network, update the `default` field of another VPC network in the same region. The previous network's `default` field will be set to `false` when a new default VPC has been defined. | [optional]
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | A unique ID that can be used to identify and reference the VPC. | [optional][readonly]
**urn** | Option<**String**> | The uniform resource name (URN) for the resource in the format do:resource_type:resource_id. | [optional]
**created_at** | Option<**String**> | A time value given in ISO8601 combined date and time format. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


