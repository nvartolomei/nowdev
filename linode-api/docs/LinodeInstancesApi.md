# \LinodeInstancesApi

All URIs are relative to *https://api.linode.com/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_linode_config**](LinodeInstancesApi.md#add_linode_config) | **POST** /linode/instances/{linodeId}/configs | Configuration Profile Create
[**add_linode_disk**](LinodeInstancesApi.md#add_linode_disk) | **POST** /linode/instances/{linodeId}/disks | Disk Create
[**add_linode_ip**](LinodeInstancesApi.md#add_linode_ip) | **POST** /linode/instances/{linodeId}/ips | IPv4 Address Allocate
[**boot_linode_instance**](LinodeInstancesApi.md#boot_linode_instance) | **POST** /linode/instances/{linodeId}/boot | Linode Boot
[**cancel_backups**](LinodeInstancesApi.md#cancel_backups) | **POST** /linode/instances/{linodeId}/backups/cancel | Backups Cancel
[**clone_linode_disk**](LinodeInstancesApi.md#clone_linode_disk) | **POST** /linode/instances/{linodeId}/disks/{diskId}/clone | Disk Clone
[**clone_linode_instance**](LinodeInstancesApi.md#clone_linode_instance) | **POST** /linode/instances/{linodeId}/clone | Linode Clone
[**create_linode_instance**](LinodeInstancesApi.md#create_linode_instance) | **POST** /linode/instances | Linode Create
[**create_snapshot**](LinodeInstancesApi.md#create_snapshot) | **POST** /linode/instances/{linodeId}/backups | Snapshot Create
[**delete_disk**](LinodeInstancesApi.md#delete_disk) | **DELETE** /linode/instances/{linodeId}/disks/{diskId} | Disk Delete
[**delete_linode_config**](LinodeInstancesApi.md#delete_linode_config) | **DELETE** /linode/instances/{linodeId}/configs/{configId} | Configuration Profile Delete
[**delete_linode_instance**](LinodeInstancesApi.md#delete_linode_instance) | **DELETE** /linode/instances/{linodeId} | Linode Delete
[**enable_backups**](LinodeInstancesApi.md#enable_backups) | **POST** /linode/instances/{linodeId}/backups/enable | Backups Enable
[**get_backup**](LinodeInstancesApi.md#get_backup) | **GET** /linode/instances/{linodeId}/backups/{backupId} | Backup View
[**get_backups**](LinodeInstancesApi.md#get_backups) | **GET** /linode/instances/{linodeId}/backups | Backups List
[**get_linode_config**](LinodeInstancesApi.md#get_linode_config) | **GET** /linode/instances/{linodeId}/configs/{configId} | Configuration Profile View
[**get_linode_configs**](LinodeInstancesApi.md#get_linode_configs) | **GET** /linode/instances/{linodeId}/configs | Configuration Profiles List
[**get_linode_disk**](LinodeInstancesApi.md#get_linode_disk) | **GET** /linode/instances/{linodeId}/disks/{diskId} | Disk View
[**get_linode_disks**](LinodeInstancesApi.md#get_linode_disks) | **GET** /linode/instances/{linodeId}/disks | Disks List
[**get_linode_firewalls**](LinodeInstancesApi.md#get_linode_firewalls) | **GET** /linode/instances/{linodeId}/firewalls | Firewalls List
[**get_linode_instance**](LinodeInstancesApi.md#get_linode_instance) | **GET** /linode/instances/{linodeId} | Linode View
[**get_linode_instances**](LinodeInstancesApi.md#get_linode_instances) | **GET** /linode/instances | Linodes List
[**get_linode_ip**](LinodeInstancesApi.md#get_linode_ip) | **GET** /linode/instances/{linodeId}/ips/{address} | IP Address View
[**get_linode_ips**](LinodeInstancesApi.md#get_linode_ips) | **GET** /linode/instances/{linodeId}/ips | Networking Information List
[**get_linode_node_balancers**](LinodeInstancesApi.md#get_linode_node_balancers) | **GET** /linode/instances/{linodeId}/nodebalancers | Linode NodeBalancers View
[**get_linode_stats**](LinodeInstancesApi.md#get_linode_stats) | **GET** /linode/instances/{linodeId}/stats | Linode Statistics View
[**get_linode_stats_by_year_month**](LinodeInstancesApi.md#get_linode_stats_by_year_month) | **GET** /linode/instances/{linodeId}/stats/{year}/{month} | Statistics View (year/month)
[**get_linode_transfer**](LinodeInstancesApi.md#get_linode_transfer) | **GET** /linode/instances/{linodeId}/transfer | Network Transfer View
[**get_linode_transfer_by_year_month**](LinodeInstancesApi.md#get_linode_transfer_by_year_month) | **GET** /linode/instances/{linodeId}/transfer/{year}/{month} | Network Transfer View (year/month)
[**get_linode_volumes**](LinodeInstancesApi.md#get_linode_volumes) | **GET** /linode/instances/{linodeId}/volumes | Linode's Volumes List
[**migrate_linode_instance**](LinodeInstancesApi.md#migrate_linode_instance) | **POST** /linode/instances/{linodeId}/migrate | DC Migration/Pending Host Migration Initiate
[**mutate_linode_instance**](LinodeInstancesApi.md#mutate_linode_instance) | **POST** /linode/instances/{linodeId}/mutate | Linode Upgrade
[**reboot_linode_instance**](LinodeInstancesApi.md#reboot_linode_instance) | **POST** /linode/instances/{linodeId}/reboot | Linode Reboot
[**rebuild_linode_instance**](LinodeInstancesApi.md#rebuild_linode_instance) | **POST** /linode/instances/{linodeId}/rebuild | Linode Rebuild
[**remove_linode_ip**](LinodeInstancesApi.md#remove_linode_ip) | **DELETE** /linode/instances/{linodeId}/ips/{address} | IPv4 Address Delete
[**rescue_linode_instance**](LinodeInstancesApi.md#rescue_linode_instance) | **POST** /linode/instances/{linodeId}/rescue | Linode Boot into Rescue Mode
[**reset_disk_password**](LinodeInstancesApi.md#reset_disk_password) | **POST** /linode/instances/{linodeId}/disks/{diskId}/password | Disk Root Password Reset
[**reset_linode_password**](LinodeInstancesApi.md#reset_linode_password) | **POST** /linode/instances/{linodeId}/password | Linode Root Password Reset
[**resize_disk**](LinodeInstancesApi.md#resize_disk) | **POST** /linode/instances/{linodeId}/disks/{diskId}/resize | Disk Resize
[**resize_linode_instance**](LinodeInstancesApi.md#resize_linode_instance) | **POST** /linode/instances/{linodeId}/resize | Linode Resize
[**restore_backup**](LinodeInstancesApi.md#restore_backup) | **POST** /linode/instances/{linodeId}/backups/{backupId}/restore | Backup Restore
[**shutdown_linode_instance**](LinodeInstancesApi.md#shutdown_linode_instance) | **POST** /linode/instances/{linodeId}/shutdown | Linode Shut Down
[**update_disk**](LinodeInstancesApi.md#update_disk) | **PUT** /linode/instances/{linodeId}/disks/{diskId} | Disk Update
[**update_linode_config**](LinodeInstancesApi.md#update_linode_config) | **PUT** /linode/instances/{linodeId}/configs/{configId} | Configuration Profile Update
[**update_linode_instance**](LinodeInstancesApi.md#update_linode_instance) | **PUT** /linode/instances/{linodeId} | Linode Update
[**update_linode_ip**](LinodeInstancesApi.md#update_linode_ip) | **PUT** /linode/instances/{linodeId}/ips/{address} | IP Address Update



## add_linode_config

> crate::models::GetLinodeConfigs200ResponseDataInner add_linode_config(linode_id, add_linode_config_request)
Configuration Profile Create

Adds a new Configuration profile to a Linode. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | ID of the Linode to look up Configuration profiles for. | [required] |
**add_linode_config_request** | [**AddLinodeConfigRequest**](AddLinodeConfigRequest.md) | The parameters to set when creating the Configuration profile. This determines which kernel, devices, how much memory, etc. a Linode boots with.  | [required] |

### Return type

[**crate::models::GetLinodeConfigs200ResponseDataInner**](getLinodeConfigs_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_linode_disk

> crate::models::GetLinodeDisks200ResponseDataInner add_linode_disk(linode_id, add_linode_disk_request)
Disk Create

Adds a new Disk to a Linode.  * You can optionally create a Disk from an Image or an Empty Disk if no Image is provided with a request.  * When creating an Empty Disk, providing a `label` is required.  * If no `label` is provided, an `image` is required instead.  * When creating a Disk from an Image, `root_pass` is required.  * The default filesystem for new Disks is `ext4`. If creating a Disk from an Image, the filesystem of the Image is used unless otherwise specified.  * When deploying a StackScript on a Disk:   * See StackScripts List ([GET /linode/stackscripts](/docs/api/stackscripts/#stackscripts-list)) for     a list of available StackScripts.   * Requires a compatible Image to be supplied.     * See StackScript View ([GET /linode/stackscript/{stackscriptId}](/docs/api/stackscripts/#stackscript-view)) for compatible Images.   * It is recommended to supply SSH keys for the root User using the `authorized_keys` field.   * You may also supply a list of usernames via the `authorized_users` field.     * These users must have an SSH Key associated with their Profiles first. See SSH Key Add ([POST /profile/sshkeys](/docs/api/profile/#ssh-key-add)) for more information. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | ID of the Linode to look up. | [required] |
**add_linode_disk_request** | [**AddLinodeDiskRequest**](AddLinodeDiskRequest.md) | The parameters to set when creating the Disk.  | [required] |

### Return type

[**crate::models::GetLinodeDisks200ResponseDataInner**](getLinodeDisks_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_linode_ip

> crate::models::GetLinodeIps200ResponseIpv4PublicInner add_linode_ip(linode_id, add_linode_ip_request)
IPv4 Address Allocate

Allocates a public or private IPv4 address to a Linode. Public IP Addresses, after the one included with each Linode, incur an additional monthly charge. If you need an additional public IP Address you must request one - please [open a support ticket](/docs/api/support/#support-ticket-open). You may not add more than one private IPv4 address to a single Linode. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | ID of the Linode to look up. | [required] |
**add_linode_ip_request** | [**AddLinodeIpRequest**](AddLinodeIpRequest.md) | Information about the address you are creating. | [required] |

### Return type

[**crate::models::GetLinodeIps200ResponseIpv4PublicInner**](getLinodeIPs_200_response_ipv4_public_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## boot_linode_instance

> serde_json::Value boot_linode_instance(linode_id, boot_linode_instance_request)
Linode Boot

Boots a Linode you have permission to modify. If no parameters are given, a Config profile will be chosen for this boot based on the following criteria:  * If there is only one Config profile for this Linode, it will be used. * If there is more than one Config profile, the last booted config will be used. * If there is more than one Config profile and none were the last to be booted (because the   Linode was never booted or the last booted config was deleted) an error will be returned. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | The ID of the Linode to boot. | [required] |
**boot_linode_instance_request** | Option<[**BootLinodeInstanceRequest**](BootLinodeInstanceRequest.md)> | Optional configuration to boot into (see above). |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_backups

> serde_json::Value cancel_backups(linode_id)
Backups Cancel

Cancels the Backup service on the given Linode. Deletes all of this Linode's existing backups forever. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | The ID of the Linode to cancel backup service for. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clone_linode_disk

> crate::models::GetLinodeDisks200ResponseDataInner clone_linode_disk(linode_id, disk_id)
Disk Clone

Copies a disk, byte-for-byte, into a new Disk belonging to the same Linode. The Linode must have enough storage space available to accept a new Disk of the same size as this one or this operation will fail. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | ID of the Linode to look up. | [required] |
**disk_id** | **i32** | ID of the Disk to clone. | [required] |

### Return type

[**crate::models::GetLinodeDisks200ResponseDataInner**](getLinodeDisks_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clone_linode_instance

> crate::models::GetLinodeInstances200ResponseDataInner clone_linode_instance(linode_id, clone_linode_instance_request)
Linode Clone

You can clone your Linode's existing Disks or Configuration profiles to another Linode on your Account. In order for this request to complete successfully, your User must have the `add_linodes` grant. Cloning to a new Linode will incur a charge on your Account.  If cloning to an existing Linode, any actions currently running or queued must be completed first before you can clone to it.  Up to five clone operations from any given source Linode can be run concurrently. If more concurrent clones are attempted, an HTTP 400 error will be returned by this endpoint.  Any [tags](/docs/api/tags/#tags-list) existing on the source Linode will be cloned to the target Linode. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | ID of the Linode to clone. | [required] |
**clone_linode_instance_request** | [**CloneLinodeInstanceRequest**](CloneLinodeInstanceRequest.md) | The requested state your Linode will be cloned into. | [required] |

### Return type

[**crate::models::GetLinodeInstances200ResponseDataInner**](getLinodeInstances_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_linode_instance

> crate::models::GetLinodeInstances200ResponseDataInner create_linode_instance(create_linode_instance_request)
Linode Create

Creates a Linode Instance on your Account. In order for this request to complete successfully, your User must have the `add_linodes` grant. Creating a new Linode will incur a charge on your Account.  Linodes can be created using one of the available Types. See Types List ([GET /linode/types](/docs/api/linode-types/#types-list)) to get more information about each Type's specs and cost.  Linodes can be created in any one of our available Regions, which are accessible from the Regions List ([GET /regions](/docs/api/regions/#regions-list)) endpoint.  In an effort to fight spam, Linode restricts outbound connections on ports 25, 465, and 587 on all Linodes for new accounts created after November 5th, 2019. For more information, see [Sending Email on Linode](/docs/guides/running-a-mail-server/#sending-email-on-linode).  Linodes can be created in a number of ways:  * Using a Linode Public Image distribution or a Private Image you created based on another Linode.   * Access the Images List ([GET /images](/docs/api/images/#images-list)) endpoint with authentication to view     all available Images.   * The Linode will be `running` after it completes `provisioning`.   * A default config with two Disks, one being a 512 swap disk, is created.     * `swap_size` can be used to customize the swap disk size.   * Requires a `root_pass` be supplied to use for the root User's Account.   * It is recommended to supply SSH keys for the root User using the `authorized_keys` field.   * You may also supply a list of usernames via the `authorized_users` field.     * These users must have an SSH Key associated with your Profile first. See SSH Key Add ([POST /profile/sshkeys](/docs/api/profile/#ssh-key-add)) for more information.  * Using a StackScript.   * See StackScripts List ([GET /linode/stackscripts](/docs/api/stackscripts/#stackscripts-list)) for     a list of available StackScripts.   * The Linode will be `running` after it completes `provisioning`.   * Requires a compatible Image to be supplied.     * See StackScript View ([GET /linode/stackscript/{stackscriptId}](/docs/api/stackscripts/#stackscript-view)) for compatible Images.   * Requires a `root_pass` be supplied to use for the root User's Account.   * It is recommended to supply SSH keys for the root User using the `authorized_keys` field.   * You may also supply a list of usernames via the `authorized_users` field.     * These users must have an SSH Key associated with your Profile first. See SSH Key Add ([POST /profile/sshkeys](/docs/api/profile/#ssh-key-add)) for more information.  * Using one of your other Linode's backups.   * You must create a Linode large enough to accommodate the Backup's size.   * The Disks and Config will match that of the Linode that was backed up.   * The `root_pass` will match that of the Linode that was backed up.  * Attached to a private VLAN.   * Review the `interfaces` property of the [Request Body Schema](/docs/api/linode-instances/#linode-create__request-body-schema) for details.   * For more information, see our guide on [Getting Started with VLANs](/docs/products/networking/vlans/get-started/).  * Create an empty Linode.   * The Linode will remain `offline` and must be manually started.     * See Linode Boot ([POST /linode/instances/{linodeId}/boot](/docs/api/linode-instances/#linode-boot)).   * Disks and Configs must be created manually.   * This is only recommended for advanced use cases.  **Important**: You must be an unrestricted User in order to add or modify tags on Linodes. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_linode_instance_request** | [**CreateLinodeInstanceRequest**](CreateLinodeInstanceRequest.md) | The requested initial state of a new Linode. | [required] |

### Return type

[**crate::models::GetLinodeInstances200ResponseDataInner**](getLinodeInstances_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_snapshot

> crate::models::GetBackups200ResponseAutomaticInnerAllOf create_snapshot(linode_id, create_snapshot_request)
Snapshot Create

Creates a snapshot Backup of a Linode.  **Important:** If you already have a snapshot of this Linode, this is a destructive action. The previous snapshot will be deleted. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | The ID of the Linode the backups belong to. | [required] |
**create_snapshot_request** | [**CreateSnapshotRequest**](CreateSnapshotRequest.md) |  | [required] |

### Return type

[**crate::models::GetBackups200ResponseAutomaticInnerAllOf**](getBackups_200_response_automatic_inner_allOf.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_disk

> serde_json::Value delete_disk(linode_id, disk_id)
Disk Delete

Deletes a Disk you have permission to `read_write`.  **Deleting a Disk is a destructive action and cannot be undone.** 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | ID of the Linode to look up. | [required] |
**disk_id** | **i32** | ID of the Disk to look up. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_linode_config

> serde_json::Value delete_linode_config(linode_id, config_id)
Configuration Profile Delete

Deletes the specified Configuration profile from the specified Linode. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | The ID of the Linode whose Configuration profile to look up. | [required] |
**config_id** | **i32** | The ID of the Configuration profile to look up. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_linode_instance

> serde_json::Value delete_linode_instance(linode_id)
Linode Delete

Deletes a Linode you have permission to `read_write`.  **Deleting a Linode is a destructive action and cannot be undone.**  Additionally, deleting a Linode:    * Gives up any IP addresses the Linode was assigned.   * Deletes all Disks, Backups, Configs, etc.   * Stops billing for the Linode and its associated services. You will be billed for time used     within the billing period the Linode was active.  Linodes that are in the process of [cloning](/docs/api/linode-instances/#linode-clone) or [backup restoration](/docs/api/linode-instances/#backup-restore) cannot be deleted. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | ID of the Linode to look up | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_backups

> serde_json::Value enable_backups(linode_id)
Backups Enable

Enables backups for the specified Linode. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | The ID of the Linode to enable backup service for. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_backup

> crate::models::GetBackups200ResponseAutomaticInnerAllOf get_backup(linode_id, backup_id)
Backup View

Returns information about a Backup. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | The ID of the Linode the Backup belongs to. | [required] |
**backup_id** | **i32** | The ID of the Backup to look up. | [required] |

### Return type

[**crate::models::GetBackups200ResponseAutomaticInnerAllOf**](getBackups_200_response_automatic_inner_allOf.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_backups

> crate::models::GetBackups200Response get_backups(linode_id)
Backups List

Returns information about this Linode's available backups. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | The ID of the Linode the backups belong to. | [required] |

### Return type

[**crate::models::GetBackups200Response**](getBackups_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_linode_config

> crate::models::GetLinodeConfigs200ResponseDataInner get_linode_config(linode_id, config_id)
Configuration Profile View

Returns information about a specific Configuration profile. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | The ID of the Linode whose Configuration profile to look up. | [required] |
**config_id** | **i32** | The ID of the Configuration profile to look up. | [required] |

### Return type

[**crate::models::GetLinodeConfigs200ResponseDataInner**](getLinodeConfigs_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_linode_configs

> crate::models::GetLinodeConfigs200Response get_linode_configs(linode_id, page, page_size)
Configuration Profiles List

Lists Configuration profiles associated with a Linode. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | ID of the Linode to look up Configuration profiles for. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**crate::models::GetLinodeConfigs200Response**](getLinodeConfigs_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_linode_disk

> crate::models::GetLinodeDisks200ResponseDataInner get_linode_disk(linode_id, disk_id)
Disk View

View Disk information for a Disk associated with this Linode. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | ID of the Linode to look up. | [required] |
**disk_id** | **i32** | ID of the Disk to look up. | [required] |

### Return type

[**crate::models::GetLinodeDisks200ResponseDataInner**](getLinodeDisks_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_linode_disks

> crate::models::GetLinodeDisks200Response get_linode_disks(linode_id, page, page_size)
Disks List

View Disk information for Disks associated with this Linode. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | ID of the Linode to look up. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**crate::models::GetLinodeDisks200Response**](getLinodeDisks_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_linode_firewalls

> crate::models::GetLinodeFirewalls200Response get_linode_firewalls(linode_id, page, page_size)
Firewalls List

View Firewall information for Firewalls associated with this Linode. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | ID of the Linode to look up. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**crate::models::GetLinodeFirewalls200Response**](getLinodeFirewalls_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_linode_instance

> crate::models::GetLinodeInstances200ResponseDataInner get_linode_instance(linode_id)
Linode View

Get a specific Linode by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | ID of the Linode to look up | [required] |

### Return type

[**crate::models::GetLinodeInstances200ResponseDataInner**](getLinodeInstances_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_linode_instances

> crate::models::GetLinodeInstances200Response get_linode_instances(page, page_size)
Linodes List

Returns a paginated list of Linodes you have permission to view. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**crate::models::GetLinodeInstances200Response**](getLinodeInstances_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_linode_ip

> crate::models::GetLinodeIps200ResponseIpv4PublicInner get_linode_ip(linode_id, address)
IP Address View

View information about the specified IP address associated with the specified Linode. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | The ID of the Linode to look up. | [required] |
**address** | **String** | The IP address to look up. | [required] |

### Return type

[**crate::models::GetLinodeIps200ResponseIpv4PublicInner**](getLinodeIPs_200_response_ipv4_public_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_linode_ips

> crate::models::GetLinodeIps200Response get_linode_ips(linode_id)
Networking Information List

Returns networking information for a single Linode. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | ID of the Linode to look up. | [required] |

### Return type

[**crate::models::GetLinodeIps200Response**](getLinodeIPs_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_linode_node_balancers

> crate::models::GetLinodeNodeBalancers200Response get_linode_node_balancers(linode_id)
Linode NodeBalancers View

Returns a list of NodeBalancers that are assigned to this Linode and readable by the requesting User.  Read permission to a NodeBalancer can be given to a User by accessing the User's Grants Update ([PUT /account/users/{username}/grants](/docs/api/account/#users-grants-update)) endpoint. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | ID of the Linode to look up | [required] |

### Return type

[**crate::models::GetLinodeNodeBalancers200Response**](getLinodeNodeBalancers_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_linode_stats

> crate::models::GetLinodeStats200Response get_linode_stats(linode_id)
Linode Statistics View

Returns CPU, IO, IPv4, and IPv6 statistics for your Linode for the past 24 hours. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | ID of the Linode to look up. | [required] |

### Return type

[**crate::models::GetLinodeStats200Response**](getLinodeStats_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_linode_stats_by_year_month

> crate::models::GetLinodeStats200Response get_linode_stats_by_year_month(linode_id, year, month)
Statistics View (year/month)

Returns statistics for a specific month. The year/month values must be either a date in the past, or the current month. If the current month, statistics will be retrieved for the past 30 days. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | ID of the Linode to look up. | [required] |
**year** | **i32** | Numeric value representing the year to look up. | [required] |
**month** | **i32** | Numeric value representing the month to look up. | [required] |

### Return type

[**crate::models::GetLinodeStats200Response**](getLinodeStats_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_linode_transfer

> crate::models::GetLinodeTransfer200Response get_linode_transfer(linode_id)
Network Transfer View

Returns a Linode's network transfer pool statistics for the current month. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | ID of the Linode to look up. | [required] |

### Return type

[**crate::models::GetLinodeTransfer200Response**](getLinodeTransfer_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_linode_transfer_by_year_month

> crate::models::GetLinodeTransferByYearMonth200Response get_linode_transfer_by_year_month(linode_id, year, month)
Network Transfer View (year/month)

Returns a Linode's network transfer statistics for a specific month. The year/month values must be either a date in the past, or the current month. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | ID of the Linode to look up. | [required] |
**year** | **i32** | Numeric value representing the year to look up. | [required] |
**month** | **i32** | Numeric value representing the month to look up. | [required] |

### Return type

[**crate::models::GetLinodeTransferByYearMonth200Response**](getLinodeTransferByYearMonth_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_linode_volumes

> crate::models::GetLinodeVolumes200Response get_linode_volumes(linode_id, page, page_size)
Linode's Volumes List

View Block Storage Volumes attached to this Linode. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | ID of the Linode to look up. | [required] |
**page** | Option<**i32**> | The page of a collection to return. |  |[default to 1]
**page_size** | Option<**i32**> | The number of items to return per page. |  |[default to 100]

### Return type

[**crate::models::GetLinodeVolumes200Response**](getLinodeVolumes_200_response.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## migrate_linode_instance

> serde_json::Value migrate_linode_instance(linode_id, migrate_linode_instance_request)
DC Migration/Pending Host Migration Initiate

Initiate a pending host migration that has been scheduled by Linode or initiate a cross data center (DC) migration.  A list of pending migrations, if any, can be accessed from [GET /account/notifications](/docs/api/account/#notifications-list). When the migration begins, your Linode will be shutdown if not already off. If the migration initiated the shutdown, it will reboot the Linode when completed.  To initiate a cross DC migration, you must pass a `region` parameter to the request body specifying the target data center region. You can view a list of all available regions and their feature capabilities from [GET /regions](/docs/api/regions/#regions-list). If your Linode has a DC migration already queued or you have initiated a previously scheduled migration, you will not be able to initiate a DC migration until it has completed.  **Note:** Next Generation Network (NGN) data centers do not support IPv6 `/116` pools or IP Failover. If you have these features enabled on your Linode and attempt to migrate to an NGN data center, the migration will not initiate. If a Linode cannot be migrated because of an incompatibility, you will be prompted to select a different data center or contact support. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | ID of the Linode to migrate. | [required] |
**migrate_linode_instance_request** | Option<[**MigrateLinodeInstanceRequest**](MigrateLinodeInstanceRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mutate_linode_instance

> serde_json::Value mutate_linode_instance(linode_id, mutate_linode_instance_request)
Linode Upgrade

Linodes created with now-deprecated Types are entitled to a free upgrade to the next generation. A mutating Linode will be allocated any new resources the upgraded Type provides, and will be subsequently restarted if it was currently running. If any actions are currently running or queued, those actions must be completed first before you can initiate a mutate. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | ID of the Linode to mutate. | [required] |
**mutate_linode_instance_request** | Option<[**MutateLinodeInstanceRequest**](MutateLinodeInstanceRequest.md)> | Whether to automatically resize disks or not. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reboot_linode_instance

> serde_json::Value reboot_linode_instance(linode_id, reboot_linode_instance_request)
Linode Reboot

Reboots a Linode you have permission to modify. If any actions are currently running or queued, those actions must be completed first before you can initiate a reboot. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | ID of the linode to reboot. | [required] |
**reboot_linode_instance_request** | Option<[**RebootLinodeInstanceRequest**](RebootLinodeInstanceRequest.md)> | Optional reboot parameters. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rebuild_linode_instance

> crate::models::GetLinodeInstances200ResponseDataInner rebuild_linode_instance(linode_id, rebuild_linode_instance_request)
Linode Rebuild

Rebuilds a Linode you have the `read_write` permission to modify. A rebuild will first shut down the Linode, delete all disks and configs on the Linode, and then deploy a new `image` to the Linode with the given attributes. Additionally:    * Requires an `image` be supplied.   * Requires a `root_pass` be supplied to use for the root User's Account.   * It is recommended to supply SSH keys for the root User using the     `authorized_keys` field. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | ID of the Linode to rebuild. | [required] |
**rebuild_linode_instance_request** | [**RebuildLinodeInstanceRequest**](RebuildLinodeInstanceRequest.md) | The requested state your Linode will be rebuilt into. | [required] |

### Return type

[**crate::models::GetLinodeInstances200ResponseDataInner**](getLinodeInstances_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_linode_ip

> serde_json::Value remove_linode_ip(linode_id, address)
IPv4 Address Delete

Deletes a public or private IPv4 address associated with this Linode. This will fail if it is the Linode's last remaining public IPv4 address. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | The ID of the Linode to look up. | [required] |
**address** | **String** | The IP address to look up. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rescue_linode_instance

> serde_json::Value rescue_linode_instance(linode_id, rescue_linode_instance_request)
Linode Boot into Rescue Mode

Rescue Mode is a safe environment for performing many system recovery and disk management tasks. Rescue Mode is based on the Finnix recovery distribution, a self-contained and bootable Linux distribution. You can also use Rescue Mode for tasks other than disaster recovery, such as formatting disks to use different filesystems, copying data between disks, and downloading files from a disk via SSH and SFTP. * Note that \"sdh\" is reserved and unavailable during rescue. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | ID of the Linode to rescue. | [required] |
**rescue_linode_instance_request** | Option<[**RescueLinodeInstanceRequest**](RescueLinodeInstanceRequest.md)> | Optional object of devices to be mounted. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_disk_password

> serde_json::Value reset_disk_password(linode_id, disk_id, reset_disk_password_request)
Disk Root Password Reset

Resets the password of a Disk you have permission to `read_write`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | ID of the Linode to look up. | [required] |
**disk_id** | **i32** | ID of the Disk to look up. | [required] |
**reset_disk_password_request** | [**ResetDiskPasswordRequest**](ResetDiskPasswordRequest.md) | The new password. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_linode_password

> serde_json::Value reset_linode_password(linode_id, reset_linode_password_request)
Linode Root Password Reset

Resets the root password for this Linode. * Your Linode must be [shut down](/docs/api/linode-instances/#linode-shut-down) for a password reset to complete. * If your Linode has more than one disk (not counting its swap disk), use the [Reset Disk Root Password](/docs/api/linode-instances/#disk-root-password-reset) endpoint to update a specific disk's root password. * A `password_reset` event is generated when a root password reset is successful. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | ID of the Linode for which to reset its root password. | [required] |
**reset_linode_password_request** | Option<[**ResetLinodePasswordRequest**](ResetLinodePasswordRequest.md)> | This Linode's new root password. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resize_disk

> serde_json::Value resize_disk(linode_id, disk_id, resize_disk_request)
Disk Resize

Resizes a Disk you have permission to `read_write`.  The Disk must not be in use. If the Disk is in use, the request will succeed but the resize will ultimately fail. For a request to succeed, the Linode must be shut down prior to resizing the Disk, or the Disk must not be assigned to the Linode's active Configuration Profile.  If you are resizing the Disk to a smaller size, it cannot be made smaller than what is required by the total size of the files current on the Disk. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | ID of the Linode to look up. | [required] |
**disk_id** | **i32** | ID of the Disk to look up. | [required] |
**resize_disk_request** | [**ResizeDiskRequest**](ResizeDiskRequest.md) | The new size of the Disk. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resize_linode_instance

> serde_json::Value resize_linode_instance(linode_id, resize_linode_instance_request)
Linode Resize

Resizes a Linode you have the `read_write` permission to a different Type. If any actions are currently running or queued, those actions must be completed first before you can initiate a resize. Additionally, the following criteria must be met in order to resize a Linode:    * The Linode must not have a pending migration.   * Your Account cannot have an outstanding balance.   * The Linode must not have more disk allocation than the new Type allows.     * In that situation, you must first delete or resize the disk to be smaller. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | ID of the Linode to resize. | [required] |
**resize_linode_instance_request** | [**ResizeLinodeInstanceRequest**](ResizeLinodeInstanceRequest.md) | The Type your current Linode will resize to, and whether to attempt to automatically resize the Linode's disks.  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## restore_backup

> serde_json::Value restore_backup(linode_id, backup_id, restore_backup_request)
Backup Restore

Restores a Linode's Backup to the specified Linode. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | The ID of the Linode that the Backup belongs to. | [required] |
**backup_id** | **i32** | The ID of the Backup to restore. | [required] |
**restore_backup_request** | [**RestoreBackupRequest**](RestoreBackupRequest.md) | Parameters to provide when restoring the Backup. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## shutdown_linode_instance

> serde_json::Value shutdown_linode_instance(linode_id)
Linode Shut Down

Shuts down a Linode you have permission to modify. If any actions are currently running or queued, those actions must be completed first before you can initiate a shutdown. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | ID of the Linode to shutdown. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_disk

> crate::models::GetLinodeDisks200ResponseDataInner update_disk(linode_id, disk_id, get_linode_disks200_response_data_inner)
Disk Update

Updates a Disk that you have permission to `read_write`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | ID of the Linode to look up. | [required] |
**disk_id** | **i32** | ID of the Disk to look up. | [required] |
**get_linode_disks200_response_data_inner** | [**GetLinodeDisks200ResponseDataInner**](GetLinodeDisks200ResponseDataInner.md) | Updates the parameters of a single Disk.  | [required] |

### Return type

[**crate::models::GetLinodeDisks200ResponseDataInner**](getLinodeDisks_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_linode_config

> crate::models::GetLinodeConfigs200ResponseDataInner update_linode_config(linode_id, config_id, get_linode_configs200_response_data_inner)
Configuration Profile Update

Updates a Configuration profile. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | The ID of the Linode whose Configuration profile to look up. | [required] |
**config_id** | **i32** | The ID of the Configuration profile to look up. | [required] |
**get_linode_configs200_response_data_inner** | [**GetLinodeConfigs200ResponseDataInner**](GetLinodeConfigs200ResponseDataInner.md) | The Configuration profile parameters to modify. | [required] |

### Return type

[**crate::models::GetLinodeConfigs200ResponseDataInner**](getLinodeConfigs_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_linode_instance

> crate::models::GetLinodeInstances200ResponseDataInner update_linode_instance(linode_id, get_linode_instances200_response_data_inner)
Linode Update

Updates a Linode that you have permission to `read_write`.  **Important**: You must be an unrestricted User in order to add or modify tags on Linodes. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | ID of the Linode to look up | [required] |
**get_linode_instances200_response_data_inner** | [**GetLinodeInstances200ResponseDataInner**](GetLinodeInstances200ResponseDataInner.md) | Any field that is not marked as `readOnly` may be updated. Fields that are marked `readOnly` will be ignored. If any updated field fails to pass validation, the Linode will not be updated.  | [required] |

### Return type

[**crate::models::GetLinodeInstances200ResponseDataInner**](getLinodeInstances_200_response_data_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_linode_ip

> crate::models::GetLinodeIps200ResponseIpv4PublicInner update_linode_ip(linode_id, address, update_linode_ip_request)
IP Address Update

Updates a the reverse DNS (RDNS) for a particular IP Address associated with this Linode.  Setting the RDNS to `null` for a public IPv4 address, resets it to the default \"ip.linodeusercontent.com\" RDNS value. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**linode_id** | **i32** | The ID of the Linode to look up. | [required] |
**address** | **String** | The IP address to look up. | [required] |
**update_linode_ip_request** | Option<[**UpdateLinodeIpRequest**](UpdateLinodeIpRequest.md)> | The information to update for the IP address. |  |

### Return type

[**crate::models::GetLinodeIps200ResponseIpv4PublicInner**](getLinodeIPs_200_response_ipv4_public_inner.md)

### Authorization

[personalAccessToken](../README.md#personalAccessToken), [oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

