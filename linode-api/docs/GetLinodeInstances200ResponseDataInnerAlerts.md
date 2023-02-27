# GetLinodeInstances200ResponseDataInnerAlerts

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cpu** | Option<**i32**> | The percentage of CPU usage required to trigger an alert. If the average CPU usage over two hours exceeds this value, we'll send you an alert. Your Linode's total CPU capacity is represented as 100%, multiplied by its number of cores.  For example, a two core Linode's CPU capacity is represented as 200%. If you want to be alerted at 90% of a two core Linode's CPU capacity, set the alert value to `180`.  The default value is 90% multiplied by the number of cores.  If the value is set to `0` (zero), the alert is disabled.  | [optional]
**network_in** | Option<**i32**> | The amount of incoming traffic, in Mbit/s, required to trigger an alert. If the average incoming traffic over two hours exceeds this value, we'll send you an alert. If this is set to `0` (zero), the alert is disabled.  | [optional]
**network_out** | Option<**i32**> | The amount of outbound traffic, in Mbit/s, required to trigger an alert. If the average outbound traffic over two hours exceeds this value, we'll send you an alert. If this is set to `0` (zero), the alert is disabled.  | [optional]
**transfer_quota** | Option<**i32**> | The percentage of network transfer that may be used before an alert is triggered. When this value is exceeded, we'll alert you. If this is set to `0` (zero), the alert is disabled.  | [optional]
**io** | Option<**i32**> | The amount of disk IO operation per second required to trigger an alert. If the average disk IO over two hours exceeds this value, we'll send you an alert. If set to `0` (zero), this alert is disabled.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


