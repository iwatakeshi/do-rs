# \SshKeysApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ssh_keys_create**](SshKeysApi.md#ssh_keys_create) | **POST** /v2/account/keys | Create a New SSH Key
[**ssh_keys_delete**](SshKeysApi.md#ssh_keys_delete) | **DELETE** /v2/account/keys/{ssh_key_identifier} | Delete an SSH Key
[**ssh_keys_get**](SshKeysApi.md#ssh_keys_get) | **GET** /v2/account/keys/{ssh_key_identifier} | Retrieve an Existing SSH Key
[**ssh_keys_list**](SshKeysApi.md#ssh_keys_list) | **GET** /v2/account/keys | List All SSH Keys
[**ssh_keys_update**](SshKeysApi.md#ssh_keys_update) | **PUT** /v2/account/keys/{ssh_key_identifier} | Update an SSH Key's Name



## ssh_keys_create

> models::SshKeysCreate201Response ssh_keys_create(ssh_keys)
Create a New SSH Key

To add a new SSH public key to your DigitalOcean account, send a POST request to `/v2/account/keys`. Set the `name` attribute to the name you wish to use and the `public_key` attribute to the full public key you are adding.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ssh_keys** | [**SshKeys**](SshKeys.md) |  | [required] |

### Return type

[**models::SshKeysCreate201Response**](sshKeys_create_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ssh_keys_delete

> ssh_keys_delete(ssh_key_identifier)
Delete an SSH Key

To destroy a public SSH key that you have in your account, send a DELETE request to `/v2/account/keys/$KEY_ID` or `/v2/account/keys/$KEY_FINGERPRINT`. A 204 status will be returned, indicating that the action was successful and that the response body is empty.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ssh_key_identifier** | **String** | Either the ID or the fingerprint of an existing SSH key. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ssh_keys_get

> models::SshKeysCreate201Response ssh_keys_get(ssh_key_identifier)
Retrieve an Existing SSH Key

To get information about a key, send a GET request to `/v2/account/keys/$KEY_ID` or `/v2/account/keys/$KEY_FINGERPRINT`. The response will be a JSON object with the key `ssh_key` and value an ssh_key object which contains the standard ssh_key attributes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ssh_key_identifier** | **String** | Either the ID or the fingerprint of an existing SSH key. | [required] |

### Return type

[**models::SshKeysCreate201Response**](sshKeys_create_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ssh_keys_list

> models::SshKeysList200Response ssh_keys_list(per_page, page)
List All SSH Keys

To list all of the keys in your account, send a GET request to `/v2/account/keys`. The response will be a JSON object with a key set to `ssh_keys`. The value of this will be an array of ssh_key objects, each of which contains the standard ssh_key attributes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | Number of items returned per page |  |[default to 20]
**page** | Option<**i32**> | Which 'page' of paginated results to return. |  |[default to 1]

### Return type

[**models::SshKeysList200Response**](sshKeys_list_200_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ssh_keys_update

> models::SshKeysCreate201Response ssh_keys_update(ssh_key_identifier, ssh_keys_update_request)
Update an SSH Key's Name

To update the name of an SSH key, send a PUT request to either `/v2/account/keys/$SSH_KEY_ID` or `/v2/account/keys/$SSH_KEY_FINGERPRINT`. Set the `name` attribute to the new name you want to use.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ssh_key_identifier** | **String** | Either the ID or the fingerprint of an existing SSH key. | [required] |
**ssh_keys_update_request** | [**SshKeysUpdateRequest**](SshKeysUpdateRequest.md) | Set the `name` attribute to the new name you want to use. | [required] |

### Return type

[**models::SshKeysCreate201Response**](sshKeys_create_201_response.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

