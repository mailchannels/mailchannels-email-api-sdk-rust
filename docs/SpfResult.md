# SpfResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**reason** | Option<**String**> | A human-readable explanation of SPF check.  | [optional]
**spf_record** | Option<**String**> | The SPF record that was used for the check.  | [optional]
**spf_record_error** | Option<**String**> | Error message if the SPF record lookup failed.  | [optional]
**verdict** | Option<**Verdict**> |  (enum: passed, failed, soft failed, temporary error, permanent error, neutral, none, unknown) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
