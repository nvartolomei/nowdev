# AddLinodeDiskRequestAllOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**size** | Option<**i32**> | The size of the Disk in MB.  Images require a minimum size. Access the Image View ([GET /images/{imageID}](/docs/api/images/#image-view)) endpoint to view its size.  | [optional]
**label** | Option<**String**> | The Disk's label is for display purposes only.  | [optional]
**filesystem** | Option<**String**> | The Disk filesystem can be one of:    * raw - No filesystem, just a raw binary stream.   * swap - Linux swap area.   * ext3 - The ext3 journaling filesystem for Linux.   * ext4 - The ext4 journaling filesystem for Linux.   * initrd - initrd (uncompressed initrd, ext2, max 32 MB).  | [optional]
**image** | Option<**String**> | An Image ID to deploy the Linode Disk from.  Access the Images List ([GET /images](/docs/api/images/#images-list)) endpoint with authentication to view all available Images. Official Linode Images start with `linode/`, while your Account's Images start with `private/`. Creating a disk from a Private Image requires `read_only` or `read_write` permissions for that Image. Access the User's Grant Update ([PUT /account/users/{username}/grants](/docs/api/account/#users-grants-update)) endpoint to adjust permissions for an Account Image.  | [optional]
**authorized_keys** | Option<**Vec<String>**> | A list of public SSH keys that will be automatically appended to the root user's `~/.ssh/authorized_keys` file when deploying from an Image.  | [optional]
**authorized_users** | Option<**Vec<String>**> | A list of usernames. If the usernames have associated SSH keys, the keys will be appended to the root users `~/.ssh/authorized_keys` file automatically when deploying from an Image.  | [optional]
**root_pass** | Option<**String**> | This sets the root user's password on a newly-created Linode Disk when deploying from an Image.  * **Required** when creating a Linode Disk from an Image, including when using a StackScript.  * Must meet a password strength score requirement that is calculated internally by the API. If the strength requirement is not met, you will receive a `Password does not meet strength requirement` error.  | [optional]
**stackscript_id** | Option<**i32**> | A StackScript ID that will cause the referenced StackScript to be run during deployment of this Linode. A compatible `image` is required to use a StackScript. To get a list of available StackScript and their permitted Images see [/stackscripts](/docs/api/stackscripts/#stackscripts-list). This field cannot be used when deploying from a Backup or a Private Image.  | [optional]
**stackscript_data** | Option<[**serde_json::Value**](.md)> | This field is required only if the StackScript being deployed requires input data from the User for successful completion. See [User Defined Fields (UDFs)](/docs/guides/writing-scripts-for-use-with-linode-stackscripts-a-tutorial/#user-defined-fields-udfs) for more details.  This field is required to be valid JSON.  Total length cannot exceed 65,535 characters.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


