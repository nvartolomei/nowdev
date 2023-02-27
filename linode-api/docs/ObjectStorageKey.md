# ObjectStorageKey

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | This keypair's unique ID | [optional][readonly]
**label** | Option<**String**> | The label given to this key. For display purposes only. | [optional]
**access_key** | Option<**String**> | This keypair's access key. This is not secret. | [optional][readonly]
**secret_key** | Option<**String**> | This keypair's secret key. Only returned on key creation. | [optional][readonly]
**limited** | Option<**bool**> | Whether or not this key is a limited access key. Will return `false` if this key grants full access to all buckets on the user's account. | [optional][readonly]
**bucket_access** | Option<[**Vec<crate::models::ObjectStorageKeyBucketAccessInner>**](ObjectStorageKey_bucket_access_inner.md)> | Defines this key as a Limited Access Key. Limited Access Keys restrict this Object Storage key's access to only the bucket(s) declared in this array and define their bucket-level permissions.     Limited Access Keys can:    * [list all buckets](/docs/api/object-storage/#object-storage-buckets-list) available on this Account, but cannot perform any actions on a bucket unless it has access to the bucket.     * [create new buckets](/docs/api/object-storage/#object-storage-bucket-create), but do not have any access to the buckets it creates, unless explicitly given access to them.     **Note:** You can create an Object Storage Limited Access Key without access to any buckets.   This is achieved by sending a request with an empty `bucket_access` array.     **Note:** If this field is omitted, a regular unlimited access key is issued.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


