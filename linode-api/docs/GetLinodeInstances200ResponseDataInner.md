# GetLinodeInstances200ResponseDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**label** | Option<**String**> | The Linode's label is for display purposes only. If no label is provided for a Linode, a default will be assigned.  Linode labels have the following constraints:    * Must begin and end with an alphanumeric character.   * May only consist of alphanumeric characters, dashes (`-`), underscores (`_`) or periods (`.`).   * Cannot have two dashes (`--`), underscores (`__`) or periods (`..`) in a row.  | [optional]
**region** | Option<**String**> | This is the [Region](/docs/api/regions/#regions-list) where the Linode was deployed. A Linode's region can only be changed by initiating a [cross data center migration](/docs/api/linode-instances/#dc-migrationpending-host-migration-initiate).  | [optional][readonly]
**image** | Option<**String**> | An Image ID to deploy the Linode Disk from.  Access the Images List ([GET /images](/docs/api/images/#images-list)) endpoint with authentication to view all available Images. Official Linode Images start with `linode/`, while your Account's Images start with `private/`. Creating a disk from a Private Image requires `read_only` or `read_write` permissions for that Image. Access the User's Grant Update ([PUT /account/users/{username}/grants](/docs/api/account/#users-grants-update)) endpoint to adjust permissions for an Account Image.  | [optional]
**r#type** | Option<**String**> | This is the [Linode Type](/docs/api/linode-types/#types-list) that this Linode was deployed with. To change a Linode's Type, use [POST /linode/instances/{linodeId}/resize](/docs/api/linode-instances/#linode-resize).  | [optional][readonly]
**group** | Option<**String**> | A deprecated property denoting a group label for this Linode.  | [optional]
**tags** | Option<**Vec<String>**> | An array of tags applied to this object.  Tags are for organizational purposes only.  | [optional]
**id** | Option<**i32**> | This Linode's ID which must be provided for all operations impacting this Linode.  | [optional][readonly]
**status** | Option<**String**> | A brief description of this Linode's current state. This field may change without direct action from you. For example, when a Linode goes into maintenance mode its status will display \"stopped\".  | [optional][readonly]
**hypervisor** | Option<**String**> | The virtualization software powering this Linode.  | [optional][readonly]
**created** | Option<**String**> | When this Linode was created. | [optional][readonly]
**updated** | Option<**String**> | When this Linode was last updated. | [optional][readonly]
**ipv4** | Option<**Vec<String>**> | This Linode's IPv4 Addresses. Each Linode is assigned a single public IPv4 address upon creation, and may get a single private IPv4 address if needed. You may need to [open a support ticket](/docs/api/support/#support-ticket-open) to get additional IPv4 addresses.  IPv4 addresses may be reassigned between your Linodes, or shared with other Linodes. See the [/networking](/docs/api/networking/) endpoints for details.  | [optional][readonly]
**ipv6** | Option<**String**> | This Linode's IPv6 SLAAC address. This address is specific to a Linode, and may not be shared. If the Linode has not been assigned an IPv6 address, the return value will be `null`.  | [optional][readonly]
**specs** | Option<[**crate::models::GetLinodeInstances200ResponseDataInnerSpecs**](getLinodeInstances_200_response_data_inner_specs.md)> |  | [optional]
**alerts** | Option<[**crate::models::GetLinodeInstances200ResponseDataInnerAlerts**](getLinodeInstances_200_response_data_inner_alerts.md)> |  | [optional]
**backups** | Option<[**crate::models::GetLinodeInstances200ResponseDataInnerBackups**](getLinodeInstances_200_response_data_inner_backups.md)> |  | [optional]
**watchdog_enabled** | Option<**bool**> | The watchdog, named Lassie, is a Shutdown Watchdog that monitors your Linode and will reboot it if it powers off unexpectedly. It works by issuing a boot job when your Linode powers off without a shutdown job being responsible. To prevent a loop, Lassie will give up if there have been more than 5 boot jobs issued within 15 minutes.  | [optional]
**host_uuid** | Option<**String**> | The Linode's host machine, as a UUID. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


