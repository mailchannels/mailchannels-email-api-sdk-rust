# \WebhooksApi

All URIs are relative to *https://api.mailchannels.net/tx/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_webhook**](WebhooksApi.md#create_webhook) | **POST** /webhook | Enroll for Webhook Notifications
[**delete_webhooks**](WebhooksApi.md#delete_webhooks) | **DELETE** /webhook | Delete Customer Webhooks
[**get_webhook_signing_key**](WebhooksApi.md#get_webhook_signing_key) | **GET** /webhook/public-key | Retrieve Webhook Signing Key
[**list_webhook_batches**](WebhooksApi.md#list_webhook_batches) | **GET** /webhook-batch | Retrieve Webhook Batches
[**list_webhooks**](WebhooksApi.md#list_webhooks) | **GET** /webhook | Retrieve Customer Webhooks
[**resend_webhook_batch**](WebhooksApi.md#resend_webhook_batch) | **POST** /webhook-batch/{batch_id}/resend | Resend Events
[**validate_webhook**](WebhooksApi.md#validate_webhook) | **POST** /webhook/validate | Validate Enrolled Webhook



## create_webhook

> create_webhook(endpoint, x_api_key)
Enroll for Webhook Notifications

Enrolls the customer to receive event notifications via webhooks.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint** | **String** | the URL to which the webhook should be sent | [required] |
**x_api_key** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_webhooks

> delete_webhooks(x_api_key)
Delete Customer Webhooks

Deletes all registered webhook endpoints for the customer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_api_key** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webhook_signing_key

> models::Key get_webhook_signing_key(id)
Retrieve Webhook Signing Key

Retrieves the public key used to verify signatures on incoming webhook payloads.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the ID of the key | [required] |

### Return type

[**models::Key**](Key.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_webhook_batches

> models::WebhookBatchResult list_webhook_batches(x_api_key, created_after, created_before, statuses, webhook, limit, offset)
Retrieve Webhook Batches

Retrieves paged webhook batches associated with the customer. The time range specified by created_after and created_before filters must not exceed 31 days. If neither is specified, the default time range is the last 3 days. Optional filters include status categories, webhook, limit and offset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_api_key** | **String** |  | [required] |
**created_after** | Option<**String**> | Inclusive lower bound(UTC) for filtering webhook batches by creation time. Formats: YYYY-MM-DD or YYYY-MM-DDTHH:MM:SSZ  |  |
**created_before** | Option<**String**> | Exclusive upper bound(UTC) for filtering webhook batches by creation time. Formats: YYYY-MM-DD or YYYY-MM-DDTHH:MM:SSZ  |  |
**statuses** | Option<[**Vec<String>**](String.md)> | Filters webhook batches by webhook response status category. Values must be unique and encoded as a comma-separated list in the query string. If not provided, batches with all categories are returned.  |  |
**webhook** | Option<**String**> | Filters webhook batches by the webhook endpoint to which events in the batch were posted.  |  |
**limit** | Option<**i32**> | The maximum number of webhook batches to return  |  |[default to 500]
**offset** | Option<**i32**> | The number of webhook batches to skip before starting to collect the result set  |  |[default to 0]

### Return type

[**models::WebhookBatchResult**](WebhookBatchResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_webhooks

> Vec<models::Webhook> list_webhooks(x_api_key)
Retrieve Customer Webhooks

Retrieves all registered webhook endpoints associated with the customer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_api_key** | **String** |  | [required] |

### Return type

[**Vec<models::Webhook>**](Webhook.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resend_webhook_batch

> models::WebhookResendResponse resend_webhook_batch(batch_id, x_api_key)
Resend Events

Synchronously resend the webhook batch with the provided batch_id for the customer. The result is returned in the response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_id** | **i64** | the ID of the batch | [required] |
**x_api_key** | **String** |  | [required] |

### Return type

[**models::WebhookResendResponse**](WebhookResendResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_webhook

> models::WebhookValidationResults validate_webhook(x_api_key, webhook_validation_request_body)
Validate Enrolled Webhook

Validates whether your enrolled webhook(s) respond with an HTTP 2xx status code. Sends a test request to each webhook containing your customer handle, a hardcoded event type(test), a hardcoded sender email(test@mailchannels.com),a timestamp, a request ID (provided or generated), and an SMTP ID. The response includes the HTTP status code and body returned by each webhook.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_api_key** | **String** |  | [required] |
**webhook_validation_request_body** | Option<[**WebhookValidationRequestBody**](WebhookValidationRequestBody.md)> |  |  |

### Return type

[**models::WebhookValidationResults**](WebhookValidationResults.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
