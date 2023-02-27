# AuthorizedApp

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | This authorization's ID, used for revoking access.  | [optional][readonly]
**label** | Option<**String**> | The name of the application you've authorized.  | [optional][readonly]
**thumbnail_url** | Option<**String**> | The URL at which this app's thumbnail may be accessed.  | [optional][readonly]
**scopes** | Option<**String**> | The OAuth scopes this app was authorized with.  This defines what parts of your Account the app is allowed to access.  | [optional][readonly]
**created** | Option<**String**> | When this app was authorized. | [optional][readonly]
**expiry** | Option<**String**> | When the app's access to your account expires. If `null`, the app's access must be revoked manually.  | [optional][readonly]
**website** | Option<**String**> | The website where you can get more information about this app.  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


