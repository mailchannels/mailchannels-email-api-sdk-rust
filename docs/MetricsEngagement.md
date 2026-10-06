# MetricsEngagement

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**buckets** | [**models::MetricsEngagementBuckets**](MetricsEngagementBuckets.md) |  |
**click** | **i32** | Count of click events by recipients.  |
**click_tracking_delivered** | **i32** | Count of recipients of delivered messages with HTML content that contains tracked click URLs, where click tracking is enabled in the send request.  |
**end_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | The end of the time range for retrieving message engagement metrics (exclusive).  | [optional]
**open** | **i32** | Count of open events by recipients.  |
**open_tracking_delivered** | **i32** | Count of recipients of delivered messages with HTML content where open tracking was enabled in the send request.  |
**start_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | The beginning of the time range for retrieving message engagement metrics (inclusive).  | [optional]
**unique_click** | Option<**i32**> | Count of distinct messages that had at least one click event. Unlike `click`, each message is counted at most once regardless of how many links were clicked or how many times. Use this to compute click rates without exceeding 100%.  | [optional]
**unique_click_tracking_delivered** | Option<**i32**> | Count of distinct messages delivered with click tracking enabled (message-level, not recipient-level). Use as the denominator when computing unique click rates.  | [optional]
**unique_open** | Option<**i32**> | Count of distinct messages that had at least one open event. Unlike `open`, each message is counted at most once regardless of how many times its tracking pixel was fired. Use this to compute open rates without exceeding 100%.  | [optional]
**unique_open_tracking_delivered** | Option<**i32**> | Count of distinct messages delivered with open tracking enabled (message-level, not recipient-level). Use as the denominator when computing unique open rates.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
