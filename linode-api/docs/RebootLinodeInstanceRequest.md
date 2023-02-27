# RebootLinodeInstanceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**config_id** | Option<**i32**> | The Linode Config ID to reboot into.  If null or omitted, the last booted config will be used.  If there was no last booted config and this Linode only has one config, it will be used.  If a config cannot be determined, an error will be returned.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


