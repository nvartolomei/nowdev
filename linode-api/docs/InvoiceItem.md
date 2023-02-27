# InvoiceItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount** | Option<**f32**> | The price, in US dollars, of the Invoice Item. Equal to the unit price multiplied by quantity. | [optional][readonly]
**tax** | Option<**f32**> | The amount of tax levied on this Item in US Dollars. | [optional][readonly]
**total** | Option<**f32**> | The price of this Item after taxes in US Dollars. | [optional][readonly]
**from** | Option<**String**> | The date the Invoice Item started, based on month. | [optional][readonly]
**label** | Option<**String**> | The Invoice Item's display label. | [optional][readonly]
**quantity** | Option<**i32**> | The quantity of this Item for the specified Invoice. | [optional][readonly]
**to** | Option<**String**> | The date the Invoice Item ended, based on month. | [optional][readonly]
**r#type** | Option<**String**> | The type of service, ether `hourly` or `misc`. | [optional][readonly]
**unit_price** | Option<**String**> | The monthly service fee in US Dollars for this Item. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


