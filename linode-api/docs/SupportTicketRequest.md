# SupportTicketRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | **String** | The full details of the issue or question.  | 
**database_id** | Option<**i32**> | The ID of the Managed Database this ticket is regarding, if relevant.  | [optional]
**domain_id** | Option<**i32**> | The ID of the Domain this ticket is regarding, if relevant.  | [optional]
**firewall_id** | Option<**i32**> | The ID of the Firewall this ticket is regarding, if relevant.  | [optional]
**linode_id** | Option<**i32**> | The ID of the Linode this ticket is regarding, if relevant.  | [optional]
**lkecluster_id** | Option<**i32**> | The ID of the Kubernetes cluster this ticket is regarding, if relevant.  | [optional]
**longviewclient_id** | Option<**i32**> | The ID of the Longview client this ticket is regarding, if relevant.  | [optional]
**nodebalancer_id** | Option<**i32**> | The ID of the NodeBalancer this ticket is regarding, if relevant.  | [optional]
**summary** | **String** | The summary or title for this SupportTicket.  | 
**managed_issue** | Option<**bool**> | Designates if this ticket is related to a [Managed service](https://www.linode.com/products/managed/). If `true`, the following constraints will apply: * No ID attributes (i.e. `linode_id`, `domain_id`, etc.) should be provided with this request. * Your account must have a [Managed service enabled](/docs/api/managed/#managed-service-enable).  | [optional]
**volume_id** | Option<**i32**> | The ID of the Volume this ticket is regarding, if relevant.  | [optional]
**vlan** | Option<**String**> | The label of the VLAN this ticket is regarding, if relevant. To view your VLANs, use the VLANs List ([GET /networking/vlans](/docs/api/networking/#vlans-list)) endpoint.  Requires a specified `region` to identify the VLAN.  | [optional]
**region** | Option<**String**> | The [Region](/docs/api/regions/) ID for the associated VLAN this ticket is regarding.  Only allowed when submitting a VLAN ticket.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


