# Personalization

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bcc** | Option<[**Vec<models::EmailAddress>**](EmailAddress.md)> |  | [optional]
**cc** | Option<[**Vec<models::EmailAddress>**](EmailAddress.md)> |  | [optional]
**dkim_domain** | Option<**String**> | If set, you must also provide the matching dkim_selector.  | [optional]
**dkim_private_key** | Option<**String**> | Encoded in Base64. If set, you must also provide the matching dkim_domain and dkim_selector.  | [optional]
**dkim_selector** | Option<**String**> | If set without a matching dkim_domain, the domain will be taken from the `from` email address.  | [optional]
**dynamic_template_data** | Option<**serde_json::Value**> | A JSON object containing key-value pairs of variables to set for template rendering. Keys must be strings, and values can be one of the following types: * string * boolean * number * list, whose values are all of permitted types * map, whose keys must be strings, and whose values are all of permitted types  | [optional]
**envelope_from** | Option<[**models::EmailAddress**](EmailAddress.md)> |  | [optional]
**from** | Option<[**models::EmailAddress**](EmailAddress.md)> |  | [optional]
**headers** | Option<**std::collections::HashMap<String, String>**> | A JSON object containing key-value pairs, where both keys (header names) and values must be strings. These pairs represent custom headers to be substituted. Please note the following restrictions and behavior: - Reserved headers: The following headers cannot be modified:   - Authentication-Results   - BCC   - CC   - Content-Transfer-Encoding   - Content-Type   - DKIM-Signature   - From   - Message-ID   - Received   - Reply-To   - Subject   - To - Header precedence: If a header is defined in both the personalizations object and the root headers, the value from personalizations will be used. - Case sensitivity: Headers are treated as case-insensitive. If multiple headers differ only by case, only one will be used, with no guarantee of which one.  | [optional]
**reply_to** | Option<[**models::EmailAddress**](EmailAddress.md)> |  | [optional]
**subject** | Option<**String**> |  | [optional]
**to** | [**Vec<models::EmailAddress>**](EmailAddress.md) |  |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
