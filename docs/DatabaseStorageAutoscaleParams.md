# DatabaseStorageAutoscaleParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enabled** | **bool** | Whether storage autoscaling is enabled for the cluster | 
**threshold_percent** | Option<**i32**> | The storage usage threshold percentage that triggers autoscaling. When storage usage exceeds this percentage, additional storage will be added automatically. | [optional]
**increment_gib** | Option<**i32**> | The amount of additional storage to add (in GiB) when autoscaling is triggered | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


