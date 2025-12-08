# \BillingApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**balance_get**](BillingApi.md#balance_get) | **GET** /v2/customers/my/balance | Get Customer Balance
[**billing_history_list**](BillingApi.md#billing_history_list) | **GET** /v2/customers/my/billing_history | List Billing History
[**invoices_get_by_uuid**](BillingApi.md#invoices_get_by_uuid) | **GET** /v2/customers/my/invoices/{invoice_uuid} | Retrieve an Invoice by UUID
[**invoices_get_csv_by_uuid**](BillingApi.md#invoices_get_csv_by_uuid) | **GET** /v2/customers/my/invoices/{invoice_uuid}/csv | Retrieve an Invoice CSV by UUID
[**invoices_get_pdf_by_uuid**](BillingApi.md#invoices_get_pdf_by_uuid) | **GET** /v2/customers/my/invoices/{invoice_uuid}/pdf | Retrieve an Invoice PDF by UUID
[**invoices_get_summary_by_uuid**](BillingApi.md#invoices_get_summary_by_uuid) | **GET** /v2/customers/my/invoices/{invoice_uuid}/summary | Retrieve an Invoice Summary by UUID
[**invoices_list**](BillingApi.md#invoices_list) | **GET** /v2/customers/my/invoices | List All Invoices



## balance_get

> models::Balance balance_get()
Get Customer Balance

To retrieve the balances on a customer's account, send a GET request to `/v2/customers/my/balance`.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Balance**](balance.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_history_list

> models::BillingHistoryList200Response billing_history_list()
List Billing History

To retrieve a list of all billing history entries, send a GET request to `/v2/customers/my/billing_history`.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::BillingHistoryList200Response**](billingHistory_list_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invoices_get_by_uuid

> models::InvoicesGetByUuid200Response invoices_get_by_uuid(invoice_uuid, per_page, page)
Retrieve an Invoice by UUID

To retrieve the invoice items for an invoice, send a GET request to `/v2/customers/my/invoices/$INVOICE_UUID`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invoice_uuid** | **String** | UUID of the invoice | [required] |
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**models::InvoicesGetByUuid200Response**](invoices_get_byUUID_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invoices_get_csv_by_uuid

> String invoices_get_csv_by_uuid(invoice_uuid)
Retrieve an Invoice CSV by UUID

To retrieve a CSV for an invoice, send a GET request to `/v2/customers/my/invoices/$INVOICE_UUID/csv`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invoice_uuid** | **String** | UUID of the invoice | [required] |

### Return type

**String**

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/csv, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invoices_get_pdf_by_uuid

> std::path::PathBuf invoices_get_pdf_by_uuid(invoice_uuid)
Retrieve an Invoice PDF by UUID

To retrieve a PDF for an invoice, send a GET request to `/v2/customers/my/invoices/$INVOICE_UUID/pdf`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invoice_uuid** | **String** | UUID of the invoice | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/pdf, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invoices_get_summary_by_uuid

> models::InvoiceSummary invoices_get_summary_by_uuid(invoice_uuid)
Retrieve an Invoice Summary by UUID

To retrieve a summary for an invoice, send a GET request to `/v2/customers/my/invoices/$INVOICE_UUID/summary`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invoice_uuid** | **String** | UUID of the invoice | [required] |

### Return type

[**models::InvoiceSummary**](invoice_summary.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invoices_list

> models::InvoicesList200Response invoices_list(per_page, page)
List All Invoices

To retrieve a list of all invoices, send a GET request to `/v2/customers/my/invoices`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**models::InvoicesList200Response**](invoices_list_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

