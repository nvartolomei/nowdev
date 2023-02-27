# Image

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The unique ID of this Image. | [optional][readonly]
**label** | Option<**String**> | A short description of the Image.  | [optional]
**created** | Option<**String**> | When this Image was created. | [optional][readonly]
**updated** | Option<**String**> | When this Image was last updated. | [optional][readonly]
**created_by** | Option<**String**> | The name of the User who created this Image, or \"linode\" for public Images.  | [optional][readonly]
**deprecated** | Option<**bool**> | Whether or not this Image is deprecated. Will only be true for deprecated public Images.  | [optional][readonly]
**description** | Option<**String**> | A detailed description of this Image. | [optional]
**is_public** | Option<**bool**> | True if the Image is a public distribution image. False if Image is private Account-specific Image. | [optional][readonly]
**size** | Option<**i32**> | The minimum size this Image needs to deploy. Size is in MB.  | [optional][readonly]
**r#type** | Option<**String**> | How the Image was created.  \"Manual\" Images can be created at any time.  \"Automatic\" Images are created automatically from a deleted Linode.  | [optional][readonly]
**expiry** | Option<**String**> | Only Images created automatically from a deleted Linode (type=automatic) will expire.  | [optional][readonly]
**eol** | Option<**String**> | The date of the public Image's planned end of life. `None` for private Images.  | [optional][readonly]
**vendor** | Option<**String**> | The upstream distribution vendor. `None` for private Images.  | [optional][readonly]
**status** | Option<**String**> | The current status of this Image.  Only Images in an \"available\" status can be deployed. Images in a \"creating\" status are being created from a Linode Disk, and will become \"available\" shortly. Images in a \"pending_upload\" status are waiting for data to be [uploaded](/docs/api/images/#image-upload), and become \"available\" after the upload and processing are complete.  The \"+order_by\" and \"+order\" operators are not available for [filtering](/docs/api/#filtering-and-sorting) on this key.  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


