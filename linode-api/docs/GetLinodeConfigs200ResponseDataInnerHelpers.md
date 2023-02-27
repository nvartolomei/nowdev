# GetLinodeConfigs200ResponseDataInnerHelpers

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**updatedb_disabled** | Option<**bool**> | Disables updatedb cron job to avoid disk thrashing. | [optional]
**distro** | Option<**bool**> | Helps maintain correct inittab/upstart console device. | [optional]
**modules_dep** | Option<**bool**> | Creates a modules dependency file for the Kernel you run. | [optional]
**network** | Option<**bool**> | Automatically configures static networking. | [optional]
**devtmpfs_automount** | Option<**bool**> | Populates the /dev directory early during boot without udev.  Defaults to false.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


