# NodeBalancer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | This NodeBalancer's unique ID.  | [optional][readonly]
**label** | Option<**String**> | This NodeBalancer's label. These must be unique on your Account.  | [optional]
**region** | Option<**String**> | The Region where this NodeBalancer is located. NodeBalancers only support backends in the same Region.  | [optional][readonly]
**hostname** | Option<**String**> | This NodeBalancer's hostname, beginning with its IP address and ending with _.ip.linodeusercontent.com_.  | [optional][readonly]
**ipv4** | Option<**String**> | This NodeBalancer's public IPv4 address.  | [optional][readonly]
**ipv6** | Option<**String**> | This NodeBalancer's public IPv6 address.  | [optional][readonly]
**created** | Option<**String**> | When this NodeBalancer was created.  | [optional][readonly]
**updated** | Option<**String**> | When this NodeBalancer was last updated.  | [optional][readonly]
**client_conn_throttle** | Option<**i32**> | Throttle connections per second.  Set to 0 (zero) to disable throttling.  | [optional]
**transfer** | Option<[**crate::models::GetLinodeNodeBalancers200ResponseDataInnerTransfer**](getLinodeNodeBalancers_200_response_data_inner_transfer.md)> |  | [optional]
**tags** | Option<**Vec<String>**> | An array of Tags applied to this object.  Tags are for organizational purposes only.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


