# GetBackups200ResponseAutomaticInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The unique ID of this Backup. | [optional][readonly]
**r#type** | Option<**String**> |  | [optional]
**status** | Option<**String**> | The current state of a specific Backup. | [optional][readonly]
**available** | Option<**bool**> | Whether this Backup is available for restoration.  Backups undergoing maintenance are not available for restoration.  | [optional][readonly]
**created** | Option<**String**> | The date the Backup was taken. | [optional][readonly]
**updated** | Option<**String**> | The date the Backup was most recently updated. | [optional][readonly]
**finished** | Option<**String**> | The date the Backup completed. | [optional][readonly]
**label** | Option<**String**> | A label for Backups that are of type `snapshot`. | [optional]
**configs** | Option<**Vec<String>**> | A list of the labels of the Configuration profiles that are part of the Backup.  | [optional][readonly]
**disks** | Option<[**Vec<crate::models::GetBackups200ResponseAutomaticInnerAllOfDisksInner>**](getBackups_200_response_automatic_inner_allOf_disks_inner.md)> | A list of the disks that are part of the Backup.  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


