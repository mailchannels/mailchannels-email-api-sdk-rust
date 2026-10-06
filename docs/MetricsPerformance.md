# MetricsPerformance

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bounced** | **i32** | Count of messages hard-bounced during the specified time range.  |
**buckets** | [**models::MetricsPerformanceBuckets**](MetricsPerformanceBuckets.md) |  |
**complained** | **i32** | Count of messages complained during the specified time range.  |
**delivered** | **i32** | Count of messages delivered during the specified time range.  |
**end_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | The end of the time range for retrieving message performance metrics (exclusive). | [optional]
**processed** | **i32** | Count of messages processed during the specified time range.  |
**start_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | The beginning of the time range for retrieving message performance metrics (inclusive).  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
