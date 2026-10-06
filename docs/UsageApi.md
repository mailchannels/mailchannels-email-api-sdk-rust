# \UsageApi

All URIs are relative to *https://api.mailchannels.net/tx/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_usage**](UsageApi.md#get_usage) | **GET** /usage | Retrieve Usage Stats



## get_usage

> models::UsageStats get_usage(x_api_key)
Retrieve Usage Stats

Retrieves usage statistics during the current billing period.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_api_key** | **String** |  | [required] |

### Return type

[**models::UsageStats**](UsageStats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
