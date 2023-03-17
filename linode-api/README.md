# Rust API client for linode-api

## Introduction
The Linode API provides the ability to programmatically manage the full
range of Linode products and services.

This reference is designed to assist application developers and system
administrators.  Each endpoint includes descriptions, request syntax, and
examples using standard HTTP requests. Response data is returned in JSON
format.


This document was generated from our OpenAPI Specification.  See the
<a target=\"_top\" href=\"https://www.openapis.org\">OpenAPI website</a> for more information.

<a target=\"_top\" href=\"/docs/api/openapi.yaml\">Download the Linode OpenAPI Specification</a>.


## Changelog

<a target=\"_top\" href=\"/docs/products/tools/api/release-notes/\">View our Changelog</a> to see release
notes on all changes made to our API.

## Access and Authentication

Some endpoints are publicly accessible without requiring authentication.
All endpoints affecting your Account, however, require either a Personal
Access Token or OAuth authentication (when using third-party
applications).

### Personal Access Token

The easiest way to access the API is with a Personal Access Token (PAT)
generated from the
<a target=\"_top\" href=\"https://cloud.linode.com/profile/tokens\">Linode Cloud Manager</a> or
the [Create Personal Access Token](/docs/api/profile/#personal-access-token-create) endpoint.

All scopes for the OAuth security model ([defined below](/docs/api/profile/#oauth)) apply to this
security model as well.

#### Authentication

| Security Scheme Type: | HTTP |
|-----------------------|------|
| **HTTP Authorization Scheme** | bearer |

### OAuth
If you only need to access the Linode API for personal use,
we recommend that you create a [personal access token](/docs/api/#personal-access-token).
If you're designing an application that can authenticate with an arbitrary Linode user, then
you should use the OAuth 2.0 workflows presented in this section.

For a more detailed example of an OAuth 2.0 implementation, see our guide on [How to Create an OAuth App with the Linode Python API Library](/docs/products/tools/api/guides/create-an-oauth-app-with-the-python-api-library/#oauth-2-authentication-exchange).

Before you implement OAuth in your application, you first need to create an OAuth client. You can do this [with the Linode API](/docs/api/account/#oauth-client-create) or [via the Cloud Manager](https://cloud.linode.com/profile/clients):

  - When creating the client, you'll supply a `label` and a `redirect_uri` (referred to as the Callback URL in the Cloud Manager).
  - The response from this endpoint will give you a `client_id` and a `secret`.
  - Clients can be public or private, and are private by default. You can choose to make the client public when it is created.
    - A private client is used with applications which can securely store the client secret (that is, the secret returned to you when you first created the client). For example, an application running on a secured server that only the developer has access to would use a private OAuth client. This is also called a confidential client in some OAuth documentation.
    - A public client is used with applications where the client secret is not guaranteed to be secure. For example, a native app running on a user's computer may not be able to keep the client secret safe, as a user could potentially inspect the source of the application. So, native apps or apps that run in a user's browser should use a public client.
    - Public and private clients follow different workflows, as described below.

#### OAuth Workflow

The OAuth workflow is a series of exchanges between your third-party app and Linode. The workflow is used
to authenticate a user before an application can start making API calls on the user's behalf.

Notes:

- With respect to the diagram in [section 1.2 of RFC 6749](https://tools.ietf.org/html/rfc6749#section-1.2), login.linode.com (referred to in this section as the *login server*)
is the Resource Owner and the Authorization Server; api.linode.com (referred to here as the *api server*) is the Resource Server.
- The OAuth spec refers to the private and public workflows listed below as the [authorization code flow](https://tools.ietf.org/html/rfc6749#section-1.3.1) and [implicit flow](https://tools.ietf.org/html/rfc6749#section-1.3.2).

| PRIVATE WORKFLOW | PUBLIC WORKFLOW |
|------------------|------------------|
| 1.  The user visits the application's website and is directed to login with Linode. | 1.  The user visits the application's website and is directed to login with Linode. |
| 2.  Your application then redirects the user to Linode's [login server](https://login.linode.com) with the client application's `client_id` and requested OAuth `scope`, which should appear in the URL of the login page. | 2.  Your application then redirects the user to Linode's [login server](https://login.linode.com) with the client application's `client_id` and requested OAuth `scope`, which should appear in the URL of the login page. |
| 3.  The user logs into the login server with their username and password. | 3.  The user logs into the login server with their username and password. |
| 4.  The login server redirects the user to the specificed redirect URL with a temporary authorization `code` (exchange code) in the URL. | 4.  The login server redirects the user back to your application with an OAuth `access_token` embedded in the redirect URL's hash. This is temporary and expires in two hours. No `refresh_token` is issued. Therefore, once the `access_token` expires, a new one will need to be issued by having the user log in again. |
| 5.  The application issues a POST request (*see additional details below*) to the login server with the exchange code, `client_id`, and the client application's `client_secret`. | |
| 6.  The login server responds to the client application with a new OAuth `access_token` and `refresh_token`. The `access_token` is set to expire in two hours. | |
| 7.  The `refresh_token` can be used by contacting the login server with the `client_id`, `client_secret`, `grant_type`, and `refresh_token` to get a new OAuth `access_token` and `refresh_token`. The new `access_token` is good for another two hours, and the new `refresh_token` can be used to extend the session again by this same method (*see additional details below*). | |

#### OAuth Private Workflow - Additional Details

The following information expands on steps 5 through 7 of the private workflow:

Once the user has logged into Linode and you have received an exchange code,
you will need to trade that exchange code for an `access_token` and `refresh_token`. You
do this by making an HTTP POST request to the following address:

```
https://login.linode.com/oauth/token
```

Make this request as `application/x-www-form-urlencoded` or as
`multipart/form-data` and include the following parameters in the POST body:

| PARAMETER | DESCRIPTION |
|-----------|-------------|
| client_id | Your app's client ID. |
| client_secret | Your app's client secret. |
| code | The code you just received from the redirect. |

You'll get a response like this:

```json
{
  \"scope\": \"linodes:read_write\",
  \"access_token\": \"03d084436a6c91fbafd5c4b20c82e5056a2e9ce1635920c30dc8d81dc7a6665c\",
  \"refresh_token\": \"f2ec9712e616fdb5a2a21aa0e88cfadea7502ebc62cf5bd758dbcd65e1803bad\",
  \"token_type\": \"bearer\",
  \"expires_in\": 7200
}
```

Included in the response is an `access_token`. With this token, you can proceed to make
authenticated HTTP requests to the API by adding this header to each request:

```
Authorization: Bearer 03d084436a6c91fbafd5c4b20c82e5056a2e9ce1635920c30dc8d81dc7a6665c
```

This `access_token` is set to expire in two hours. To refresh access prior to expiration, make another request to the same URL with the following parameters in the POST body:

| PARAMETER | DESCRIPTION |
|-----------|-------------|
| grant_type | The grant type you're using. Use \"refresh_token\" when refreshing access. |
| client_id | Your app's client ID. |
| client_secret | Your app's client secret. |
| refresh_token | The `refresh_token` received from the previous response. |

You'll get another response with an updated `access_token` and `refresh_token`, which can then be used to refresh access again.

#### OAuth Reference

| Security Scheme Type | OAuth 2.0 |
|-----------------------|--------|
| **Authorization URL** | https://login.linode.com/oauth/authorize |
| **Token URL** | https://login.linode.com/oauth/token |
| **Scopes** | <ul><li>`account:read_only` - Allows access to GET information about your Account.</li><li>`account:read_write` - Allows access to all endpoints related to your Account.</li><li>`databases:read_only` - Allows access to GET Managed Databases on your Account.</li><li>`databases:read_write` - Allows access to all endpoints related to your Managed Databases.</li><li>`domains:read_only` - Allows access to GET Domains on your Account.</li><li>`domains:read_write` - Allows access to all Domain endpoints.</li><li>`events:read_only` - Allows access to GET your Events.</li><li>`events:read_write` - Allows access to all endpoints related to your Events.</li><li>`firewall:read_only` - Allows access to GET information about your Firewalls.</li><li>`firewall:read_write` - Allows access to all Firewall endpoints.</li><li>`images:read_only` - Allows access to GET your Images.</li><li>`images:read_write` - Allows access to all endpoints related to your Images.</li><li>`ips:read_only` - Allows access to GET your ips.</li><li>`ips:read_write` - Allows access to all endpoints related to your ips.</li><li>`linodes:read_only` - Allows access to GET Linodes on your Account.</li><li>`linodes:read_write` - Allow access to all endpoints related to your Linodes.</li><li>`lke:read_only` - Allows access to GET LKE Clusters on your Account.</li><li>`lke:read_write` - Allows access to all endpoints related to LKE Clusters on your Account.</li><li>`longview:read_only` - Allows access to GET your Longview Clients.</li><li>`longview:read_write` - Allows access to all endpoints related to your Longview Clients.</li><li>`nodebalancers:read_only` - Allows access to GET NodeBalancers on your Account.</li><li>`nodebalancers:read_write` - Allows access to all NodeBalancer endpoints.</li><li>`object_storage:read_only` - Allows access to GET information related to your Object Storage.</li><li>`object_storage:read_write` - Allows access to all Object Storage endpoints.</li><li>`stackscripts:read_only` - Allows access to GET your StackScripts.</li><li>`stackscripts:read_write` - Allows access to all endpoints related to your StackScripts.</li><li>`volumes:read_only` - Allows access to GET your Volumes.</li><li>`volumes:read_write` - Allows access to all endpoints related to your Volumes.</li></ul><br/>|

## Requests

Requests must be made over HTTPS to ensure transactions are encrypted. The
following Request methods are supported:

| METHOD | USAGE |
|--------|-------|
| GET    | Retrieves data about collections and individual resources. |
| POST   | For collections, creates a new resource of that type. Also used to perform actions on action endpoints. |
| PUT    | Updates an existing resource. |
| DELETE | Deletes a resource. This is a destructive action. |


## Responses

Actions will return one following HTTP response status codes:

| STATUS  | DESCRIPTION |
|---------|-------------|
| 200 OK  | The request was successful. |
| 202 Accepted | The request was successful, but processing has not been completed. The response body includes a \"warnings\" array containing the details of incomplete processes. |
| 204 No Content | The server successfully fulfilled the request and there is no additional content to send. |
| 299 Deprecated | The request was successful, but involved a deprecated endpoint. The response body includes a \"warnings\" array containing warning messages. |
| 400 Bad Request | You submitted an invalid request (missing parameters, etc.). |
| 401 Unauthorized | You failed to authenticate for this resource. |
| 403 Forbidden | You are authenticated, but don't have permission to do this. |
| 404 Not Found | The resource you're requesting does not exist. |
| 429 Too Many Requests | You've hit a rate limit. |
| 500 Internal Server Error | Please [open a Support Ticket](/docs/api/support/#support-ticket-open). |

## Errors

Success is indicated via <a href=\"https://en.wikipedia.org/wiki/List_of_HTTP_status_codes\" target=\"_top\">Standard HTTP status codes</a>.
`2xx` codes indicate success, `4xx` codes indicate a request error, and
`5xx` errors indicate a server error. A
request error might be an invalid input, a required parameter being omitted,
or a malformed request. A server error means something went wrong processing
your request. If this occurs, please
[open a Support Ticket](/docs/api/support/#support-ticket-open)
and let us know. Though errors are logged and we work quickly to resolve issues,
opening a ticket and providing us with reproducable steps and data is always helpful.

The `errors` field is an array of the things that went wrong with your request.
We will try to include as many of the problems in the response as possible,
but it's conceivable that fixing these errors and resubmitting may result in
new errors coming back once we are able to get further along in the process
of handling your request.


Within each error object, the `field` parameter will be included if the error
pertains to a specific field in the JSON you've submitted. This will be
omitted if there is no relevant field. The `reason` is a human-readable
explanation of the error, and will always be included.

## Pagination

Resource lists are always paginated. The response will look similar to this:

```json
{
    \"data\": [ ... ],
    \"page\": 1,
    \"pages\": 3,
    \"results\": 300
}
```

* Pages start at 1. You may retrieve a specific page of results by adding
`?page=x` to your URL (for example, `?page=4`). If the value of `page`
exceeds `2^64/page_size`, the last possible page will be returned.


* Page sizes default to 100,
and can be set to return between 25 and 500. Page size can be set using
`?page_size=x`.

## Filtering and Sorting

Collections are searchable by fields they include, marked in the spec as
`x-linode-filterable: true`. Filters are passed
in the `X-Filter` header and are formatted as JSON objects. Here is a request
call for Linode Types in our \"standard\" class:

```Shell
curl \"https://api.linode.com/v4/linode/types\" \\
  -H 'X-Filter: { \"class\": \"standard\" }'
```

The filter object's keys are the keys of the object you're filtering,
and the values are accepted values. You can add multiple filters by
including more than one key. For example, filtering for \"standard\" Linode
Types that offer one vcpu:

```Shell
 curl \"https://api.linode.com/v4/linode/types\" \\
  -H 'X-Filter: { \"class\": \"standard\", \"vcpus\": 1 }'
```

In the above example, both filters are combined with an \"and\" operation.
However, if you wanted either Types with one vcpu or Types in our \"standard\"
class, you can add an operator:

 ```Shell
curl \"https://api.linode.com/v4/linode/types\" \\
  -H 'X-Filter: { \"+or\": [ { \"vcpus\": 1 }, { \"class\": \"standard\" } ] }'
```

Each filter in the `+or` array is its own filter object, and all conditions
in it are combined with an \"and\" operation as they were in the previous example.

Other operators are also available. Operators are keys of a Filter JSON
object. Their value must be of the appropriate type, and they are evaluated
as described below:

| OPERATOR | TYPE   | DESCRIPTION                       |
|----------|--------|-----------------------------------|
| +and     | array  | All conditions must be true.       |
| +or      | array  | One condition must be true.        |
| +gt      | number | Value must be greater than number. |
| +gte     | number | Value must be greater than or equal to number. |
| +lt      | number | Value must be less than number. |
| +lte     | number | Value must be less than or equal to number. |
| +contains | string | Given string must be in the value. |
| +neq      | string | Does not equal the value.          |
| +order_by | string | Attribute to order the results by - must be filterable. |
| +order    | string | Either \"asc\" or \"desc\". Defaults to \"asc\". Requires `+order_by`. |

For example, filtering for [Linode Types](/docs/api/linode-types/)
that offer memory equal to or higher than 61440:

```Shell
curl \"https://api.linode.com/v4/linode/types\" \\
  -H '
    X-Filter: {
      \"memory\": {
        \"+gte\": 61440
      }
    }'
```

You can combine and nest operators to construct arbitrarily-complex queries.
For example, give me all [Linode Types](/docs/api/linode-types/)
which are either `standard` or `highmem` class, or
have between 12 and 20 vcpus:

```Shell
curl \"https://api.linode.com/v4/linode/types\" \\
  -H '
    X-Filter: {
      \"+or\": [
        {
          \"+or\": [
            {
              \"class\": \"standard\"
            },
            {
              \"class\": \"highmem\"
            }
          ]
        },
        {
          \"+and\": [
            {
              \"vcpus\": {
                \"+gte\": 12
              }
            },
            {
              \"vcpus\": {
                \"+lte\": 20
              }
            }
          ]
        }
      ]
    }'
```
## Time Values

All times returned by the API are in UTC, regardless of the timezone configured within your user's profile (see `timezone` property within [Profile View](/docs/api/profile/#profile-view__responses)).

## Rate Limiting

Rate limits on API requests help maintain the health and stability of the Linode API. Accordingly, every endpoint of the Linode API applies a rate limit on a per user basis as determined by OAuth token for authenticated requests or IP address for public endpoints.

Each rate limit consists of a total number of requests and a time window. For example, if an endpoint has a rate limit of 800 requests per minute, then up to 800 requests over a one minute window are permitted. Subsequent requests to an endpoint after hitting a rate limit return a 429 error. You can successfully remake requests to that endpoint after the rate limit window resets.

### Linode APIv4 Rate Limits

With the Linode API, you can generally make up to 1,600 general API requests every two minutes. Additionally, all endpoints have a rate limit of 800 requests per minute unless otherwise specified below.

**Note:** There may be rate limiting applied at other levels outside of the API, for example, at the load balancer.

Creating Linodes has a dedicated rate limit of 10 requests per 30 seconds. That endpoint is:

* [Linode Create](/docs/api/linode-instances/#linode-create)

`/stats` endpoints have their own dedicated rate limits of 100 requests per minute. These endpoints are:

* [View Linode Statistics](/docs/api/linode-instances/#linode-statistics-view)
* [View Linode Statistics (year/month)](/docs/api/linode-instances/#statistics-yearmonth-view)
* [View NodeBalancer Statistics](/docs/api/nodebalancers/#nodebalancer-statistics-view)
* [List Managed Stats](/docs/api/managed/#managed-stats-list)

Object Storage endpoints have a dedicated rate limit of 750 requests per second. The Object Storage endpoints are:

* [Object Storage Endpoints](/docs/api/object-storage/)

Opening Support Tickets has a dedicated rate limit of 2 requests per minute. That endpoint is:

* [Open Support Ticket](/docs/api/support/#support-ticket-open)

Accepting Service Transfers has a dedicated rate limit of 2 requests per minute. That endpoint is:

* [Service Transfer Accept](/docs/api/account/#service-transfer-accept)

### Rate Limit HTTP Response Headers

The Linode API includes the following HTTP response headers which are designed to help you avoid hitting rate limits that might disrupt your applications:

* **X-RateLimit-Limit**: The maximum number of permitted requests during the rate limit window for this endpoint.
* **X-RateLimit-Remaining**: The remaining number of permitted requests in the current rate limit window.
* **X-RateLimit-Reset**: The time when the current rate limit window rests in UTC epoch seconds.
* **Retry-After**: The remaining time in seconds until the current rate limit window resets.

There are many ways to access header information for your requests, depending on how you are accessing the Linode API. For example, to view HTTP response headers when making requests with `curl`, use the `-i` or `--include` option as follows:

```Shell
curl -i https://api.linode.com/v4/regions
```

## CLI (Command Line Interface)

The <a href=\"https://github.com/linode/linode-cli\" target=\"_top\">Linode CLI</a> allows you to easily
work with the API using intuitive and simple syntax. It requires a
[Personal Access Token](/docs/api/#personal-access-token)
for authentication, and gives you access to all of the features and functionality
of the Linode API that are documented here with CLI examples.

Endpoints that do not have CLI examples are currently unavailable through the CLI, but
can be accessed via other methods such as Shell commands and other third-party applications.


For more information, please visit [https://linode.com](https://linode.com)

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 4.145.0
- Package version: 4.145.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `linode-api` and add the following to `Cargo.toml` under `[dependencies]`:

```
linode-api = { path = "./linode-api" }
```

## Documentation for API Endpoints

All URIs are relative to *https://api.linode.com/v4*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*LinodeInstancesApi* | [**add_linode_config**](docs/LinodeInstancesApi.md#add_linode_config) | **POST** /linode/instances/{linodeId}/configs | Configuration Profile Create
*LinodeInstancesApi* | [**add_linode_disk**](docs/LinodeInstancesApi.md#add_linode_disk) | **POST** /linode/instances/{linodeId}/disks | Disk Create
*LinodeInstancesApi* | [**add_linode_ip**](docs/LinodeInstancesApi.md#add_linode_ip) | **POST** /linode/instances/{linodeId}/ips | IPv4 Address Allocate
*LinodeInstancesApi* | [**boot_linode_instance**](docs/LinodeInstancesApi.md#boot_linode_instance) | **POST** /linode/instances/{linodeId}/boot | Linode Boot
*LinodeInstancesApi* | [**cancel_backups**](docs/LinodeInstancesApi.md#cancel_backups) | **POST** /linode/instances/{linodeId}/backups/cancel | Backups Cancel
*LinodeInstancesApi* | [**clone_linode_disk**](docs/LinodeInstancesApi.md#clone_linode_disk) | **POST** /linode/instances/{linodeId}/disks/{diskId}/clone | Disk Clone
*LinodeInstancesApi* | [**clone_linode_instance**](docs/LinodeInstancesApi.md#clone_linode_instance) | **POST** /linode/instances/{linodeId}/clone | Linode Clone
*LinodeInstancesApi* | [**create_linode_instance**](docs/LinodeInstancesApi.md#create_linode_instance) | **POST** /linode/instances | Linode Create
*LinodeInstancesApi* | [**create_snapshot**](docs/LinodeInstancesApi.md#create_snapshot) | **POST** /linode/instances/{linodeId}/backups | Snapshot Create
*LinodeInstancesApi* | [**delete_disk**](docs/LinodeInstancesApi.md#delete_disk) | **DELETE** /linode/instances/{linodeId}/disks/{diskId} | Disk Delete
*LinodeInstancesApi* | [**delete_linode_config**](docs/LinodeInstancesApi.md#delete_linode_config) | **DELETE** /linode/instances/{linodeId}/configs/{configId} | Configuration Profile Delete
*LinodeInstancesApi* | [**delete_linode_instance**](docs/LinodeInstancesApi.md#delete_linode_instance) | **DELETE** /linode/instances/{linodeId} | Linode Delete
*LinodeInstancesApi* | [**enable_backups**](docs/LinodeInstancesApi.md#enable_backups) | **POST** /linode/instances/{linodeId}/backups/enable | Backups Enable
*LinodeInstancesApi* | [**get_backup**](docs/LinodeInstancesApi.md#get_backup) | **GET** /linode/instances/{linodeId}/backups/{backupId} | Backup View
*LinodeInstancesApi* | [**get_backups**](docs/LinodeInstancesApi.md#get_backups) | **GET** /linode/instances/{linodeId}/backups | Backups List
*LinodeInstancesApi* | [**get_linode_config**](docs/LinodeInstancesApi.md#get_linode_config) | **GET** /linode/instances/{linodeId}/configs/{configId} | Configuration Profile View
*LinodeInstancesApi* | [**get_linode_configs**](docs/LinodeInstancesApi.md#get_linode_configs) | **GET** /linode/instances/{linodeId}/configs | Configuration Profiles List
*LinodeInstancesApi* | [**get_linode_disk**](docs/LinodeInstancesApi.md#get_linode_disk) | **GET** /linode/instances/{linodeId}/disks/{diskId} | Disk View
*LinodeInstancesApi* | [**get_linode_disks**](docs/LinodeInstancesApi.md#get_linode_disks) | **GET** /linode/instances/{linodeId}/disks | Disks List
*LinodeInstancesApi* | [**get_linode_firewalls**](docs/LinodeInstancesApi.md#get_linode_firewalls) | **GET** /linode/instances/{linodeId}/firewalls | Firewalls List
*LinodeInstancesApi* | [**get_linode_instance**](docs/LinodeInstancesApi.md#get_linode_instance) | **GET** /linode/instances/{linodeId} | Linode View
*LinodeInstancesApi* | [**get_linode_instances**](docs/LinodeInstancesApi.md#get_linode_instances) | **GET** /linode/instances | Linodes List
*LinodeInstancesApi* | [**get_linode_ip**](docs/LinodeInstancesApi.md#get_linode_ip) | **GET** /linode/instances/{linodeId}/ips/{address} | IP Address View
*LinodeInstancesApi* | [**get_linode_ips**](docs/LinodeInstancesApi.md#get_linode_ips) | **GET** /linode/instances/{linodeId}/ips | Networking Information List
*LinodeInstancesApi* | [**get_linode_node_balancers**](docs/LinodeInstancesApi.md#get_linode_node_balancers) | **GET** /linode/instances/{linodeId}/nodebalancers | Linode NodeBalancers View
*LinodeInstancesApi* | [**get_linode_stats**](docs/LinodeInstancesApi.md#get_linode_stats) | **GET** /linode/instances/{linodeId}/stats | Linode Statistics View
*LinodeInstancesApi* | [**get_linode_stats_by_year_month**](docs/LinodeInstancesApi.md#get_linode_stats_by_year_month) | **GET** /linode/instances/{linodeId}/stats/{year}/{month} | Statistics View (year/month)
*LinodeInstancesApi* | [**get_linode_transfer**](docs/LinodeInstancesApi.md#get_linode_transfer) | **GET** /linode/instances/{linodeId}/transfer | Network Transfer View
*LinodeInstancesApi* | [**get_linode_transfer_by_year_month**](docs/LinodeInstancesApi.md#get_linode_transfer_by_year_month) | **GET** /linode/instances/{linodeId}/transfer/{year}/{month} | Network Transfer View (year/month)
*LinodeInstancesApi* | [**get_linode_volumes**](docs/LinodeInstancesApi.md#get_linode_volumes) | **GET** /linode/instances/{linodeId}/volumes | Linode's Volumes List
*LinodeInstancesApi* | [**migrate_linode_instance**](docs/LinodeInstancesApi.md#migrate_linode_instance) | **POST** /linode/instances/{linodeId}/migrate | DC Migration/Pending Host Migration Initiate
*LinodeInstancesApi* | [**mutate_linode_instance**](docs/LinodeInstancesApi.md#mutate_linode_instance) | **POST** /linode/instances/{linodeId}/mutate | Linode Upgrade
*LinodeInstancesApi* | [**reboot_linode_instance**](docs/LinodeInstancesApi.md#reboot_linode_instance) | **POST** /linode/instances/{linodeId}/reboot | Linode Reboot
*LinodeInstancesApi* | [**rebuild_linode_instance**](docs/LinodeInstancesApi.md#rebuild_linode_instance) | **POST** /linode/instances/{linodeId}/rebuild | Linode Rebuild
*LinodeInstancesApi* | [**remove_linode_ip**](docs/LinodeInstancesApi.md#remove_linode_ip) | **DELETE** /linode/instances/{linodeId}/ips/{address} | IPv4 Address Delete
*LinodeInstancesApi* | [**rescue_linode_instance**](docs/LinodeInstancesApi.md#rescue_linode_instance) | **POST** /linode/instances/{linodeId}/rescue | Linode Boot into Rescue Mode
*LinodeInstancesApi* | [**reset_disk_password**](docs/LinodeInstancesApi.md#reset_disk_password) | **POST** /linode/instances/{linodeId}/disks/{diskId}/password | Disk Root Password Reset
*LinodeInstancesApi* | [**reset_linode_password**](docs/LinodeInstancesApi.md#reset_linode_password) | **POST** /linode/instances/{linodeId}/password | Linode Root Password Reset
*LinodeInstancesApi* | [**resize_disk**](docs/LinodeInstancesApi.md#resize_disk) | **POST** /linode/instances/{linodeId}/disks/{diskId}/resize | Disk Resize
*LinodeInstancesApi* | [**resize_linode_instance**](docs/LinodeInstancesApi.md#resize_linode_instance) | **POST** /linode/instances/{linodeId}/resize | Linode Resize
*LinodeInstancesApi* | [**restore_backup**](docs/LinodeInstancesApi.md#restore_backup) | **POST** /linode/instances/{linodeId}/backups/{backupId}/restore | Backup Restore
*LinodeInstancesApi* | [**shutdown_linode_instance**](docs/LinodeInstancesApi.md#shutdown_linode_instance) | **POST** /linode/instances/{linodeId}/shutdown | Linode Shut Down
*LinodeInstancesApi* | [**update_disk**](docs/LinodeInstancesApi.md#update_disk) | **PUT** /linode/instances/{linodeId}/disks/{diskId} | Disk Update
*LinodeInstancesApi* | [**update_linode_config**](docs/LinodeInstancesApi.md#update_linode_config) | **PUT** /linode/instances/{linodeId}/configs/{configId} | Configuration Profile Update
*LinodeInstancesApi* | [**update_linode_instance**](docs/LinodeInstancesApi.md#update_linode_instance) | **PUT** /linode/instances/{linodeId} | Linode Update
*LinodeInstancesApi* | [**update_linode_ip**](docs/LinodeInstancesApi.md#update_linode_ip) | **PUT** /linode/instances/{linodeId}/ips/{address} | IP Address Update
*LinodeTypesApi* | [**get_linode_type**](docs/LinodeTypesApi.md#get_linode_type) | **GET** /linode/types/{typeId} | Type View
*LinodeTypesApi* | [**get_linode_types**](docs/LinodeTypesApi.md#get_linode_types) | **GET** /linode/types | Types List
*VolumesApi* | [**attach_volume**](docs/VolumesApi.md#attach_volume) | **POST** /volumes/{volumeId}/attach | Volume Attach
*VolumesApi* | [**clone_volume**](docs/VolumesApi.md#clone_volume) | **POST** /volumes/{volumeId}/clone | Volume Clone
*VolumesApi* | [**create_volume**](docs/VolumesApi.md#create_volume) | **POST** /volumes | Volume Create
*VolumesApi* | [**delete_volume**](docs/VolumesApi.md#delete_volume) | **DELETE** /volumes/{volumeId} | Volume Delete
*VolumesApi* | [**detach_volume**](docs/VolumesApi.md#detach_volume) | **POST** /volumes/{volumeId}/detach | Volume Detach
*VolumesApi* | [**get_volume**](docs/VolumesApi.md#get_volume) | **GET** /volumes/{volumeId} | Volume View
*VolumesApi* | [**get_volumes**](docs/VolumesApi.md#get_volumes) | **GET** /volumes | Volumes List
*VolumesApi* | [**resize_volume**](docs/VolumesApi.md#resize_volume) | **POST** /volumes/{volumeId}/resize | Volume Resize
*VolumesApi* | [**update_volume**](docs/VolumesApi.md#update_volume) | **PUT** /volumes/{volumeId} | Volume Update


## Documentation For Models

 - [Account](docs/Account.md)
 - [AccountActivePromotionsInner](docs/AccountActivePromotionsInner.md)
 - [AccountCreditCard](docs/AccountCreditCard.md)
 - [AccountSettings](docs/AccountSettings.md)
 - [AddLinodeConfigRequest](docs/AddLinodeConfigRequest.md)
 - [AddLinodeDiskRequest](docs/AddLinodeDiskRequest.md)
 - [AddLinodeDiskRequestAllOf](docs/AddLinodeDiskRequestAllOf.md)
 - [AddLinodeIpRequest](docs/AddLinodeIpRequest.md)
 - [AttachVolumeRequest](docs/AttachVolumeRequest.md)
 - [AuthorizedApp](docs/AuthorizedApp.md)
 - [Backup](docs/Backup.md)
 - [BootLinodeInstanceRequest](docs/BootLinodeInstanceRequest.md)
 - [CloneLinodeInstanceRequest](docs/CloneLinodeInstanceRequest.md)
 - [CloneVolumeRequest](docs/CloneVolumeRequest.md)
 - [CreateLinodeInstanceRequest](docs/CreateLinodeInstanceRequest.md)
 - [CreateLinodeInstanceRequestAllOf](docs/CreateLinodeInstanceRequestAllOf.md)
 - [CreateLinodeInstanceRequestAllOf1](docs/CreateLinodeInstanceRequestAllOf1.md)
 - [CreateLinodeInstanceRequestAllOf1InterfacesInner](docs/CreateLinodeInstanceRequestAllOf1InterfacesInner.md)
 - [CreateSnapshotRequest](docs/CreateSnapshotRequest.md)
 - [CreateVolumeRequest](docs/CreateVolumeRequest.md)
 - [CreditCard](docs/CreditCard.md)
 - [CreditCardData](docs/CreditCardData.md)
 - [Database](docs/Database.md)
 - [DatabaseBackup](docs/DatabaseBackup.md)
 - [DatabaseBackupSnapshot](docs/DatabaseBackupSnapshot.md)
 - [DatabaseCredentials](docs/DatabaseCredentials.md)
 - [DatabaseEngine](docs/DatabaseEngine.md)
 - [DatabaseHosts](docs/DatabaseHosts.md)
 - [DatabaseMongoDb](docs/DatabaseMongoDb.md)
 - [DatabaseMongoDbHosts](docs/DatabaseMongoDbHosts.md)
 - [DatabaseMongoDbRequest](docs/DatabaseMongoDbRequest.md)
 - [DatabaseMySql](docs/DatabaseMySql.md)
 - [DatabaseMySqlRequest](docs/DatabaseMySqlRequest.md)
 - [DatabasePostgreSql](docs/DatabasePostgreSql.md)
 - [DatabasePostgreSqlHosts](docs/DatabasePostgreSqlHosts.md)
 - [DatabasePostgreSqlRequest](docs/DatabasePostgreSqlRequest.md)
 - [DatabaseSsl](docs/DatabaseSsl.md)
 - [DatabaseType](docs/DatabaseType.md)
 - [DatabaseTypeEngine](docs/DatabaseTypeEngine.md)
 - [DatabaseTypeEngines](docs/DatabaseTypeEngines.md)
 - [DatabaseTypeEnginesMysqlInner](docs/DatabaseTypeEnginesMysqlInner.md)
 - [DatabaseTypeEnginesMysqlInnerPrice](docs/DatabaseTypeEnginesMysqlInnerPrice.md)
 - [DatabaseUpdates](docs/DatabaseUpdates.md)
 - [Device](docs/Device.md)
 - [Devices](docs/Devices.md)
 - [Disk](docs/Disk.md)
 - [DiskRequest](docs/DiskRequest.md)
 - [Domain](docs/Domain.md)
 - [DomainRecord](docs/DomainRecord.md)
 - [EntityTransfer](docs/EntityTransfer.md)
 - [EntityTransferEntities](docs/EntityTransferEntities.md)
 - [ErrorObject](docs/ErrorObject.md)
 - [Event](docs/Event.md)
 - [EventEntity](docs/EventEntity.md)
 - [EventSecondaryEntity](docs/EventSecondaryEntity.md)
 - [Firewall](docs/Firewall.md)
 - [FirewallDevices](docs/FirewallDevices.md)
 - [FirewallDevicesEntity](docs/FirewallDevicesEntity.md)
 - [FirewallRuleConfig](docs/FirewallRuleConfig.md)
 - [GetBackups200Response](docs/GetBackups200Response.md)
 - [GetBackups200ResponseAutomaticInner](docs/GetBackups200ResponseAutomaticInner.md)
 - [GetBackups200ResponseAutomaticInnerAllOf](docs/GetBackups200ResponseAutomaticInnerAllOf.md)
 - [GetBackups200ResponseAutomaticInnerAllOf1](docs/GetBackups200ResponseAutomaticInnerAllOf1.md)
 - [GetBackups200ResponseAutomaticInnerAllOfDisksInner](docs/GetBackups200ResponseAutomaticInnerAllOfDisksInner.md)
 - [GetBackups200ResponseSnapshot](docs/GetBackups200ResponseSnapshot.md)
 - [GetLinodeConfigs200Response](docs/GetLinodeConfigs200Response.md)
 - [GetLinodeConfigs200ResponseDataInner](docs/GetLinodeConfigs200ResponseDataInner.md)
 - [GetLinodeConfigs200ResponseDataInnerDevices](docs/GetLinodeConfigs200ResponseDataInnerDevices.md)
 - [GetLinodeConfigs200ResponseDataInnerDevicesSda](docs/GetLinodeConfigs200ResponseDataInnerDevicesSda.md)
 - [GetLinodeConfigs200ResponseDataInnerHelpers](docs/GetLinodeConfigs200ResponseDataInnerHelpers.md)
 - [GetLinodeDisks200Response](docs/GetLinodeDisks200Response.md)
 - [GetLinodeDisks200ResponseDataInner](docs/GetLinodeDisks200ResponseDataInner.md)
 - [GetLinodeFirewalls200Response](docs/GetLinodeFirewalls200Response.md)
 - [GetLinodeFirewalls200ResponseDataInner](docs/GetLinodeFirewalls200ResponseDataInner.md)
 - [GetLinodeFirewalls200ResponseDataInnerRules](docs/GetLinodeFirewalls200ResponseDataInnerRules.md)
 - [GetLinodeFirewalls200ResponseDataInnerRulesInboundInner](docs/GetLinodeFirewalls200ResponseDataInnerRulesInboundInner.md)
 - [GetLinodeFirewalls200ResponseDataInnerRulesInboundInnerAddresses](docs/GetLinodeFirewalls200ResponseDataInnerRulesInboundInnerAddresses.md)
 - [GetLinodeInstances200Response](docs/GetLinodeInstances200Response.md)
 - [GetLinodeInstances200ResponseDataInner](docs/GetLinodeInstances200ResponseDataInner.md)
 - [GetLinodeInstances200ResponseDataInnerAlerts](docs/GetLinodeInstances200ResponseDataInnerAlerts.md)
 - [GetLinodeInstances200ResponseDataInnerBackups](docs/GetLinodeInstances200ResponseDataInnerBackups.md)
 - [GetLinodeInstances200ResponseDataInnerBackupsSchedule](docs/GetLinodeInstances200ResponseDataInnerBackupsSchedule.md)
 - [GetLinodeInstances200ResponseDataInnerSpecs](docs/GetLinodeInstances200ResponseDataInnerSpecs.md)
 - [GetLinodeInstancesDefaultResponse](docs/GetLinodeInstancesDefaultResponse.md)
 - [GetLinodeInstancesDefaultResponseErrorsInner](docs/GetLinodeInstancesDefaultResponseErrorsInner.md)
 - [GetLinodeIps200Response](docs/GetLinodeIps200Response.md)
 - [GetLinodeIps200ResponseIpv4](docs/GetLinodeIps200ResponseIpv4.md)
 - [GetLinodeIps200ResponseIpv4PrivateInner](docs/GetLinodeIps200ResponseIpv4PrivateInner.md)
 - [GetLinodeIps200ResponseIpv4PublicInner](docs/GetLinodeIps200ResponseIpv4PublicInner.md)
 - [GetLinodeIps200ResponseIpv6](docs/GetLinodeIps200ResponseIpv6.md)
 - [GetLinodeIps200ResponseIpv6Global](docs/GetLinodeIps200ResponseIpv6Global.md)
 - [GetLinodeIps200ResponseIpv6LinkLocal](docs/GetLinodeIps200ResponseIpv6LinkLocal.md)
 - [GetLinodeIps200ResponseIpv6Slaac](docs/GetLinodeIps200ResponseIpv6Slaac.md)
 - [GetLinodeNodeBalancers200Response](docs/GetLinodeNodeBalancers200Response.md)
 - [GetLinodeNodeBalancers200ResponseDataInner](docs/GetLinodeNodeBalancers200ResponseDataInner.md)
 - [GetLinodeNodeBalancers200ResponseDataInnerTransfer](docs/GetLinodeNodeBalancers200ResponseDataInnerTransfer.md)
 - [GetLinodeStats200Response](docs/GetLinodeStats200Response.md)
 - [GetLinodeStats200ResponseIo](docs/GetLinodeStats200ResponseIo.md)
 - [GetLinodeStats200ResponseNetv4](docs/GetLinodeStats200ResponseNetv4.md)
 - [GetLinodeStats200ResponseNetv6](docs/GetLinodeStats200ResponseNetv6.md)
 - [GetLinodeTransfer200Response](docs/GetLinodeTransfer200Response.md)
 - [GetLinodeTransferByYearMonth200Response](docs/GetLinodeTransferByYearMonth200Response.md)
 - [GetLinodeTypes200Response](docs/GetLinodeTypes200Response.md)
 - [GetLinodeTypes200ResponseDataInner](docs/GetLinodeTypes200ResponseDataInner.md)
 - [GetLinodeTypes200ResponseDataInnerAddons](docs/GetLinodeTypes200ResponseDataInnerAddons.md)
 - [GetLinodeTypes200ResponseDataInnerAddonsBackups](docs/GetLinodeTypes200ResponseDataInnerAddonsBackups.md)
 - [GetLinodeTypes200ResponseDataInnerAddonsBackupsPrice](docs/GetLinodeTypes200ResponseDataInnerAddonsBackupsPrice.md)
 - [GetLinodeTypes200ResponseDataInnerPrice](docs/GetLinodeTypes200ResponseDataInnerPrice.md)
 - [GetLinodeVolumes200Response](docs/GetLinodeVolumes200Response.md)
 - [GetLinodeVolumes200ResponseDataInner](docs/GetLinodeVolumes200ResponseDataInner.md)
 - [GooglePayData](docs/GooglePayData.md)
 - [Grant](docs/Grant.md)
 - [GrantsResponse](docs/GrantsResponse.md)
 - [GrantsResponseGlobal](docs/GrantsResponseGlobal.md)
 - [GrantsResponseLinodeInner](docs/GrantsResponseLinodeInner.md)
 - [Image](docs/Image.md)
 - [Invoice](docs/Invoice.md)
 - [InvoiceItem](docs/InvoiceItem.md)
 - [InvoiceTaxSummaryInner](docs/InvoiceTaxSummaryInner.md)
 - [IpAddress](docs/IpAddress.md)
 - [IpAddressPrivate](docs/IpAddressPrivate.md)
 - [IpAddressV6LinkLocal](docs/IpAddressV6LinkLocal.md)
 - [IpAddressV6Slaac](docs/IpAddressV6Slaac.md)
 - [IpAddressesAssignRequest](docs/IpAddressesAssignRequest.md)
 - [IpAddressesAssignRequestAssignmentsInner](docs/IpAddressesAssignRequestAssignmentsInner.md)
 - [IpAddressesShareRequest](docs/IpAddressesShareRequest.md)
 - [Ipv6Pool](docs/Ipv6Pool.md)
 - [Ipv6Range](docs/Ipv6Range.md)
 - [Ipv6RangeBgp](docs/Ipv6RangeBgp.md)
 - [Kernel](docs/Kernel.md)
 - [Linode](docs/Linode.md)
 - [LinodeConfig](docs/LinodeConfig.md)
 - [LinodeConfigInterface](docs/LinodeConfigInterface.md)
 - [LinodeRequest](docs/LinodeRequest.md)
 - [LinodeStats](docs/LinodeStats.md)
 - [LinodeType](docs/LinodeType.md)
 - [LkeCluster](docs/LkeCluster.md)
 - [LkeClusterControlPlane](docs/LkeClusterControlPlane.md)
 - [LkeNodePool](docs/LkeNodePool.md)
 - [LkeNodePoolAutoscaler](docs/LkeNodePoolAutoscaler.md)
 - [LkeNodePoolNodesInner](docs/LkeNodePoolNodesInner.md)
 - [LkeNodePoolRequestBody](docs/LkeNodePoolRequestBody.md)
 - [LkeNodePoolRequestBodyAutoscaler](docs/LkeNodePoolRequestBodyAutoscaler.md)
 - [LkeNodePoolRequestBodyDisksInner](docs/LkeNodePoolRequestBodyDisksInner.md)
 - [LkeNodeStatus](docs/LkeNodeStatus.md)
 - [LkeVersion](docs/LkeVersion.md)
 - [Login](docs/Login.md)
 - [LongviewClient](docs/LongviewClient.md)
 - [LongviewClientApps](docs/LongviewClientApps.md)
 - [LongviewPlan](docs/LongviewPlan.md)
 - [LongviewSubscription](docs/LongviewSubscription.md)
 - [LongviewSubscriptionPrice](docs/LongviewSubscriptionPrice.md)
 - [Maintenance](docs/Maintenance.md)
 - [MaintenanceEntity](docs/MaintenanceEntity.md)
 - [ManagedContact](docs/ManagedContact.md)
 - [ManagedContactPhone](docs/ManagedContactPhone.md)
 - [ManagedCredential](docs/ManagedCredential.md)
 - [ManagedIssue](docs/ManagedIssue.md)
 - [ManagedIssueEntity](docs/ManagedIssueEntity.md)
 - [ManagedLinodeSettings](docs/ManagedLinodeSettings.md)
 - [ManagedLinodeSettingsSsh](docs/ManagedLinodeSettingsSsh.md)
 - [ManagedService](docs/ManagedService.md)
 - [MigrateLinodeInstanceRequest](docs/MigrateLinodeInstanceRequest.md)
 - [MutateLinodeInstanceRequest](docs/MutateLinodeInstanceRequest.md)
 - [NodeBalancer](docs/NodeBalancer.md)
 - [NodeBalancerConfig](docs/NodeBalancerConfig.md)
 - [NodeBalancerConfigNodesStatus](docs/NodeBalancerConfigNodesStatus.md)
 - [NodeBalancerNode](docs/NodeBalancerNode.md)
 - [NodeBalancerStats](docs/NodeBalancerStats.md)
 - [NodeBalancerStatsData](docs/NodeBalancerStatsData.md)
 - [NodeBalancerStatsDataTraffic](docs/NodeBalancerStatsDataTraffic.md)
 - [Notification](docs/Notification.md)
 - [NotificationEntity](docs/NotificationEntity.md)
 - [OAuthClient](docs/OAuthClient.md)
 - [ObjectStorageBucket](docs/ObjectStorageBucket.md)
 - [ObjectStorageCluster](docs/ObjectStorageCluster.md)
 - [ObjectStorageKey](docs/ObjectStorageKey.md)
 - [ObjectStorageKeyBucketAccessInner](docs/ObjectStorageKeyBucketAccessInner.md)
 - [ObjectStorageObject](docs/ObjectStorageObject.md)
 - [ObjectStorageSsl](docs/ObjectStorageSsl.md)
 - [ObjectStorageSslResponse](docs/ObjectStorageSslResponse.md)
 - [PaginationEnvelope](docs/PaginationEnvelope.md)
 - [PayPal](docs/PayPal.md)
 - [PayPalData](docs/PayPalData.md)
 - [PayPalExecute](docs/PayPalExecute.md)
 - [Payment](docs/Payment.md)
 - [PaymentMethod](docs/PaymentMethod.md)
 - [PaymentMethodData](docs/PaymentMethodData.md)
 - [PaymentMethodDataOneOf](docs/PaymentMethodDataOneOf.md)
 - [PaymentMethodDataOneOf1](docs/PaymentMethodDataOneOf1.md)
 - [PaymentMethodDataOneOf2](docs/PaymentMethodDataOneOf2.md)
 - [PaymentRequest](docs/PaymentRequest.md)
 - [PersonalAccessToken](docs/PersonalAccessToken.md)
 - [Profile](docs/Profile.md)
 - [ProfileReferrals](docs/ProfileReferrals.md)
 - [Promotion](docs/Promotion.md)
 - [RebootLinodeInstanceRequest](docs/RebootLinodeInstanceRequest.md)
 - [RebuildLinodeInstanceRequest](docs/RebuildLinodeInstanceRequest.md)
 - [Region](docs/Region.md)
 - [RegionResolvers](docs/RegionResolvers.md)
 - [RescueDevices](docs/RescueDevices.md)
 - [RescueLinodeInstanceRequest](docs/RescueLinodeInstanceRequest.md)
 - [RescueLinodeInstanceRequestDevices](docs/RescueLinodeInstanceRequestDevices.md)
 - [ResetDiskPasswordRequest](docs/ResetDiskPasswordRequest.md)
 - [ResetLinodePasswordRequest](docs/ResetLinodePasswordRequest.md)
 - [ResizeDiskRequest](docs/ResizeDiskRequest.md)
 - [ResizeLinodeInstanceRequest](docs/ResizeLinodeInstanceRequest.md)
 - [ResizeVolumeRequest](docs/ResizeVolumeRequest.md)
 - [RestoreBackupRequest](docs/RestoreBackupRequest.md)
 - [SecurityQuestion](docs/SecurityQuestion.md)
 - [SecurityQuestionsGet](docs/SecurityQuestionsGet.md)
 - [SecurityQuestionsGetSecurityQuestionsInner](docs/SecurityQuestionsGetSecurityQuestionsInner.md)
 - [SecurityQuestionsPost](docs/SecurityQuestionsPost.md)
 - [SecurityQuestionsPostSecurityQuestionsInner](docs/SecurityQuestionsPostSecurityQuestionsInner.md)
 - [ServiceTransfer](docs/ServiceTransfer.md)
 - [ServiceTransferEntities](docs/ServiceTransferEntities.md)
 - [SshKey](docs/SshKey.md)
 - [StackScript](docs/StackScript.md)
 - [StackScriptUserDefinedFieldsInner](docs/StackScriptUserDefinedFieldsInner.md)
 - [StatsData](docs/StatsData.md)
 - [StatsDataAvailable](docs/StatsDataAvailable.md)
 - [StatsDataAvailableCpuInner](docs/StatsDataAvailableCpuInner.md)
 - [SupportTicket](docs/SupportTicket.md)
 - [SupportTicketEntity](docs/SupportTicketEntity.md)
 - [SupportTicketReply](docs/SupportTicketReply.md)
 - [SupportTicketRequest](docs/SupportTicketRequest.md)
 - [Tag](docs/Tag.md)
 - [Transfer](docs/Transfer.md)
 - [TrustedDevice](docs/TrustedDevice.md)
 - [UpdateLinodeIpRequest](docs/UpdateLinodeIpRequest.md)
 - [UpdateVolumeRequest](docs/UpdateVolumeRequest.md)
 - [UpdateVolumeRequestAllOf](docs/UpdateVolumeRequestAllOf.md)
 - [User](docs/User.md)
 - [UserDefinedField](docs/UserDefinedField.md)
 - [Vlans](docs/Vlans.md)
 - [Volume](docs/Volume.md)
 - [WarningObject](docs/WarningObject.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

support@linode.com

