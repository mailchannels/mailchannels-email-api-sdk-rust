# DkimKeyPairCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**algorithm** | Option<**Algorithm**> | Algorithm used for the new key pair Currently, only RSA is supported.  (enum: rsa) | [optional][default to Rsa]
**key_length** | Option<**i32**> | Key length in bits. For RSA, must be a multiple of 1024. Common values: 1024 or 2048. Defaults to 2048 bits.  | [optional][default to 2048]
**selector** | **String** | Selector for the new key pair  |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
