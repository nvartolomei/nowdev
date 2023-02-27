# DatabaseMySqlRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**label** | **String** | A unique, user-defined string referring to the Managed Database. | 
**r#type** | **String** | The Linode Instance type used by the Managed Database for its nodes. | 
**engine** | **String** | The Managed Database engine in engine/version format. | 
**region** | **String** | The [Region](/docs/api/regions/) ID for the Managed Database. | 
**encrypted** | Option<**bool**> | Whether the Managed Databases is encrypted. | [optional][default to false]
**allow_list** | Option<**Vec<String>**> | A list of IP addresses that can access the Managed Database. Each item can be a single IP address or a range in CIDR format.  By default, this is an empty array (`[]`), which blocks all connections (both public and private) to the Managed Database.  If `0.0.0.0/0` is a value in this list, then all IP addresses can access the Managed Database.  | [optional]
**cluster_size** | Option<**i32**> | The number of Linode Instance nodes deployed to the Managed Database.  Choosing 3 nodes creates a high availability cluster consisting of 1 primary node and 2 replica nodes.  | [optional][default to Variant1]
**ssl_connection** | Option<**bool**> | Whether to require SSL credentials to establish a connection to the Managed Database.  Use the **Managed MySQL Database Credentials View** ([GET /databases/mysql/instances/{instanceId}/credentials](/docs/api/databases/#managed-mysql-database-credentials-view)) command for access information.  | [optional][default to true]
**replication_type** | Option<**String**> | The replication method used for the Managed Database.  Defaults to `none` for a single cluster and `semi_synch` for a high availability cluster.  Must be `none` for a single node cluster.  Must be `asynch` or `semi_synch` for a high availability cluster.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


