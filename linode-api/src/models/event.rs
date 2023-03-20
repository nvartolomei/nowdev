/*
 * Linode API
 *
 * ## Introduction The Linode API provides the ability to programmatically manage the full range of Linode products and services.  This reference is designed to assist application developers and system administrators.  Each endpoint includes descriptions, request syntax, and examples using standard HTTP requests. Response data is returned in JSON format.   This document was generated from our OpenAPI Specification.  See the <a target=\"_top\" href=\"https://www.openapis.org\">OpenAPI website</a> for more information.  <a target=\"_top\" href=\"/docs/api/openapi.yaml\">Download the Linode OpenAPI Specification</a>.   ## Changelog  <a target=\"_top\" href=\"/docs/products/tools/api/release-notes/\">View our Changelog</a> to see release notes on all changes made to our API.  ## Access and Authentication  Some endpoints are publicly accessible without requiring authentication. All endpoints affecting your Account, however, require either a Personal Access Token or OAuth authentication (when using third-party applications).  ### Personal Access Token  The easiest way to access the API is with a Personal Access Token (PAT) generated from the <a target=\"_top\" href=\"https://cloud.linode.com/profile/tokens\">Linode Cloud Manager</a> or the [Create Personal Access Token](/docs/api/profile/#personal-access-token-create) endpoint.  All scopes for the OAuth security model ([defined below](/docs/api/profile/#oauth)) apply to this security model as well.  #### Authentication  | Security Scheme Type: | HTTP | |-----------------------|------| | **HTTP Authorization Scheme** | bearer |  ### OAuth If you only need to access the Linode API for personal use, we recommend that you create a [personal access token](/docs/api/#personal-access-token). If you're designing an application that can authenticate with an arbitrary Linode user, then you should use the OAuth 2.0 workflows presented in this section.  For a more detailed example of an OAuth 2.0 implementation, see our guide on [How to Create an OAuth App with the Linode Python API Library](/docs/products/tools/api/guides/create-an-oauth-app-with-the-python-api-library/#oauth-2-authentication-exchange).  Before you implement OAuth in your application, you first need to create an OAuth client. You can do this [with the Linode API](/docs/api/account/#oauth-client-create) or [via the Cloud Manager](https://cloud.linode.com/profile/clients):    - When creating the client, you'll supply a `label` and a `redirect_uri` (referred to as the Callback URL in the Cloud Manager).   - The response from this endpoint will give you a `client_id` and a `secret`.   - Clients can be public or private, and are private by default. You can choose to make the client public when it is created.     - A private client is used with applications which can securely store the client secret (that is, the secret returned to you when you first created the client). For example, an application running on a secured server that only the developer has access to would use a private OAuth client. This is also called a confidential client in some OAuth documentation.     - A public client is used with applications where the client secret is not guaranteed to be secure. For example, a native app running on a user's computer may not be able to keep the client secret safe, as a user could potentially inspect the source of the application. So, native apps or apps that run in a user's browser should use a public client.     - Public and private clients follow different workflows, as described below.  #### OAuth Workflow  The OAuth workflow is a series of exchanges between your third-party app and Linode. The workflow is used to authenticate a user before an application can start making API calls on the user's behalf.  Notes:  - With respect to the diagram in [section 1.2 of RFC 6749](https://tools.ietf.org/html/rfc6749#section-1.2), login.linode.com (referred to in this section as the *login server*) is the Resource Owner and the Authorization Server; api.linode.com (referred to here as the *api server*) is the Resource Server. - The OAuth spec refers to the private and public workflows listed below as the [authorization code flow](https://tools.ietf.org/html/rfc6749#section-1.3.1) and [implicit flow](https://tools.ietf.org/html/rfc6749#section-1.3.2).  | PRIVATE WORKFLOW | PUBLIC WORKFLOW | |------------------|------------------| | 1.  The user visits the application's website and is directed to login with Linode. | 1.  The user visits the application's website and is directed to login with Linode. | | 2.  Your application then redirects the user to Linode's [login server](https://login.linode.com) with the client application's `client_id` and requested OAuth `scope`, which should appear in the URL of the login page. | 2.  Your application then redirects the user to Linode's [login server](https://login.linode.com) with the client application's `client_id` and requested OAuth `scope`, which should appear in the URL of the login page. | | 3.  The user logs into the login server with their username and password. | 3.  The user logs into the login server with their username and password. | | 4.  The login server redirects the user to the specificed redirect URL with a temporary authorization `code` (exchange code) in the URL. | 4.  The login server redirects the user back to your application with an OAuth `access_token` embedded in the redirect URL's hash. This is temporary and expires in two hours. No `refresh_token` is issued. Therefore, once the `access_token` expires, a new one will need to be issued by having the user log in again. | | 5.  The application issues a POST request (*see additional details below*) to the login server with the exchange code, `client_id`, and the client application's `client_secret`. | | | 6.  The login server responds to the client application with a new OAuth `access_token` and `refresh_token`. The `access_token` is set to expire in two hours. | | | 7.  The `refresh_token` can be used by contacting the login server with the `client_id`, `client_secret`, `grant_type`, and `refresh_token` to get a new OAuth `access_token` and `refresh_token`. The new `access_token` is good for another two hours, and the new `refresh_token` can be used to extend the session again by this same method (*see additional details below*). | |  #### OAuth Private Workflow - Additional Details  The following information expands on steps 5 through 7 of the private workflow:  Once the user has logged into Linode and you have received an exchange code, you will need to trade that exchange code for an `access_token` and `refresh_token`. You do this by making an HTTP POST request to the following address:  ``` https://login.linode.com/oauth/token ```  Make this request as `application/x-www-form-urlencoded` or as `multipart/form-data` and include the following parameters in the POST body:  | PARAMETER | DESCRIPTION | |-----------|-------------| | client_id | Your app's client ID. | | client_secret | Your app's client secret. | | code | The code you just received from the redirect. |  You'll get a response like this:  ```json {   \"scope\": \"linodes:read_write\",   \"access_token\": \"03d084436a6c91fbafd5c4b20c82e5056a2e9ce1635920c30dc8d81dc7a6665c\",   \"refresh_token\": \"f2ec9712e616fdb5a2a21aa0e88cfadea7502ebc62cf5bd758dbcd65e1803bad\",   \"token_type\": \"bearer\",   \"expires_in\": 7200 } ```  Included in the response is an `access_token`. With this token, you can proceed to make authenticated HTTP requests to the API by adding this header to each request:  ``` Authorization: Bearer 03d084436a6c91fbafd5c4b20c82e5056a2e9ce1635920c30dc8d81dc7a6665c ```  This `access_token` is set to expire in two hours. To refresh access prior to expiration, make another request to the same URL with the following parameters in the POST body:  | PARAMETER | DESCRIPTION | |-----------|-------------| | grant_type | The grant type you're using. Use \"refresh_token\" when refreshing access. | | client_id | Your app's client ID. | | client_secret | Your app's client secret. | | refresh_token | The `refresh_token` received from the previous response. |  You'll get another response with an updated `access_token` and `refresh_token`, which can then be used to refresh access again.  #### OAuth Reference  | Security Scheme Type | OAuth 2.0 | |-----------------------|--------| | **Authorization URL** | https://login.linode.com/oauth/authorize | | **Token URL** | https://login.linode.com/oauth/token | | **Scopes** | <ul><li>`account:read_only` - Allows access to GET information about your Account.</li><li>`account:read_write` - Allows access to all endpoints related to your Account.</li><li>`databases:read_only` - Allows access to GET Managed Databases on your Account.</li><li>`databases:read_write` - Allows access to all endpoints related to your Managed Databases.</li><li>`domains:read_only` - Allows access to GET Domains on your Account.</li><li>`domains:read_write` - Allows access to all Domain endpoints.</li><li>`events:read_only` - Allows access to GET your Events.</li><li>`events:read_write` - Allows access to all endpoints related to your Events.</li><li>`firewall:read_only` - Allows access to GET information about your Firewalls.</li><li>`firewall:read_write` - Allows access to all Firewall endpoints.</li><li>`images:read_only` - Allows access to GET your Images.</li><li>`images:read_write` - Allows access to all endpoints related to your Images.</li><li>`ips:read_only` - Allows access to GET your ips.</li><li>`ips:read_write` - Allows access to all endpoints related to your ips.</li><li>`linodes:read_only` - Allows access to GET Linodes on your Account.</li><li>`linodes:read_write` - Allow access to all endpoints related to your Linodes.</li><li>`lke:read_only` - Allows access to GET LKE Clusters on your Account.</li><li>`lke:read_write` - Allows access to all endpoints related to LKE Clusters on your Account.</li><li>`longview:read_only` - Allows access to GET your Longview Clients.</li><li>`longview:read_write` - Allows access to all endpoints related to your Longview Clients.</li><li>`nodebalancers:read_only` - Allows access to GET NodeBalancers on your Account.</li><li>`nodebalancers:read_write` - Allows access to all NodeBalancer endpoints.</li><li>`object_storage:read_only` - Allows access to GET information related to your Object Storage.</li><li>`object_storage:read_write` - Allows access to all Object Storage endpoints.</li><li>`stackscripts:read_only` - Allows access to GET your StackScripts.</li><li>`stackscripts:read_write` - Allows access to all endpoints related to your StackScripts.</li><li>`volumes:read_only` - Allows access to GET your Volumes.</li><li>`volumes:read_write` - Allows access to all endpoints related to your Volumes.</li></ul><br/>|  ## Requests  Requests must be made over HTTPS to ensure transactions are encrypted. The following Request methods are supported:  | METHOD | USAGE | |--------|-------| | GET    | Retrieves data about collections and individual resources. | | POST   | For collections, creates a new resource of that type. Also used to perform actions on action endpoints. | | PUT    | Updates an existing resource. | | DELETE | Deletes a resource. This is a destructive action. |   ## Responses  Actions will return one following HTTP response status codes:  | STATUS  | DESCRIPTION | |---------|-------------| | 200 OK  | The request was successful. | | 202 Accepted | The request was successful, but processing has not been completed. The response body includes a \"warnings\" array containing the details of incomplete processes. | | 204 No Content | The server successfully fulfilled the request and there is no additional content to send. | | 299 Deprecated | The request was successful, but involved a deprecated endpoint. The response body includes a \"warnings\" array containing warning messages. | | 400 Bad Request | You submitted an invalid request (missing parameters, etc.). | | 401 Unauthorized | You failed to authenticate for this resource. | | 403 Forbidden | You are authenticated, but don't have permission to do this. | | 404 Not Found | The resource you're requesting does not exist. | | 429 Too Many Requests | You've hit a rate limit. | | 500 Internal Server Error | Please [open a Support Ticket](/docs/api/support/#support-ticket-open). |  ## Errors  Success is indicated via <a href=\"https://en.wikipedia.org/wiki/List_of_HTTP_status_codes\" target=\"_top\">Standard HTTP status codes</a>. `2xx` codes indicate success, `4xx` codes indicate a request error, and `5xx` errors indicate a server error. A request error might be an invalid input, a required parameter being omitted, or a malformed request. A server error means something went wrong processing your request. If this occurs, please [open a Support Ticket](/docs/api/support/#support-ticket-open) and let us know. Though errors are logged and we work quickly to resolve issues, opening a ticket and providing us with reproducable steps and data is always helpful.  The `errors` field is an array of the things that went wrong with your request. We will try to include as many of the problems in the response as possible, but it's conceivable that fixing these errors and resubmitting may result in new errors coming back once we are able to get further along in the process of handling your request.   Within each error object, the `field` parameter will be included if the error pertains to a specific field in the JSON you've submitted. This will be omitted if there is no relevant field. The `reason` is a human-readable explanation of the error, and will always be included.  ## Pagination  Resource lists are always paginated. The response will look similar to this:  ```json {     \"data\": [ ... ],     \"page\": 1,     \"pages\": 3,     \"results\": 300 } ```  * Pages start at 1. You may retrieve a specific page of results by adding `?page=x` to your URL (for example, `?page=4`). If the value of `page` exceeds `2^64/page_size`, the last possible page will be returned.   * Page sizes default to 100, and can be set to return between 25 and 500. Page size can be set using `?page_size=x`.  ## Filtering and Sorting  Collections are searchable by fields they include, marked in the spec as `x-linode-filterable: true`. Filters are passed in the `X-Filter` header and are formatted as JSON objects. Here is a request call for Linode Types in our \"standard\" class:  ```Shell curl \"https://api.linode.com/v4/linode/types\" \\   -H 'X-Filter: { \"class\": \"standard\" }' ```  The filter object's keys are the keys of the object you're filtering, and the values are accepted values. You can add multiple filters by including more than one key. For example, filtering for \"standard\" Linode Types that offer one vcpu:  ```Shell  curl \"https://api.linode.com/v4/linode/types\" \\   -H 'X-Filter: { \"class\": \"standard\", \"vcpus\": 1 }' ```  In the above example, both filters are combined with an \"and\" operation. However, if you wanted either Types with one vcpu or Types in our \"standard\" class, you can add an operator:   ```Shell curl \"https://api.linode.com/v4/linode/types\" \\   -H 'X-Filter: { \"+or\": [ { \"vcpus\": 1 }, { \"class\": \"standard\" } ] }' ```  Each filter in the `+or` array is its own filter object, and all conditions in it are combined with an \"and\" operation as they were in the previous example.  Other operators are also available. Operators are keys of a Filter JSON object. Their value must be of the appropriate type, and they are evaluated as described below:  | OPERATOR | TYPE   | DESCRIPTION                       | |----------|--------|-----------------------------------| | +and     | array  | All conditions must be true.       | | +or      | array  | One condition must be true.        | | +gt      | number | Value must be greater than number. | | +gte     | number | Value must be greater than or equal to number. | | +lt      | number | Value must be less than number. | | +lte     | number | Value must be less than or equal to number. | | +contains | string | Given string must be in the value. | | +neq      | string | Does not equal the value.          | | +order_by | string | Attribute to order the results by - must be filterable. | | +order    | string | Either \"asc\" or \"desc\". Defaults to \"asc\". Requires `+order_by`. |  For example, filtering for [Linode Types](/docs/api/linode-types/) that offer memory equal to or higher than 61440:  ```Shell curl \"https://api.linode.com/v4/linode/types\" \\   -H '     X-Filter: {       \"memory\": {         \"+gte\": 61440       }     }' ```  You can combine and nest operators to construct arbitrarily-complex queries. For example, give me all [Linode Types](/docs/api/linode-types/) which are either `standard` or `highmem` class, or have between 12 and 20 vcpus:  ```Shell curl \"https://api.linode.com/v4/linode/types\" \\   -H '     X-Filter: {       \"+or\": [         {           \"+or\": [             {               \"class\": \"standard\"             },             {               \"class\": \"highmem\"             }           ]         },         {           \"+and\": [             {               \"vcpus\": {                 \"+gte\": 12               }             },             {               \"vcpus\": {                 \"+lte\": 20               }             }           ]         }       ]     }' ``` ## Time Values  All times returned by the API are in UTC, regardless of the timezone configured within your user's profile (see `timezone` property within [Profile View](/docs/api/profile/#profile-view__responses)).  ## Rate Limiting  Rate limits on API requests help maintain the health and stability of the Linode API. Accordingly, every endpoint of the Linode API applies a rate limit on a per user basis as determined by OAuth token for authenticated requests or IP address for public endpoints.  Each rate limit consists of a total number of requests and a time window. For example, if an endpoint has a rate limit of 800 requests per minute, then up to 800 requests over a one minute window are permitted. Subsequent requests to an endpoint after hitting a rate limit return a 429 error. You can successfully remake requests to that endpoint after the rate limit window resets.  ### Linode APIv4 Rate Limits  With the Linode API, you can generally make up to 1,600 general API requests every two minutes. Additionally, all endpoints have a rate limit of 800 requests per minute unless otherwise specified below.  **Note:** There may be rate limiting applied at other levels outside of the API, for example, at the load balancer.  Creating Linodes has a dedicated rate limit of 10 requests per 30 seconds. That endpoint is:  * [Linode Create](/docs/api/linode-instances/#linode-create)  `/stats` endpoints have their own dedicated rate limits of 100 requests per minute. These endpoints are:  * [View Linode Statistics](/docs/api/linode-instances/#linode-statistics-view) * [View Linode Statistics (year/month)](/docs/api/linode-instances/#statistics-yearmonth-view) * [View NodeBalancer Statistics](/docs/api/nodebalancers/#nodebalancer-statistics-view) * [List Managed Stats](/docs/api/managed/#managed-stats-list)  Object Storage endpoints have a dedicated rate limit of 750 requests per second. The Object Storage endpoints are:  * [Object Storage Endpoints](/docs/api/object-storage/)  Opening Support Tickets has a dedicated rate limit of 2 requests per minute. That endpoint is:  * [Open Support Ticket](/docs/api/support/#support-ticket-open)  Accepting Service Transfers has a dedicated rate limit of 2 requests per minute. That endpoint is:  * [Service Transfer Accept](/docs/api/account/#service-transfer-accept)  ### Rate Limit HTTP Response Headers  The Linode API includes the following HTTP response headers which are designed to help you avoid hitting rate limits that might disrupt your applications:  * **X-RateLimit-Limit**: The maximum number of permitted requests during the rate limit window for this endpoint. * **X-RateLimit-Remaining**: The remaining number of permitted requests in the current rate limit window. * **X-RateLimit-Reset**: The time when the current rate limit window rests in UTC epoch seconds. * **Retry-After**: The remaining time in seconds until the current rate limit window resets.  There are many ways to access header information for your requests, depending on how you are accessing the Linode API. For example, to view HTTP response headers when making requests with `curl`, use the `-i` or `--include` option as follows:  ```Shell curl -i https://api.linode.com/v4/regions ```  ## CLI (Command Line Interface)  The <a href=\"https://github.com/linode/linode-cli\" target=\"_top\">Linode CLI</a> allows you to easily work with the API using intuitive and simple syntax. It requires a [Personal Access Token](/docs/api/#personal-access-token) for authentication, and gives you access to all of the features and functionality of the Linode API that are documented here with CLI examples.  Endpoints that do not have CLI examples are currently unavailable through the CLI, but can be accessed via other methods such as Shell commands and other third-party applications.
 *
 * The version of the OpenAPI document: 4.149.0
 * Contact: support@linode.com
 * Generated by: https://openapi-generator.tech
 */

/// Event : A collection of Event objects. An Event is an action taken against an entity related to your Account. For example, booting a Linode would create an Event. The Events returned depends on your grants.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Event {
    /// The unique ID of this Event.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// The action that caused this Event. New actions may be added in the future.
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
    /// When this Event was created.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// The total duration in seconds that it takes for the Event to complete.
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<f32>,
    #[serde(rename = "entity", skip_serializing_if = "Option::is_none")]
    pub entity: Option<Box<crate::models::EventEntity>>,
    #[serde(rename = "secondary_entity", skip_serializing_if = "Option::is_none")]
    pub secondary_entity: Option<Box<crate::models::EventSecondaryEntity>>,
    /// A percentage estimating the amount of time remaining for an Event. Returns `null` for notification events.
    #[serde(rename = "percent_complete", skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<i32>,
    /// The rate of completion of the Event. Only some Events will return rate; for example, migration and resize Events.
    #[serde(rename = "rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<String>,
    /// If this Event has been read.
    #[serde(rename = "read", skip_serializing_if = "Option::is_none")]
    pub read: Option<bool>,
    /// If this Event has been seen.
    #[serde(rename = "seen", skip_serializing_if = "Option::is_none")]
    pub seen: Option<bool>,
    /// The current status of this Event.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// The estimated time remaining until the completion of this Event. This value is only returned for some in-progress migration events. For all other in-progress events, the `percent_complete` attribute will indicate about how much more work is to be done.
    #[serde(
        rename = "time_remaining",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub time_remaining: Option<Option<String>>,
    /// The username of the User who caused the Event.
    #[serde(
        rename = "username",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub username: Option<Option<String>>,
    /// Provides additional information about the event. Additional information may include, but is not limited to, a more detailed representation of events which can help diagnose non-obvious failures.
    #[serde(
        rename = "message",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub message: Option<Option<String>>,
}

impl Event {
    /// A collection of Event objects. An Event is an action taken against an entity related to your Account. For example, booting a Linode would create an Event. The Events returned depends on your grants.
    pub fn new() -> Event {
        Event {
            id: None,
            action: None,
            created: None,
            duration: None,
            entity: None,
            secondary_entity: None,
            percent_complete: None,
            rate: None,
            read: None,
            seen: None,
            status: None,
            time_remaining: None,
            username: None,
            message: None,
        }
    }
}

/// The action that caused this Event. New actions may be added in the future.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "account_update")]
    AccountUpdate,
    #[serde(rename = "account_settings_update")]
    AccountSettingsUpdate,
    #[serde(rename = "backups_enable")]
    BackupsEnable,
    #[serde(rename = "backups_cancel")]
    BackupsCancel,
    #[serde(rename = "backups_restore")]
    BackupsRestore,
    #[serde(rename = "community_question_reply")]
    CommunityQuestionReply,
    #[serde(rename = "community_like")]
    CommunityLike,
    #[serde(rename = "credit_card_updated")]
    CreditCardUpdated,
    #[serde(rename = "disk_create")]
    DiskCreate,
    #[serde(rename = "disk_delete")]
    DiskDelete,
    #[serde(rename = "disk_update")]
    DiskUpdate,
    #[serde(rename = "disk_duplicate")]
    DiskDuplicate,
    #[serde(rename = "disk_imagize")]
    DiskImagize,
    #[serde(rename = "disk_resize")]
    DiskResize,
    #[serde(rename = "dns_record_create")]
    DnsRecordCreate,
    #[serde(rename = "dns_record_delete")]
    DnsRecordDelete,
    #[serde(rename = "dns_record_update")]
    DnsRecordUpdate,
    #[serde(rename = "dns_zone_create")]
    DnsZoneCreate,
    #[serde(rename = "dns_zone_delete")]
    DnsZoneDelete,
    #[serde(rename = "dns_zone_import")]
    DnsZoneImport,
    #[serde(rename = "dns_zone_update")]
    DnsZoneUpdate,
    #[serde(rename = "entity_transfer_accept")]
    EntityTransferAccept,
    #[serde(rename = "entity_transfer_cancel")]
    EntityTransferCancel,
    #[serde(rename = "entity_transfer_create")]
    EntityTransferCreate,
    #[serde(rename = "entity_transfer_fail")]
    EntityTransferFail,
    #[serde(rename = "entity_transfer_stale")]
    EntityTransferStale,
    #[serde(rename = "firewall_create")]
    FirewallCreate,
    #[serde(rename = "firewall_delete")]
    FirewallDelete,
    #[serde(rename = "firewall_disable")]
    FirewallDisable,
    #[serde(rename = "firewall_enable")]
    FirewallEnable,
    #[serde(rename = "firewall_update")]
    FirewallUpdate,
    #[serde(rename = "firewall_device_add")]
    FirewallDeviceAdd,
    #[serde(rename = "firewall_device_remove")]
    FirewallDeviceRemove,
    #[serde(rename = "host_reboot")]
    HostReboot,
    #[serde(rename = "image_delete")]
    ImageDelete,
    #[serde(rename = "image_update")]
    ImageUpdate,
    #[serde(rename = "image_upload")]
    ImageUpload,
    #[serde(rename = "ipaddress_update")]
    IpaddressUpdate,
    #[serde(rename = "lassie_reboot")]
    LassieReboot,
    #[serde(rename = "lish_boot")]
    LishBoot,
    #[serde(rename = "linode_addip")]
    LinodeAddip,
    #[serde(rename = "linode_boot")]
    LinodeBoot,
    #[serde(rename = "linode_clone")]
    LinodeClone,
    #[serde(rename = "linode_create")]
    LinodeCreate,
    #[serde(rename = "linode_delete")]
    LinodeDelete,
    #[serde(rename = "linode_update")]
    LinodeUpdate,
    #[serde(rename = "linode_deleteip")]
    LinodeDeleteip,
    #[serde(rename = "linode_migrate")]
    LinodeMigrate,
    #[serde(rename = "linode_migrate_datacenter")]
    LinodeMigrateDatacenter,
    #[serde(rename = "linode_migrate_datacenter_create")]
    LinodeMigrateDatacenterCreate,
    #[serde(rename = "linode_mutate")]
    LinodeMutate,
    #[serde(rename = "linode_mutate_create")]
    LinodeMutateCreate,
    #[serde(rename = "linode_reboot")]
    LinodeReboot,
    #[serde(rename = "linode_rebuild")]
    LinodeRebuild,
    #[serde(rename = "linode_resize")]
    LinodeResize,
    #[serde(rename = "linode_resize_create")]
    LinodeResizeCreate,
    #[serde(rename = "linode_shutdown")]
    LinodeShutdown,
    #[serde(rename = "linode_snapshot")]
    LinodeSnapshot,
    #[serde(rename = "linode_config_create")]
    LinodeConfigCreate,
    #[serde(rename = "linode_config_delete")]
    LinodeConfigDelete,
    #[serde(rename = "linode_config_update")]
    LinodeConfigUpdate,
    #[serde(rename = "lke_node_create")]
    LkeNodeCreate,
    #[serde(rename = "longviewclient_create")]
    LongviewclientCreate,
    #[serde(rename = "longviewclient_delete")]
    LongviewclientDelete,
    #[serde(rename = "longviewclient_update")]
    LongviewclientUpdate,
    #[serde(rename = "managed_disabled")]
    ManagedDisabled,
    #[serde(rename = "managed_enabled")]
    ManagedEnabled,
    #[serde(rename = "managed_service_create")]
    ManagedServiceCreate,
    #[serde(rename = "managed_service_delete")]
    ManagedServiceDelete,
    #[serde(rename = "nodebalancer_create")]
    NodebalancerCreate,
    #[serde(rename = "nodebalancer_delete")]
    NodebalancerDelete,
    #[serde(rename = "nodebalancer_update")]
    NodebalancerUpdate,
    #[serde(rename = "nodebalancer_config_create")]
    NodebalancerConfigCreate,
    #[serde(rename = "nodebalancer_config_delete")]
    NodebalancerConfigDelete,
    #[serde(rename = "nodebalancer_config_update")]
    NodebalancerConfigUpdate,
    #[serde(rename = "nodebalancer_node_create")]
    NodebalancerNodeCreate,
    #[serde(rename = "nodebalancer_node_delete")]
    NodebalancerNodeDelete,
    #[serde(rename = "nodebalancer_node_update")]
    NodebalancerNodeUpdate,
    #[serde(rename = "oauth_client_create")]
    OauthClientCreate,
    #[serde(rename = "oauth_client_delete")]
    OauthClientDelete,
    #[serde(rename = "oauth_client_secret_reset")]
    OauthClientSecretReset,
    #[serde(rename = "oauth_client_update")]
    OauthClientUpdate,
    #[serde(rename = "password_reset")]
    PasswordReset,
    #[serde(rename = "payment_method_add")]
    PaymentMethodAdd,
    #[serde(rename = "payment_submitted")]
    PaymentSubmitted,
    #[serde(rename = "profile_update")]
    ProfileUpdate,
    #[serde(rename = "stackscript_create")]
    StackscriptCreate,
    #[serde(rename = "stackscript_delete")]
    StackscriptDelete,
    #[serde(rename = "stackscript_update")]
    StackscriptUpdate,
    #[serde(rename = "stackscript_publicize")]
    StackscriptPublicize,
    #[serde(rename = "stackscript_revise")]
    StackscriptRevise,
    #[serde(rename = "tag_create")]
    TagCreate,
    #[serde(rename = "tag_delete")]
    TagDelete,
    #[serde(rename = "tfa_disabled")]
    TfaDisabled,
    #[serde(rename = "tfa_enabled")]
    TfaEnabled,
    #[serde(rename = "ticket_attachment_upload")]
    TicketAttachmentUpload,
    #[serde(rename = "ticket_create")]
    TicketCreate,
    #[serde(rename = "ticket_update")]
    TicketUpdate,
    #[serde(rename = "token_create")]
    TokenCreate,
    #[serde(rename = "token_delete")]
    TokenDelete,
    #[serde(rename = "token_update")]
    TokenUpdate,
    #[serde(rename = "user_create")]
    UserCreate,
    #[serde(rename = "user_update")]
    UserUpdate,
    #[serde(rename = "user_delete")]
    UserDelete,
    #[serde(rename = "user_ssh_key_add")]
    UserSshKeyAdd,
    #[serde(rename = "user_ssh_key_delete")]
    UserSshKeyDelete,
    #[serde(rename = "user_ssh_key_update")]
    UserSshKeyUpdate,
    #[serde(rename = "vlan_attach")]
    VlanAttach,
    #[serde(rename = "vlan_detach")]
    VlanDetach,
    #[serde(rename = "volume_attach")]
    VolumeAttach,
    #[serde(rename = "volume_clone")]
    VolumeClone,
    #[serde(rename = "volume_create")]
    VolumeCreate,
    #[serde(rename = "volume_delete")]
    VolumeDelete,
    #[serde(rename = "volume_update")]
    VolumeUpdate,
    #[serde(rename = "volume_detach")]
    VolumeDetach,
    #[serde(rename = "volume_resize")]
    VolumeResize,
}

impl Default for Action {
    fn default() -> Action {
        Self::AccountUpdate
    }
}
/// The current status of this Event.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "finished")]
    Finished,
    #[serde(rename = "notification")]
    Notification,
    #[serde(rename = "scheduled")]
    Scheduled,
    #[serde(rename = "started")]
    Started,
}

impl Default for Status {
    fn default() -> Status {
        Self::Failed
    }
}
