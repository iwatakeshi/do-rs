# InvoicePreview

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**invoice_uuid** | Option<**String**> | The UUID of the invoice. The canonical reference for the invoice. | [optional]
**invoice_id** | Option<**String**> | ID of the invoice. Listed on the face of the invoice PDF as the \"Invoice number\". | [optional]
**amount** | Option<**String**> | Total amount of the invoice, in USD.  This will reflect month-to-date usage in the invoice preview. | [optional]
**invoice_period** | Option<**String**> | Billing period of usage for which the invoice is issued, in `YYYY-MM`  format. | [optional]
**updated_at** | Option<**String**> | Time the invoice was last updated.  This is only included with the invoice preview. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


