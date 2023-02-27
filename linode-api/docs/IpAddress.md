# IpAddress

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address** | Option<**String**> | The IP address.  | [optional][readonly]
**gateway** | Option<**String**> | The default gateway for this address.  | [optional][readonly]
**subnet_mask** | Option<**String**> | The mask that separates host bits from network bits for this address.  | [optional][readonly]
**prefix** | Option<**i32**> | The number of bits set in the subnet mask.  | [optional][readonly]
**r#type** | Option<**String**> | The type of address this is.  | [optional][readonly]
**public** | Option<**bool**> | Whether this is a public or private IP address.  | [optional][readonly]
**rdns** | Option<**String**> | The reverse DNS assigned to this address. For public IPv4 addresses, this will be set to a default value provided by Linode if not explicitly set.  | [optional]
**linode_id** | Option<**i32**> | The ID of the Linode this address currently belongs to. For IPv4 addresses, this is by default the Linode that this address was assigned to on creation, and these addresses my be moved using the [/networking/ipv4/assign](/docs/api/networking/#ips-to-linodes-assign) endpoint. For SLAAC and link-local addresses, this value may not be changed.  | [optional][readonly]
**region** | Option<**String**> | The Region this IP address resides in.  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


