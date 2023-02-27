# DatabaseType

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID representing the Managed Database node plan type. | [optional][readonly]
**label** | Option<**String**> | A human-readable string that describes each plan type. For display purposes only. | [optional][readonly]
**engines** | Option<[**crate::models::DatabaseTypeEngines**](DatabaseType_engines.md)> |  | [optional]
**memory** | Option<**i32**> | The amount of RAM allocated to Database created of this plan type. The value is represented in megabytes. | [optional]
**disk** | Option<**i32**> | The amount of disk space set aside for Databases of this plan type. The value is represented in megabytes. | [optional]
**vcpus** | Option<**i32**> | The integer of number CPUs allocated to databases of this plan type. | [optional]
**deprecated** | Option<**bool**> | Whether this Database plan type has been deprecated and is no longer available. | [optional]
**class** | Option<**String**> | The compute class category. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


