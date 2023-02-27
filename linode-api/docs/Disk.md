# Disk

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | This Disk's ID which must be provided for all operations impacting this Disk.  | [optional][readonly]
**label** | Option<**String**> | The Disk's label is for display purposes only.  | [optional]
**status** | Option<**String**> | A brief description of this Disk's current state. This field may change without direct action from you, as a result of operations performed to the Disk or the Linode containing the Disk.  | [optional][readonly]
**size** | Option<**i32**> | The size of the Disk in MB. | [optional]
**filesystem** | Option<**String**> | The Disk filesystem can be one of:    * raw - No filesystem, just a raw binary stream.   * swap - Linux swap area.   * ext3 - The ext3 journaling filesystem for Linux.   * ext4 - The ext4 journaling filesystem for Linux.   * initrd - initrd (uncompressed initrd, ext2, max 32 MB).  | [optional]
**created** | Option<**String**> | When this Disk was created. | [optional][readonly]
**updated** | Option<**String**> | When this Disk was last updated. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


