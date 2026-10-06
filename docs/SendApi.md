# \SendApi

All URIs are relative to *https://api.mailchannels.net/tx/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**queue_email**](SendApi.md#queue_email) | **POST** /send-async | Send an Email Asynchronously
[**send_email**](SendApi.md#send_email) | **POST** /send | Send an Email



## queue_email

> models::AsyncSendResponse queue_email(x_api_key, mail_send_body)
Send an Email Asynchronously

Queues an email message for asynchronous processing and returns immediately with a request ID.  The email will be processed in the background, and you'll receive webhook events for all delivery status updates (e.g. dropped, processed, delivered, hard-bounced). These webhook events are identical to those sent for the synchronous /send endpoint.  Use this endpoint when you need to send emails without waiting for processing to complete. This can improve your application's response time, especially when sending to multiple recipients.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_api_key** | **String** |  | [required] |
**mail_send_body** | [**MailSendBody**](MailSendBody.md) |  | [required] |

### Return type

[**models::AsyncSendResponse**](AsyncSendResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_email

> models::Message send_email(x_api_key, mail_send_body, dry_run)
Send an Email

Sends an email message to one or more recipients.  **Click Tracking Notes:** Only links (`<a>` tags) meeting all of the following conditions are processed for click tracking: - The URL is non-empty. - The URL starts with \"http\" or \"https\". - The link does not have a clicktracking attribute set to 'off'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_api_key** | **String** |  | [required] |
**mail_send_body** | [**MailSendBody**](MailSendBody.md) |  | [required] |
**dry_run** | Option<**bool**> | When present and set to true, the message will not be sent. Instead, the fully rendered message is returned. This can be useful for testing.  |  |

### Return type

[**models::Message**](Message.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
