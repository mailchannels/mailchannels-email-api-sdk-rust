# UsageStats

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**monthly_limit** | **i32** | The effective monthly limit for the current billing period. A limit of zero means the account cannot send any messages. For sub-accounts with no explicit limit set (i.e., -1), the monthly limit for the parent account is returned.  |
**period_end_date** | Option<**chrono::NaiveDate**> | The end date of the current billing period (ISO 8601 format). | [optional]
**period_start_date** | Option<**chrono::NaiveDate**> | The start date of the current billing period (ISO 8601 format). | [optional]
**total_usage** | **i64** | The total usage for the current billing period. |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
