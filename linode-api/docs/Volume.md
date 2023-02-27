# Volume

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The unique ID of this Volume. | [optional][readonly]
**label** | Option<**String**> | The Volume's label is for display purposes only.  | [optional]
**filesystem_path** | Option<**String**> | The full filesystem path for the Volume based on the Volume's label. Path is /dev/disk/by-id/scsi-0Linode_Volume_ + Volume label.  | [optional][readonly]
**status** | Option<**String**> | The current status of the volume.  Can be one of:    * `creating` - the Volume is being created and is not yet available     for use.   * `active` - the Volume is online and available for use.   * `resizing` - the Volume is in the process of upgrading     its current capacity.  | [optional][readonly]
**size** | Option<**i32**> | The Volume's size, in GiB.  | [optional]
**region** | Option<**String**> | The unique ID of this Region. | [optional][readonly]
**linode_id** | Option<**i32**> | If a Volume is attached to a specific Linode, the ID of that Linode will be displayed here.  | [optional]
**linode_label** | Option<**String**> | If a Volume is attached to a specific Linode, the label of that Linode will be displayed here.  | [optional][readonly]
**created** | Option<**String**> | When this Volume was created. | [optional][readonly]
**updated** | Option<**String**> | When this Volume was last updated. | [optional][readonly]
**tags** | Option<**Vec<String>**> | An array of Tags applied to this object.  Tags are for organizational purposes only.  | [optional]
**hardware_type** | Option<**String**> | The storage type of this Volume. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


