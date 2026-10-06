# \MetricsApi

All URIs are relative to *https://api.mailchannels.net/tx/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_engagement_metrics**](MetricsApi.md#get_engagement_metrics) | **GET** /metrics/engagement | Retrieve Engagement Metrics
[**get_performance_metrics**](MetricsApi.md#get_performance_metrics) | **GET** /metrics/performance | Retrieve Performance Metrics
[**get_recipient_behaviour_metrics**](MetricsApi.md#get_recipient_behaviour_metrics) | **GET** /metrics/recipient-behaviour | Retrieve Recipient Behaviour Metrics
[**get_sender_metrics**](MetricsApi.md#get_sender_metrics) | **GET** /metrics/senders/{sender_type} | Retrieve Sender Metrics
[**get_volume_metrics**](MetricsApi.md#get_volume_metrics) | **GET** /metrics/volume | Retrieve Volume Metrics



## get_engagement_metrics

> models::MetricsEngagement get_engagement_metrics(x_api_key, start_time, end_time, campaign_id, interval)
Retrieve Engagement Metrics

Retrieve engagement metrics for messages sent from your account, including counts of open and click events. Supports optional filters for time range, and campaign ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_api_key** | **String** |  | [required] |
**start_time** | Option<**String**> | The beginning of the time range for retrieving message engagement metrics (inclusive). Formats: YYYY-MM-DD or YYYY-MM-DDTHH:MM:SSZ. Defaults to one month ago if not provided.  |  |
**end_time** | Option<**String**> | The end of the time range for retrieving message engagement metrics (exclusive). Formats: YYYY-MM-DD or YYYY-MM-DDTHH:MM:SSZ. Defaults to the current time if not provided.  |  |
**campaign_id** | Option<**String**> | The ID of the campaign to filter metrics by. If not provided, metrics for all campaigns will be returned.  |  |
**interval** | Option<**String**> | The interval for aggregating metrics data. Allowed values:   - hour: Hourly breakdown   - day: Daily breakdown (default)   - week: Weekly breakdown   - month: Monthly breakdown  |  |[default to day]

### Return type

[**models::MetricsEngagement**](MetricsEngagement.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_performance_metrics

> models::MetricsPerformance get_performance_metrics(x_api_key, start_time, end_time, campaign_id, interval)
Retrieve Performance Metrics

Retrieve performance metrics for messages sent from your account, including counts of processed, delivered, hard-bounced, and complained events. Supports optional filters for time range, and campaign ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_api_key** | **String** |  | [required] |
**start_time** | Option<**String**> | The beginning of the time range for retrieving message performance metrics (inclusive). Formats: YYYY-MM-DD or YYYY-MM-DDTHH:MM:SSZ. Defaults to one month ago if not provided.  |  |
**end_time** | Option<**String**> | The end of the time range for retrieving message performance metrics (exclusive). Formats: YYYY-MM-DD or YYYY-MM-DDTHH:MM:SSZ. Defaults to the current time if not provided.  |  |
**campaign_id** | Option<**String**> | The ID of the campaign to filter metrics by. If not provided, metrics for all campaigns will be returned.  |  |
**interval** | Option<**String**> | The interval for aggregating metrics data. Allowed values:   - hour: Hourly breakdown   - day: Daily breakdown (default)   - week: Weekly breakdown   - month: Monthly breakdown  |  |[default to day]

### Return type

[**models::MetricsPerformance**](MetricsPerformance.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recipient_behaviour_metrics

> models::MetricsRecipientBehaviour get_recipient_behaviour_metrics(x_api_key, start_time, end_time, campaign_id, interval)
Retrieve Recipient Behaviour Metrics

Retrieve recipient behaviour metrics for messages sent from your account, including counts of unsubscribed events. Supports optional filters for time range, and campaign ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_api_key** | **String** |  | [required] |
**start_time** | Option<**String**> | The beginning of the time range for retrieving recipient behaviour metrics (inclusive). Formats: YYYY-MM-DD or YYYY-MM-DDTHH:MM:SSZ. Defaults to one month ago if not provided.  |  |
**end_time** | Option<**String**> | The end of the time range for retrieving recipient behaviour metrics (exclusive). Formats: YYYY-MM-DD or YYYY-MM-DDTHH:MM:SSZ. Defaults to the current time if not provided.  |  |
**campaign_id** | Option<**String**> | The ID of the campaign to filter metrics by. If not provided, metrics for all campaigns will be returned.  |  |
**interval** | Option<**String**> | The interval for aggregating metrics data. Allowed values:   - hour: Hourly breakdown   - day: Daily breakdown (default)   - week: Weekly breakdown   - month: Monthly breakdown  |  |[default to day]

### Return type

[**models::MetricsRecipientBehaviour**](MetricsRecipientBehaviour.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sender_metrics

> models::MetricsSenderResponse get_sender_metrics(sender_type, x_api_key, start_time, end_time, limit, offset, sort_order)
Retrieve Sender Metrics

Retrieves a list of senders, either sub-accounts or campaigns, with their associated message metrics. Sorted by total # of sent messages (processed + dropped) Supports optional filter for time range, and optional settings for limit, offset, and sort order. Note: senders without any messages in the given time range will not be included in the results. The default time range is from one month ago to now, and the default sort order is descending.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sender_type** | **String** |  | [required] |
**x_api_key** | **String** |  | [required] |
**start_time** | Option<**String**> | The beginning of the time range for retrieving top senders metrics (inclusive). Formats: YYYY-MM-DD or YYYY-MM-DDTHH:MM:SSZ Defaults to one month ago if not provided.  |  |
**end_time** | Option<**String**> | The end of the time range for retrieving top senders metrics (exclusive). Formats: YYYY-MM-DD or YYYY-MM-DDTHH:MM:SSZ Defaults to the current time if not provided.  |  |
**limit** | Option<**i32**> | The maximum number of senders to return The default is 10.  |  |[default to 10]
**offset** | Option<**i32**> | The number of senders to skip before returning results.  |  |[default to 0]
**sort_order** | Option<**String**> | The order in which to sort the results, based on total messages (processed + dropped).  |  |[default to desc]

### Return type

[**models::MetricsSenderResponse**](MetricsSenderResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_volume_metrics

> models::MetricsVolume get_volume_metrics(x_api_key, start_time, end_time, campaign_id, interval)
Retrieve Volume Metrics

Retrieve volume metrics for messages sent from your account, including counts of processed, delivered and dropped events. Supports optional filters for time range and campaign ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_api_key** | **String** |  | [required] |
**start_time** | Option<**String**> | The beginning of the time range for retrieving message volume metrics (inclusive). Formats: YYYY-MM-DD or YYYY-MM-DDTHH:MM:SSZ. Defaults to one month ago if not provided.  |  |
**end_time** | Option<**String**> | The end of the time range for retrieving message volume metrics (exclusive). Formats: YYYY-MM-DD or YYYY-MM-DDTHH:MM:SSZ. Defaults to the current time if not provided.  |  |
**campaign_id** | Option<**String**> | The ID of the campaign to filter metrics by. If not provided, metrics for all campaigns will be returned.  |  |
**interval** | Option<**String**> | The interval for aggregating metrics data. Allowed values:   - hour: Hourly breakdown   - day: Daily breakdown (default)   - week: Weekly breakdown   - month: Monthly breakdown  |  |[default to day]

### Return type

[**models::MetricsVolume**](MetricsVolume.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
