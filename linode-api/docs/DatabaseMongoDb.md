# DatabaseMongoDb

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
**hosts** | Option<[**crate::models::DatabaseMongoDbHosts**](DatabaseMongoDB_hosts.md)> |  | [optional]
**peers** | Option<**Vec<String>**> | An array of peer addresses for this Database.  | [optional]
**replica_set** | Option<**String**> | Label for configuring a MongoDB [replica set](https://www.mongodb.com/docs/manual/replication/). Choose the same label on multiple Databases to include them in the same replica set.  If `null`, the Database is not included in any replica set.  | [optional]
**ssl_connection** | Option<**bool**> | Whether to require SSL credentials to establish a connection to the Managed Database.  Use the **Managed MongoDB Database Credentials View** ([GET /databases/mongodb/instances/{instanceId}/credentials](/docs/api/databases/#managed-mongodb-database-credentials-view)) command for access information.  | [optional][default to true]
**compression_type** | Option<**String**> | The type of data compression for this Database.  Snappy has a lower comparative compression ratio and resource consumption rate.  Zlip has a higher comparative compression ratio and resource consumption rate.  | [optional][default to None]
**storage_engine** | Option<**String**> | The type of storage engine for this Database.  **Note:** MMAPV1 is not available for MongoDB versions 4.0 and above.  | [optional][default to Wiredtiger]
**port** | Option<**i32**> | The access port for this Managed Database. | [optional]
**created** | Option<**String**> | When this Managed Database was created. | [optional][readonly]
**updated** | Option<**String**> | When this Managed Database was last updated. | [optional][readonly]
**updates** | Option<[**crate::models::DatabaseUpdates**](Database_updates.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


