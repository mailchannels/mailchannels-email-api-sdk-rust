# \DkimApi

All URIs are relative to *https://api.mailchannels.net/tx/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**check_domain**](DkimApi.md#check_domain) | **POST** /check-domain | DKIM, SPF & Domain Lockdown Check
[**create_dkim_key**](DkimApi.md#create_dkim_key) | **POST** /domains/{domain}/dkim-keys | Create DKIM Key Pair
[**list_dkim_keys**](DkimApi.md#list_dkim_keys) | **GET** /domains/{domain}/dkim-keys | Retrieve DKIM Keys
[**rotate_dkim_key**](DkimApi.md#rotate_dkim_key) | **POST** /domains/{domain}/dkim-keys/{selector}/rotate | Rotate DKIM Key Pair
[**update_dkim_key**](DkimApi.md#update_dkim_key) | **PATCH** /domains/{domain}/dkim-keys/{selector} | Update DKIM Key Status



## check_domain

> models::CheckDomainResult check_domain(x_api_key, check_domain_body)
DKIM, SPF & Domain Lockdown Check

Validates a domain's email authentication setup by retrieving its DKIM, SPF, and Domain Lockdown status. This endpoint checks whether the domain is properly configured for secure email delivery.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_api_key** | **String** |  | [required] |
**check_domain_body** | [**CheckDomainBody**](CheckDomainBody.md) |  | [required] |

### Return type

[**models::CheckDomainResult**](CheckDomainResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dkim_key

> models::DkimKeyInfo create_dkim_key(domain, x_api_key, dkim_key_pair_create_request)
Create DKIM Key Pair

Create a DKIM key pair for a specified domain and selector using the specified algorithm and key length, for the current customer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** |  | [required] |
**x_api_key** | **String** |  | [required] |
**dkim_key_pair_create_request** | [**DkimKeyPairCreateRequest**](DkimKeyPairCreateRequest.md) |  | [required] |

### Return type

[**models::DkimKeyInfo**](DKIMKeyInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_dkim_keys

> models::DkimKeyList list_dkim_keys(domain, x_api_key, selector, status, offset, limit, include_dns_record)
Retrieve DKIM Keys

Search for DKIM keys by domain, with optional filters. If selector is provided, at most one key will be returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** |  | [required] |
**x_api_key** | **String** |  | [required] |
**selector** | Option<**String**> |  |  |
**status** | Option<**String**> |  |  |
**offset** | Option<**i32**> | Number of keys to skip before returning results. The default is 0.  |  |[default to 0]
**limit** | Option<**i32**> | Maximum number of keys to return. The default is 10.  |  |[default to 10]
**include_dns_record** | Option<**bool**> | If true, includes the suggested DKIM DNS record for each returned key. Defaults to false.  |  |[default to false]

### Return type

[**models::DkimKeyList**](DKIMKeyList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rotate_dkim_key

> models::DkimKeyRotateResponse rotate_dkim_key(domain, selector, x_api_key, dkim_key_rotate_request)
Rotate DKIM Key Pair

Rotate an active DKIM key pair. Mark the original key as 'rotated', and create a new key pair with the required new key selector, reusing the same algorithm and key length. The rotated key remains valid for signing for a 3-day grace period, and is automatically changed to 'retired' 2 weeks after rotation. Publish the new key to its DNS TXT record before rotated key expires for signing as emails sent with an unpublished key will fail DKIM validation by receiving providers. After the grace period, only the new key is valid for signing if published.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** |  | [required] |
**selector** | **String** |  | [required] |
**x_api_key** | **String** |  | [required] |
**dkim_key_rotate_request** | [**DkimKeyRotateRequest**](DkimKeyRotateRequest.md) |  | [required] |

### Return type

[**models::DkimKeyRotateResponse**](DKIMKeyRotateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dkim_key

> update_dkim_key(domain, selector, x_api_key, dkim_key_pair_update_request)
Update DKIM Key Status

Update fields of an existing DKIM key pair for the specified domain and selector, for the current customer. Currently, only the status field can be updated. revoked: Indicates that the key is compromised and should not be used. retired: Indicates that the key has been rotated and is no longer in use. rotated: Indicates that the key is going through the rotation process. Only active key pairs can be updated to this status, and no new key pair is created. The rotated key can be used to sign emails for 3 days after the status update, and will automatically change to 'retired' 2 weeks after update. For a smooth key transition, it is recommended to create and publish a new key pair before signing is disabled for the rotated key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** |  | [required] |
**selector** | **String** |  | [required] |
**x_api_key** | **String** |  | [required] |
**dkim_key_pair_update_request** | [**DkimKeyPairUpdateRequest**](DkimKeyPairUpdateRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
