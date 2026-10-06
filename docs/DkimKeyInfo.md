# DkimKeyInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**algorithm** | **String** | Algorithm used for the key pair  |
**created_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Timestamp when the key pair was created  | [optional]
**dkim_dns_records** | Option<[**Vec<models::DkimDnsRecord>**](DKIMDnsRecord.md)> | Suggested DNS records for the DKIM key  | [optional]
**domain** | **String** | Domain associated with the key pair  |
**grace_period_expires_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | UTC timestamp after which you can no longer use the rotated key for signing  | [optional]
**key_length** | Option<**i32**> | Key length in bits  | [optional]
**public_key** | **String** |  |
**retires_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | UTC timestamp when a rotated key pair is retired  | [optional]
**selector** | **String** | Selector assigned to the key pair  |
**status** | **Status** |  (enum: active, retired, revoked, rotated) |
**status_modified_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Timestamp when the key was last modified  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
