# ManagedIssue

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | This Issue's unique ID.  | [optional][readonly]
**created** | Option<**String**> | When this Issue was created. Issues are created in response to issues detected with Managed Services, so this is also when the Issue was detected.  | [optional][readonly]
**services** | Option<**Vec<i32>**> | An array of Managed Service IDs that were affected by this Issue.  | [optional][readonly]
**entity** | Option<[**crate::models::ManagedIssueEntity**](ManagedIssue_entity.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


