# FirewallRuleTarget

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**addresses** | Option<**Vec<String>**> | An array of strings containing the IPv4 addresses, IPv6 addresses, IPv4 CIDRs, and/or IPv6 CIDRs to which the firewall will allow traffic. | [optional]
**droplet_ids** | Option<**Vec<i32>**> | An array containing the IDs of the Droplets to which the firewall will allow traffic. | [optional]
**load_balancer_uids** | Option<**Vec<String>**> | An array containing the IDs of the load balancers to which the firewall will allow traffic. | [optional]
**kubernetes_ids** | Option<**Vec<String>**> | An array containing the IDs of the Kubernetes clusters to which the firewall will allow traffic. | [optional]
**tags** | Option<**Vec<String>**> | An array containing the names of Tags corresponding to groups of Droplets to which the firewall will allow traffic. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


