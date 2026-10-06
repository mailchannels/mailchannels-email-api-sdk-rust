# MetricsVolume

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**buckets** | [**models::MetricsVolumeBuckets**](MetricsVolumeBuckets.md) |  |
**delivered** | **i32** | Count of messages delivered during the specified time range.  |
**dropped** | **i32** | Count of messages dropped during the specified time range.  |
**end_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | The end of the time range for retrieving message volume metrics (exclusive).  | [optional]
**processed** | **i32** | Count of messages processed during the specified time range.  |
**start_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | The beginning of the time range for retrieving message volume metrics (inclusive).  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
