# MetricsRecipientBehaviour

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**buckets** | [**models::MetricsRecipientBehaviourBuckets**](MetricsRecipientBehaviourBuckets.md) |  |
**end_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | The end of the time range for retrieving recipient behaviour metrics (exclusive).  | [optional]
**start_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | The beginning of the time range for retrieving recipient behaviour metrics (inclusive).  | [optional]
**unsubscribe_delivered** | **i32** | Count of recipients of delivered messages that include at least one of the unsubscribe link or unsubscribe headers. Since the unsubscribe feature requires exactly one recipient per message, this count also represents the total number of delivered messages.  |
**unsubscribed** | **i32** | Count of unsubscribed events by recipients.  |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
