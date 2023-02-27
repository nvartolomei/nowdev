# GrantsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**global** | Option<[**crate::models::GrantsResponseGlobal**](GrantsResponse_global.md)> |  | [optional]
**linode** | Option<[**Vec<crate::models::GrantsResponseLinodeInner>**](GrantsResponse_linode_inner.md)> | The grants this User has for each Linode that is owned by this Account.  | [optional]
**database** | Option<[**Vec<crate::models::GrantsResponseLinodeInner>**](GrantsResponse_linode_inner.md)> | The grants this User has for each Database that is owned by this Account.  | [optional]
**domain** | Option<[**Vec<crate::models::GrantsResponseLinodeInner>**](GrantsResponse_linode_inner.md)> | The grants this User has for each Domain that is owned by this Account.  | [optional]
**nodebalancer** | Option<[**Vec<crate::models::GrantsResponseLinodeInner>**](GrantsResponse_linode_inner.md)> | The grants this User has for each NodeBalancer that is owned by this Account.  | [optional]
**image** | Option<[**Vec<crate::models::GrantsResponseLinodeInner>**](GrantsResponse_linode_inner.md)> | The grants this User has for each Image that is owned by this Account.  | [optional]
**longview** | Option<[**Vec<crate::models::GrantsResponseLinodeInner>**](GrantsResponse_linode_inner.md)> | The grants this User has for each Longview Client that is owned by this Account.  | [optional]
**stackscript** | Option<[**Vec<crate::models::GrantsResponseLinodeInner>**](GrantsResponse_linode_inner.md)> | The grants this User has for each StackScript that is owned by this Account.  | [optional]
**volume** | Option<[**Vec<crate::models::GrantsResponseLinodeInner>**](GrantsResponse_linode_inner.md)> | The grants this User has for each Block Storage Volume that is owned by this Account.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


