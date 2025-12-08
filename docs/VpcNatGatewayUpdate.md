# VpcNatGatewayUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The human-readable name of the VPC NAT gateway. | 
**size** | **i32** | The size of the VPC NAT gateway. | 
**vpcs** | Option<[**Vec<models::VpcNatGatewayUpdateVpcsInner>**](vpc_nat_gateway_update_vpcs_inner.md)> | An array of VPCs associated with the VPC NAT gateway. | [optional]
**udp_timeout_seconds** | Option<**i32**> | The UDP timeout in seconds for the VPC NAT gateway. | [optional]
**icmp_timeout_seconds** | Option<**i32**> | The ICMP timeout in seconds for the VPC NAT gateway. | [optional]
**tcp_timeout_seconds** | Option<**i32**> | The TCP timeout in seconds for the VPC NAT gateway. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


