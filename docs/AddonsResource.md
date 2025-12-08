# AddonsResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uuid** | **String** | The unique identifier for the addon resource. | 
**name** | **String** | The name of the addon resource. | 
**state** | **String** | The state the resource is currently in. | 
**app_name** | Option<**String**> | The name of the application associated with the resource. | [optional]
**app_slug** | **String** | The slug identifier for the application associated with the resource. | 
**plan_name** | Option<**String**> | The name of the plan associated with the resource. | [optional]
**plan_slug** | **String** | The slug identifier for the plan associated with the resource. | 
**plan_price_per_month** | Option<**i32**> | The price of the plan per month in US dollars. | [optional]
**has_config** | **bool** | Indicates if the resource has configuration values set by the vendor. | 
**metadata** | Option<[**Vec<models::AddonsResourceMetadata>**](addons_resource_metadata.md)> | Metadata associated with the resource, set by the user. | [optional]
**sso_url** | Option<**String**> | The Single Sign-On URL for the resource, if applicable. | [optional]
**message** | Option<**String**> | A message related to the resource, if applicable. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


