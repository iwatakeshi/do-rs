# VpcNatGatewayCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The human-readable name of the VPC NAT gateway. | 
**r#type** | **String** | The type of the VPC NAT gateway. | 
**region** | **String** | The region in which the VPC NAT gateway is created. | 
**size** | **i32** | The size of the VPC NAT gateway. | 
**vpcs** | [**Vec<models::VpcNatGatewayCreateVpcsInner>**](vpc_nat_gateway_create_vpcs_inner.md) | An array of VPCs associated with the VPC NAT gateway. | 
**udp_timeout_seconds** | Option<**i32**> | The UDP timeout in seconds for the VPC NAT gateway. | [optional]
**icmp_timeout_seconds** | Option<**i32**> | The ICMP timeout in seconds for the VPC NAT gateway. | [optional]
**tcp_timeout_seconds** | Option<**i32**> | The TCP timeout in seconds for the VPC NAT gateway. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


