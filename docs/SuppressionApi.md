# \SuppressionApi

All URIs are relative to *https://api.mailchannels.net/tx/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_suppressions**](SuppressionApi.md#create_suppressions) | **POST** /suppression-list | Create Suppression Entries
[**delete_suppression**](SuppressionApi.md#delete_suppression) | **DELETE** /suppression-list/recipients/{recipient} | Delete Suppression Entry
[**list_suppressions**](SuppressionApi.md#list_suppressions) | **GET** /suppression-list | Retrieve Suppression List



## create_suppressions

> create_suppressions(x_api_key, suppression_list_input)
Create Suppression Entries

Creates suppression entries for the specified account. Parent accounts can create suppression entries for all associated sub-accounts. If suppression_type is not provided, it defaults to 'non-transactional'. The operation is atomic, meaning all entries are successfully added or none are added if an error occurs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_api_key** | **String** |  | [required] |
**suppression_list_input** | [**SuppressionListInput**](SuppressionListInput.md) | The details of the suppression entries to create. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_suppression

> delete_suppression(recipient, x_api_key, source)
Delete Suppression Entry

Deletes suppression entry associated with the account based on the specified recipient and source. If source is not provided, it defaults to 'api'. If source is set to 'all', all suppression entries related to the specified recipient will be deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**recipient** | **String** |  | [required] |
**x_api_key** | **String** |  | [required] |
**source** | Option<**String**> |  |  |[default to api]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_suppressions

> models::SuppressionListResponse list_suppressions(x_api_key, recipient, source, created_before, created_after, limit, offset)
Retrieve Suppression List

Retrieve suppression entries associated with the specified account. Supports filtering by recipient, source and creation date range. The response is paginated, with a default limit of 1000 entries per page and an offset of 0.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_api_key** | **String** |  | [required] |
**recipient** | Option<**String**> |  |  |
**source** | Option<**String**> |  |  |
**created_before** | Option<**String**> | The date and/or time before which the suppression entries were created. Format: YYYY-MM-DD or YYYY-MM-DDTHH:MM:SSZ  |  |
**created_after** | Option<**String**> | The date and/or time after which the suppression entries were created. Format: YYYY-MM-DD or YYYY-MM-DDTHH:MM:SSZ  |  |
**limit** | Option<**i32**> | The maximum number of suppression entries to return. The default is 1000.  |  |[default to 1000]
**offset** | Option<**i32**> | The number of suppression entries to skip before returning results. The default is 0.  |  |[default to 0]

### Return type

[**models::SuppressionListResponse**](SuppressionListResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
