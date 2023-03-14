# \LinodeTypesApi

All URIs are relative to *https://api.linode.com/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_linode_type**](LinodeTypesApi.md#get_linode_type) | **GET** /linode/types/{typeId} | Type View
[**get_linode_types**](LinodeTypesApi.md#get_linode_types) | **GET** /linode/types | Types List



## get_linode_type

> crate::models::GetLinodeTypes200ResponseDataInner get_linode_type(type_id)
Type View

Returns information about a specific Linode Type, including pricing and specifications. This is used when [creating](/docs/api/linode-instances/#linode-create) or [resizing](/docs/api/linode-instances/#linode-resize) Linodes. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**type_id** | **String** | The ID of the Linode Type to look up. | [required] |

### Return type

[**crate::models::GetLinodeTypes200ResponseDataInner**](getLinodeTypes_200_response_data_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_linode_types

> crate::models::GetLinodeTypes200Response get_linode_types()
Types List

Returns collection of Linode Types, including pricing and specifications for each Type. These are used when [creating](/docs/api/linode-instances/#linode-create) or [resizing](/docs/api/linode-instances/#linode-resize) Linodes. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetLinodeTypes200Response**](getLinodeTypes_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

