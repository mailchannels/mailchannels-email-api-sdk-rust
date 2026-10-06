# DkimResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dkim_domain** | Option<**String**> |  | [optional]
**dkim_key_status** | Option<**String**> | The human readable status of the DKIM key used for verification. This field is only present if the DKIM check was performed using a DKIM key managed by MailChannels. If a DKIM key is present in the request, this field will not be included.  | [optional]
**dkim_selector** | Option<**String**> |  | [optional]
**reason** | Option<**String**> | A human-readable explanation of DKIM check. | [optional]
**verdict** | Option<**Verdict**> |  (enum: passed, failed) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
