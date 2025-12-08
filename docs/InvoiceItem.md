# InvoiceItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**product** | Option<**String**> | Name of the product being billed in the invoice item. | [optional]
**resource_uuid** | Option<**String**> | UUID of the resource billing in the invoice item if available. | [optional]
**resource_id** | Option<**String**> | ID of the resource billing in the invoice item if available. | [optional]
**group_description** | Option<**String**> | Description of the invoice item when it is a grouped set of usage, such  as DOKS or databases. | [optional]
**description** | Option<**String**> | Description of the invoice item. | [optional]
**amount** | Option<**String**> | Billed amount of this invoice item. Billed in USD. | [optional]
**duration** | Option<**String**> | Duration of time this invoice item was used and subsequently billed. | [optional]
**duration_unit** | Option<**String**> | Unit of time for duration. | [optional]
**start_time** | Option<**String**> | Time the invoice item began to be billed for usage. | [optional]
**end_time** | Option<**String**> | Time the invoice item stopped being billed for usage. | [optional]
**project_name** | Option<**String**> | Name of the DigitalOcean Project this resource belongs to. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


