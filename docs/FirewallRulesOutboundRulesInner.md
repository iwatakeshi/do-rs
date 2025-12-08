# FirewallRulesOutboundRulesInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**protocol** | **String** | The type of traffic to be allowed. This may be one of `tcp`, `udp`, or `icmp`. | 
**ports** | **String** | The ports on which traffic will be allowed specified as a string containing a single port, a range (e.g. \"8000-9000\"), or \"0\" when all ports are open for a protocol. For ICMP rules this parameter will always return \"0\". | 
**destinations** | [**models::FirewallRuleTarget**](firewall_rule_target.md) | An object specifying locations to which outbound traffic that will be allowed. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


