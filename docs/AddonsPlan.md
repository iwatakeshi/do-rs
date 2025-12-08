# AddonsPlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | ID of a given plan. | 
**app_id** | **i32** | ID of the app associated with this plan. | 
**display_name** | **String** | Display name for a given plan. | 
**description** | Option<**String**> | Description of an app plan. | [optional]
**slug** | **String** | Slug identifier for the plan. | 
**price_per_month** | **i32** | Price of a month's usage of the plan in US dollars. | 
**active** | **bool** | Indicates if the plan is currently active. | 
**state** | **String** | Current state of the plan. | 
**features** | Option<[**Vec<models::AddonsFeature>**](addons_feature.md)> | List of features included in the plan. | [optional]
**created_at** | **String** | Timestamp when the plan was created. | 
**updated_at** | **String** | Timestamp when the plan was last updated. | 
**available** | **bool** | Indicates if the plan is available for selection. | 
**uuid** | **String** | Unique identifier for the plan. | 
**by_default** | **bool** | Indicates if this plan is the default option for the app. | 
**dimensions** | Option<[**Vec<models::AddonsDimensionWithPrice>**](addons_dimension_with_price.md)> | List of dimensions associated with the plan, each with its own pricing. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


