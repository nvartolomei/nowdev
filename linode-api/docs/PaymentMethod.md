# PaymentMethod

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The unique ID of this Payment Method. | [optional]
**r#type** | Option<**String**> | The type of Payment Method. | [optional]
**is_default** | Option<**bool**> | Whether this Payment Method is the default method for automatically processing service charges.  | [optional]
**created** | Option<**String**> | When the Payment Method was added to the Account. | [optional][readonly]
**data** | Option<[**crate::models::PaymentMethodData**](PaymentMethod_data.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


