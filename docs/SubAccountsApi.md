# \SubAccountsApi

All URIs are relative to *https://api.mailchannels.net/tx/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**activate_subaccount**](SubAccountsApi.md#activate_subaccount) | **POST** /sub-account/{handle}/activate | Activate Sub-account
[**create_subaccount**](SubAccountsApi.md#create_subaccount) | **POST** /sub-account | Create Sub-account
[**create_subaccount_api_key**](SubAccountsApi.md#create_subaccount_api_key) | **POST** /sub-account/{handle}/api-key | Create Sub-account API Key
[**create_subaccount_smtp_password**](SubAccountsApi.md#create_subaccount_smtp_password) | **POST** /sub-account/{handle}/smtp-password | Create Sub-account SMTP Password
[**delete_subaccount**](SubAccountsApi.md#delete_subaccount) | **DELETE** /sub-account/{handle} | Delete Sub-account
[**delete_subaccount_api_key**](SubAccountsApi.md#delete_subaccount_api_key) | **DELETE** /sub-account/{handle}/api-key/{id} | Delete Sub-account API Key
[**delete_subaccount_limit**](SubAccountsApi.md#delete_subaccount_limit) | **DELETE** /sub-account/{handle}/limit | Delete Sub-account Limit
[**delete_subaccount_smtp_password**](SubAccountsApi.md#delete_subaccount_smtp_password) | **DELETE** /sub-account/{handle}/smtp-password/{id} | Delete Sub-account SMTP Password
[**get_subaccount_limit**](SubAccountsApi.md#get_subaccount_limit) | **GET** /sub-account/{handle}/limit | Retrieve Sub-account Limit
[**get_subaccount_usage**](SubAccountsApi.md#get_subaccount_usage) | **GET** /sub-account/{handle}/usage | Retrieve Sub-account Usage Stats
[**list_subaccount_api_keys**](SubAccountsApi.md#list_subaccount_api_keys) | **GET** /sub-account/{handle}/api-key | Retrieve Sub-account API Keys
[**list_subaccount_smtp_passwords**](SubAccountsApi.md#list_subaccount_smtp_passwords) | **GET** /sub-account/{handle}/smtp-password | Retrieve Sub-account SMTP Passwords
[**list_subaccounts**](SubAccountsApi.md#list_subaccounts) | **GET** /sub-account | Retrieve Sub-accounts
[**set_subaccount_limit**](SubAccountsApi.md#set_subaccount_limit) | **PUT** /sub-account/{handle}/limit | Set Sub-account Limit
[**suspend_subaccount**](SubAccountsApi.md#suspend_subaccount) | **POST** /sub-account/{handle}/suspend | Suspend Sub-account



## activate_subaccount

> activate_subaccount(handle, x_api_key)
Activate Sub-account

Activates a suspended sub-account identified by its handle, restoring its ability to send emails.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**handle** | **String** | Handle of sub-account to be activated. | [required] |
**x_api_key** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_subaccount

> models::SubAccountDetails create_subaccount(x_api_key, sub_account_data)
Create Sub-account

Creates a new sub-account under the parent account. Each sub-account must have a unique handle composed solely of lowercase alphanumeric characters. If no handle is provided, a random handle will be generated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_api_key** | **String** |  | [required] |
**sub_account_data** | Option<[**SubAccountData**](SubAccountData.md)> | The details of the sub-account to create. |  |

### Return type

[**models::SubAccountDetails**](SubAccountDetails.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_subaccount_api_key

> models::ApiKey create_subaccount_api_key(handle, x_api_key)
Create Sub-account API Key

Creates a new API key for the specified sub-account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**handle** | **String** | Handle of the sub-account to create API key for. | [required] |
**x_api_key** | **String** |  | [required] |

### Return type

[**models::ApiKey**](APIKey.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_subaccount_smtp_password

> models::SmtpPassword create_subaccount_smtp_password(handle, x_api_key)
Create Sub-account SMTP Password

Creates a new SMTP password for the specified sub-account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**handle** | **String** | Handle of the sub-account to create SMTP password for. | [required] |
**x_api_key** | **String** |  | [required] |

### Return type

[**models::SmtpPassword**](SMTPPassword.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_subaccount

> delete_subaccount(handle, x_api_key)
Delete Sub-account

Deletes the sub-account identified by its handle.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**handle** | **String** | Handle of sub-account to be deleted. | [required] |
**x_api_key** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_subaccount_api_key

> delete_subaccount_api_key(handle, id, x_api_key)
Delete Sub-account API Key

Deletes the API key identified by its ID for the specified sub-account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**handle** | **String** | Handle of the sub-account for which the API key should be deleted.  | [required] |
**id** | **i32** | The ID of the API key to delete. | [required] |
**x_api_key** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_subaccount_limit

> delete_subaccount_limit(handle, x_api_key)
Delete Sub-account Limit

Deletes the limit for the specified sub-account. After a successful deletion, the specified sub-account will be limited to the parent account's limit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**handle** | **String** | Handle of the sub-account to delete limit for. | [required] |
**x_api_key** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_subaccount_smtp_password

> delete_subaccount_smtp_password(handle, id, x_api_key)
Delete Sub-account SMTP Password

Deletes the SMTP password identified by its ID for the specified sub-account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**handle** | **String** | Handle of the sub-account for which the SMTP password should be deleted. | [required] |
**id** | **i32** | The ID of the SMTP password to delete. | [required] |
**x_api_key** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subaccount_limit

> models::Limit get_subaccount_limit(handle, x_api_key)
Retrieve Sub-account Limit

Retrieves the limit of a specified sub-account. A value of -1 indicates that the sub-account inherits the parent account's limit, allowing the sub-account to utilize any remaining capacity within the parent account's allocation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**handle** | **String** | Handle of the sub-account to retrieve the limit for. | [required] |
**x_api_key** | **String** |  | [required] |

### Return type

[**models::Limit**](Limit.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subaccount_usage

> models::UsageStats get_subaccount_usage(handle, x_api_key)
Retrieve Sub-account Usage Stats

Retrieves usage statistics for the specified sub-account during the current billing period.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**handle** | **String** | Handle of the sub-account to query usage stats for. | [required] |
**x_api_key** | **String** |  | [required] |

### Return type

[**models::UsageStats**](UsageStats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_subaccount_api_keys

> Vec<models::ApiKey> list_subaccount_api_keys(handle, x_api_key, limit, offset)
Retrieve Sub-account API Keys

Retrieves details of all API keys associated with the specified sub-account. For security reasons, the full API key is **not** returned; only the key ID and a partially redacted version are provided.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**handle** | **String** | Handle of the sub-account to retrieve the API key for. | [required] |
**x_api_key** | **String** |  | [required] |
**limit** | Option<**i32**> |  |  |[default to 100]
**offset** | Option<**i32**> |  |  |[default to 0]

### Return type

[**Vec<models::ApiKey>**](APIKey.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_subaccount_smtp_passwords

> Vec<models::SmtpPassword> list_subaccount_smtp_passwords(handle, x_api_key)
Retrieve Sub-account SMTP Passwords

Retrieves details of all SMTP passwords associated with the specified sub-account. For security, the full SMTP password is **not** returned; only the password ID and a partially redacted version are provided.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**handle** | **String** | Handle of the sub-account to retrieve the SMTP password for. | [required] |
**x_api_key** | **String** |  | [required] |

### Return type

[**Vec<models::SmtpPassword>**](SMTPPassword.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_subaccounts

> Vec<models::SubAccountDetails> list_subaccounts(x_api_key, limit, offset)
Retrieve Sub-accounts

Retrieves all sub-accounts associated with the parent account. The response is paginated with a default limit of 1000 sub-accounts per page and an offset of 0.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_api_key** | **String** |  | [required] |
**limit** | Option<**i32**> |  |  |[default to 1000]
**offset** | Option<**i32**> |  |  |[default to 0]

### Return type

[**Vec<models::SubAccountDetails>**](SubAccountDetails.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_subaccount_limit

> models::LimitUpdateResult set_subaccount_limit(handle, x_api_key, limit_input)
Set Sub-account Limit

Sets the limit for the specified sub-account. The minimum allowed sends is 0.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**handle** | **String** | Handle of the sub-account to set limit for. | [required] |
**x_api_key** | **String** |  | [required] |
**limit_input** | [**LimitInput**](LimitInput.md) | The value the sub-account limit to set. | [required] |

### Return type

[**models::LimitUpdateResult**](LimitUpdateResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## suspend_subaccount

> suspend_subaccount(handle, x_api_key)
Suspend Sub-account

Suspends the sub-account identified by its handle. This action disables the account, preventing it from sending any emails until it is reactivated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**handle** | **String** | Handle of sub-account to be suspended. | [required] |
**x_api_key** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
