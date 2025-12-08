# SubscriptionTierBase

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the subscription tier. | [optional]
**slug** | Option<**String**> | The slug identifier of the subscription tier. | [optional]
**included_repositories** | Option<**i32**> | The number of repositories included in the subscription tier. `0` indicates that the subscription tier includes unlimited repositories. | [optional]
**included_storage_bytes** | Option<**i32**> | The amount of storage included in the subscription tier in bytes. | [optional]
**allow_storage_overage** | Option<**bool**> | A boolean indicating whether the subscription tier supports additional storage above what is included in the base plan at an additional cost per GiB used. | [optional]
**included_bandwidth_bytes** | Option<**i32**> | The amount of outbound data transfer included in the subscription tier in bytes. | [optional]
**monthly_price_in_cents** | Option<**i32**> | The monthly cost of the subscription tier in cents. | [optional]
**storage_overage_price_in_cents** | Option<**i32**> | The price paid in cents per GiB for additional storage beyond what is included in the subscription plan. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


