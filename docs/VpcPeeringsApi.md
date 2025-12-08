# \VpcPeeringsApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**vpc_peerings_create**](VpcPeeringsApi.md#vpc_peerings_create) | **POST** /v2/vpc_peerings | Create a New VPC Peering
[**vpc_peerings_delete**](VpcPeeringsApi.md#vpc_peerings_delete) | **DELETE** /v2/vpc_peerings/{vpc_peering_id} | Delete a VPC peering
[**vpc_peerings_get**](VpcPeeringsApi.md#vpc_peerings_get) | **GET** /v2/vpc_peerings/{vpc_peering_id} | Retrieve an Existing VPC Peering
[**vpc_peerings_list**](VpcPeeringsApi.md#vpc_peerings_list) | **GET** /v2/vpc_peerings | List All VPC Peerings
[**vpc_peerings_patch**](VpcPeeringsApi.md#vpc_peerings_patch) | **PATCH** /v2/vpc_peerings/{vpc_peering_id} | Update a VPC peering



## vpc_peerings_create

> models::VpcPeeringsCreate202Response vpc_peerings_create(vpc_peerings_create_request)
Create a New VPC Peering

To create a new VPC Peering, send a POST request to `/v2/vpc_peerings`  specifying a name and a list of two VPC IDs to peer. The response code, 202  Accepted, does not indicate the success or failure of the operation, just  that the request has been accepted for processing. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vpc_peerings_create_request** | [**VpcPeeringsCreateRequest**](VpcPeeringsCreateRequest.md) |  | [required] |

### Return type

[**models::VpcPeeringsCreate202Response**](vpcPeerings_create_202_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vpc_peerings_delete

> models::VpcPeeringsDelete202Response vpc_peerings_delete(vpc_peering_id)
Delete a VPC peering

To delete a VPC peering, send a DELETE request to `/v2/vpc_peerings/$VPC_PEERING_ID`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vpc_peering_id** | **uuid::Uuid** | A unique identifier for a VPC peering. | [required] |

### Return type

[**models::VpcPeeringsDelete202Response**](vpcPeerings_delete_202_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vpc_peerings_get

> models::VpcPeeringsGet200Response vpc_peerings_get(vpc_peering_id)
Retrieve an Existing VPC Peering

To show information about an existing VPC Peering, send a GET request to `/v2/vpc_peerings/$VPC_PEERING_ID`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vpc_peering_id** | **uuid::Uuid** | A unique identifier for a VPC peering. | [required] |

### Return type

[**models::VpcPeeringsGet200Response**](vpcPeerings_get_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vpc_peerings_list

> models::VpcPeeringsList200Response vpc_peerings_list(per_page, page, region)
List All VPC Peerings

To list all of the VPC peerings on your account, send a GET request to `/v2/vpc_peerings`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]
**region** | Option<[**RegionSlug**](.md)> | The slug identifier for the region where the resource is available. |  |

### Return type

[**models::VpcPeeringsList200Response**](vpcPeerings_list_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vpc_peerings_patch

> models::VpcPeeringsGet200Response vpc_peerings_patch(vpc_peering_id, vpc_peering_updatable)
Update a VPC peering

To update the name of a VPC peering, send a PATCH request to `/v2/vpc_peerings/$VPC_PEERING_ID` with the new `name` in the request body. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vpc_peering_id** | **uuid::Uuid** | A unique identifier for a VPC peering. | [required] |
**vpc_peering_updatable** | [**VpcPeeringUpdatable**](VpcPeeringUpdatable.md) |  | [required] |

### Return type

[**models::VpcPeeringsGet200Response**](vpcPeerings_get_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

