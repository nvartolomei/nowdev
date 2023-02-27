# CloneLinodeInstanceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**region** | Option<**String**> | This is the Region where the Linode will be deployed. To view all available Regions you can deploy to see [/regions](/docs/api/regions/#regions-list). * Region can only be provided and is required when cloning to a new Linode.  | [optional]
**r#type** | Option<**String**> | A Linode's Type determines what resources are available to it, including disk space, memory, and virtual cpus. The amounts available to a specific Linode are returned as `specs` on the Linode object.  To view all available Linode Types you can deploy with see [/linode/types](/docs/api/linode-types/#types-list).  * Type can only be provided and is required when cloning to a new Linode.  | [optional]
**linode_id** | Option<**i32**> | If an existing Linode is the target for the clone, the ID of that Linode. The existing Linode must have enough resources to accept the clone.  | [optional]
**label** | Option<**String**> | The label to assign this Linode when cloning to a new Linode. * Can only be provided when cloning to a new Linode. * Defaults to \"linode\".  | [optional]
**group** | Option<**String**> | A label used to group Linodes for display. Linodes are not required to have a group.  | [optional]
**backups_enabled** | Option<**bool**> | If this field is set to `true`, the created Linode will automatically be enrolled in the Linode Backup service. This will incur an additional charge. Pricing is included in the response from [/linodes/types](/docs/api/linode-types/#types-list).  * Can only be included when cloning to a new Linode.  | [optional]
**disks** | Option<**Vec<i32>**> | An array of disk IDs. * If the `disks` parameter **is not provided**, then **no extra disks will be cloned** from the source Linode. All disks associated with the configuration profiles specified by the `configs` parameter will still be cloned. * **If an empty array is provided** for the `disks` parameter, then **no extra disks will be cloned** from the source Linode. All disks associated with the configuration profiles specified by the `configs` parameter will still be cloned. * **If a non-empty array is provided** for the `disks` parameter, then **the disks specified in the array will be cloned** from the source Linode, in addition to any disks associated with the configuration profiles specified by the `configs` parameter.  | [optional]
**configs** | Option<**Vec<i32>**> | An array of configuration profile IDs. * If the `configs` parameter **is not provided**, then **all configuration profiles and their associated disks will be cloned** from the source Linode. Any disks specified by the `disks` parameter will also be cloned. * **If an empty array is provided** for the `configs` parameter, then **no configuration profiles (nor their associated disks) will be cloned** from the source Linode. Any disks specified by the `disks` parameter will still be cloned. * **If a non-empty array is provided** for the `configs` parameter, then **the configuration profiles specified in the array (and their associated disks) will be cloned** from the source Linode. Any disks specified by the `disks` parameter will also be cloned.  | [optional]
**private_ip** | Option<**bool**> | If true, the created Linode will have private networking enabled and assigned a private IPv4 address. * Can only be provided when cloning to a new Linode.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


