# PersonalAccessToken

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | This token's unique ID, which can be used to revoke it.  | [optional][readonly]
**scopes** | Option<**String**> | The scopes this token was created with. These define what parts of the Account the token can be used to access. Many command-line tools, such as the <a target=\"_top\" href=\"https://github.com/linode/linode-cli\">Linode CLI</a>, require tokens with access to `*`. Tokens with more restrictive scopes are generally more secure.  | [optional][readonly]
**created** | Option<**String**> | The date and time this token was created.  | [optional][readonly]
**label** | Option<**String**> | This token's label.  This is for display purposes only, but can be used to more easily track what you're using each token for.  | [optional]
**token** | Option<**String**> | The token used to access the API.  When the token is created, the full token is returned here.  Otherwise, only the first 16 characters are returned.  | [optional][readonly]
**expiry** | Option<**String**> | When this token will expire.  Personal Access Tokens cannot be renewed, so after this time the token will be completely unusable and a new token will need to be generated.  Tokens may be created with \"null\" as their expiry and will never expire unless revoked.  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


