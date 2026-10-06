# WebhookBatch

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**batch_id** | **i64** | Unique identifier for the webhook batch  |
**created_at** | **chrono::DateTime<chrono::FixedOffset>** | Timestamp of when the webhook batch was created  |
**customer_handle** | **String** | Customer handle associated with the webhook batch  |
**duration** | Option<[**models::WebhookBatchDuration**](WebhookBatchDuration.md)> |  | [optional]
**event_count** | **i32** | Number of events in the webhook batch  |
**status** | **Status** | Status of the webhook batch. no_response: no response returned from the webhook endpoint.  (enum: 1xx_response, 2xx_response, 3xx_response, 4xx_response, 5xx_response, no_response) |
**status_code** | Option<**i32**> | HTTP status code returned by the webhook endpoint  | [optional]
**webhook** | **String** | Webhook endpoint to which events in the batch were posted |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
