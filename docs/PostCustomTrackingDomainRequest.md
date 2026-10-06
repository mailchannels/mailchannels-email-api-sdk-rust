# PostCustomTrackingDomainRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**hostname** | **String** | The hostname to register as a custom tracking domain (e.g., click.example.com). The hostname must have a CNAME record pointing to `links.mailchannels.net`.  |
**name** | **String** | A unique label used to select this domain at message send time |
**scope** | **Scope** | The event type this domain handles (enum: click, open, unsubscribe) |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
