# AppComponentInstanceBase

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**instance_count** | Option<**i64**> | The amount of instances that this component should be scaled to. Default: 1. Must not be set if autoscaling is used. | [optional][default to 1]
**instance_size_slug** | Option<**String**> | The instance size to use for this component. Default: `apps-s-1vcpu-0.5gb` | [optional]
**autoscaling** | Option<[**models::AppComponentInstanceBaseAutoscaling**](app_component_instance_base_autoscaling.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


