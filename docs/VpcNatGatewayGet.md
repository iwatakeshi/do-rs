# VpcNatGatewayGet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The unique identifier for the VPC NAT gateway. This is automatically generated upon creation. | [optional]
**name** | Option<**String**> | The human-readable name of the VPC NAT gateway. | [optional]
**r#type** | Option<**String**> | The type of the VPC NAT gateway. | [optional]
**state** | Option<**String**> | The current state of the VPC NAT gateway. | [optional]
**region** | Option<**String**> | The region in which the VPC NAT gateway is created. | [optional]
**size** | Option<**i32**> | The size of the VPC NAT gateway. | [optional]
**vpcs** | Option<[**Vec<models::VpcNatGatewayGetVpcsInner>**](vpc_nat_gateway_get_vpcs_inner.md)> | An array of VPCs associated with the VPC NAT gateway. | [optional]
**egresses** | Option<[**models::VpcNatGatewayGetEgresses**](vpc_nat_gateway_get_egresses.md)> |  | [optional]
**udp_timeout_seconds** | Option<**i32**> | The UDP timeout in seconds for the VPC NAT gateway. | [optional]
**icmp_timeout_seconds** | Option<**i32**> | The ICMP timeout in seconds for the VPC NAT gateway. | [optional]
**tcp_timeout_seconds** | Option<**i32**> | The TCP timeout in seconds for the VPC NAT gateway. | [optional]
**created_at** | Option<**String**> | A time value given in ISO8601 combined date and time format that represents when the VPC NAT gateway was created. | [optional]
**updated_at** | Option<**String**> | A time value given in ISO8601 combined date and time format that represents when the VPC NAT gateway was last updated. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


