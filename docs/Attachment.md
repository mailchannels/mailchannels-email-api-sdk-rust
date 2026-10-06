# Attachment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**content** | **String** | the attachment data, encoded in base64 |
**content_id** | Option<**String**> | A unique identifier for this attachment. When set, the attachment is embedded inline in the message body (Content-Disposition: inline) instead of offered as a downloadable attachment, and can be referenced from HTML content via a `cid:` URI, e.g. `<img src=\"cid:logo123\">` refers to an attachment with `content_id: logo123` (RFC 2392). Must be unique across all attachments in the request.  | [optional]
**filename** | **String** | the name of the attachment file |
**r#type** | Option<**String**> | the MIME type of the attachment | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
