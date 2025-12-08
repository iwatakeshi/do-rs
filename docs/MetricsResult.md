# MetricsResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**metric** | **std::collections::HashMap<String, String>** | An object containing the metric's labels. These labels are key/value pairs that vary depending on the metric being queried. For example, load balancer metrics contain a `lb_id` label, while Droplet metrics contain a `host_id` label, and App Platform metrics contain a `app_component` label. | 
**values** | [**Vec<Vec<models::MetricsResultValuesInnerInner>>**](Vec.md) | An array of values for the metric. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


