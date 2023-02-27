# PaymentRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cvv** | Option<**String**> | CVV (Card Verification Value) of the credit card to be used for the Payment. Required if paying by credit card.  | [optional]
**usd** | **String** | The amount in US Dollars of the Payment.  * Can begin with or without `$`. * Commas (`,`) are not accepted. * Must end with a decimal expression, such as `.00` or `.99`. * Minimum: `$5.00` or the Account balance, whichever is lower. * Maximum: `$2000.00` or the Account balance up to `$50000.00`, whichever is greater.  | 
**payment_method_id** | Option<**i32**> | The ID of the Payment Method to apply to the Payment.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


