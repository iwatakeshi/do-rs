# AutoscalePool

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | A unique identifier for each autoscale pool instance. This is automatically generated upon autoscale pool creation. | 
**name** | **String** | The human-readable name set for the autoscale pool. | 
**config** | [**models::AutoscalePoolConfig**](autoscale_pool_config.md) |  | 
**droplet_template** | [**models::AutoscalePoolDropletTemplate**](autoscale_pool_droplet_template.md) |  | 
**current_utilization** | Option<[**models::CurrentUtilization**](current_utilization.md)> |  | [optional]
**created_at** | **String** | A time value given in ISO8601 combined date and time format that represents when the autoscale pool was created. | 
**updated_at** | **String** | A time value given in ISO8601 combined date and time format that represents when the autoscale pool was last updated. | 
**status** | **String** | The current status of the autoscale pool. | 
**active_resources_count** | **i32** | The number of active Droplets in the autoscale pool. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


