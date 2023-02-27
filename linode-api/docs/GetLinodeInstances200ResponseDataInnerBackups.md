# GetLinodeInstances200ResponseDataInnerBackups

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enabled** | Option<**bool**> | If this Linode has the Backup service enabled. To enable backups, see [POST /linode/instances/{linodeId}/backups/enable](/docs/api/linode-instances/#backups-enable).  | [optional][readonly]
**available** | Option<**bool**> | Whether Backups for this Linode are available for restoration.  Backups undergoing maintenance are not available for restoration.  | [optional][readonly]
**schedule** | Option<[**crate::models::GetLinodeInstances200ResponseDataInnerBackupsSchedule**](getLinodeInstances_200_response_data_inner_backups_schedule.md)> |  | [optional]
**last_successful** | Option<**String**> | The last successful backup date. 'null' if there was no previous backup. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


