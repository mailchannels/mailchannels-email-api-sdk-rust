# CheckDomainBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dkim_settings** | Option<[**Vec<models::DkimSetting>**](DkimSetting.md)> | Each item may include DKIM domain, selector and private key. Up to 10 items are allowed. The absence or presence of these fields affects how DKIM settings are validated: 1. If dkim_domain, dkim_selector, and dkim_private_key are all present, verify using the provided domain, selector, and key. 2. If dkim_domain and dkim_selector are present, use the stored private key for the given domain and selector. 3. If only dkim_domain is present, use all stored keys for the given domain. 4. If none are present, use all stored keys for the domain provided in the `domain` field of the request. 5. If dkim_private_key is present, dkim_selector must be present. 6. If dkim_selector is present and dkim_domain is not, the domain will be taken from the `domain` field of the request.  | [optional]
**domain** | **String** | Domain used for sending emails. If dkim_settings are not provided, or dkim_settings are provided with no dkim_domain, the stored dkim settings for this domain will be used.  |
**envelope_from_domain** | Option<**String**> | Optional envelope-from domain. During message delivery, SPF and Domain Lockdown verification are evaluated  against the envelope sender domain. If your envelope-from domain differs from the domain used for sending messages, provide it here to ensure both checks are run against the correct domain. Otherwise, SPF or Domain Lockdown failures may cause message delivery to fail.  | [optional]
**sender_id** | Option<**String**> | Used exclusively for [Domain Lockdown](https://support.mailchannels.com/hc/en-us/articles/16918954360845-Secure-your-domain-name-against-spoofing-with-Domain-Lockdown) verification. If you're not using `senderid` to associate your domain with your account, you can disregard this field. The corresponding value is included in the X-MailChannels-SenderId header of emails sent via MailChannels.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
