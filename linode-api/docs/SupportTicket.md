# SupportTicket

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The ID of the Support Ticket.  | [optional][readonly]
**attachments** | Option<**Vec<String>**> | A list of filenames representing attached files associated with this Ticket.  | [optional][readonly]
**closed** | Option<**String**> | The date and time this Ticket was closed.  | [optional][readonly]
**closable** | Option<**bool**> | Whether the Support Ticket may be closed.  | [optional]
**description** | Option<**String**> | The full details of the issue or question.  | [optional][readonly]
**entity** | Option<[**crate::models::SupportTicketEntity**](SupportTicket_entity.md)> |  | [optional]
**gravatar_id** | Option<**String**> | The Gravatar ID of the User who opened this Ticket.  | [optional][readonly]
**opened** | Option<**String**> | The date and time this Ticket was created.  | [optional][readonly]
**opened_by** | Option<**String**> | The User who opened this Ticket.  | [optional][readonly]
**status** | Option<**String**> | The current status of this Ticket. | [optional][readonly]
**summary** | Option<**String**> | The summary or title for this Ticket.  | [optional][readonly]
**updated** | Option<**String**> | The date and time this Ticket was last updated.  | [optional][readonly]
**updated_by** | Option<**String**> | The User who last updated this Ticket.  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


