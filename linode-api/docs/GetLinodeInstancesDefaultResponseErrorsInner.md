# GetLinodeInstancesDefaultResponseErrorsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**reason** | Option<**String**> | What happened to cause this error. In most cases, this can be fixed immediately by changing the data you sent in the request, but in some cases you will be instructed to [open a Support Ticket](/docs/api/support/#support-ticket-open) or perform some other action before you can complete the request successfully.  | [optional]
**field** | Option<**String**> | The field in the request that caused this error. This may be a path, separated by periods in the case of nested fields. In some cases this may come back as \"null\" if the error is not specific to any single element of the request.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


