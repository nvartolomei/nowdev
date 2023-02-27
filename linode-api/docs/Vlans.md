# Vlans

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**label** | Option<**String**> | The name of this VLAN. | [optional][readonly]
**region** | Option<**String**> | This VLAN's data center region.  **Note:** Currently, a VLAN can only be assigned to a Linode within the same data center region.  | [optional][readonly]
**linodes** | Option<**Vec<i32>**> | An array of Linode IDs attached to this VLAN.  | [optional][readonly]
**created** | Option<**String**> | The date this VLAN was created.  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


