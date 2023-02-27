# DatabaseUpdates

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**frequency** | Option<**String**> | Whether maintenance occurs on a weekly or monthly basis. | [optional][default to Weekly]
**duration** | Option<**i32**> | The maximum maintenance window time in hours. | [optional]
**hour_of_day** | Option<**i32**> | The hour to begin maintenance based in UTC time. | [optional]
**day_of_week** | Option<**i32**> | The day to perform maintenance. 1=Monday, 2=Tuesday, etc. | [optional]
**week_of_month** | Option<**i32**> | The week of the month to perform `monthly` frequency updates. Defaults to `null`.  * Required for `monthly` frequency updates.  * Must be `null` for `weekly` frequency updates.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


