# \NfsActionsApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**nfs_create_action**](NfsActionsApi.md#nfs_create_action) | **POST** /v2/nfs/{nfs_id}/actions | Initiate an NFS action



## nfs_create_action

> models::NfsActionsResponse nfs_create_action(nfs_id, nfs_create_action_request)
Initiate an NFS action

To execute an action (such as resize) on a specified NFS share,  send a POST request to `/v2/nfs/{nfs_id}/actions`. In the JSON body  to the request, set the `type` attribute to on of the supported action types:  | Action                           | Details | | -------------------------------- | ----------- | | <nobr>`resize`</nobr>            | Resizes an NFS share. Set the size_gib attribute to a desired value in GiB | | <nobr>`snapshot`</nobr>          | Takes a snapshot of an NFS share | | <nobr>`attach`</nobr>            | Attaches an NFS share to a VPC. Set the vpc_id attribute to the desired VPC ID | | <nobr>`detach`</nobr>            | Detaches an NFS share from a VPC. Set the vpc_id attribute to the desired VPC ID | 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**nfs_id** | **String** | The unique ID of the NFS share | [required] |
**nfs_create_action_request** | [**NfsCreateActionRequest**](NfsCreateActionRequest.md) | The `type` attribute set in the request body will specify the  action that will be taken on the NFS share. Some actions will require additional attributes to be set as well.  | [required] |

### Return type

[**models::NfsActionsResponse**](nfs_actions_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

