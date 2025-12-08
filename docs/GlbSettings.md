# GlbSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**target_protocol** | Option<**String**> | The protocol used for forwarding traffic from the load balancer to the target backends. The possible values are `http`, `https` and `http2`. | [optional]
**target_port** | Option<**i32**> | An integer representing the port on the target backends which the load balancer will forward traffic to. | [optional]
**cdn** | Option<[**models::GlbSettingsCdn**](glb_settings_cdn.md)> |  | [optional]
**region_priorities** | Option<**std::collections::HashMap<String, i32>**> | A map of region string to an integer priority value indicating preference for which regional target a Global load balancer will forward traffic to. A lower value indicates a higher priority. | [optional]
**failover_threshold** | Option<**i32**> | An integer value as a percentage to indicate failure threshold to decide how the regional priorities will take effect. A value of `50` would indicate that the Global load balancer will choose a lower priority region to forward traffic to once this failure threshold has been reached for the higher priority region. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


