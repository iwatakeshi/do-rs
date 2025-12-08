# \VpcnatGatewaysApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**vpcnatgateways_create**](VpcnatGatewaysApi.md#vpcnatgateways_create) | **POST** /v2/vpc_nat_gateways | Create a New VPC NAT Gateway
[**vpcnatgateways_delete**](VpcnatGatewaysApi.md#vpcnatgateways_delete) | **DELETE** /v2/vpc_nat_gateways/{id} | Delete VPC NAT Gateway
[**vpcnatgateways_get**](VpcnatGatewaysApi.md#vpcnatgateways_get) | **GET** /v2/vpc_nat_gateways/{id} | Retrieve an Existing VPC NAT Gateway
[**vpcnatgateways_list**](VpcnatGatewaysApi.md#vpcnatgateways_list) | **GET** /v2/vpc_nat_gateways | List All VPC NAT Gateways
[**vpcnatgateways_update**](VpcnatGatewaysApi.md#vpcnatgateways_update) | **PUT** /v2/vpc_nat_gateways/{id} | Update VPC NAT Gateway



## vpcnatgateways_create

> models::VpcnatgatewaysCreate202Response vpcnatgateways_create(vpc_nat_gateway_create)
Create a New VPC NAT Gateway

To create a new VPC NAT gateway, send a POST request to `/v2/vpc_nat_gateways` setting the required attributes.  The response body will contain a JSON object with a key called `vpc_nat_gateway` containing the standard attributes for the new VPC NAT gateway. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vpc_nat_gateway_create** | Option<[**VpcNatGatewayCreate**](VpcNatGatewayCreate.md)> |  |  |

### Return type

[**models::VpcnatgatewaysCreate202Response**](vpcnatgateways_create_202_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vpcnatgateways_delete

> vpcnatgateways_delete(id)
Delete VPC NAT Gateway

To destroy a VPC NAT Gateway, send a DELETE request to the `/v2/vpc_nat_gateways/$VPC_NAT_GATEWAY_ID` endpoint.  A successful response will include a 202 response code and no content.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The unique identifier of the VPC NAT gateway. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vpcnatgateways_get

> models::VpcnatgatewaysGet200Response vpcnatgateways_get(id)
Retrieve an Existing VPC NAT Gateway

To show information about an individual VPC NAT gateway, send a GET request to `/v2/vpc_nat_gateways/$VPC_NAT_GATEWAY_ID`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The unique identifier of the VPC NAT gateway. | [required] |

### Return type

[**models::VpcnatgatewaysGet200Response**](vpcnatgateways_get_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vpcnatgateways_list

> models::VpcnatgatewaysList200Response vpcnatgateways_list(per_page, page, state, region, r#type, name)
List All VPC NAT Gateways

To list all VPC NAT gateways in your team, send a GET request to `/v2/vpc_nat_gateways`. The response body will be a JSON object with a key of `vpc_nat_gateways` containing an array of VPC NAT gateway objects. These each contain the standard VPC NAT gateway attributes. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]
**state** | Option<**String**> | The current state of the VPC NAT gateway. |  |
**region** | Option<**String**> | The region where the VPC NAT gateway is located. |  |
**r#type** | Option<**String**> | The type of the VPC NAT gateway. |  |
**name** | Option<**String**> | The name of the VPC NAT gateway. |  |

### Return type

[**models::VpcnatgatewaysList200Response**](vpcnatgateways_list_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vpcnatgateways_update

> models::VpcnatgatewaysUpdate200Response vpcnatgateways_update(id, vpc_nat_gateway_update)
Update VPC NAT Gateway

To update the configuration of an existing VPC NAT Gateway, send a PUT request to `/v2/vpc_nat_gateways/$VPC_NAT_GATEWAY_ID`. The request must contain a full representation of the VPC NAT Gateway including existing attributes.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The unique identifier of the VPC NAT gateway. | [required] |
**vpc_nat_gateway_update** | Option<[**VpcNatGatewayUpdate**](VpcNatGatewayUpdate.md)> |  |  |

### Return type

[**models::VpcnatgatewaysUpdate200Response**](vpcnatgateways_update_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

