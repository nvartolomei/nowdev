# Account

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active_promotions** | Option<[**Vec<crate::models::AccountActivePromotionsInner>**](Account_active_promotions_inner.md)> |  | [optional][readonly]
**active_since** | Option<**String**> | The datetime of when the account was activated. | [optional][readonly]
**address_1** | Option<**String**> | First line of this Account's billing address. | [optional]
**address_2** | Option<**String**> | Second line of this Account's billing address. | [optional]
**balance** | Option<**f32**> | This Account's balance, in US dollars. | [optional][readonly]
**balance_uninvoiced** | Option<**f32**> | This Account's current estimated invoice in US dollars. This is not your final invoice balance. Transfer charges are not included in the estimate.  | [optional][readonly]
**billing_source** | Option<**String**> | The source of service charges for this Account, as determined by its relationship with Akamai. Accounts that are associated with Akamai-specific customers return a value of `akamai`. All other Accounts return a value of `linode`.  | [optional][readonly]
**capabilities** | Option<**Vec<String>**> | A list of capabilities your account supports.  | [optional][readonly]
**city** | Option<**String**> | The city for this Account's billing address. | [optional]
**credit_card** | Option<[**crate::models::AccountCreditCard**](Account_credit_card.md)> |  | [optional]
**company** | Option<**String**> | The company name associated with this Account. | [optional]
**country** | Option<**String**> | The two-letter ISO 3166 country code of this Account's billing address.  | [optional]
**email** | Option<**String**> | The email address of the person associated with this Account. | [optional]
**first_name** | Option<**String**> | The first name of the person associated with this Account. | [optional]
**last_name** | Option<**String**> | The last name of the person associated with this Account. | [optional]
**phone** | Option<**String**> | The phone number associated with this Account. | [optional]
**state** | Option<**String**> | If billing address is in the United States (US) or Canada (CA), only the two-letter ISO 3166 State or Province code are accepted. If entering a US military address, state abbreviations (AA, AE, AP) should be entered. If the address is outside the US or CA, this is the Province associated with the Account's billing address.  | [optional]
**tax_id** | Option<**String**> | The tax identification number associated with this Account, for tax calculations in some countries. If you do not live in a country that collects tax, this should be an empty string (`\"\"`).  | [optional]
**euuid** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | An external unique identifier for this account.  | [optional][readonly]
**zip** | Option<**String**> | The zip code of this Account's billing address. The following restrictions apply:  - May only consist of letters, numbers, spaces, and hyphens. - Must not contain more than 9 letter or number characters.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


