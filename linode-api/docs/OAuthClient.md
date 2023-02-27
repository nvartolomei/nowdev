# OAuthClient

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The OAuth Client ID.  This is used to identify the client, and is a publicly-known value (it is not a secret).  | [optional][readonly]
**redirect_uri** | Option<**String**> | The location a successful log in from <a target=\"_top\" href=\"https://login.linode.com\">https://login.linode.com</a> should be redirected to for this client.  The receiver of this redirect should be ready to accept an OAuth exchange code and finish the OAuth exchange.  | [optional]
**label** | Option<**String**> | The name of this application.  This will be presented to users when they are asked to grant it access to their Account.  | [optional]
**status** | Option<**String**> | The status of this application.  `active` by default.  | [optional][readonly]
**secret** | Option<**String**> | The OAuth Client secret, used in the OAuth exchange.  This is returned as `<REDACTED>` except when an OAuth Client is created or its secret is reset.  This is a secret, and should not be shared or disclosed publicly.  | [optional][readonly]
**thumbnail_url** | Option<**String**> | The URL where this client's thumbnail may be viewed, or `null` if this client does not have a thumbnail set.  | [optional][readonly]
**public** | Option<**bool**> | If this is a public or private OAuth Client.  Public clients have a slightly different authentication workflow than private clients.  See the <a target=\"_top\" href=\"https://oauth.net/2/\">OAuth spec</a> for more details.  | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


