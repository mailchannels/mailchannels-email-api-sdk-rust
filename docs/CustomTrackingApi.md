# \CustomTrackingApi

All URIs are relative to *https://api.mailchannels.net/tx/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_custom_tracking_domain**](CustomTrackingApi.md#create_custom_tracking_domain) | **POST** /custom-tracking-domains | Register Custom Tracking Domain
[**delete_custom_tracking_domain**](CustomTrackingApi.md#delete_custom_tracking_domain) | **DELETE** /custom-tracking-domains/{hostname}/{scope} | Delete Custom Tracking Domain
[**list_custom_tracking_domains**](CustomTrackingApi.md#list_custom_tracking_domains) | **GET** /custom-tracking-domains | Retrieve Custom Tracking Domains
[**update_custom_tracking_domain**](CustomTrackingApi.md#update_custom_tracking_domain) | **PATCH** /custom-tracking-domains/{hostname}/{scope} | Update Custom Tracking Domain



## create_custom_tracking_domain

> models::CustomTrackingDomain create_custom_tracking_domain(x_api_key, post_custom_tracking_domain_request)
Register Custom Tracking Domain

Register a custom branded domain for click tracking, open tracking, or unsubscribe handling. By default, MailChannels uses shared domains for these links. Using a custom domain improves brand consistency by replacing shared domains with your own (e.g., click.example.com). Once registered, select the domain at send time using its `name`.  Before registration completes, two DNS records must be in place: 1. A TXT record at `_mailchannels-verify.<hostname>` containing the verification token    (returned in the 202 response). 2. A CNAME record at `<hostname>` pointing to `links.mailchannels.net`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_api_key** | **String** |  | [required] |
**post_custom_tracking_domain_request** | [**PostCustomTrackingDomainRequest**](PostCustomTrackingDomainRequest.md) |  | [required] |

### Return type

[**models::CustomTrackingDomain**](CustomTrackingDomain.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_custom_tracking_domain

> delete_custom_tracking_domain(hostname, scope, x_api_key)
Delete Custom Tracking Domain

Permanently delete an existing custom tracking domain for the given hostname and scope. The domain can be re-registered if needed. WARNING: Any tracking links or unsubscribe URLs in previously sent emails using this domain will stop working immediately.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hostname** | **String** |  | [required] |
**scope** | **String** |  | [required] |
**x_api_key** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_custom_tracking_domains

> models::CustomTrackingDomainListResponse list_custom_tracking_domains(x_api_key, name, status, scope, limit, offset)
Retrieve Custom Tracking Domains

Retrieve all custom tracking domains registered under your account. Optional filters include domain name, status, scope, limit and offset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_api_key** | **String** |  | [required] |
**name** | Option<**String**> | Filter by custom tracking domain label |  |
**status** | Option<**String**> | Filter by status |  |
**scope** | Option<**String**> | Filter by scope |  |
**limit** | Option<**i32**> | The maximum number of domains to return. The default is 100. |  |[default to 100]
**offset** | Option<**i32**> | The number of domains to skip before returning results. The default is 0. |  |[default to 0]

### Return type

[**models::CustomTrackingDomainListResponse**](CustomTrackingDomainListResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_custom_tracking_domain

> models::CustomTrackingDomain update_custom_tracking_domain(hostname, scope, x_api_key, patch_custom_tracking_domain_request)
Update Custom Tracking Domain

Update an existing custom tracking domain by its hostname and scope. Supports updating the custom tracking domain's name or toggling its active status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hostname** | **String** |  | [required] |
**scope** | **String** |  | [required] |
**x_api_key** | **String** |  | [required] |
**patch_custom_tracking_domain_request** | [**PatchCustomTrackingDomainRequest**](PatchCustomTrackingDomainRequest.md) |  | [required] |

### Return type

[**models::CustomTrackingDomain**](CustomTrackingDomain.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
