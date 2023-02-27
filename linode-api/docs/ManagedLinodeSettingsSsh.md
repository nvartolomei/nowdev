# ManagedLinodeSettingsSsh

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access** | Option<**bool**> | If true, Linode special forces may access this Linode over ssh to respond to Issues.  | [optional][default to true]
**user** | Option<**String**> | The specific user, if any, Linode's special forces should use when accessing this Linode to respond to an issue.  The default `null` value corresponds to the root user.  | [optional]
**ip** | Option<**String**> | The IP Linode special forces should use to access this Linode when responding to an Issue.  By default, any of a Linode's IP addresses can be used for incident response access.  | [optional][default to any]
**port** | Option<**i32**> | The port Linode special forces should use to access this Linode over ssh to respond to an Issue.  The default `null` value corresponds to port 22.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


