# SshKey

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The unique identifier of an SSH Key object.  | [optional][readonly]
**label** | Option<**String**> | A label for the SSH Key.  | [optional]
**ssh_key** | Option<**String**> | The public SSH Key, which is used to authenticate to the root user of the Linodes you deploy.  Accepted formats: * ssh-dss * ssh-rsa * ecdsa-sha2-nistp * ssh-ed25519 * sk-ecdsa-sha2-nistp256 (Akamai-specific)  | [optional]
**created** | Option<**String**> | The date this key was added.  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


