# IpAddressesAssignRequestAssignmentsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address** | Option<**String**> | The IPv4 address or IPv6 range for this assignment. * Must be an IPv4 address or an IPv6 range you can access in the Region specified. * IPv6 ranges must include a prefix length of `/56` or `/64`, for example: `2001:db8:3c4d:15::/64`. * Assignment of an IPv6 range to a Linode updates the route target of the range to the assigned Linode's SLAAC address. * May be a public or private address.  | [optional]
**linode_id** | Option<**i32**> | The ID of the Linode to assign this address to. The IP's previous Linode will lose this address, and must end up with at least one public address and no more than one private address once all assignments have been made.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


