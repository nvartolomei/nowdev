# EventEntity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The unique ID for an Event's entity.   Some Event entities do not have IDs associated with them, so they will not be returned when filtering by ID. These Events include:   * `account`   * `profile`  Entities for some Events are assigned the ID of the Linode they correspond to. When filtering by ID for these Events, use the corresponding Linode's ID. These Events include:   * `disks`   * `backups`   Tag Events use a tag's name for the entity ID field. When filtering by ID for tag Events, supply the name of the tag.  | [optional]
**label** | Option<**String**> | The current label of this object. The label may reflect changes that occur with this Event.  | [optional]
**r#type** | Option<**String**> | The type of entity that is being referenced by the Event.  | [optional][readonly]
**url** | Option<**String**> | The URL where you can access the object this Event is for. If a relative URL, it is relative to the domain you retrieved the Event from.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


