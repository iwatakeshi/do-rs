# PartnerAttachment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | A unique ID that can be used to identify and reference the partner attachment. | [optional][readonly]
**name** | Option<**String**> | The name of the partner attachment. Must be unique and may only contain alphanumeric characters, dashes, and periods. | [optional]
**state** | Option<**String**> | The current operational state of the attachment. | [optional][readonly]
**connection_bandwidth_in_mbps** | Option<**i32**> | The bandwidth (in Mbps) of the connection. | [optional]
**region** | Option<**String**> | The region where the partner attachment is located. | [optional]
**naas_provider** | Option<**String**> | The Network as a Service (NaaS) provider for the partner attachment. | [optional]
**vpc_ids** | Option<**Vec<String>**> | An array of VPC network IDs. | [optional]
**bgp** | Option<[**models::PartnerAttachmentBgp**](partner_attachment_bgp.md)> |  | [optional]
**created_at** | Option<**String**> | A time value given in ISO8601 combined date and time format. | [optional][readonly]
**parent_uuid** | Option<**String**> | Associated partner attachment UUID | [optional][readonly]
**children** | Option<**Vec<String>**> | An array of associated partner attachment UUIDs. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


