# Domain

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | This Domain's unique ID | [optional][readonly]
**r#type** | Option<**String**> | Whether this Domain represents the authoritative source of information for the domain it describes (\"master\"), or whether it is a read-only copy of a master (\"slave\").  | [optional]
**domain** | Option<**String**> | The domain this Domain represents. Domain labels cannot be longer than 63 characters and must conform to [RFC1035](https://tools.ietf.org/html/rfc1035). Domains must be unique on Linode's platform, including across different Linode accounts; there cannot be two Domains representing the same domain.  | [optional]
**group** | Option<**String**> | The group this Domain belongs to.  This is for display purposes only.  | [optional]
**status** | Option<**String**> | Used to control whether this Domain is currently being rendered.  | [optional][default to Active]
**description** | Option<**String**> | A description for this Domain. This is for display purposes only.  | [optional]
**soa_email** | Option<**String**> | Start of Authority email address. This is required for `type` master Domains.  | [optional]
**retry_sec** | Option<**i32**> | The interval, in seconds, at which a failed refresh should be retried.  * Valid values are 0, 300, 3600, 7200, 14400, 28800, 57600, 86400, 172800, 345600, 604800, 1209600, and 2419200.  * Any other value is rounded up to the nearest valid value.  * A value of 0 is equivalent to the default value of 14400.  | [optional][default to 0]
**master_ips** | Option<**Vec<String>**> | The IP addresses representing the master DNS for this Domain. At least one value is required for `type` slave Domains. The total combined length of all data within this array cannot exceed 1000 characters.  | [optional]
**axfr_ips** | Option<**Vec<String>**> | The list of IPs that may perform a zone transfer for this Domain. The total combined length of all data within this array cannot exceed 1000 characters.  **Note**: This is potentially dangerous, and should be set to an empty list unless you intend to use it.  | [optional]
**expire_sec** | Option<**i32**> | The amount of time in seconds that may pass before this Domain is no longer authoritative.  * Valid values are 0, 300, 3600, 7200, 14400, 28800, 57600, 86400, 172800, 345600, 604800, 1209600, and 2419200.  * Any other value is rounded up to the nearest valid value.  * A value of 0 is equivalent to the default value of 1209600.  | [optional][default to 0]
**refresh_sec** | Option<**i32**> | The amount of time in seconds before this Domain should be refreshed.  * Valid values are 0, 300, 3600, 7200, 14400, 28800, 57600, 86400, 172800, 345600, 604800, 1209600, and 2419200.  * Any other value is rounded up to the nearest valid value.  * A value of 0 is equivalent to the default value of 14400.  | [optional][default to 0]
**ttl_sec** | Option<**i32**> | \"Time to Live\" - the amount of time in seconds that this Domain's records may be cached by resolvers or other domain servers. * Valid values are 0, 300, 3600, 7200, 14400, 28800, 57600, 86400, 172800, 345600, 604800, 1209600, and 2419200. * Any other value is rounded up to the nearest valid value. * A value of 0 is equivalent to the default value of 86400.  | [optional][default to 0]
**tags** | Option<**Vec<String>**> | An array of tags applied to this object.  Tags are for organizational purposes only.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


