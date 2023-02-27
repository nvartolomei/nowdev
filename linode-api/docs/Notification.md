# Notification

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**entity** | Option<[**crate::models::NotificationEntity**](Notification_entity.md)> |  | [optional]
**label** | Option<**String**> | A short description of this Notification.  | [optional][readonly]
**message** | Option<**String**> | A human-readable description of the Notification. | [optional][readonly]
**body** | Option<**String**> | A full description of this Notification, in markdown format.  Not all Notifications include bodies.  | [optional][readonly]
**r#type** | Option<**String**> | The type of Notification this is. | [optional][readonly]
**severity** | Option<**String**> | The severity of this Notification.  This field can be used to decide how prominently to display the Notification, what color to make the display text, etc.  | [optional][readonly]
**when** | Option<**String**> | If this Notification is of an Event that will happen at a fixed, future time, this is when the named action will be taken. For example, if a Linode is to be migrated in response to a Security Advisory, this field will contain the approximate time the Linode will be taken offline for migration.  | [optional][readonly]
**until** | Option<**String**> | If this Notification has a duration, this will be the ending time for the Event/action. For example, if there is scheduled maintenance for one of our systems, `until` would be set to the end of the maintenance window.  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


