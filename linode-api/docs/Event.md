# Event

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The unique ID of this Event. | [optional][readonly]
**action** | Option<**String**> | The action that caused this Event. New actions may be added in the future.  | [optional][readonly]
**created** | Option<**String**> | When this Event was created. | [optional][readonly]
**duration** | Option<**f32**> | The total duration in seconds that it takes for the Event to complete.  | [optional][readonly]
**entity** | Option<[**crate::models::EventEntity**](Event_entity.md)> |  | [optional]
**secondary_entity** | Option<[**crate::models::EventSecondaryEntity**](Event_secondary_entity.md)> |  | [optional]
**percent_complete** | Option<**i32**> | A percentage estimating the amount of time remaining for an Event. Returns `null` for notification events.  | [optional][readonly]
**rate** | Option<**String**> | The rate of completion of the Event. Only some Events will return rate; for example, migration and resize Events.  | [optional][readonly]
**read** | Option<**bool**> | If this Event has been read. | [optional][readonly]
**seen** | Option<**bool**> | If this Event has been seen. | [optional][readonly]
**status** | Option<**String**> | The current status of this Event. | [optional][readonly]
**time_remaining** | Option<**String**> | The estimated time remaining until the completion of this Event. This value is only returned for some in-progress migration events. For all other in-progress events, the `percent_complete` attribute will indicate about how much more work is to be done.  | [optional][readonly]
**username** | Option<**String**> | The username of the User who caused the Event.  | [optional][readonly]
**message** | Option<**String**> | Provides additional information about the event. Additional information may include, but is not limited to, a more detailed representation of events which can help diagnose non-obvious failures.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


