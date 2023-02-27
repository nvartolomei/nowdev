# Maintenance

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> | The type of maintenance.  | [optional]
**status** | Option<**String**> | The maintenance status.  Maintenance progresses in the following sequence: pending, started, then completed.  | [optional]
**reason** | Option<**String**> | The reason maintenance is being performed.  | [optional]
**when** | Option<**String**> | When the maintenance will begin.  [Filterable](/docs/api/#filtering-and-sorting) with the following parameters:  * A single value in date-time string format (\"%Y-%m-%dT%H:%M:%S\"), which returns only matches to that value.  * A dictionary containing pairs of inequality operator string keys (\"+or\", \"+gt\", \"+gte\", \"+lt\", \"+lte\", or \"+neq\") and single date-time string format values (\"%Y-%m-%dT%H:%M:%S\"). \"+or\" accepts an array of values that may consist of single date-time strings or dictionaries of inequality operator pairs.  | [optional]
**entity** | Option<[**crate::models::MaintenanceEntity**](Maintenance_entity.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


