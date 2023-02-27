# DatabaseBackupSnapshot

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**label** | **String** | The label for the Database snapshot backup.  * Must include only ASCII letters or numbers. * Must be unique among other backup labels for this Database.  | 
**target** | Option<**String**> | The Database cluster target. If the Database is a high availability cluster, choosing `secondary` creates a snapshot backup of a replica node.  | [optional][default to Primary]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


