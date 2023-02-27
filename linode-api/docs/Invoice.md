# Invoice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The Invoice's unique ID. | [optional][readonly]
**date** | Option<**String**> | When this Invoice was generated. | [optional][readonly]
**label** | Option<**String**> | The Invoice's display label. | [optional][readonly]
**subtotal** | Option<**f32**> | The amount of the Invoice before taxes in US Dollars. | [optional][readonly]
**tax** | Option<**f32**> | The amount of tax levied on the Invoice in US Dollars. | [optional][readonly]
**tax_summary** | Option<[**Vec<crate::models::InvoiceTaxSummaryInner>**](Invoice_tax_summary_inner.md)> | The amount of tax broken down into subtotals by source. | [optional][readonly]
**total** | Option<**f32**> | The amount of the Invoice after taxes in US Dollars. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


