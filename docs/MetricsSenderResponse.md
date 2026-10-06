# MetricsSenderResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**end_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**limit** | **i32** |  |
**offset** | **i32** |  |
**senders** | [**Vec<models::MetricsSender>**](MetricsSender.md) |  |
**start_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**total** | **i32** | The total number of senders in this category that sent messages in the given time range.  |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
