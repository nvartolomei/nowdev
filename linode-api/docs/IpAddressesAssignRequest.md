# IpAddressesAssignRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**region** | **String** | The ID of the Region in which these assignments are to take place. All IPs and Linodes must exist in this Region.  | 
**assignments** | [**Vec<crate::models::IpAddressesAssignRequestAssignmentsInner>**](IPAddressesAssignRequest_assignments_inner.md) | The list of assignments to make. You must have read_write access to all IPs being assigned and all Linodes being assigned to in order for the assignments to succeed.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


