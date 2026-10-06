# WebhookResendResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**batch_id** | **i64** | Unique identifier for the webhook batch  |
**created_at** | **chrono::DateTime<chrono::FixedOffset>** | Timestamp of when the webhook batch was created |
**customer_handle** | **String** | Customer handle associated with the webhook batch |
**duration_in_ms** | Option<**i32**> | Duration of the webhook batch in milliseconds, measured from the time the request was sent to the webhook endpoint until the response was received. Null indicates that no response was returned from the webhook endpoint.  | [optional]
**event_count** | **i32** | Number of events in the webhook batch |
**status_code** | Option<**i32**> | HTTP status code returned by the webhook endpoint. Valid values are 100-599. Null indicates that no response was returned from the webhook endpoint.  | [optional]
**webhook** | **String** | Webhook URL to which events in the batch were posted |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
