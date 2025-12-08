# AppProposeResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**app_is_static** | Option<**bool**> | Indicates whether the app is a static app. | [optional]
**app_name_available** | Option<**bool**> | Indicates whether the app name is available. | [optional]
**app_name_suggestion** | Option<**String**> | The suggested name if the proposed app name is unavailable. | [optional]
**existing_static_apps** | Option<**String**> | The maximum number of free static apps the account can have. We will charge you for any additional static apps. | [optional]
**spec** | Option<[**models::AppSpec**](app_spec.md)> |  | [optional]
**app_cost** | Option<**i32**> | The monthly cost of the proposed app in USD. | [optional]
**app_tier_downgrade_cost** | Option<**i32**> | The monthly cost of the proposed app in USD using the previous pricing plan tier. For example, if you propose an app that uses the Professional tier, the `app_tier_downgrade_cost` field displays the monthly cost of the app if it were to use the Basic tier. If the proposed app already uses the lest expensive tier, the field is empty. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


