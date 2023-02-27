# User

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**username** | Option<**String**> | The User's username. This is used for logging in, and may also be displayed alongside actions the User performs (for example, in Events or public StackScripts).  | [optional]
**email** | Option<**String**> | The email address for the User. Linode sends emails to this address for account management communications. May be used for other communications as configured.  | [optional]
**restricted** | Option<**bool**> | If true, the User must be granted access to perform actions or access entities on this Account. See User Grants View ([GET /account/users/{username}/grants](/docs/api/account/#users-grants-view)) for details on how to configure grants for a restricted User.  | [optional]
**ssh_keys** | Option<**Vec<String>**> | A list of SSH Key labels added by this User.  Users can add keys with the SSH Key Add ([POST /profile/sshkeys](/docs/api/profile/#ssh-key-add)) command.  These keys are deployed when this User is included in the `authorized_users` field of the following requests: - Linode Create ([POST /linode/instances](/docs/api/linode-instances/#linode-create)) - Linode Rebuild ([POST /linode/instances/{linodeId}/rebuild](/docs/api/linode-instances/#linode-rebuild)) - Disk Create ([POST /linode/instances/{linodeId}/disks](/docs/api/linode-instances/#disk-create))  | [optional][readonly]
**tfa_enabled** | Option<**bool**> | A boolean value indicating if the User has Two Factor Authentication (TFA) enabled. See the Create Two Factor Secret ([POST /profile/tfa-enable](/docs/api/profile/#two-factor-secret-create)) endpoint to enable TFA.  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


