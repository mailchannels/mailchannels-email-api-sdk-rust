# SuppressionListInput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**add_to_sub_accounts** | Option<**bool**> | If true, the parent account creates suppression entries for all associated sub-accounts. This field is only applicable to parent accounts. Sub-accounts cannot create entries for other sub-accounts.  | [optional][default to false]
**suppression_entries** | [**Vec<models::SuppressionEntry>**](SuppressionEntry.md) | The total number of suppression entries to create, for the parent and/or its sub-accounts, must not exceed 1000. |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
