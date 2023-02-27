# StackScript

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The unique ID of this StackScript. | [optional][readonly]
**username** | Option<**String**> | The User who created the StackScript.  | [optional][readonly]
**user_gravatar_id** | Option<**String**> | The Gravatar ID for the User who created the StackScript.  | [optional][readonly]
**label** | Option<**String**> | The StackScript's label is for display purposes only.  | [optional]
**description** | Option<**String**> | A description for the StackScript.  | [optional]
**images** | Option<**Vec<String>**> | An array of Image IDs. These are the Images that can be deployed with this StackScript.  `any/all` indicates that all available Images, including private Images, are accepted.  | [optional]
**deployments_total** | Option<**i32**> | The total number of times this StackScript has been deployed.  | [optional][readonly]
**deployments_active** | Option<**i32**> | Count of currently active, deployed Linodes created from this StackScript.  | [optional][readonly]
**is_public** | Option<**bool**> | This determines whether other users can use your StackScript. **Once a StackScript is made public, it cannot be made private.**  | [optional]
**created** | Option<**String**> | The date this StackScript was created.  | [optional][readonly]
**updated** | Option<**String**> | The date this StackScript was last updated.  | [optional][readonly]
**rev_note** | Option<**String**> | This field allows you to add notes for the set of revisions made to this StackScript.  | [optional]
**script** | Option<**String**> | The script to execute when provisioning a new Linode with this StackScript.  | [optional]
**user_defined_fields** | Option<[**Vec<crate::models::StackScriptUserDefinedFieldsInner>**](StackScript_user_defined_fields_inner.md)> | This is a list of fields defined with a special syntax inside this StackScript that allow for supplying customized parameters during deployment. See [Declare User-Defined Fields (UDFs)](/docs/products/tools/stackscripts/guides/write-a-custom-script/#declare-user-defined-fields-udfs) for more information.  | [optional][readonly]
**mine** | Option<**bool**> | Returns `true` if this StackScript is owned by the account of the user making the request, and the user making the request is unrestricted or has access to this StackScript.  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


