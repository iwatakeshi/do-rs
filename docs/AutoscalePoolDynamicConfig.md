# AutoscalePoolDynamicConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**min_instances** | **i32** | The minimum number of Droplets in an autoscale pool. | 
**max_instances** | **i32** | The maximum number of Droplets in an autoscale pool. | 
**target_cpu_utilization** | Option<**f32**> | Target CPU utilization as a decimal. | [optional]
**target_memory_utilization** | Option<**f32**> | Target memory utilization as a decimal. | [optional]
**cooldown_minutes** | Option<**i32**> | The number of minutes to wait between scaling events in an autoscale pool. Defaults to 10 minutes. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


