# CreditCard

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**card_number** | **String** | Your credit card number. No spaces or dashes allowed. | 
**expiry_month** | **i32** | A value from 1-12 representing the expiration month of your credit card.    * 1 = January   * 2 = February   * 3 = March   * Etc.  | 
**expiry_year** | **i32** | A four-digit integer representing the expiration year of your credit card.  The combination of `expiry_month` and `expiry_year` must result in a month/year combination of the current month or in the future. An expiration date set in the past is invalid.  | 
**cvv** | **String** | CVV (Card Verification Value) of the credit card, typically found on the back of the card.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


