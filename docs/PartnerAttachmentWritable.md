# PartnerAttachmentWritable

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the partner attachment. Must be unique and may only contain alphanumeric characters, dashes, and periods. | 
**connection_bandwidth_in_mbps** | **i32** | Bandwidth (in Mbps) of the connection. | 
**region** | **String** | The region to create the partner attachment. | 
**naas_provider** | **String** |  | 
**vpc_ids** | **Vec<String>** | An array of VPCs IDs. | 
**parent_uuid** | Option<**String**> | Optional associated partner attachment UUID | [optional]
**bgp** | Option<[**models::PartnerAttachmentWritableBgp**](partner_attachment_writable_bgp.md)> |  | [optional]
**redundancy_zone** | Option<**String**> | Optional redundancy zone for the partner attachment. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


