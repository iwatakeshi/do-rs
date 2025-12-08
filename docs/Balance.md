# Balance

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**month_to_date_balance** | Option<**String**> | Balance as of the `generated_at` time.  This value includes the `account_balance` and `month_to_date_usage`. | [optional]
**account_balance** | Option<**String**> | Current balance of the customer's most recent billing activity.  Does not reflect `month_to_date_usage`. | [optional]
**month_to_date_usage** | Option<**String**> | Amount used in the current billing period as of the `generated_at` time. | [optional]
**generated_at** | Option<**String**> | The time at which balances were most recently generated. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


