# TrustedDevice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The unique ID for this TrustedDevice | [optional][readonly]
**created** | Option<**String**> | When this Remember Me session was started.  This corresponds to the time of login with the \"Remember Me\" box checked.  | [optional][readonly]
**expiry** | Option<**String**> | When this TrustedDevice session expires.  Sessions typically last 30 days.  | [optional][readonly]
**user_agent** | Option<**String**> | The User Agent of the browser that created this TrustedDevice session.  | [optional][readonly]
**last_authenticated** | Option<**String**> | The last time this TrustedDevice was successfully used to authenticate to <a target=\"_top\" href=\"https://login.linode.com\">login.linode.com</a>.  | [optional][readonly]
**last_remote_addr** | Option<**String**> | The last IP Address to successfully authenticate with this TrustedDevice.  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


