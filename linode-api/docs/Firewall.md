# Firewall

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The Firewall's unique ID.  | [optional][readonly]
**label** | Option<**String**> | The Firewall's label, for display purposes only.  Firewall labels have the following constraints:    * Must begin and end with an alphanumeric character.   * May only consist of alphanumeric characters, dashes (`-`), underscores (`_`) or periods (`.`).   * Cannot have two dashes (`--`), underscores (`__`) or periods (`..`) in a row.   * Must be between 3 and 32 characters.   * Must be unique.  | [optional]
**created** | Option<**String**> | When this Firewall was created.  | [optional][readonly]
**updated** | Option<**String**> | When this Firewall was last updated.  | [optional][readonly]
**status** | Option<**String**> | The status of this Firewall.    * When a Firewall is first created its status is `enabled`.   * Use the [Update Firewall](/docs/api/networking/#firewall-update) endpoint to set a Firewall's status to `enabled` or `disabled`.   * Use the [Delete Firewall](/docs/api/networking/#firewall-delete) endpoint to delete a Firewall.  | [optional][readonly]
**rules** | Option<[**crate::models::GetLinodeFirewalls200ResponseDataInnerRules**](getLinodeFirewalls_200_response_data_inner_rules.md)> |  | [optional]
**tags** | Option<**Vec<String>**> | An array of tags applied to this object. Tags are for organizational purposes only.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


