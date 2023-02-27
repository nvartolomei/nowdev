# MigrateLinodeInstanceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**region** | Option<**String**> | The region to which the Linode will be migrated. Must be a valid region slug. A list of regions can be viewed by using the [GET /regions](/docs/api/regions/#regions-list) endpoint. A cross data center migration will cancel a pending migration that has not yet been initiated. A cross data center migration will initiate a `linode_migrate_datacenter_create` event.  | [optional]
**upgrade** | Option<**bool**> | When initiating a cross DC migration, setting this value to true will also ensure that the Linode is upgraded to the latest generation of hardware that corresponds to your Linode's Type, if any free upgrades are available for it. If no free upgrades are available, and this value is set to true, then the endpoint will return a 400 error code and the migration will not be performed. If the data center set in the `region` field does not allow upgrades, then the endpoint will return a 400 error code and the migration will not be performed.  | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


