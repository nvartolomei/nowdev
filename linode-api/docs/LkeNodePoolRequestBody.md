# LkeNodePoolRequestBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**autoscaler** | Option<[**crate::models::LkeNodePoolRequestBodyAutoscaler**](LKENodePoolRequestBody_autoscaler.md)> |  | [optional]
**r#type** | Option<**String**> | The Linode Type for all of the nodes in the Node Pool. | [optional]
**count** | Option<**i32**> | The number of nodes in the Node Pool. | [optional]
**disks** | Option<[**Vec<crate::models::LkeNodePoolRequestBodyDisksInner>**](LKENodePoolRequestBody_disks_inner.md)> | **Note**: This field should be omitted except for special use cases. The disks specified here are partitions in *addition* to the primary partition and reduce the size of the primary partition, which can lead to stability problems for the Node.  This Node Pool's custom disk layout. Each item in this array will create a new disk partition for each node in this Node Pool.    * The custom disk layout is applied to each node in this Node Pool.   * The maximum number of custom disk partitions that can be configured is 7.   * Once the requested disk paritions are allocated, the remaining disk space is allocated to the node's boot disk.   * A Node Pool's custom disk layout is immutable over the lifetime of the Node Pool.  | [optional]
**tags** | Option<**Vec<String>**> | An array of tags applied to this object. Tags are for organizational purposes only.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


