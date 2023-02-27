# GetLinodeFirewalls200ResponseDataInnerRules

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**inbound** | Option<[**Vec<crate::models::GetLinodeFirewalls200ResponseDataInnerRulesInboundInner>**](getLinodeFirewalls_200_response_data_inner_rules_inbound_inner.md)> | The inbound rules for the firewall, as a JSON array.  | [optional]
**outbound** | Option<[**Vec<crate::models::GetLinodeFirewalls200ResponseDataInnerRulesInboundInner>**](getLinodeFirewalls_200_response_data_inner_rules_inbound_inner.md)> | The outbound rules for the firewall, as a JSON array.  | [optional]
**inbound_policy** | Option<**String**> | The default behavior for inbound traffic. This setting can be overridden by [updating](/docs/api/networking/#firewall-rules-update) the `inbound.action` property of the Firewall Rule.  | [optional]
**outbound_policy** | Option<**String**> | The default behavior for outbound traffic. This setting can be overridden by [updating](/docs/api/networking/#firewall-rules-update) the `outbound.action` property of the Firewall Rule.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


