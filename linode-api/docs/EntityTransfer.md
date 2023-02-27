# EntityTransfer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**token** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The token used to identify and accept or cancel this transfer.  | [optional]
**status** | Option<**String**> | The status of the transfer request.  `accepted`: The transfer has been accepted by another user and is currently in progress. Transfers can take up to 3 hours to complete.  `cancelled`: The transfer has been cancelled by the sender.  `completed`: The transfer has completed successfully.  `failed`: The transfer has failed after initiation.  `pending`: The transfer is ready to be accepted.  `stale`: The transfer has exceeded its expiration date. It can no longer be accepted or cancelled.  | [optional]
**created** | Option<**String**> | When this transfer was created.  | [optional]
**updated** | Option<**String**> | When this transfer was last updated.  | [optional]
**is_sender** | Option<**bool**> | If the requesting account created this transfer.  | [optional]
**expiry** | Option<**String**> | When this transfer expires. Transfers will automatically expire 24 hours after creation.  | [optional]
**entities** | Option<[**crate::models::EntityTransferEntities**](EntityTransfer_entities.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


