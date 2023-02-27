# LinodeConfigInterface

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**label** | Option<**String**> | The name of this interface.  Required for `vlan` purpose interfaces. Must be an empty string or `null` for `public` purpose interfaces.  If the VLAN label is new, a VLAN is created. Up to 10 VLANs can be created in each data center region. To view your active VLANs, use the [VLANs List](/docs/api/networking/#vlans-list) endpoint.  May only consist of ASCII letters, numbers, and dashes (`-`).  Must be unique among the Linode's interfaces.  | [optional]
**ipam_address** | Option<**String**> | This Network Interface's private IP address in Classless Inter-Domain Routing (CIDR) notation.  Only used for `vlan` purpose interfaces. Must be an empty string or `null` for `public` purpose interfaces.  The Linode is configured to use this address for the associated interface upon reboot if Network Helper is enabled. If Network Helper is disabled, the address can be enabled with [manual static IP configuration](/docs/guides/manual-network-configuration/).  Must be unique among the Linode's interfaces.  | [optional]
**purpose** | Option<**String**> | The type of interface.  * `public`   * Only one `public` interface per Linode can be defined.   * The Linode's default public IPv4 address is assigned to the `public` interface.   * A Linode must have a public interface in the first/eth0 position to be reachable via the public internet upon boot without additional system configuration. If no `public` interface is configured, the Linode is not directly reachable via the public internet. In this case, access can only be established via LISH or other Linodes connected to the same VLAN.  * `vlan`   * Configuring a `vlan` purpose interface attaches this Linode to the VLAN with the specified `label`.   * The Linode is configured to use the specified `ipam_address`, if any.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


