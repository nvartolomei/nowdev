# GetLinodeIps200ResponseIpv4PrivateInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address** | Option<**String**> | The private IPv4 address.  | [optional][readonly]
**gateway** | Option<**String**> | The default gateway for this address.  | [optional][readonly]
**subnet_mask** | Option<**String**> | The mask that separates host bits from network bits for this address.  | [optional][readonly]
**prefix** | Option<**i32**> | The number of bits set in the subnet mask.  | [optional][readonly]
**r#type** | Option<**String**> | The type of address this is.  | [optional][readonly]
**public** | Option<**bool**> | Whether this is a public or private IP address.  | [optional][readonly]
**rdns** | Option<**String**> | The reverse DNS assigned to this address.  | [optional]
**linode_id** | Option<**i32**> | The ID of the Linode this address currently belongs to.  | [optional][readonly]
**region** | Option<**String**> | The Region this address resides in.  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


