# GetLinodeInstances200ResponseDataInnerBackupsSchedule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**day** | Option<**String**> | The day of the week that your Linode's weekly Backup is taken. If not set manually, a day will be chosen for you. Backups are taken every day, but backups taken on this day are preferred when selecting backups to retain for a longer period.   If not set manually, then when backups are initially enabled, this may come back as `Scheduling` until the `day` is automatically selected.  | [optional]
**window** | Option<**String**> | The window in which your backups will be taken, in UTC. A backups window is a two-hour span of time in which the backup may occur.   For example, `W10` indicates that your backups should be taken between 10:00 and 12:00. If you do not choose a backup window, one will be selected for you automatically.   If not set manually, when backups are initially enabled this may come back as `Scheduling` until the `window` is automatically selected.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


