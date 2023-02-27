# GetLinodeFirewalls200ResponseDataInnerRulesInboundInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**protocol** | Option<**String**> | The type of network traffic to allow.  | [optional]
**ports** | Option<**String**> | A string representing the port or ports on which traffic will be allowed:  - The string may be a single port, a range of ports, or a comma-separated list of single ports and port ranges. A space is permitted following each comma. - A range of ports is inclusive of the start and end values for the range. The end value of the range must be greater than the start value. - Ports must be within 1 and 65535, and may not contain any leading zeroes. For example, port \"080\" is not allowed. - Ports may not be specified if a rule's protocol is `ICMP` or `IPENCAP`. - At least one port must be specified if a rule's protocol is `TCP` or `UDP`. - The ports string can have up to 15 *pieces*, where a single port is treated as one piece, and a port range is treated as two pieces. For example, the string \"22-24, 80, 443\" has four pieces.  | [optional]
**addresses** | Option<[**crate::models::GetLinodeFirewalls200ResponseDataInnerRulesInboundInnerAddresses**](getLinodeFirewalls_200_response_data_inner_rules_inbound_inner_addresses.md)> |  | [optional]
**action** | Option<**String**> | Controls whether traffic is accepted or dropped by this rule. Overrides the Firewall's `inbound_policy` if this is an inbound rule, or the `outbound_policy` if this is an outbound rule.  | [optional]
**label** | Option<**String**> | Used to identify this rule. For display purposes only.  | [optional]
**description** | Option<**String**> | Used to describe this rule. For display purposes only.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


