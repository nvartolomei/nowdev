# LongviewClient

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | This Client's unique ID.  | [optional][readonly]
**label** | Option<**String**> | This Client's unique label. This is for display purposes only.  | [optional]
**api_key** | Option<**String**> | The API key for this Client, used when configuring the Longview Client application on your Linode.  Returns as `[REDACTED]` if you do not have read-write access to this client.  | [optional][readonly]
**install_code** | Option<**String**> | The install code for this Client, used when configuring the Longview Client application on your Linode.  Returns as `[REDACTED]` if you do not have read-write access to this client.  | [optional][readonly]
**apps** | Option<[**crate::models::LongviewClientApps**](LongviewClient_apps.md)> |  | [optional]
**created** | Option<**String**> | When this Longview Client was created.  | [optional][readonly]
**updated** | Option<**String**> | When this Longview Client was last updated.  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


