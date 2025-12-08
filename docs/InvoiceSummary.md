# InvoiceSummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**invoice_uuid** | Option<**String**> | UUID of the invoice | [optional]
**invoice_id** | Option<**String**> | ID of the invoice | [optional]
**billing_period** | Option<**String**> | Billing period of usage for which the invoice is issued, in `YYYY-MM`  format. | [optional]
**amount** | Option<**String**> | Total amount of the invoice, in USD.  This will reflect month-to-date usage in the invoice preview. | [optional]
**user_name** | Option<**String**> | Name of the DigitalOcean customer being invoiced. | [optional]
**user_billing_address** | Option<[**models::BillingAddress**](billing_address.md)> | The billing address of the customer being invoiced. | [optional]
**user_company** | Option<**String**> | Company of the DigitalOcean customer being invoiced, if set. | [optional]
**user_email** | Option<**String**> | Email of the DigitalOcean customer being invoiced. | [optional]
**product_charges** | Option<[**models::ProductUsageCharges**](product_usage_charges.md)> | A summary of the product usage charges contributing to the invoice.  This will include an amount, and grouped aggregates by resource type  under the `items` key. | [optional]
**overages** | Option<[**models::SimpleCharge**](simple_charge.md)> | A summary of the overages contributing to the invoice. | [optional]
**taxes** | Option<[**models::SimpleCharge**](simple_charge.md)> | A summary of the taxes contributing to the invoice. | [optional]
**credits_and_adjustments** | Option<[**models::SimpleCharge**](simple_charge.md)> | A summary of the credits and adjustments contributing to the invoice. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


