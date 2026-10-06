# SuppressionEntryResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**notes** | Option<**String**> |  | [optional]
**recipient** | **String** |  |
**sender** | Option<**String**> |  | [optional]
**source** | Option<**Source**> |  (enum: api, unsubscribe_link, list_unsubscribe, hard_bounce, spam_complaint) | [optional]
**suppression_types** | Option<**Vec<SuppressionTypes>**> |  (enum: transactional, non-transactional) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
