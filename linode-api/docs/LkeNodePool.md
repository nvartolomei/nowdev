# LkeNodePool

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**autoscaler** | Option<[**crate::models::LkeNodePoolAutoscaler**](LKENodePool_autoscaler.md)> |  | [optional]
**r#type** | Option<**String**> | The Linode Type for all of the nodes in the Node Pool. | [optional]
**count** | Option<**i32**> | The number of nodes in the Node Pool. | [optional]
**disks** | Option<[**Vec<crate::models::LkeNodePoolRequestBodyDisksInner>**](LKENodePoolRequestBody_disks_inner.md)> | This Node Pool's custom disk layout.  | [optional]
**id** | Option<**i32**> | This Node Pool's unique ID.  | [optional]
**nodes** | Option<[**Vec<crate::models::LkeNodePoolNodesInner>**](LKENodePool_nodes_inner.md)> | Status information for the Nodes which are members of this Node Pool. If a Linode has not been provisioned for a given Node slot, the instance_id will be returned as null.  | [optional]
**tags** | Option<**Vec<String>**> | An array of tags applied to this object. Tags are for organizational purposes only.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


