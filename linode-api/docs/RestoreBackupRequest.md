# RestoreBackupRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**linode_id** | **i32** | The ID of the Linode to restore a Backup to.  | 
**overwrite** | Option<**bool**> | If True, deletes all Disks and Configs on the target Linode before restoring.  If False, and the Disk image size is larger than the available space on the Linode, an error message indicating insufficient space is returned.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


