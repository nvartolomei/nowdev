# Database

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | A unique ID that can be used to identify and reference the Managed Database. | [optional][readonly]
**label** | Option<**String**> | A unique, user-defined string referring to the Managed Database. | [optional]
**r#type** | Option<**String**> | The Linode Instance type used by the Managed Database for its nodes. | [optional]
**engine** | Option<**String**> | The Managed Database engine type. | [optional][readonly]
**version** | Option<**String**> | The Managed Database engine version. | [optional][readonly]
**region** | Option<**String**> | The [Region](/docs/api/regions/) ID for the Managed Database. | [optional]
**status** | Option<**String**> | The operating status of the Managed Database. | [optional][readonly]
**encrypted** | Option<**bool**> | Whether the Managed Databases is encrypted. | [optional][default to false]
**allow_list** | Option<**Vec<String>**> | A list of IP addresses that can access the Managed Database. Each item can be a single IP address or a range in CIDR format.  By default, this is an empty array (`[]`), which blocks all connections (both public and private) to the Managed Database.  If `0.0.0.0/0` is a value in this list, then all IP addresses can access the Managed Database.  | [optional]
**cluster_size** | Option<**i32**> | The number of Linode Instance nodes deployed to the Managed Database.  Choosing 3 nodes creates a high availability cluster consisting of 1 primary node and 2 replica nodes.  | [optional][default to Variant1]
**hosts** | Option<[**crate::models::DatabaseHosts**](Database_hosts.md)> |  | [optional]
**created** | Option<**String**> | When this Managed Database was created. | [optional][readonly]
**updated** | Option<**String**> | When this Managed Database was last updated. | [optional][readonly]
**updates** | Option<[**crate::models::DatabaseUpdates**](Database_updates.md)> |  | [optional]
**instance_uri** | Option<**String**> | Append this to `https://api.linode.com` to run commands for the Managed Database.  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


