# Ipv6RangeBgp

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**range** | Option<**String**> | The IPv6 range of addresses in this pool.  | [optional][readonly]
**prefix** | Option<**i32**> | The prefix length of the address, denoting how many addresses can be assigned from this range calculated as 2 <sup>128-prefix</sup>.  | [optional]
**region** | Option<**String**> | The region for this range of IPv6 addresses.  | [optional][readonly]
**is_bgp** | Option<**bool**> | Whether this IPv6 range is shared.  | [optional][readonly]
**linodes** | Option<**Vec<i32>**> | A list of Linodes targeted by this IPv6 range. Includes Linodes with IP sharing.  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


