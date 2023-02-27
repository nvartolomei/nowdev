# ObjectStorageObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of this object or prefix.  | [optional]
**etag** | Option<**String**> | An MD-5 hash of the object. `null` if this object represents a prefix.  | [optional]
**last_modified** | Option<**String**> | The date and time this object was last modified. `null` if this object represents a prefix.  | [optional]
**owner** | Option<**String**> | The owner of this object, as a UUID. `null` if this object represents a prefix.  | [optional]
**size** | Option<**i32**> | The size of this object, in bytes. `null` if this object represents a prefix.  | [optional]
**next_marker** | Option<**String**> | Returns the value you should pass to the `marker` query parameter to get the next page of objects. If there is no next page, `null` will be returned.  | [optional]
**is_truncated** | Option<**bool**> | Designates if there is another page of bucket objects. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


