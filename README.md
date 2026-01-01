# Rust API client for digitalocean

# Introduction

The DigitalOcean API allows you to manage Droplets and resources within the
DigitalOcean cloud in a simple, programmatic way using conventional HTTP requests.

All of the functionality that you are familiar with in the DigitalOcean
control panel is also available through the API, allowing you to script the
complex actions that your situation requires.

The API documentation will start with a general overview about the design
and technology that has been implemented, followed by reference information
about specific endpoints.

## Requests

Any tool that is fluent in HTTP can communicate with the API simply by
requesting the correct URI. Requests should be made using the HTTPS protocol
so that traffic is encrypted. The interface responds to different methods
depending on the action required.

|Method|Usage|
|--- |--- |
|GET|For simple retrieval of information about your account, Droplets, or environment, you should use the GET method.  The information you request will be returned to you as a JSON object. The attributes defined by the JSON object can be used to form additional requests.  Any request using the GET method is read-only and will not affect any of the objects you are querying.|
|DELETE|To destroy a resource and remove it from your account and environment, the DELETE method should be used.  This will remove the specified object if it is found.  If it is not found, the operation will return a response indicating that the object was not found. This idempotency means that you do not have to check for a resource's availability prior to issuing a delete command, the final state will be the same regardless of its existence.|
|PUT|To update the information about a resource in your account, the PUT method is available. Like the DELETE Method, the PUT method is idempotent.  It sets the state of the target using the provided values, regardless of their current values. Requests using the PUT method do not need to check the current attributes of the object.|
|PATCH|Some resources support partial modification. In these cases, the PATCH method is available. Unlike PUT which generally requires a complete representation of a resource, a PATCH request is a set of instructions on how to modify a resource updating only specific attributes.|
|POST|To create a new object, your request should specify the POST method. The POST request includes all of the attributes necessary to create a new object.  When you wish to create a new object, send a POST request to the target endpoint.|
|HEAD|Finally, to retrieve metadata information, you should use the HEAD method to get the headers.  This returns only the header of what would be returned with an associated GET request. Response headers contain some useful information about your API access and the results that are available for your request. For instance, the headers contain your current rate-limit value and the amount of time available until the limit resets. It also contains metrics about the total number of objects found, pagination information, and the total content length.|


## HTTP Statuses

Along with the HTTP methods that the API responds to, it will also return
standard HTTP statuses, including error codes.

In the event of a problem, the status will contain the error code, while the
body of the response will usually contain additional information about the
problem that was encountered.

In general, if the status returned is in the 200 range, it indicates that
the request was fulfilled successfully and that no error was encountered.

Return codes in the 400 range typically indicate that there was an issue
with the request that was sent. Among other things, this could mean that you
did not authenticate correctly, that you are requesting an action that you
do not have authorization for, that the object you are requesting does not
exist, or that your request is malformed.

If you receive a status in the 500 range, this generally indicates a
server-side problem. This means that we are having an issue on our end and
cannot fulfill your request currently.

400 and 500 level error responses will include a JSON object in their body,
including the following attributes:

|Name|Type|Description|
|--- |--- |--- |
|id|string|A short identifier corresponding to the HTTP status code returned. For example, the ID for a response returning a 404 status code would be \"not_found.\"|
|message|string|A message providing additional information about the error, including details to help resolve it when possible.|
|request_id|string|Optionally, some endpoints may include a request ID that should be provided when reporting bugs or opening support tickets to help identify the issue.|

### Example Error Response

```
    HTTP/1.1 403 Forbidden
    {
      \"id\":       \"forbidden\",
      \"message\":  \"You do not have access for the attempted action.\"
    }
```

## Responses

When a request is successful, a response body will typically be sent back in
the form of a JSON object. An exception to this is when a DELETE request is
processed, which will result in a successful HTTP 204 status and an empty
response body.

Inside of this JSON object, the resource root that was the target of the
request will be set as the key. This will be the singular form of the word
if the request operated on a single object, and the plural form of the word
if a collection was processed.

For example, if you send a GET request to `/v2/droplets/$DROPLET_ID` you
will get back an object with a key called \"`droplet`\". However, if you send
the GET request to the general collection at `/v2/droplets`, you will get
back an object with a key called \"`droplets`\".

The value of these keys will generally be a JSON object for a request on a
single object and an array of objects for a request on a collection of
objects.

### Response for a Single Object

```json
    {
        \"droplet\": {
            \"name\": \"example.com\"
            . . .
        }
    }
```

### Response for an Object Collection

```json
    {
        \"droplets\": [
            {
                \"name\": \"example.com\"
                . . .
            },
            {
                \"name\": \"second.com\"
                . . .
            }
        ]
    }
```

## Meta

In addition to the main resource root, the response may also contain a
`meta` object. This object contains information about the response itself.

The `meta` object contains a `total` key that is set to the total number of
objects returned by the request. This has implications on the `links` object
and pagination.

The `meta` object will only be displayed when it has a value. Currently, the
`meta` object will have a value when a request is made on a collection (like
`droplets` or `domains`).


### Sample Meta Object

```json
    {
        . . .
        \"meta\": {
            \"total\": 43
        }
        . . .
    }
```

## Links & Pagination

The `links` object is returned as part of the response body when pagination
is enabled. By default, 20 objects are returned per page. If the response
contains 20 objects or fewer, no `links` object will be returned. If the
response contains more than 20 objects, the first 20 will be returned along
with the `links` object.

You can request a different pagination limit or force pagination by
appending `?per_page=` to the request with the number of items you would
like per page. For instance, to show only two results per page, you could
add `?per_page=2` to the end of your query. The maximum number of results
per page is 200.

The `links` object contains a `pages` object. The `pages` object, in turn,
contains keys indicating the relationship of additional pages. The values of
these are the URLs of the associated pages. The keys will be one of the
following:

*   **first**: The URI of the first page of results.
*   **prev**: The URI of the previous sequential page of results.
*   **next**: The URI of the next sequential page of results.
*   **last**: The URI of the last page of results.

The `pages` object will only include the links that make sense. So for the
first page of results, no `first` or `prev` links will ever be set. This
convention holds true in other situations where a link would not make sense.

### Sample Links Object

```json
    {
        . . .
        \"links\": {
            \"pages\": {
                \"last\": \"https://api.digitalocean.com/v2/images?page=2\",
                \"next\": \"https://api.digitalocean.com/v2/images?page=2\"
            }
        }
        . . .
    }
```

## Rate Limit

Requests through the API are rate limited per OAuth token. Current rate limits:

*   5,000 requests per hour
*   250 requests per minute (5% of the hourly total)

Once you exceed either limit, you will be rate limited until the next cycle
starts. Space out any requests that you would otherwise issue in bursts for
the best results.

The rate limiting information is contained within the response headers of
each request. The relevant headers are:

*   **ratelimit-limit**: The number of requests that can be made per hour.
*   **ratelimit-remaining**: The number of requests that remain before you hit your request limit. See the information below for how the request limits expire.
*   **ratelimit-reset**: This represents the time when the oldest request will expire. The value is given in [Unix epoch time](http://en.wikipedia.org/wiki/Unix_time). See below for more information about how request limits expire.

More rate limiting information is returned only within burst limit error response headers:
*   **retry-after**: The number of seconds to wait before making another request when rate limited.

As long as the `ratelimit-remaining` count is above zero, you will be able
to make additional requests.

The way that a request expires and is removed from the current limit count
is important to understand. Rather than counting all of the requests for an
hour and resetting the `ratelimit-remaining` value at the end of the hour,
each request instead has its own timer.

This means that each request contributes toward the `ratelimit-remaining`
count for one complete hour after the request is made. When that request's
timer runs out, it is no longer counted towards the request limit.

This has implications on the meaning of the `ratelimit-reset` header as
well. Because the entire rate limit is not reset at one time, the value of
this header is set to the time when the _oldest_ request will expire.

Keep this in mind if you see your `ratelimit-reset` value change, but not
move an entire hour into the future.

If the `ratelimit-remaining` reaches zero, subsequent requests will receive
a 429 error code until the request reset has been reached. 

`ratelimit-remaining` reaching zero can also indicate that the \"burst limit\" of 250 
requests per minute limit was met, even if the 5,000 requests per hour limit was not. 
In this case, the 429 error response will include a `retry-after` header to indicate how 
long to wait (in seconds) until the request may be retried.

You can see the format of the response in the examples. 

**Note:** The following endpoints have special rate limit requirements that
are independent of the limits defined above.

*   Only 10 `GET` requests to the `/v2/account/keys` endpoint to list SSH keys can be made per 60 seconds.
*   Only 5 requests to any and all `v2/cdn/endpoints` can be made per 10 seconds. This includes `v2/cdn/endpoints`, 
    `v2/cdn/endpoints/$ENDPOINT_ID`, and `v2/cdn/endpoints/$ENDPOINT_ID/cache`.
*   Only 50 strings within the `files` json struct in the `v2/cdn/endpoints/$ENDPOINT_ID/cache` [payload](https://docs.digitalocean.com/reference/api/api-reference/#operation/cdn_purge_cache) 
    can be requested every 20 seconds.

### Sample Rate Limit Headers

```
    . . .
    ratelimit-limit: 1200
    ratelimit-remaining: 1193
    rateLimit-reset: 1402425459
    . . .
```

  ### Sample Rate Limit Headers When Burst Limit is Reached:

```
    . . .
    ratelimit-limit: 5000
    ratelimit-remaining: 0
    rateLimit-reset: 1402425459
    retry-after: 29
    . . .
```

### Sample Rate Exceeded Response

```
    429 Too Many Requests
    {
            id: \"too_many_requests\",
            message: \"API Rate limit exceeded.\"
    }
```

## Curl Examples

Throughout this document, some example API requests will be given using the
`curl` command. This will allow us to demonstrate the various endpoints in a
simple, textual format.
  
  These examples assume that you are using a Linux or macOS command line. To run
these commands on a Windows machine, you can either use cmd.exe, PowerShell, or WSL:

* For cmd.exe, use the `set VAR=VALUE` [syntax](https://docs.microsoft.com/en-us/windows-server/administration/windows-commands/set_1)
to define environment variables, call them with `%VAR%`, then replace all backslashes (`\\`) in the examples with carets (`^`).

* For PowerShell, use the `$Env:VAR = \"VALUE\"` [syntax](https://docs.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_environment_variables?view=powershell-7.2)
to define environment variables, call them with `$Env:VAR`, then replace `curl` with `curl.exe` and all backslashes (`\\`) in the examples with backticks (`` ` ``).

* WSL is a compatibility layer that allows you to emulate a Linux terminal on a Windows machine.
Install WSL with our [community tutorial](https://www.digitalocean.com/community/tutorials/how-to-install-the-windows-subsystem-for-linux-2-on-microsoft-windows-10), 
then follow this API documentation normally.

The names of account-specific references (like Droplet IDs, for instance)
will be represented by variables. For instance, a Droplet ID may be
represented by a variable called `$DROPLET_ID`. You can set the associated
variables in your environment if you wish to use the examples without
modification.

The first variable that you should set to get started is your OAuth
authorization token. The next section will go over the details of this, but
you can set an environmental variable for it now.

Generate a token by going to the [Apps & API](https://cloud.digitalocean.com/settings/applications)
section of the DigitalOcean control panel. Use an existing token if you have
saved one, or generate a new token with the \"Generate new token\" button.
Copy the generated token and use it to set and export the TOKEN variable in
your environment as the example shows.

You may also wish to set some other variables now or as you go along. For
example, you may wish to set the `DROPLET_ID` variable to one of your
Droplet IDs since this will be used frequently in the API.

If you are following along, make sure you use a Droplet ID that you control
so that your commands will execute correctly.

If you need access to the headers of a response through `curl`, you can pass
the `-i` flag to display the header information along with the body. If you
are only interested in the header, you can instead pass the `-I` flag, which
will exclude the response body entirely.


### Set and Export your OAuth Token

```
export DIGITALOCEAN_TOKEN=your_token_here
```

### Set and Export a Variable

```
export DROPLET_ID=1111111
```

## Parameters

There are two different ways to pass parameters in a request with the API.

When passing parameters to create or update an object, parameters should be
passed as a JSON object containing the appropriate attribute names and
values as key-value pairs. When you use this format, you should specify that
you are sending a JSON object in the header. This is done by setting the
`Content-Type` header to `application/json`. This ensures that your request
is interpreted correctly.

When passing parameters to filter a response on GET requests, parameters can
be passed using standard query attributes. In this case, the parameters
would be embedded into the URI itself by appending a `?` to the end of the
URI and then setting each attribute with an equal sign. Attributes can be
separated with a `&`. Tools like `curl` can create the appropriate URI when
given parameters and values; this can also be done using the `-F` flag and
then passing the key and value as an argument. The argument should take the
form of a quoted string with the attribute being set to a value with an
equal sign.

### Pass Parameters as a JSON Object

```
    curl -H \"Authorization: Bearer $DIGITALOCEAN_TOKEN\" \\
        -H \"Content-Type: application/json\" \\
        -d '{\"name\": \"example.com\", \"ip_address\": \"127.0.0.1\"}' \\
        -X POST \"https://api.digitalocean.com/v2/domains\"
```

### Pass Filter Parameters as a Query String

```
     curl -H \"Authorization: Bearer $DIGITALOCEAN_TOKEN\" \\
         -X GET \\
         \"https://api.digitalocean.com/v2/images?private=true\"
```

## Cross Origin Resource Sharing

In order to make requests to the API from other domains, the API implements
Cross Origin Resource Sharing (CORS) support.

CORS support is generally used to create AJAX requests outside of the domain
that the request originated from. This is necessary to implement projects
like control panels utilizing the API. This tells the browser that it can
send requests to an outside domain.

The procedure that the browser initiates in order to perform these actions
(other than GET requests) begins by sending a \"preflight\" request. This sets
the `Origin` header and uses the `OPTIONS` method. The server will reply
back with the methods it allows and some of the limits it imposes. The
client then sends the actual request if it falls within the allowed
constraints.

This process is usually done in the background by the browser, but you can
use curl to emulate this process using the example provided. The headers
that will be set to show the constraints are:

*   **Access-Control-Allow-Origin**: This is the domain that is sent by the client or browser as the origin of the request. It is set through an `Origin` header.
*   **Access-Control-Allow-Methods**: This specifies the allowed options for requests from that domain. This will generally be all available methods.
*   **Access-Control-Expose-Headers**: This will contain the headers that will be available to requests from the origin domain.
*   **Access-Control-Max-Age**: This is the length of time that the access is considered valid. After this expires, a new preflight should be sent.
*   **Access-Control-Allow-Credentials**: This will be set to `true`. It basically allows you to send your OAuth token for authentication.

You should not need to be concerned with the details of these headers,
because the browser will typically do all of the work for you.



## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 2.0
- Package version: 0.1.7
- Generator version: 7.16.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `digitalocean` and add the following to `Cargo.toml` under `[dependencies]`:

```
digitalocean = { path = "./digitalocean" }
```

## Documentation for API Endpoints

All URIs are relative to *https://api.digitalocean.com*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AccountApi* | [**account_get**](docs/AccountApi.md#account_get) | **GET** /v2/account | Get User Information
*ActionsApi* | [**actions_get**](docs/ActionsApi.md#actions_get) | **GET** /v2/actions/{action_id} | Retrieve an Existing Action
*ActionsApi* | [**actions_list**](docs/ActionsApi.md#actions_list) | **GET** /v2/actions | List All Actions
*AddOnsApi* | [**addons_create**](docs/AddOnsApi.md#addons_create) | **POST** /v2/add-ons/saas | Create/Provision a New Add-on Resource
*AddOnsApi* | [**addons_delete**](docs/AddOnsApi.md#addons_delete) | **DELETE** /v2/add-ons/saas/{resource_uuid} | Delete/Deprovision an Add-on Resource
*AddOnsApi* | [**addons_get**](docs/AddOnsApi.md#addons_get) | **GET** /v2/add-ons/saas/{resource_uuid} | Get details on an Add-On Resource
*AddOnsApi* | [**addons_get_app**](docs/AddOnsApi.md#addons_get_app) | **GET** /v2/add-ons/apps | List Available Add-On Applications
*AddOnsApi* | [**addons_get_app_metadata**](docs/AddOnsApi.md#addons_get_app_metadata) | **GET** /v2/add-ons/apps/{app_slug}/metadata | Get Metadata for an Add-On Application
*AddOnsApi* | [**addons_list**](docs/AddOnsApi.md#addons_list) | **GET** /v2/add-ons/saas | List all Add-On Resources
*AddOnsApi* | [**addons_patch**](docs/AddOnsApi.md#addons_patch) | **PATCH** /v2/add-ons/saas/{resource_uuid} | Update the name for an Add-On Resource
*AddOnsApi* | [**addons_patch_plan**](docs/AddOnsApi.md#addons_patch_plan) | **PATCH** /v2/add-ons/saas/{resource_uuid}/plan | Update the plan for an Add-On Resource
*AppsApi* | [**apps_assign_alert_destinations**](docs/AppsApi.md#apps_assign_alert_destinations) | **POST** /v2/apps/{app_id}/alerts/{alert_id}/destinations | Update destinations for alerts
*AppsApi* | [**apps_cancel_deployment**](docs/AppsApi.md#apps_cancel_deployment) | **POST** /v2/apps/{app_id}/deployments/{deployment_id}/cancel | Cancel a Deployment
*AppsApi* | [**apps_commit_rollback**](docs/AppsApi.md#apps_commit_rollback) | **POST** /v2/apps/{app_id}/rollback/commit | Commit App Rollback
*AppsApi* | [**apps_create**](docs/AppsApi.md#apps_create) | **POST** /v2/apps | Create a New App
*AppsApi* | [**apps_create_deployment**](docs/AppsApi.md#apps_create_deployment) | **POST** /v2/apps/{app_id}/deployments | Create an App Deployment
*AppsApi* | [**apps_create_rollback**](docs/AppsApi.md#apps_create_rollback) | **POST** /v2/apps/{app_id}/rollback | Rollback App
*AppsApi* | [**apps_delete**](docs/AppsApi.md#apps_delete) | **DELETE** /v2/apps/{id} | Delete an App
*AppsApi* | [**apps_get**](docs/AppsApi.md#apps_get) | **GET** /v2/apps/{id} | Retrieve an Existing App
*AppsApi* | [**apps_get_deployment**](docs/AppsApi.md#apps_get_deployment) | **GET** /v2/apps/{app_id}/deployments/{deployment_id} | Retrieve an App Deployment
*AppsApi* | [**apps_get_exec**](docs/AppsApi.md#apps_get_exec) | **GET** /v2/apps/{app_id}/deployments/{deployment_id}/components/{component_name}/exec | Retrieve Exec URL for Deployment
*AppsApi* | [**apps_get_exec_active_deployment**](docs/AppsApi.md#apps_get_exec_active_deployment) | **GET** /v2/apps/{app_id}/components/{component_name}/exec | Retrieve Exec URL
*AppsApi* | [**apps_get_health**](docs/AppsApi.md#apps_get_health) | **GET** /v2/apps/{app_id}/health | Retrieve App Health
*AppsApi* | [**apps_get_instance_size**](docs/AppsApi.md#apps_get_instance_size) | **GET** /v2/apps/tiers/instance_sizes/{slug} | Retrieve an Instance Size
*AppsApi* | [**apps_get_instances**](docs/AppsApi.md#apps_get_instances) | **GET** /v2/apps/{app_id}/instances | Retrieve App Instances
*AppsApi* | [**apps_get_job_invocation**](docs/AppsApi.md#apps_get_job_invocation) | **GET** /v2/apps/{app_id}/job-invocations/{job_invocation_id} | Get Job Invocations
*AppsApi* | [**apps_get_job_invocation_logs**](docs/AppsApi.md#apps_get_job_invocation_logs) | **GET** /v2/apps/{app_id}/jobs/{job_name}/invocations/{job_invocation_id}/logs | Retrieve Job Invocation Logs
*AppsApi* | [**apps_get_logs**](docs/AppsApi.md#apps_get_logs) | **GET** /v2/apps/{app_id}/deployments/{deployment_id}/components/{component_name}/logs | Retrieve Deployment Logs
*AppsApi* | [**apps_get_logs_active_deployment**](docs/AppsApi.md#apps_get_logs_active_deployment) | **GET** /v2/apps/{app_id}/components/{component_name}/logs | Retrieve Active Deployment Logs
*AppsApi* | [**apps_get_logs_active_deployment_aggregate**](docs/AppsApi.md#apps_get_logs_active_deployment_aggregate) | **GET** /v2/apps/{app_id}/logs | Retrieve Active Deployment Aggregate Logs
*AppsApi* | [**apps_get_logs_aggregate**](docs/AppsApi.md#apps_get_logs_aggregate) | **GET** /v2/apps/{app_id}/deployments/{deployment_id}/logs | Retrieve Aggregate Deployment Logs
*AppsApi* | [**apps_get_metrics_bandwidth_daily**](docs/AppsApi.md#apps_get_metrics_bandwidth_daily) | **GET** /v2/apps/{app_id}/metrics/bandwidth_daily | Retrieve App Daily Bandwidth Metrics
*AppsApi* | [**apps_list**](docs/AppsApi.md#apps_list) | **GET** /v2/apps | List All Apps
*AppsApi* | [**apps_list_alerts**](docs/AppsApi.md#apps_list_alerts) | **GET** /v2/apps/{app_id}/alerts | List all app alerts
*AppsApi* | [**apps_list_deployments**](docs/AppsApi.md#apps_list_deployments) | **GET** /v2/apps/{app_id}/deployments | List App Deployments
*AppsApi* | [**apps_list_instance_sizes**](docs/AppsApi.md#apps_list_instance_sizes) | **GET** /v2/apps/tiers/instance_sizes | List Instance Sizes
*AppsApi* | [**apps_list_job_invocations**](docs/AppsApi.md#apps_list_job_invocations) | **GET** /v2/apps/{app_id}/job-invocations | List Job Invocations
*AppsApi* | [**apps_list_metrics_bandwidth_daily**](docs/AppsApi.md#apps_list_metrics_bandwidth_daily) | **POST** /v2/apps/metrics/bandwidth_daily | Retrieve Multiple Apps' Daily Bandwidth Metrics
*AppsApi* | [**apps_list_regions**](docs/AppsApi.md#apps_list_regions) | **GET** /v2/apps/regions | List App Regions
*AppsApi* | [**apps_restart**](docs/AppsApi.md#apps_restart) | **POST** /v2/apps/{app_id}/restart | Restart an App
*AppsApi* | [**apps_revert_rollback**](docs/AppsApi.md#apps_revert_rollback) | **POST** /v2/apps/{app_id}/rollback/revert | Revert App Rollback
*AppsApi* | [**apps_update**](docs/AppsApi.md#apps_update) | **PUT** /v2/apps/{id} | Update an App
*AppsApi* | [**apps_validate_app_spec**](docs/AppsApi.md#apps_validate_app_spec) | **POST** /v2/apps/propose | Propose an App Spec
*AppsApi* | [**apps_validate_rollback**](docs/AppsApi.md#apps_validate_rollback) | **POST** /v2/apps/{app_id}/rollback/validate | Validate App Rollback
*ByoipPrefixesApi* | [**byoip_prefixes_create**](docs/ByoipPrefixesApi.md#byoip_prefixes_create) | **POST** /v2/byoip_prefixes | Create a BYOIP Prefix
*ByoipPrefixesApi* | [**byoip_prefixes_delete**](docs/ByoipPrefixesApi.md#byoip_prefixes_delete) | **DELETE** /v2/byoip_prefixes/{byoip_prefix_uuid} | Delete a BYOIP Prefix
*ByoipPrefixesApi* | [**byoip_prefixes_get**](docs/ByoipPrefixesApi.md#byoip_prefixes_get) | **GET** /v2/byoip_prefixes/{byoip_prefix_uuid} | Get a BYOIP Prefix
*ByoipPrefixesApi* | [**byoip_prefixes_list**](docs/ByoipPrefixesApi.md#byoip_prefixes_list) | **GET** /v2/byoip_prefixes | List BYOIP Prefixes
*ByoipPrefixesApi* | [**byoip_prefixes_list_resources**](docs/ByoipPrefixesApi.md#byoip_prefixes_list_resources) | **GET** /v2/byoip_prefixes/{byoip_prefix_uuid}/ips | List BYOIP Prefix Resources
*ByoipPrefixesApi* | [**byoip_prefixes_patch**](docs/ByoipPrefixesApi.md#byoip_prefixes_patch) | **PATCH** /v2/byoip_prefixes/{byoip_prefix_uuid} | Update a BYOIP Prefix
*BillingApi* | [**balance_get**](docs/BillingApi.md#balance_get) | **GET** /v2/customers/my/balance | Get Customer Balance
*BillingApi* | [**billing_history_list**](docs/BillingApi.md#billing_history_list) | **GET** /v2/customers/my/billing_history | List Billing History
*BillingApi* | [**billing_insights_list**](docs/BillingApi.md#billing_insights_list) | **GET** /v2/billing/{account_urn}/insights/{start_date}/{end_date} | List Billing Insights
*BillingApi* | [**invoices_get_by_uuid**](docs/BillingApi.md#invoices_get_by_uuid) | **GET** /v2/customers/my/invoices/{invoice_uuid} | Retrieve an Invoice by UUID
*BillingApi* | [**invoices_get_csv_by_uuid**](docs/BillingApi.md#invoices_get_csv_by_uuid) | **GET** /v2/customers/my/invoices/{invoice_uuid}/csv | Retrieve an Invoice CSV by UUID
*BillingApi* | [**invoices_get_pdf_by_uuid**](docs/BillingApi.md#invoices_get_pdf_by_uuid) | **GET** /v2/customers/my/invoices/{invoice_uuid}/pdf | Retrieve an Invoice PDF by UUID
*BillingApi* | [**invoices_get_summary_by_uuid**](docs/BillingApi.md#invoices_get_summary_by_uuid) | **GET** /v2/customers/my/invoices/{invoice_uuid}/summary | Retrieve an Invoice Summary by UUID
*BillingApi* | [**invoices_list**](docs/BillingApi.md#invoices_list) | **GET** /v2/customers/my/invoices | List All Invoices
*BlockStorageApi* | [**volume_snapshots_create**](docs/BlockStorageApi.md#volume_snapshots_create) | **POST** /v2/volumes/{volume_id}/snapshots | Create Snapshot from a Volume
*BlockStorageApi* | [**volume_snapshots_delete_by_id**](docs/BlockStorageApi.md#volume_snapshots_delete_by_id) | **DELETE** /v2/volumes/snapshots/{snapshot_id} | Delete a Volume Snapshot
*BlockStorageApi* | [**volume_snapshots_get_by_id**](docs/BlockStorageApi.md#volume_snapshots_get_by_id) | **GET** /v2/volumes/snapshots/{snapshot_id} | Retrieve an Existing Volume Snapshot
*BlockStorageApi* | [**volume_snapshots_list**](docs/BlockStorageApi.md#volume_snapshots_list) | **GET** /v2/volumes/{volume_id}/snapshots | List Snapshots for a Volume
*BlockStorageApi* | [**volumes_create**](docs/BlockStorageApi.md#volumes_create) | **POST** /v2/volumes | Create a New Block Storage Volume
*BlockStorageApi* | [**volumes_delete**](docs/BlockStorageApi.md#volumes_delete) | **DELETE** /v2/volumes/{volume_id} | Delete a Block Storage Volume
*BlockStorageApi* | [**volumes_delete_by_name**](docs/BlockStorageApi.md#volumes_delete_by_name) | **DELETE** /v2/volumes | Delete a Block Storage Volume by Name
*BlockStorageApi* | [**volumes_get**](docs/BlockStorageApi.md#volumes_get) | **GET** /v2/volumes/{volume_id} | Retrieve an Existing Block Storage Volume
*BlockStorageApi* | [**volumes_list**](docs/BlockStorageApi.md#volumes_list) | **GET** /v2/volumes | List All Block Storage Volumes
*BlockStorageActionsApi* | [**volume_actions_get**](docs/BlockStorageActionsApi.md#volume_actions_get) | **GET** /v2/volumes/{volume_id}/actions/{action_id} | Retrieve an Existing Volume Action
*BlockStorageActionsApi* | [**volume_actions_list**](docs/BlockStorageActionsApi.md#volume_actions_list) | **GET** /v2/volumes/{volume_id}/actions | List All Actions for a Volume
*BlockStorageActionsApi* | [**volume_actions_post**](docs/BlockStorageActionsApi.md#volume_actions_post) | **POST** /v2/volumes/actions | Initiate A Block Storage Action By Volume Name
*BlockStorageActionsApi* | [**volume_actions_post_by_id**](docs/BlockStorageActionsApi.md#volume_actions_post_by_id) | **POST** /v2/volumes/{volume_id}/actions | Initiate A Block Storage Action By Volume Id
*CdnEndpointsApi* | [**cdn_create_endpoint**](docs/CdnEndpointsApi.md#cdn_create_endpoint) | **POST** /v2/cdn/endpoints | Create a New CDN Endpoint
*CdnEndpointsApi* | [**cdn_delete_endpoint**](docs/CdnEndpointsApi.md#cdn_delete_endpoint) | **DELETE** /v2/cdn/endpoints/{cdn_id} | Delete a CDN Endpoint
*CdnEndpointsApi* | [**cdn_get_endpoint**](docs/CdnEndpointsApi.md#cdn_get_endpoint) | **GET** /v2/cdn/endpoints/{cdn_id} | Retrieve an Existing CDN Endpoint
*CdnEndpointsApi* | [**cdn_list_endpoints**](docs/CdnEndpointsApi.md#cdn_list_endpoints) | **GET** /v2/cdn/endpoints | List All CDN Endpoints
*CdnEndpointsApi* | [**cdn_purge_cache**](docs/CdnEndpointsApi.md#cdn_purge_cache) | **DELETE** /v2/cdn/endpoints/{cdn_id}/cache | Purge the Cache for an Existing CDN Endpoint
*CdnEndpointsApi* | [**cdn_update_endpoints**](docs/CdnEndpointsApi.md#cdn_update_endpoints) | **PUT** /v2/cdn/endpoints/{cdn_id} | Update a CDN Endpoint
*CertificatesApi* | [**certificates_create**](docs/CertificatesApi.md#certificates_create) | **POST** /v2/certificates | Create a New Certificate
*CertificatesApi* | [**certificates_delete**](docs/CertificatesApi.md#certificates_delete) | **DELETE** /v2/certificates/{certificate_id} | Delete a Certificate
*CertificatesApi* | [**certificates_get**](docs/CertificatesApi.md#certificates_get) | **GET** /v2/certificates/{certificate_id} | Retrieve an Existing Certificate
*CertificatesApi* | [**certificates_list**](docs/CertificatesApi.md#certificates_list) | **GET** /v2/certificates | List All Certificates
*Class1ClickApplicationsApi* | [**one_clicks_install_kubernetes**](docs/Class1ClickApplicationsApi.md#one_clicks_install_kubernetes) | **POST** /v2/1-clicks/kubernetes | Install Kubernetes 1-Click Applications
*Class1ClickApplicationsApi* | [**one_clicks_list**](docs/Class1ClickApplicationsApi.md#one_clicks_list) | **GET** /v2/1-clicks | List 1-Click Applications
*ContainerRegistriesApi* | [**registries_create**](docs/ContainerRegistriesApi.md#registries_create) | **POST** /v2/registries | [Public Preview] Create Container Registry
*ContainerRegistriesApi* | [**registries_delete**](docs/ContainerRegistriesApi.md#registries_delete) | **DELETE** /v2/registries/{registry_name} | [Public Preview] Delete Container Registry By Name
*ContainerRegistriesApi* | [**registries_delete_repository**](docs/ContainerRegistriesApi.md#registries_delete_repository) | **DELETE** /v2/registries/{registry_name}/repositories/{repository_name} | [Public Preview] Delete Container Registry Repository
*ContainerRegistriesApi* | [**registries_delete_repository_manifest**](docs/ContainerRegistriesApi.md#registries_delete_repository_manifest) | **DELETE** /v2/registries/{registry_name}/repositories/{repository_name}/digests/{manifest_digest} | [Public Preview] Delete Container Registry Repository Manifest
*ContainerRegistriesApi* | [**registries_delete_repository_tag**](docs/ContainerRegistriesApi.md#registries_delete_repository_tag) | **DELETE** /v2/registries/{registry_name}/repositories/{repository_name}/tags/{repository_tag} | [Public Preview] Delete Container Registry Repository Tag
*ContainerRegistriesApi* | [**registries_get**](docs/ContainerRegistriesApi.md#registries_get) | **GET** /v2/registries/{registry_name} | [Public Preview] Get a Container Registry By Name
*ContainerRegistriesApi* | [**registries_get_docker_credentials**](docs/ContainerRegistriesApi.md#registries_get_docker_credentials) | **GET** /v2/registries/{registry_name}/docker-credentials | [Public Preview] Get Docker Credentials By Registry Name
*ContainerRegistriesApi* | [**registries_get_garbage_collection**](docs/ContainerRegistriesApi.md#registries_get_garbage_collection) | **GET** /v2/registries/{registry_name}/garbage-collection | [Public Preview] Get Active Garbage Collection
*ContainerRegistriesApi* | [**registries_get_options**](docs/ContainerRegistriesApi.md#registries_get_options) | **GET** /v2/registries/options | [Public Preview] List Registry Options (Subscription Tiers and Available Regions)
*ContainerRegistriesApi* | [**registries_get_subscription**](docs/ContainerRegistriesApi.md#registries_get_subscription) | **GET** /v2/registries/subscription | [Public Preview] Get Subscription Information
*ContainerRegistriesApi* | [**registries_list**](docs/ContainerRegistriesApi.md#registries_list) | **GET** /v2/registries | [Public Preview] List All Container Registries
*ContainerRegistriesApi* | [**registries_list_garbage_collections**](docs/ContainerRegistriesApi.md#registries_list_garbage_collections) | **GET** /v2/registries/{registry_name}/garbage-collections | [Public Preview] List Garbage Collections
*ContainerRegistriesApi* | [**registries_list_repositories_v2**](docs/ContainerRegistriesApi.md#registries_list_repositories_v2) | **GET** /v2/registries/{registry_name}/repositoriesV2 | [Public Preview] List All Container Registry Repositories (V2)
*ContainerRegistriesApi* | [**registries_list_repository_manifests**](docs/ContainerRegistriesApi.md#registries_list_repository_manifests) | **GET** /v2/registries/{registry_name}/repositories/{repository_name}/digests | [Public Preview] List All Container Registry Repository Manifests
*ContainerRegistriesApi* | [**registries_list_repository_tags**](docs/ContainerRegistriesApi.md#registries_list_repository_tags) | **GET** /v2/registries/{registry_name}/repositories/{repository_name}/tags | [Public Preview] List All Container Registry Repository Tags
*ContainerRegistriesApi* | [**registries_run_garbage_collection**](docs/ContainerRegistriesApi.md#registries_run_garbage_collection) | **POST** /v2/registries/{registry_name}/garbage-collection | [Public Preview] Start Garbage Collection
*ContainerRegistriesApi* | [**registries_update_garbage_collection**](docs/ContainerRegistriesApi.md#registries_update_garbage_collection) | **PUT** /v2/registries/{registry_name}/garbage-collection/{garbage_collection_uuid} | [Public Preview] Update Garbage Collection
*ContainerRegistriesApi* | [**registries_update_subscription**](docs/ContainerRegistriesApi.md#registries_update_subscription) | **POST** /v2/registries/subscription | [Public Preview] Update Subscription Tier
*ContainerRegistriesApi* | [**registries_validate_name**](docs/ContainerRegistriesApi.md#registries_validate_name) | **POST** /v2/registries/validate-name | [Public Preview] Validate a Container Registry Name
*ContainerRegistryApi* | [**registry_create**](docs/ContainerRegistryApi.md#registry_create) | **POST** /v2/registry | Create Container Registry
*ContainerRegistryApi* | [**registry_delete**](docs/ContainerRegistryApi.md#registry_delete) | **DELETE** /v2/registry | Delete Container Registry
*ContainerRegistryApi* | [**registry_delete_repository_manifest**](docs/ContainerRegistryApi.md#registry_delete_repository_manifest) | **DELETE** /v2/registry/{registry_name}/repositories/{repository_name}/digests/{manifest_digest} | Delete Container Registry Repository Manifest
*ContainerRegistryApi* | [**registry_delete_repository_tag**](docs/ContainerRegistryApi.md#registry_delete_repository_tag) | **DELETE** /v2/registry/{registry_name}/repositories/{repository_name}/tags/{repository_tag} | Delete Container Registry Repository Tag
*ContainerRegistryApi* | [**registry_get**](docs/ContainerRegistryApi.md#registry_get) | **GET** /v2/registry | Get Container Registry Information
*ContainerRegistryApi* | [**registry_get_docker_credentials**](docs/ContainerRegistryApi.md#registry_get_docker_credentials) | **GET** /v2/registry/docker-credentials | Get Docker Credentials for Container Registry
*ContainerRegistryApi* | [**registry_get_garbage_collection**](docs/ContainerRegistryApi.md#registry_get_garbage_collection) | **GET** /v2/registry/{registry_name}/garbage-collection | Get Active Garbage Collection
*ContainerRegistryApi* | [**registry_get_options**](docs/ContainerRegistryApi.md#registry_get_options) | **GET** /v2/registry/options | List Registry Options (Subscription Tiers and Available Regions)
*ContainerRegistryApi* | [**registry_get_subscription**](docs/ContainerRegistryApi.md#registry_get_subscription) | **GET** /v2/registry/subscription | Get Subscription Information
*ContainerRegistryApi* | [**registry_list_garbage_collections**](docs/ContainerRegistryApi.md#registry_list_garbage_collections) | **GET** /v2/registry/{registry_name}/garbage-collections | List Garbage Collections
*ContainerRegistryApi* | [**registry_list_repositories**](docs/ContainerRegistryApi.md#registry_list_repositories) | **GET** /v2/registry/{registry_name}/repositories | List All Container Registry Repositories
*ContainerRegistryApi* | [**registry_list_repositories_v2**](docs/ContainerRegistryApi.md#registry_list_repositories_v2) | **GET** /v2/registry/{registry_name}/repositoriesV2 | List All Container Registry Repositories (V2)
*ContainerRegistryApi* | [**registry_list_repository_manifests**](docs/ContainerRegistryApi.md#registry_list_repository_manifests) | **GET** /v2/registry/{registry_name}/repositories/{repository_name}/digests | List All Container Registry Repository Manifests
*ContainerRegistryApi* | [**registry_list_repository_tags**](docs/ContainerRegistryApi.md#registry_list_repository_tags) | **GET** /v2/registry/{registry_name}/repositories/{repository_name}/tags | List All Container Registry Repository Tags
*ContainerRegistryApi* | [**registry_run_garbage_collection**](docs/ContainerRegistryApi.md#registry_run_garbage_collection) | **POST** /v2/registry/{registry_name}/garbage-collection | Start Garbage Collection
*ContainerRegistryApi* | [**registry_update_garbage_collection**](docs/ContainerRegistryApi.md#registry_update_garbage_collection) | **PUT** /v2/registry/{registry_name}/garbage-collection/{garbage_collection_uuid} | Update Garbage Collection
*ContainerRegistryApi* | [**registry_update_subscription**](docs/ContainerRegistryApi.md#registry_update_subscription) | **POST** /v2/registry/subscription | Update Subscription Tier
*ContainerRegistryApi* | [**registry_validate_name**](docs/ContainerRegistryApi.md#registry_validate_name) | **POST** /v2/registry/validate-name | Validate a Container Registry Name
*DatabasesApi* | [**databases_add**](docs/DatabasesApi.md#databases_add) | **POST** /v2/databases/{database_cluster_uuid}/dbs | Add a New Database
*DatabasesApi* | [**databases_add_connection_pool**](docs/DatabasesApi.md#databases_add_connection_pool) | **POST** /v2/databases/{database_cluster_uuid}/pools | Add a New Connection Pool (PostgreSQL)
*DatabasesApi* | [**databases_add_user**](docs/DatabasesApi.md#databases_add_user) | **POST** /v2/databases/{database_cluster_uuid}/users | Add a Database User
*DatabasesApi* | [**databases_create_cluster**](docs/DatabasesApi.md#databases_create_cluster) | **POST** /v2/databases | Create a New Database Cluster
*DatabasesApi* | [**databases_create_kafka_schema**](docs/DatabasesApi.md#databases_create_kafka_schema) | **POST** /v2/databases/{database_cluster_uuid}/schema-registry | Create Schema Registry for Kafka Cluster 
*DatabasesApi* | [**databases_create_kafka_topic**](docs/DatabasesApi.md#databases_create_kafka_topic) | **POST** /v2/databases/{database_cluster_uuid}/topics | Create Topic for a Kafka Cluster
*DatabasesApi* | [**databases_create_logsink**](docs/DatabasesApi.md#databases_create_logsink) | **POST** /v2/databases/{database_cluster_uuid}/logsink | Create Logsink for a Database Cluster 
*DatabasesApi* | [**databases_create_replica**](docs/DatabasesApi.md#databases_create_replica) | **POST** /v2/databases/{database_cluster_uuid}/replicas | Create a Read-only Replica
*DatabasesApi* | [**databases_delete**](docs/DatabasesApi.md#databases_delete) | **DELETE** /v2/databases/{database_cluster_uuid}/dbs/{database_name} | Delete a Database
*DatabasesApi* | [**databases_delete_connection_pool**](docs/DatabasesApi.md#databases_delete_connection_pool) | **DELETE** /v2/databases/{database_cluster_uuid}/pools/{pool_name} | Delete a Connection Pool (PostgreSQL)
*DatabasesApi* | [**databases_delete_kafka_schema**](docs/DatabasesApi.md#databases_delete_kafka_schema) | **DELETE** /v2/databases/{database_cluster_uuid}/schema-registry/{subject_name} | Delete a Kafka Schema by Subject Name 
*DatabasesApi* | [**databases_delete_kafka_topic**](docs/DatabasesApi.md#databases_delete_kafka_topic) | **DELETE** /v2/databases/{database_cluster_uuid}/topics/{topic_name} | Delete Topic for a Kafka Cluster
*DatabasesApi* | [**databases_delete_logsink**](docs/DatabasesApi.md#databases_delete_logsink) | **DELETE** /v2/databases/{database_cluster_uuid}/logsink/{logsink_id} | Delete Logsink for a Database Cluster 
*DatabasesApi* | [**databases_delete_online_migration**](docs/DatabasesApi.md#databases_delete_online_migration) | **DELETE** /v2/databases/{database_cluster_uuid}/online-migration/{migration_id} | Stop an Online Migration
*DatabasesApi* | [**databases_delete_opensearch_index**](docs/DatabasesApi.md#databases_delete_opensearch_index) | **DELETE** /v2/databases/{database_cluster_uuid}/indexes/{index_name} | Delete Index for OpenSearch Cluster
*DatabasesApi* | [**databases_delete_user**](docs/DatabasesApi.md#databases_delete_user) | **DELETE** /v2/databases/{database_cluster_uuid}/users/{username} | Remove a Database User
*DatabasesApi* | [**databases_destroy_cluster**](docs/DatabasesApi.md#databases_destroy_cluster) | **DELETE** /v2/databases/{database_cluster_uuid} | Destroy a Database Cluster
*DatabasesApi* | [**databases_destroy_replica**](docs/DatabasesApi.md#databases_destroy_replica) | **DELETE** /v2/databases/{database_cluster_uuid}/replicas/{replica_name} | Destroy a Read-only Replica
*DatabasesApi* | [**databases_get**](docs/DatabasesApi.md#databases_get) | **GET** /v2/databases/{database_cluster_uuid}/dbs/{database_name} | Retrieve an Existing Database
*DatabasesApi* | [**databases_get_autoscale**](docs/DatabasesApi.md#databases_get_autoscale) | **GET** /v2/databases/{database_cluster_uuid}/autoscale | Retrieve Autoscale Configuration for a Database Cluster
*DatabasesApi* | [**databases_get_ca**](docs/DatabasesApi.md#databases_get_ca) | **GET** /v2/databases/{database_cluster_uuid}/ca | Retrieve the Public Certificate
*DatabasesApi* | [**databases_get_cluster**](docs/DatabasesApi.md#databases_get_cluster) | **GET** /v2/databases/{database_cluster_uuid} | Retrieve an Existing Database Cluster
*DatabasesApi* | [**databases_get_cluster_metrics_credentials**](docs/DatabasesApi.md#databases_get_cluster_metrics_credentials) | **GET** /v2/databases/metrics/credentials | Retrieve Database Clusters' Metrics Endpoint Credentials
*DatabasesApi* | [**databases_get_config**](docs/DatabasesApi.md#databases_get_config) | **GET** /v2/databases/{database_cluster_uuid}/config | Retrieve an Existing Database Cluster Configuration
*DatabasesApi* | [**databases_get_connection_pool**](docs/DatabasesApi.md#databases_get_connection_pool) | **GET** /v2/databases/{database_cluster_uuid}/pools/{pool_name} | Retrieve Existing Connection Pool (PostgreSQL)
*DatabasesApi* | [**databases_get_eviction_policy**](docs/DatabasesApi.md#databases_get_eviction_policy) | **GET** /v2/databases/{database_cluster_uuid}/eviction_policy | Retrieve the Eviction Policy for a Caching or Valkey Cluster
*DatabasesApi* | [**databases_get_kafka_schema**](docs/DatabasesApi.md#databases_get_kafka_schema) | **GET** /v2/databases/{database_cluster_uuid}/schema-registry/{subject_name} | Get a Kafka Schema by Subject Name 
*DatabasesApi* | [**databases_get_kafka_schema_config**](docs/DatabasesApi.md#databases_get_kafka_schema_config) | **GET** /v2/databases/{database_cluster_uuid}/schema-registry/config | Retrieve Schema Registry Configuration for a kafka Cluster
*DatabasesApi* | [**databases_get_kafka_schema_subject_config**](docs/DatabasesApi.md#databases_get_kafka_schema_subject_config) | **GET** /v2/databases/{database_cluster_uuid}/schema-registry/config/{subject_name} | Retrieve Schema Registry Configuration for a Subject of kafka Cluster
*DatabasesApi* | [**databases_get_kafka_schema_version**](docs/DatabasesApi.md#databases_get_kafka_schema_version) | **GET** /v2/databases/{database_cluster_uuid}/schema-registry/{subject_name}/versions/{version} | Get Kafka Schema by Subject Version
*DatabasesApi* | [**databases_get_kafka_topic**](docs/DatabasesApi.md#databases_get_kafka_topic) | **GET** /v2/databases/{database_cluster_uuid}/topics/{topic_name} | Get Topic for a Kafka Cluster
*DatabasesApi* | [**databases_get_logsink**](docs/DatabasesApi.md#databases_get_logsink) | **GET** /v2/databases/{database_cluster_uuid}/logsink/{logsink_id} | Get Logsink for a Database Cluster 
*DatabasesApi* | [**databases_get_migration_status**](docs/DatabasesApi.md#databases_get_migration_status) | **GET** /v2/databases/{database_cluster_uuid}/online-migration | Retrieve the Status of an Online Migration
*DatabasesApi* | [**databases_get_replica**](docs/DatabasesApi.md#databases_get_replica) | **GET** /v2/databases/{database_cluster_uuid}/replicas/{replica_name} | Retrieve an Existing Read-only Replica
*DatabasesApi* | [**databases_get_sql_mode**](docs/DatabasesApi.md#databases_get_sql_mode) | **GET** /v2/databases/{database_cluster_uuid}/sql_mode | Retrieve the SQL Modes for a MySQL Cluster
*DatabasesApi* | [**databases_get_user**](docs/DatabasesApi.md#databases_get_user) | **GET** /v2/databases/{database_cluster_uuid}/users/{username} | Retrieve an Existing Database User
*DatabasesApi* | [**databases_install_update**](docs/DatabasesApi.md#databases_install_update) | **PUT** /v2/databases/{database_cluster_uuid}/install_update | Start Database Maintenance
*DatabasesApi* | [**databases_list**](docs/DatabasesApi.md#databases_list) | **GET** /v2/databases/{database_cluster_uuid}/dbs | List All Databases
*DatabasesApi* | [**databases_list_backups**](docs/DatabasesApi.md#databases_list_backups) | **GET** /v2/databases/{database_cluster_uuid}/backups | List Backups for a Database Cluster
*DatabasesApi* | [**databases_list_clusters**](docs/DatabasesApi.md#databases_list_clusters) | **GET** /v2/databases | List All Database Clusters
*DatabasesApi* | [**databases_list_connection_pools**](docs/DatabasesApi.md#databases_list_connection_pools) | **GET** /v2/databases/{database_cluster_uuid}/pools | List Connection Pools (PostgreSQL)
*DatabasesApi* | [**databases_list_events_logs**](docs/DatabasesApi.md#databases_list_events_logs) | **GET** /v2/databases/{database_cluster_uuid}/events | List all Events Logs
*DatabasesApi* | [**databases_list_firewall_rules**](docs/DatabasesApi.md#databases_list_firewall_rules) | **GET** /v2/databases/{database_cluster_uuid}/firewall | List Firewall Rules (Trusted Sources) for a Database Cluster
*DatabasesApi* | [**databases_list_kafka_schemas**](docs/DatabasesApi.md#databases_list_kafka_schemas) | **GET** /v2/databases/{database_cluster_uuid}/schema-registry | List Schemas for Kafka Cluster 
*DatabasesApi* | [**databases_list_kafka_topics**](docs/DatabasesApi.md#databases_list_kafka_topics) | **GET** /v2/databases/{database_cluster_uuid}/topics | List Topics for a Kafka Cluster
*DatabasesApi* | [**databases_list_logsink**](docs/DatabasesApi.md#databases_list_logsink) | **GET** /v2/databases/{database_cluster_uuid}/logsink | List Logsinks for a Database Cluster 
*DatabasesApi* | [**databases_list_opeasearch_indexes**](docs/DatabasesApi.md#databases_list_opeasearch_indexes) | **GET** /v2/databases/{database_cluster_uuid}/indexes | List Indexes for a OpenSearch Cluster
*DatabasesApi* | [**databases_list_options**](docs/DatabasesApi.md#databases_list_options) | **GET** /v2/databases/options | List Database Options
*DatabasesApi* | [**databases_list_replicas**](docs/DatabasesApi.md#databases_list_replicas) | **GET** /v2/databases/{database_cluster_uuid}/replicas | List All Read-only Replicas
*DatabasesApi* | [**databases_list_users**](docs/DatabasesApi.md#databases_list_users) | **GET** /v2/databases/{database_cluster_uuid}/users | List all Database Users
*DatabasesApi* | [**databases_patch_config**](docs/DatabasesApi.md#databases_patch_config) | **PATCH** /v2/databases/{database_cluster_uuid}/config | Update the Database Configuration for an Existing Database
*DatabasesApi* | [**databases_promote_replica**](docs/DatabasesApi.md#databases_promote_replica) | **PUT** /v2/databases/{database_cluster_uuid}/replicas/{replica_name}/promote | Promote a Read-only Replica to become a Primary Cluster
*DatabasesApi* | [**databases_reset_auth**](docs/DatabasesApi.md#databases_reset_auth) | **POST** /v2/databases/{database_cluster_uuid}/users/{username}/reset_auth | Reset a Database User's Password or Authentication Method
*DatabasesApi* | [**databases_update_autoscale**](docs/DatabasesApi.md#databases_update_autoscale) | **PUT** /v2/databases/{database_cluster_uuid}/autoscale | Configure Autoscale Settings for a Database Cluster
*DatabasesApi* | [**databases_update_cluster_metrics_credentials**](docs/DatabasesApi.md#databases_update_cluster_metrics_credentials) | **PUT** /v2/databases/metrics/credentials | Update Database Clusters' Metrics Endpoint Credentials
*DatabasesApi* | [**databases_update_cluster_size**](docs/DatabasesApi.md#databases_update_cluster_size) | **PUT** /v2/databases/{database_cluster_uuid}/resize | Resize a Database Cluster
*DatabasesApi* | [**databases_update_connection_pool**](docs/DatabasesApi.md#databases_update_connection_pool) | **PUT** /v2/databases/{database_cluster_uuid}/pools/{pool_name} | Update Connection Pools (PostgreSQL)
*DatabasesApi* | [**databases_update_eviction_policy**](docs/DatabasesApi.md#databases_update_eviction_policy) | **PUT** /v2/databases/{database_cluster_uuid}/eviction_policy | Configure the Eviction Policy for a Caching or Valkey Cluster
*DatabasesApi* | [**databases_update_firewall_rules**](docs/DatabasesApi.md#databases_update_firewall_rules) | **PUT** /v2/databases/{database_cluster_uuid}/firewall | Update Firewall Rules (Trusted Sources) for a Database
*DatabasesApi* | [**databases_update_kafka_schema_config**](docs/DatabasesApi.md#databases_update_kafka_schema_config) | **PUT** /v2/databases/{database_cluster_uuid}/schema-registry/config | Update Schema Registry Configuration for a kafka Cluster
*DatabasesApi* | [**databases_update_kafka_schema_subject_config**](docs/DatabasesApi.md#databases_update_kafka_schema_subject_config) | **PUT** /v2/databases/{database_cluster_uuid}/schema-registry/config/{subject_name} | Update Schema Registry Configuration for a Subject of kafka Cluster
*DatabasesApi* | [**databases_update_kafka_topic**](docs/DatabasesApi.md#databases_update_kafka_topic) | **PUT** /v2/databases/{database_cluster_uuid}/topics/{topic_name} | Update Topic for a Kafka Cluster
*DatabasesApi* | [**databases_update_logsink**](docs/DatabasesApi.md#databases_update_logsink) | **PUT** /v2/databases/{database_cluster_uuid}/logsink/{logsink_id} | Update Logsink for a Database Cluster 
*DatabasesApi* | [**databases_update_maintenance_window**](docs/DatabasesApi.md#databases_update_maintenance_window) | **PUT** /v2/databases/{database_cluster_uuid}/maintenance | Configure a Database Cluster's Maintenance Window
*DatabasesApi* | [**databases_update_major_version**](docs/DatabasesApi.md#databases_update_major_version) | **PUT** /v2/databases/{database_cluster_uuid}/upgrade | Upgrade Major Version for a Database
*DatabasesApi* | [**databases_update_online_migration**](docs/DatabasesApi.md#databases_update_online_migration) | **PUT** /v2/databases/{database_cluster_uuid}/online-migration | Start an Online Migration
*DatabasesApi* | [**databases_update_region**](docs/DatabasesApi.md#databases_update_region) | **PUT** /v2/databases/{database_cluster_uuid}/migrate | Migrate a Database Cluster to a New Region
*DatabasesApi* | [**databases_update_sql_mode**](docs/DatabasesApi.md#databases_update_sql_mode) | **PUT** /v2/databases/{database_cluster_uuid}/sql_mode | Update SQL Mode for a Cluster
*DatabasesApi* | [**databases_update_user**](docs/DatabasesApi.md#databases_update_user) | **PUT** /v2/databases/{database_cluster_uuid}/users/{username} | Update a Database User
*DomainRecordsApi* | [**domains_create_record**](docs/DomainRecordsApi.md#domains_create_record) | **POST** /v2/domains/{domain_name}/records | Create a New Domain Record
*DomainRecordsApi* | [**domains_delete_record**](docs/DomainRecordsApi.md#domains_delete_record) | **DELETE** /v2/domains/{domain_name}/records/{domain_record_id} | Delete a Domain Record
*DomainRecordsApi* | [**domains_get_record**](docs/DomainRecordsApi.md#domains_get_record) | **GET** /v2/domains/{domain_name}/records/{domain_record_id} | Retrieve an Existing Domain Record
*DomainRecordsApi* | [**domains_list_records**](docs/DomainRecordsApi.md#domains_list_records) | **GET** /v2/domains/{domain_name}/records | List All Domain Records
*DomainRecordsApi* | [**domains_patch_record**](docs/DomainRecordsApi.md#domains_patch_record) | **PATCH** /v2/domains/{domain_name}/records/{domain_record_id} | Update a Domain Record
*DomainRecordsApi* | [**domains_update_record**](docs/DomainRecordsApi.md#domains_update_record) | **PUT** /v2/domains/{domain_name}/records/{domain_record_id} | Update a Domain Record
*DomainsApi* | [**domains_create**](docs/DomainsApi.md#domains_create) | **POST** /v2/domains | Create a New Domain
*DomainsApi* | [**domains_delete**](docs/DomainsApi.md#domains_delete) | **DELETE** /v2/domains/{domain_name} | Delete a Domain
*DomainsApi* | [**domains_get**](docs/DomainsApi.md#domains_get) | **GET** /v2/domains/{domain_name} | Retrieve an Existing Domain
*DomainsApi* | [**domains_list**](docs/DomainsApi.md#domains_list) | **GET** /v2/domains | List All Domains
*DropletActionsApi* | [**droplet_actions_get**](docs/DropletActionsApi.md#droplet_actions_get) | **GET** /v2/droplets/{droplet_id}/actions/{action_id} | Retrieve a Droplet Action
*DropletActionsApi* | [**droplet_actions_list**](docs/DropletActionsApi.md#droplet_actions_list) | **GET** /v2/droplets/{droplet_id}/actions | List Actions for a Droplet
*DropletActionsApi* | [**droplet_actions_post**](docs/DropletActionsApi.md#droplet_actions_post) | **POST** /v2/droplets/{droplet_id}/actions | Initiate a Droplet Action
*DropletActionsApi* | [**droplet_actions_post_by_tag**](docs/DropletActionsApi.md#droplet_actions_post_by_tag) | **POST** /v2/droplets/actions | Acting on Tagged Droplets
*DropletAutoscalePoolsApi* | [**autoscalepools_create**](docs/DropletAutoscalePoolsApi.md#autoscalepools_create) | **POST** /v2/droplets/autoscale | Create a New Autoscale Pool
*DropletAutoscalePoolsApi* | [**autoscalepools_delete**](docs/DropletAutoscalePoolsApi.md#autoscalepools_delete) | **DELETE** /v2/droplets/autoscale/{autoscale_pool_id} | Delete autoscale pool
*DropletAutoscalePoolsApi* | [**autoscalepools_delete_dangerous**](docs/DropletAutoscalePoolsApi.md#autoscalepools_delete_dangerous) | **DELETE** /v2/droplets/autoscale/{autoscale_pool_id}/dangerous | Delete autoscale pool and resources
*DropletAutoscalePoolsApi* | [**autoscalepools_get**](docs/DropletAutoscalePoolsApi.md#autoscalepools_get) | **GET** /v2/droplets/autoscale/{autoscale_pool_id} | Retrieve an Existing Autoscale Pool
*DropletAutoscalePoolsApi* | [**autoscalepools_list**](docs/DropletAutoscalePoolsApi.md#autoscalepools_list) | **GET** /v2/droplets/autoscale | List All Autoscale Pools
*DropletAutoscalePoolsApi* | [**autoscalepools_list_history**](docs/DropletAutoscalePoolsApi.md#autoscalepools_list_history) | **GET** /v2/droplets/autoscale/{autoscale_pool_id}/history | List history events
*DropletAutoscalePoolsApi* | [**autoscalepools_list_members**](docs/DropletAutoscalePoolsApi.md#autoscalepools_list_members) | **GET** /v2/droplets/autoscale/{autoscale_pool_id}/members | List members
*DropletAutoscalePoolsApi* | [**autoscalepools_update**](docs/DropletAutoscalePoolsApi.md#autoscalepools_update) | **PUT** /v2/droplets/autoscale/{autoscale_pool_id} | Update Autoscale Pool
*DropletsApi* | [**droplets_create**](docs/DropletsApi.md#droplets_create) | **POST** /v2/droplets | Create a New Droplet
*DropletsApi* | [**droplets_destroy**](docs/DropletsApi.md#droplets_destroy) | **DELETE** /v2/droplets/{droplet_id} | Delete an Existing Droplet
*DropletsApi* | [**droplets_destroy_by_tag**](docs/DropletsApi.md#droplets_destroy_by_tag) | **DELETE** /v2/droplets | Deleting Droplets by Tag
*DropletsApi* | [**droplets_destroy_retry_with_associated_resources**](docs/DropletsApi.md#droplets_destroy_retry_with_associated_resources) | **POST** /v2/droplets/{droplet_id}/destroy_with_associated_resources/retry | Retry a Droplet Destroy with Associated Resources Request
*DropletsApi* | [**droplets_destroy_with_associated_resources_dangerous**](docs/DropletsApi.md#droplets_destroy_with_associated_resources_dangerous) | **DELETE** /v2/droplets/{droplet_id}/destroy_with_associated_resources/dangerous | Destroy a Droplet and All of its Associated Resources (Dangerous)
*DropletsApi* | [**droplets_destroy_with_associated_resources_selective**](docs/DropletsApi.md#droplets_destroy_with_associated_resources_selective) | **DELETE** /v2/droplets/{droplet_id}/destroy_with_associated_resources/selective | Selectively Destroy a Droplet and its Associated Resources
*DropletsApi* | [**droplets_get**](docs/DropletsApi.md#droplets_get) | **GET** /v2/droplets/{droplet_id} | Retrieve an Existing Droplet
*DropletsApi* | [**droplets_get_backup_policy**](docs/DropletsApi.md#droplets_get_backup_policy) | **GET** /v2/droplets/{droplet_id}/backups/policy | Retrieve the Backup Policy for an Existing Droplet
*DropletsApi* | [**droplets_get_destroy_associated_resources_status**](docs/DropletsApi.md#droplets_get_destroy_associated_resources_status) | **GET** /v2/droplets/{droplet_id}/destroy_with_associated_resources/status | Check Status of a Droplet Destroy with Associated Resources Request
*DropletsApi* | [**droplets_list**](docs/DropletsApi.md#droplets_list) | **GET** /v2/droplets | List All Droplets
*DropletsApi* | [**droplets_list_associated_resources**](docs/DropletsApi.md#droplets_list_associated_resources) | **GET** /v2/droplets/{droplet_id}/destroy_with_associated_resources | List Associated Resources for a Droplet
*DropletsApi* | [**droplets_list_backup_policies**](docs/DropletsApi.md#droplets_list_backup_policies) | **GET** /v2/droplets/backups/policies | List Backup Policies for All Existing Droplets
*DropletsApi* | [**droplets_list_backups**](docs/DropletsApi.md#droplets_list_backups) | **GET** /v2/droplets/{droplet_id}/backups | List Backups for a Droplet
*DropletsApi* | [**droplets_list_firewalls**](docs/DropletsApi.md#droplets_list_firewalls) | **GET** /v2/droplets/{droplet_id}/firewalls | List all Firewalls Applied to a Droplet
*DropletsApi* | [**droplets_list_kernels**](docs/DropletsApi.md#droplets_list_kernels) | **GET** /v2/droplets/{droplet_id}/kernels | List All Available Kernels for a Droplet
*DropletsApi* | [**droplets_list_neighbors**](docs/DropletsApi.md#droplets_list_neighbors) | **GET** /v2/droplets/{droplet_id}/neighbors | List Neighbors for a Droplet
*DropletsApi* | [**droplets_list_neighbors_ids**](docs/DropletsApi.md#droplets_list_neighbors_ids) | **GET** /v2/reports/droplet_neighbors_ids | List All Droplet Neighbors
*DropletsApi* | [**droplets_list_snapshots**](docs/DropletsApi.md#droplets_list_snapshots) | **GET** /v2/droplets/{droplet_id}/snapshots | List Snapshots for a Droplet
*DropletsApi* | [**droplets_list_supported_backup_policies**](docs/DropletsApi.md#droplets_list_supported_backup_policies) | **GET** /v2/droplets/backups/supported_policies | List Supported Droplet Backup Policies
*FirewallsApi* | [**firewalls_add_rules**](docs/FirewallsApi.md#firewalls_add_rules) | **POST** /v2/firewalls/{firewall_id}/rules | Add Rules to a Firewall
*FirewallsApi* | [**firewalls_add_tags**](docs/FirewallsApi.md#firewalls_add_tags) | **POST** /v2/firewalls/{firewall_id}/tags | Add Tags to a Firewall
*FirewallsApi* | [**firewalls_assign_droplets**](docs/FirewallsApi.md#firewalls_assign_droplets) | **POST** /v2/firewalls/{firewall_id}/droplets | Add Droplets to a Firewall
*FirewallsApi* | [**firewalls_create**](docs/FirewallsApi.md#firewalls_create) | **POST** /v2/firewalls | Create a New Firewall
*FirewallsApi* | [**firewalls_delete**](docs/FirewallsApi.md#firewalls_delete) | **DELETE** /v2/firewalls/{firewall_id} | Delete a Firewall
*FirewallsApi* | [**firewalls_delete_droplets**](docs/FirewallsApi.md#firewalls_delete_droplets) | **DELETE** /v2/firewalls/{firewall_id}/droplets | Remove Droplets from a Firewall
*FirewallsApi* | [**firewalls_delete_rules**](docs/FirewallsApi.md#firewalls_delete_rules) | **DELETE** /v2/firewalls/{firewall_id}/rules | Remove Rules from a Firewall
*FirewallsApi* | [**firewalls_delete_tags**](docs/FirewallsApi.md#firewalls_delete_tags) | **DELETE** /v2/firewalls/{firewall_id}/tags | Remove Tags from a Firewall
*FirewallsApi* | [**firewalls_get**](docs/FirewallsApi.md#firewalls_get) | **GET** /v2/firewalls/{firewall_id} | Retrieve an Existing Firewall
*FirewallsApi* | [**firewalls_list**](docs/FirewallsApi.md#firewalls_list) | **GET** /v2/firewalls | List All Firewalls
*FirewallsApi* | [**firewalls_update**](docs/FirewallsApi.md#firewalls_update) | **PUT** /v2/firewalls/{firewall_id} | Update a Firewall
*FloatingIpActionsApi* | [**floating_ips_action_get**](docs/FloatingIpActionsApi.md#floating_ips_action_get) | **GET** /v2/floating_ips/{floating_ip}/actions/{action_id} | Retrieve an Existing Floating IP Action
*FloatingIpActionsApi* | [**floating_ips_action_list**](docs/FloatingIpActionsApi.md#floating_ips_action_list) | **GET** /v2/floating_ips/{floating_ip}/actions | List All Actions for a Floating IP
*FloatingIpActionsApi* | [**floating_ips_action_post**](docs/FloatingIpActionsApi.md#floating_ips_action_post) | **POST** /v2/floating_ips/{floating_ip}/actions | Initiate a Floating IP Action
*FloatingIpsApi* | [**floating_ips_create**](docs/FloatingIpsApi.md#floating_ips_create) | **POST** /v2/floating_ips | Create a New Floating IP
*FloatingIpsApi* | [**floating_ips_delete**](docs/FloatingIpsApi.md#floating_ips_delete) | **DELETE** /v2/floating_ips/{floating_ip} | Delete a Floating IP
*FloatingIpsApi* | [**floating_ips_get**](docs/FloatingIpsApi.md#floating_ips_get) | **GET** /v2/floating_ips/{floating_ip} | Retrieve an Existing Floating IP
*FloatingIpsApi* | [**floating_ips_list**](docs/FloatingIpsApi.md#floating_ips_list) | **GET** /v2/floating_ips | List All Floating IPs
*FunctionsApi* | [**functions_create_namespace**](docs/FunctionsApi.md#functions_create_namespace) | **POST** /v2/functions/namespaces | Create Namespace
*FunctionsApi* | [**functions_create_trigger**](docs/FunctionsApi.md#functions_create_trigger) | **POST** /v2/functions/namespaces/{namespace_id}/triggers | Create Trigger
*FunctionsApi* | [**functions_delete_namespace**](docs/FunctionsApi.md#functions_delete_namespace) | **DELETE** /v2/functions/namespaces/{namespace_id} | Delete Namespace
*FunctionsApi* | [**functions_delete_trigger**](docs/FunctionsApi.md#functions_delete_trigger) | **DELETE** /v2/functions/namespaces/{namespace_id}/triggers/{trigger_name} | Delete Trigger
*FunctionsApi* | [**functions_get_namespace**](docs/FunctionsApi.md#functions_get_namespace) | **GET** /v2/functions/namespaces/{namespace_id} | Get Namespace
*FunctionsApi* | [**functions_get_trigger**](docs/FunctionsApi.md#functions_get_trigger) | **GET** /v2/functions/namespaces/{namespace_id}/triggers/{trigger_name} | Get Trigger
*FunctionsApi* | [**functions_list_namespaces**](docs/FunctionsApi.md#functions_list_namespaces) | **GET** /v2/functions/namespaces | List Namespaces
*FunctionsApi* | [**functions_list_triggers**](docs/FunctionsApi.md#functions_list_triggers) | **GET** /v2/functions/namespaces/{namespace_id}/triggers | List Triggers
*FunctionsApi* | [**functions_update_trigger**](docs/FunctionsApi.md#functions_update_trigger) | **PUT** /v2/functions/namespaces/{namespace_id}/triggers/{trigger_name} | Update Trigger
*GradientAiPlatformApi* | [**genai_attach_agent**](docs/GradientAiPlatformApi.md#genai_attach_agent) | **POST** /v2/gen-ai/agents/{parent_agent_uuid}/child_agents/{child_agent_uuid} | Add Agent Route to an Agent
*GradientAiPlatformApi* | [**genai_attach_agent_function**](docs/GradientAiPlatformApi.md#genai_attach_agent_function) | **POST** /v2/gen-ai/agents/{agent_uuid}/functions | Add Function Route to an Agent
*GradientAiPlatformApi* | [**genai_attach_knowledge_base**](docs/GradientAiPlatformApi.md#genai_attach_knowledge_base) | **POST** /v2/gen-ai/agents/{agent_uuid}/knowledge_bases/{knowledge_base_uuid} | Attach Knowledge Base to an Agent
*GradientAiPlatformApi* | [**genai_attach_knowledge_bases**](docs/GradientAiPlatformApi.md#genai_attach_knowledge_bases) | **POST** /v2/gen-ai/agents/{agent_uuid}/knowledge_bases | Attach Knowledge Bases to an Agent
*GradientAiPlatformApi* | [**genai_cancel_indexing_job**](docs/GradientAiPlatformApi.md#genai_cancel_indexing_job) | **PUT** /v2/gen-ai/indexing_jobs/{uuid}/cancel | Cancel Indexing Job for a Knowledge Base
*GradientAiPlatformApi* | [**genai_create_agent**](docs/GradientAiPlatformApi.md#genai_create_agent) | **POST** /v2/gen-ai/agents | Create an Agent
*GradientAiPlatformApi* | [**genai_create_agent_api_key**](docs/GradientAiPlatformApi.md#genai_create_agent_api_key) | **POST** /v2/gen-ai/agents/{agent_uuid}/api_keys | Create an Agent API Key
*GradientAiPlatformApi* | [**genai_create_anthropic_api_key**](docs/GradientAiPlatformApi.md#genai_create_anthropic_api_key) | **POST** /v2/gen-ai/anthropic/keys | Create Anthropic API Key
*GradientAiPlatformApi* | [**genai_create_data_source_file_upload_presigned_urls**](docs/GradientAiPlatformApi.md#genai_create_data_source_file_upload_presigned_urls) | **POST** /v2/gen-ai/knowledge_bases/data_sources/file_upload_presigned_urls | Create Presigned URLs for Data Source File Upload
*GradientAiPlatformApi* | [**genai_create_evaluation_dataset**](docs/GradientAiPlatformApi.md#genai_create_evaluation_dataset) | **POST** /v2/gen-ai/evaluation_datasets | Create Evaluation Dataset
*GradientAiPlatformApi* | [**genai_create_evaluation_dataset_file_upload_presigned_urls**](docs/GradientAiPlatformApi.md#genai_create_evaluation_dataset_file_upload_presigned_urls) | **POST** /v2/gen-ai/evaluation_datasets/file_upload_presigned_urls | Create Presigned URLs for Evaluation Dataset File Upload
*GradientAiPlatformApi* | [**genai_create_evaluation_test_case**](docs/GradientAiPlatformApi.md#genai_create_evaluation_test_case) | **POST** /v2/gen-ai/evaluation_test_cases | Create Evaluation Test Case.
*GradientAiPlatformApi* | [**genai_create_indexing_job**](docs/GradientAiPlatformApi.md#genai_create_indexing_job) | **POST** /v2/gen-ai/indexing_jobs | Start Indexing Job for a Knowledge Base
*GradientAiPlatformApi* | [**genai_create_knowledge_base**](docs/GradientAiPlatformApi.md#genai_create_knowledge_base) | **POST** /v2/gen-ai/knowledge_bases | Create a Knowledge Base
*GradientAiPlatformApi* | [**genai_create_knowledge_base_data_source**](docs/GradientAiPlatformApi.md#genai_create_knowledge_base_data_source) | **POST** /v2/gen-ai/knowledge_bases/{knowledge_base_uuid}/data_sources | Add Data Source to a Knowledge Base
*GradientAiPlatformApi* | [**genai_create_model_api_key**](docs/GradientAiPlatformApi.md#genai_create_model_api_key) | **POST** /v2/gen-ai/models/api_keys | Create a Model API Key
*GradientAiPlatformApi* | [**genai_create_oauth2_dropbox_tokens**](docs/GradientAiPlatformApi.md#genai_create_oauth2_dropbox_tokens) | **POST** /v2/gen-ai/oauth2/dropbox/tokens | Get Oauth2 Dropbox Tokens
*GradientAiPlatformApi* | [**genai_create_openai_api_key**](docs/GradientAiPlatformApi.md#genai_create_openai_api_key) | **POST** /v2/gen-ai/openai/keys | Create OpenAI API Key
*GradientAiPlatformApi* | [**genai_create_scheduled_indexing**](docs/GradientAiPlatformApi.md#genai_create_scheduled_indexing) | **POST** /v2/gen-ai/scheduled-indexing | Create scheduled indexing for knowledge base
*GradientAiPlatformApi* | [**genai_create_workspace**](docs/GradientAiPlatformApi.md#genai_create_workspace) | **POST** /v2/gen-ai/workspaces | Create a Workspace
*GradientAiPlatformApi* | [**genai_delete_agent**](docs/GradientAiPlatformApi.md#genai_delete_agent) | **DELETE** /v2/gen-ai/agents/{uuid} | Delete an Agent
*GradientAiPlatformApi* | [**genai_delete_agent_api_key**](docs/GradientAiPlatformApi.md#genai_delete_agent_api_key) | **DELETE** /v2/gen-ai/agents/{agent_uuid}/api_keys/{api_key_uuid} | Delete API Key for an Agent 
*GradientAiPlatformApi* | [**genai_delete_anthropic_api_key**](docs/GradientAiPlatformApi.md#genai_delete_anthropic_api_key) | **DELETE** /v2/gen-ai/anthropic/keys/{api_key_uuid} | Delete Anthropic API Key
*GradientAiPlatformApi* | [**genai_delete_knowledge_base**](docs/GradientAiPlatformApi.md#genai_delete_knowledge_base) | **DELETE** /v2/gen-ai/knowledge_bases/{uuid} | Delete a Knowledge Base
*GradientAiPlatformApi* | [**genai_delete_knowledge_base_data_source**](docs/GradientAiPlatformApi.md#genai_delete_knowledge_base_data_source) | **DELETE** /v2/gen-ai/knowledge_bases/{knowledge_base_uuid}/data_sources/{data_source_uuid} | Delete a Data Source from a Knowledge Base
*GradientAiPlatformApi* | [**genai_delete_model_api_key**](docs/GradientAiPlatformApi.md#genai_delete_model_api_key) | **DELETE** /v2/gen-ai/models/api_keys/{api_key_uuid} | Delete API Key for a Model
*GradientAiPlatformApi* | [**genai_delete_openai_api_key**](docs/GradientAiPlatformApi.md#genai_delete_openai_api_key) | **DELETE** /v2/gen-ai/openai/keys/{api_key_uuid} | Delete OpenAI API Key
*GradientAiPlatformApi* | [**genai_delete_scheduled_indexing**](docs/GradientAiPlatformApi.md#genai_delete_scheduled_indexing) | **DELETE** /v2/gen-ai/scheduled-indexing/{uuid} | Delete Scheduled Indexing
*GradientAiPlatformApi* | [**genai_delete_workspace**](docs/GradientAiPlatformApi.md#genai_delete_workspace) | **DELETE** /v2/gen-ai/workspaces/{workspace_uuid} | Delete a Workspace
*GradientAiPlatformApi* | [**genai_detach_agent**](docs/GradientAiPlatformApi.md#genai_detach_agent) | **DELETE** /v2/gen-ai/agents/{parent_agent_uuid}/child_agents/{child_agent_uuid} | Delete Agent Route for an Agent
*GradientAiPlatformApi* | [**genai_detach_agent_function**](docs/GradientAiPlatformApi.md#genai_detach_agent_function) | **DELETE** /v2/gen-ai/agents/{agent_uuid}/functions/{function_uuid} | Delete Function Route for an Agent
*GradientAiPlatformApi* | [**genai_detach_knowledge_base**](docs/GradientAiPlatformApi.md#genai_detach_knowledge_base) | **DELETE** /v2/gen-ai/agents/{agent_uuid}/knowledge_bases/{knowledge_base_uuid} | Detach Knowledge Base from an Agent
*GradientAiPlatformApi* | [**genai_get_agent**](docs/GradientAiPlatformApi.md#genai_get_agent) | **GET** /v2/gen-ai/agents/{uuid} | Retrieve an Existing Agent
*GradientAiPlatformApi* | [**genai_get_agent_children**](docs/GradientAiPlatformApi.md#genai_get_agent_children) | **GET** /v2/gen-ai/agents/{uuid}/child_agents | View Agent Routes
*GradientAiPlatformApi* | [**genai_get_agent_usage**](docs/GradientAiPlatformApi.md#genai_get_agent_usage) | **GET** /v2/gen-ai/agents/{uuid}/usage | Get Agent Usage
*GradientAiPlatformApi* | [**genai_get_anthropic_api_key**](docs/GradientAiPlatformApi.md#genai_get_anthropic_api_key) | **GET** /v2/gen-ai/anthropic/keys/{api_key_uuid} | Get Anthropic API Key
*GradientAiPlatformApi* | [**genai_get_evaluation_run**](docs/GradientAiPlatformApi.md#genai_get_evaluation_run) | **GET** /v2/gen-ai/evaluation_runs/{evaluation_run_uuid} | Retrieve Information About an Existing Evaluation Run
*GradientAiPlatformApi* | [**genai_get_evaluation_run_prompt_results**](docs/GradientAiPlatformApi.md#genai_get_evaluation_run_prompt_results) | **GET** /v2/gen-ai/evaluation_runs/{evaluation_run_uuid}/results/{prompt_id} | Retrieve Results of an Evaluation Run Prompt
*GradientAiPlatformApi* | [**genai_get_evaluation_run_results**](docs/GradientAiPlatformApi.md#genai_get_evaluation_run_results) | **GET** /v2/gen-ai/evaluation_runs/{evaluation_run_uuid}/results | Retrieve Results of an Evaluation Run
*GradientAiPlatformApi* | [**genai_get_evaluation_test_case**](docs/GradientAiPlatformApi.md#genai_get_evaluation_test_case) | **GET** /v2/gen-ai/evaluation_test_cases/{test_case_uuid} | Retrieve Information About an Existing Evaluation Test Case
*GradientAiPlatformApi* | [**genai_get_indexing_job**](docs/GradientAiPlatformApi.md#genai_get_indexing_job) | **GET** /v2/gen-ai/indexing_jobs/{uuid} | Retrieve Status of Indexing Job for a Knowledge Base
*GradientAiPlatformApi* | [**genai_get_indexing_job_details_signed_url**](docs/GradientAiPlatformApi.md#genai_get_indexing_job_details_signed_url) | **GET** /v2/gen-ai/indexing_jobs/{indexing_job_uuid}/details_signed_url | Get Signed URL for Indexing Job Details
*GradientAiPlatformApi* | [**genai_get_knowledge_base**](docs/GradientAiPlatformApi.md#genai_get_knowledge_base) | **GET** /v2/gen-ai/knowledge_bases/{uuid} | Retrieve Information About an Existing Knowledge Base
*GradientAiPlatformApi* | [**genai_get_oauth2_url**](docs/GradientAiPlatformApi.md#genai_get_oauth2_url) | **GET** /v2/gen-ai/oauth2/url | Get Oauth2 URL
*GradientAiPlatformApi* | [**genai_get_openai_api_key**](docs/GradientAiPlatformApi.md#genai_get_openai_api_key) | **GET** /v2/gen-ai/openai/keys/{api_key_uuid} | Get OpenAI API Key
*GradientAiPlatformApi* | [**genai_get_scheduled_indexing**](docs/GradientAiPlatformApi.md#genai_get_scheduled_indexing) | **GET** /v2/gen-ai/scheduled-indexing/knowledge-base/{knowledge_base_uuid} | Get Scheduled Indexing for Knowledge Base
*GradientAiPlatformApi* | [**genai_get_workspace**](docs/GradientAiPlatformApi.md#genai_get_workspace) | **GET** /v2/gen-ai/workspaces/{workspace_uuid} | Retrieve an Existing Workspace
*GradientAiPlatformApi* | [**genai_list_agent_api_keys**](docs/GradientAiPlatformApi.md#genai_list_agent_api_keys) | **GET** /v2/gen-ai/agents/{agent_uuid}/api_keys | List Agent API Keys
*GradientAiPlatformApi* | [**genai_list_agent_versions**](docs/GradientAiPlatformApi.md#genai_list_agent_versions) | **GET** /v2/gen-ai/agents/{uuid}/versions | List Agent Versions
*GradientAiPlatformApi* | [**genai_list_agents**](docs/GradientAiPlatformApi.md#genai_list_agents) | **GET** /v2/gen-ai/agents | List Agents
*GradientAiPlatformApi* | [**genai_list_agents_by_anthropic_key**](docs/GradientAiPlatformApi.md#genai_list_agents_by_anthropic_key) | **GET** /v2/gen-ai/anthropic/keys/{uuid}/agents | List agents by Anthropic key
*GradientAiPlatformApi* | [**genai_list_agents_by_openai_key**](docs/GradientAiPlatformApi.md#genai_list_agents_by_openai_key) | **GET** /v2/gen-ai/openai/keys/{uuid}/agents | List agents by OpenAI key
*GradientAiPlatformApi* | [**genai_list_agents_by_workspace**](docs/GradientAiPlatformApi.md#genai_list_agents_by_workspace) | **GET** /v2/gen-ai/workspaces/{workspace_uuid}/agents | List agents by Workspace
*GradientAiPlatformApi* | [**genai_list_anthropic_api_keys**](docs/GradientAiPlatformApi.md#genai_list_anthropic_api_keys) | **GET** /v2/gen-ai/anthropic/keys | List Anthropic API Keys
*GradientAiPlatformApi* | [**genai_list_datacenter_regions**](docs/GradientAiPlatformApi.md#genai_list_datacenter_regions) | **GET** /v2/gen-ai/regions | List Datacenter Regions
*GradientAiPlatformApi* | [**genai_list_evaluation_metrics**](docs/GradientAiPlatformApi.md#genai_list_evaluation_metrics) | **GET** /v2/gen-ai/evaluation_metrics | List Evaluation Metrics
*GradientAiPlatformApi* | [**genai_list_evaluation_runs_by_test_case**](docs/GradientAiPlatformApi.md#genai_list_evaluation_runs_by_test_case) | **GET** /v2/gen-ai/evaluation_test_cases/{evaluation_test_case_uuid}/evaluation_runs | List Evaluation Runs by Test Case
*GradientAiPlatformApi* | [**genai_list_evaluation_test_cases**](docs/GradientAiPlatformApi.md#genai_list_evaluation_test_cases) | **GET** /v2/gen-ai/evaluation_test_cases | List Evaluation Test Cases
*GradientAiPlatformApi* | [**genai_list_evaluation_test_cases_by_workspace**](docs/GradientAiPlatformApi.md#genai_list_evaluation_test_cases_by_workspace) | **GET** /v2/gen-ai/workspaces/{workspace_uuid}/evaluation_test_cases | List Evaluation Test Cases by Workspace
*GradientAiPlatformApi* | [**genai_list_indexing_job_data_sources**](docs/GradientAiPlatformApi.md#genai_list_indexing_job_data_sources) | **GET** /v2/gen-ai/indexing_jobs/{indexing_job_uuid}/data_sources | List Data Sources for Indexing Job for a Knowledge Base
*GradientAiPlatformApi* | [**genai_list_indexing_jobs**](docs/GradientAiPlatformApi.md#genai_list_indexing_jobs) | **GET** /v2/gen-ai/indexing_jobs | List Indexing Jobs for a Knowledge Base
*GradientAiPlatformApi* | [**genai_list_indexing_jobs_by_knowledge_base**](docs/GradientAiPlatformApi.md#genai_list_indexing_jobs_by_knowledge_base) | **GET** /v2/gen-ai/knowledge_bases/{knowledge_base_uuid}/indexing_jobs | List Indexing Jobs for a Knowledge Base
*GradientAiPlatformApi* | [**genai_list_knowledge_base_data_sources**](docs/GradientAiPlatformApi.md#genai_list_knowledge_base_data_sources) | **GET** /v2/gen-ai/knowledge_bases/{knowledge_base_uuid}/data_sources | List Data Sources for a Knowledge Base
*GradientAiPlatformApi* | [**genai_list_knowledge_bases**](docs/GradientAiPlatformApi.md#genai_list_knowledge_bases) | **GET** /v2/gen-ai/knowledge_bases | List Knowledge Bases
*GradientAiPlatformApi* | [**genai_list_model_api_keys**](docs/GradientAiPlatformApi.md#genai_list_model_api_keys) | **GET** /v2/gen-ai/models/api_keys | List Model API Keys
*GradientAiPlatformApi* | [**genai_list_models**](docs/GradientAiPlatformApi.md#genai_list_models) | **GET** /v2/gen-ai/models | List Available Models
*GradientAiPlatformApi* | [**genai_list_openai_api_keys**](docs/GradientAiPlatformApi.md#genai_list_openai_api_keys) | **GET** /v2/gen-ai/openai/keys | List OpenAI API Keys
*GradientAiPlatformApi* | [**genai_list_workspaces**](docs/GradientAiPlatformApi.md#genai_list_workspaces) | **GET** /v2/gen-ai/workspaces | List Workspaces
*GradientAiPlatformApi* | [**genai_regenerate_agent_api_key**](docs/GradientAiPlatformApi.md#genai_regenerate_agent_api_key) | **PUT** /v2/gen-ai/agents/{agent_uuid}/api_keys/{api_key_uuid}/regenerate | Regenerate API Key for an Agent
*GradientAiPlatformApi* | [**genai_regenerate_model_api_key**](docs/GradientAiPlatformApi.md#genai_regenerate_model_api_key) | **PUT** /v2/gen-ai/models/api_keys/{api_key_uuid}/regenerate | Regenerate API Key for a Model
*GradientAiPlatformApi* | [**genai_rollback_to_agent_version**](docs/GradientAiPlatformApi.md#genai_rollback_to_agent_version) | **PUT** /v2/gen-ai/agents/{uuid}/versions | Rollback to Agent Version
*GradientAiPlatformApi* | [**genai_run_evaluation_test_case**](docs/GradientAiPlatformApi.md#genai_run_evaluation_test_case) | **POST** /v2/gen-ai/evaluation_runs | Run an Evaluation Test Case
*GradientAiPlatformApi* | [**genai_update_agent**](docs/GradientAiPlatformApi.md#genai_update_agent) | **PUT** /v2/gen-ai/agents/{uuid} | Update an Agent
*GradientAiPlatformApi* | [**genai_update_agent_api_key**](docs/GradientAiPlatformApi.md#genai_update_agent_api_key) | **PUT** /v2/gen-ai/agents/{agent_uuid}/api_keys/{api_key_uuid} | Update API Key for an Agent
*GradientAiPlatformApi* | [**genai_update_agent_deployment_visibility**](docs/GradientAiPlatformApi.md#genai_update_agent_deployment_visibility) | **PUT** /v2/gen-ai/agents/{uuid}/deployment_visibility | Update Agent Status
*GradientAiPlatformApi* | [**genai_update_agent_function**](docs/GradientAiPlatformApi.md#genai_update_agent_function) | **PUT** /v2/gen-ai/agents/{agent_uuid}/functions/{function_uuid} | Update Function Route for an Agent
*GradientAiPlatformApi* | [**genai_update_agents_workspace**](docs/GradientAiPlatformApi.md#genai_update_agents_workspace) | **PUT** /v2/gen-ai/workspaces/{workspace_uuid}/agents | Move Agents to a Workspace
*GradientAiPlatformApi* | [**genai_update_anthropic_api_key**](docs/GradientAiPlatformApi.md#genai_update_anthropic_api_key) | **PUT** /v2/gen-ai/anthropic/keys/{api_key_uuid} | Update Anthropic API Key
*GradientAiPlatformApi* | [**genai_update_attached_agent**](docs/GradientAiPlatformApi.md#genai_update_attached_agent) | **PUT** /v2/gen-ai/agents/{parent_agent_uuid}/child_agents/{child_agent_uuid} | Update Agent Route for an Agent
*GradientAiPlatformApi* | [**genai_update_evaluation_test_case**](docs/GradientAiPlatformApi.md#genai_update_evaluation_test_case) | **PUT** /v2/gen-ai/evaluation_test_cases/{test_case_uuid} | Update an Evaluation Test Case.
*GradientAiPlatformApi* | [**genai_update_knowledge_base**](docs/GradientAiPlatformApi.md#genai_update_knowledge_base) | **PUT** /v2/gen-ai/knowledge_bases/{uuid} | Update a Knowledge Base
*GradientAiPlatformApi* | [**genai_update_model_api_key**](docs/GradientAiPlatformApi.md#genai_update_model_api_key) | **PUT** /v2/gen-ai/models/api_keys/{api_key_uuid} | Update API Key for a Model
*GradientAiPlatformApi* | [**genai_update_openai_api_key**](docs/GradientAiPlatformApi.md#genai_update_openai_api_key) | **PUT** /v2/gen-ai/openai/keys/{api_key_uuid} | Update OpenAI API Key
*GradientAiPlatformApi* | [**genai_update_workspace**](docs/GradientAiPlatformApi.md#genai_update_workspace) | **PUT** /v2/gen-ai/workspaces/{workspace_uuid} | Update a Workspace
*ImageActionsApi* | [**image_actions_get**](docs/ImageActionsApi.md#image_actions_get) | **GET** /v2/images/{image_id}/actions/{action_id} | Retrieve an Existing Action
*ImageActionsApi* | [**image_actions_list**](docs/ImageActionsApi.md#image_actions_list) | **GET** /v2/images/{image_id}/actions | List All Actions for an Image
*ImageActionsApi* | [**image_actions_post**](docs/ImageActionsApi.md#image_actions_post) | **POST** /v2/images/{image_id}/actions | Initiate an Image Action
*ImagesApi* | [**images_create_custom**](docs/ImagesApi.md#images_create_custom) | **POST** /v2/images | Create a Custom Image
*ImagesApi* | [**images_delete**](docs/ImagesApi.md#images_delete) | **DELETE** /v2/images/{image_id} | Delete an Image
*ImagesApi* | [**images_get**](docs/ImagesApi.md#images_get) | **GET** /v2/images/{image_id} | Retrieve an Existing Image
*ImagesApi* | [**images_list**](docs/ImagesApi.md#images_list) | **GET** /v2/images | List All Images
*ImagesApi* | [**images_update**](docs/ImagesApi.md#images_update) | **PUT** /v2/images/{image_id} | Update an Image
*KubernetesApi* | [**kubernetes_add_node_pool**](docs/KubernetesApi.md#kubernetes_add_node_pool) | **POST** /v2/kubernetes/clusters/{cluster_id}/node_pools | Add a Node Pool to a Kubernetes Cluster
*KubernetesApi* | [**kubernetes_add_registries**](docs/KubernetesApi.md#kubernetes_add_registries) | **POST** /v2/kubernetes/registries | Add Container Registries to Kubernetes Clusters
*KubernetesApi* | [**kubernetes_add_registry**](docs/KubernetesApi.md#kubernetes_add_registry) | **POST** /v2/kubernetes/registry | Add Container Registry to Kubernetes Clusters
*KubernetesApi* | [**kubernetes_create_cluster**](docs/KubernetesApi.md#kubernetes_create_cluster) | **POST** /v2/kubernetes/clusters | Create a New Kubernetes Cluster
*KubernetesApi* | [**kubernetes_delete_cluster**](docs/KubernetesApi.md#kubernetes_delete_cluster) | **DELETE** /v2/kubernetes/clusters/{cluster_id} | Delete a Kubernetes Cluster
*KubernetesApi* | [**kubernetes_delete_node**](docs/KubernetesApi.md#kubernetes_delete_node) | **DELETE** /v2/kubernetes/clusters/{cluster_id}/node_pools/{node_pool_id}/nodes/{node_id} | Delete a Node in a Kubernetes Cluster
*KubernetesApi* | [**kubernetes_delete_node_pool**](docs/KubernetesApi.md#kubernetes_delete_node_pool) | **DELETE** /v2/kubernetes/clusters/{cluster_id}/node_pools/{node_pool_id} | Delete a Node Pool in a Kubernetes Cluster
*KubernetesApi* | [**kubernetes_destroy_associated_resources_dangerous**](docs/KubernetesApi.md#kubernetes_destroy_associated_resources_dangerous) | **DELETE** /v2/kubernetes/clusters/{cluster_id}/destroy_with_associated_resources/dangerous | Delete a Cluster and All of its Associated Resources (Dangerous)
*KubernetesApi* | [**kubernetes_destroy_associated_resources_selective**](docs/KubernetesApi.md#kubernetes_destroy_associated_resources_selective) | **DELETE** /v2/kubernetes/clusters/{cluster_id}/destroy_with_associated_resources/selective | Selectively Delete a Cluster and its Associated Resources
*KubernetesApi* | [**kubernetes_get_available_upgrades**](docs/KubernetesApi.md#kubernetes_get_available_upgrades) | **GET** /v2/kubernetes/clusters/{cluster_id}/upgrades | Retrieve Available Upgrades for an Existing Kubernetes Cluster
*KubernetesApi* | [**kubernetes_get_cluster**](docs/KubernetesApi.md#kubernetes_get_cluster) | **GET** /v2/kubernetes/clusters/{cluster_id} | Retrieve an Existing Kubernetes Cluster
*KubernetesApi* | [**kubernetes_get_cluster_lint_results**](docs/KubernetesApi.md#kubernetes_get_cluster_lint_results) | **GET** /v2/kubernetes/clusters/{cluster_id}/clusterlint | Fetch Clusterlint Diagnostics for a Kubernetes Cluster
*KubernetesApi* | [**kubernetes_get_cluster_user**](docs/KubernetesApi.md#kubernetes_get_cluster_user) | **GET** /v2/kubernetes/clusters/{cluster_id}/user | Retrieve User Information for a Kubernetes Cluster
*KubernetesApi* | [**kubernetes_get_credentials**](docs/KubernetesApi.md#kubernetes_get_credentials) | **GET** /v2/kubernetes/clusters/{cluster_id}/credentials | Retrieve Credentials for a Kubernetes Cluster
*KubernetesApi* | [**kubernetes_get_kubeconfig**](docs/KubernetesApi.md#kubernetes_get_kubeconfig) | **GET** /v2/kubernetes/clusters/{cluster_id}/kubeconfig | Retrieve the kubeconfig for a Kubernetes Cluster
*KubernetesApi* | [**kubernetes_get_node_pool**](docs/KubernetesApi.md#kubernetes_get_node_pool) | **GET** /v2/kubernetes/clusters/{cluster_id}/node_pools/{node_pool_id} | Retrieve a Node Pool for a Kubernetes Cluster
*KubernetesApi* | [**kubernetes_get_status_messages**](docs/KubernetesApi.md#kubernetes_get_status_messages) | **GET** /v2/kubernetes/clusters/{cluster_id}/status_messages | Fetch Status Messages for a Kubernetes Cluster
*KubernetesApi* | [**kubernetes_list_associated_resources**](docs/KubernetesApi.md#kubernetes_list_associated_resources) | **GET** /v2/kubernetes/clusters/{cluster_id}/destroy_with_associated_resources | List Associated Resources for Cluster Deletion
*KubernetesApi* | [**kubernetes_list_clusters**](docs/KubernetesApi.md#kubernetes_list_clusters) | **GET** /v2/kubernetes/clusters | List All Kubernetes Clusters
*KubernetesApi* | [**kubernetes_list_node_pools**](docs/KubernetesApi.md#kubernetes_list_node_pools) | **GET** /v2/kubernetes/clusters/{cluster_id}/node_pools | List All Node Pools in a Kubernetes Clusters
*KubernetesApi* | [**kubernetes_list_options**](docs/KubernetesApi.md#kubernetes_list_options) | **GET** /v2/kubernetes/options | List Available Regions, Node Sizes, and Versions of Kubernetes
*KubernetesApi* | [**kubernetes_recycle_node_pool**](docs/KubernetesApi.md#kubernetes_recycle_node_pool) | **POST** /v2/kubernetes/clusters/{cluster_id}/node_pools/{node_pool_id}/recycle | Recycle a Kubernetes Node Pool
*KubernetesApi* | [**kubernetes_remove_registries**](docs/KubernetesApi.md#kubernetes_remove_registries) | **DELETE** /v2/kubernetes/registries | Remove Container Registries from Kubernetes Clusters
*KubernetesApi* | [**kubernetes_remove_registry**](docs/KubernetesApi.md#kubernetes_remove_registry) | **DELETE** /v2/kubernetes/registry | Remove Container Registry from Kubernetes Clusters
*KubernetesApi* | [**kubernetes_run_cluster_lint**](docs/KubernetesApi.md#kubernetes_run_cluster_lint) | **POST** /v2/kubernetes/clusters/{cluster_id}/clusterlint | Run Clusterlint Checks on a Kubernetes Cluster
*KubernetesApi* | [**kubernetes_update_cluster**](docs/KubernetesApi.md#kubernetes_update_cluster) | **PUT** /v2/kubernetes/clusters/{cluster_id} | Update a Kubernetes Cluster
*KubernetesApi* | [**kubernetes_update_node_pool**](docs/KubernetesApi.md#kubernetes_update_node_pool) | **PUT** /v2/kubernetes/clusters/{cluster_id}/node_pools/{node_pool_id} | Update a Node Pool in a Kubernetes Cluster
*KubernetesApi* | [**kubernetes_upgrade_cluster**](docs/KubernetesApi.md#kubernetes_upgrade_cluster) | **POST** /v2/kubernetes/clusters/{cluster_id}/upgrade | Upgrade a Kubernetes Cluster
*LoadBalancersApi* | [**load_balancers_add_droplets**](docs/LoadBalancersApi.md#load_balancers_add_droplets) | **POST** /v2/load_balancers/{lb_id}/droplets | Add Droplets to a Load Balancer
*LoadBalancersApi* | [**load_balancers_add_forwarding_rules**](docs/LoadBalancersApi.md#load_balancers_add_forwarding_rules) | **POST** /v2/load_balancers/{lb_id}/forwarding_rules | Add Forwarding Rules to a Load Balancer
*LoadBalancersApi* | [**load_balancers_create**](docs/LoadBalancersApi.md#load_balancers_create) | **POST** /v2/load_balancers | Create a New Load Balancer
*LoadBalancersApi* | [**load_balancers_delete**](docs/LoadBalancersApi.md#load_balancers_delete) | **DELETE** /v2/load_balancers/{lb_id} | Delete a Load Balancer
*LoadBalancersApi* | [**load_balancers_delete_cache**](docs/LoadBalancersApi.md#load_balancers_delete_cache) | **DELETE** /v2/load_balancers/{lb_id}/cache | Delete a Global Load Balancer CDN Cache
*LoadBalancersApi* | [**load_balancers_get**](docs/LoadBalancersApi.md#load_balancers_get) | **GET** /v2/load_balancers/{lb_id} | Retrieve an Existing Load Balancer
*LoadBalancersApi* | [**load_balancers_list**](docs/LoadBalancersApi.md#load_balancers_list) | **GET** /v2/load_balancers | List All Load Balancers
*LoadBalancersApi* | [**load_balancers_remove_droplets**](docs/LoadBalancersApi.md#load_balancers_remove_droplets) | **DELETE** /v2/load_balancers/{lb_id}/droplets | Remove Droplets from a Load Balancer
*LoadBalancersApi* | [**load_balancers_remove_forwarding_rules**](docs/LoadBalancersApi.md#load_balancers_remove_forwarding_rules) | **DELETE** /v2/load_balancers/{lb_id}/forwarding_rules | Remove Forwarding Rules from a Load Balancer
*LoadBalancersApi* | [**load_balancers_update**](docs/LoadBalancersApi.md#load_balancers_update) | **PUT** /v2/load_balancers/{lb_id} | Update a Load Balancer
*MonitoringApi* | [**monitoring_create_alert_policy**](docs/MonitoringApi.md#monitoring_create_alert_policy) | **POST** /v2/monitoring/alerts | Create Alert Policy
*MonitoringApi* | [**monitoring_create_destination**](docs/MonitoringApi.md#monitoring_create_destination) | **POST** /v2/monitoring/sinks/destinations | Create Logging Destination
*MonitoringApi* | [**monitoring_create_sink**](docs/MonitoringApi.md#monitoring_create_sink) | **POST** /v2/monitoring/sinks | Create Sink
*MonitoringApi* | [**monitoring_delete_alert_policy**](docs/MonitoringApi.md#monitoring_delete_alert_policy) | **DELETE** /v2/monitoring/alerts/{alert_uuid} | Delete an Alert Policy
*MonitoringApi* | [**monitoring_delete_destination**](docs/MonitoringApi.md#monitoring_delete_destination) | **DELETE** /v2/monitoring/sinks/destinations/{destination_uuid} | Delete Logging Destination
*MonitoringApi* | [**monitoring_delete_sink**](docs/MonitoringApi.md#monitoring_delete_sink) | **DELETE** /v2/monitoring/sinks/{sink_uuid} | Delete Sink
*MonitoringApi* | [**monitoring_get_alert_policy**](docs/MonitoringApi.md#monitoring_get_alert_policy) | **GET** /v2/monitoring/alerts/{alert_uuid} | Retrieve an Existing Alert Policy
*MonitoringApi* | [**monitoring_get_app_cpu_percentage_metrics**](docs/MonitoringApi.md#monitoring_get_app_cpu_percentage_metrics) | **GET** /v2/monitoring/metrics/apps/cpu_percentage | Get App CPU Percentage Metrics
*MonitoringApi* | [**monitoring_get_app_memory_percentage_metrics**](docs/MonitoringApi.md#monitoring_get_app_memory_percentage_metrics) | **GET** /v2/monitoring/metrics/apps/memory_percentage | Get App Memory Percentage Metrics
*MonitoringApi* | [**monitoring_get_app_restart_count_metrics_yml**](docs/MonitoringApi.md#monitoring_get_app_restart_count_metrics_yml) | **GET** /v2/monitoring/metrics/apps/restart_count | Get App Restart Count Metrics
*MonitoringApi* | [**monitoring_get_destination**](docs/MonitoringApi.md#monitoring_get_destination) | **GET** /v2/monitoring/sinks/destinations/{destination_uuid} | Get Logging Destination
*MonitoringApi* | [**monitoring_get_droplet_autoscale_current_cpu_utilization_yml**](docs/MonitoringApi.md#monitoring_get_droplet_autoscale_current_cpu_utilization_yml) | **GET** /v2/monitoring/metrics/droplet_autoscale/current_cpu_utilization | Get Droplet Autoscale Pool Current Average CPU utilization
*MonitoringApi* | [**monitoring_get_droplet_autoscale_current_instances**](docs/MonitoringApi.md#monitoring_get_droplet_autoscale_current_instances) | **GET** /v2/monitoring/metrics/droplet_autoscale/current_instances | Get Droplet Autoscale Pool Current Size
*MonitoringApi* | [**monitoring_get_droplet_autoscale_current_memory_utilization**](docs/MonitoringApi.md#monitoring_get_droplet_autoscale_current_memory_utilization) | **GET** /v2/monitoring/metrics/droplet_autoscale/current_memory_utilization | Get Droplet Autoscale Pool Current Average Memory utilization
*MonitoringApi* | [**monitoring_get_droplet_autoscale_target_cpu_utilization**](docs/MonitoringApi.md#monitoring_get_droplet_autoscale_target_cpu_utilization) | **GET** /v2/monitoring/metrics/droplet_autoscale/target_cpu_utilization | Get Droplet Autoscale Pool Target Average CPU utilization
*MonitoringApi* | [**monitoring_get_droplet_autoscale_target_instances**](docs/MonitoringApi.md#monitoring_get_droplet_autoscale_target_instances) | **GET** /v2/monitoring/metrics/droplet_autoscale/target_instances | Get Droplet Autoscale Pool Target Size
*MonitoringApi* | [**monitoring_get_droplet_autoscale_target_memory_utilization**](docs/MonitoringApi.md#monitoring_get_droplet_autoscale_target_memory_utilization) | **GET** /v2/monitoring/metrics/droplet_autoscale/target_memory_utilization | Get Droplet Autoscale Pool Target Average Memory utilization
*MonitoringApi* | [**monitoring_get_droplet_bandwidth_metrics**](docs/MonitoringApi.md#monitoring_get_droplet_bandwidth_metrics) | **GET** /v2/monitoring/metrics/droplet/bandwidth | Get Droplet Bandwidth Metrics
*MonitoringApi* | [**monitoring_get_droplet_cpu_metrics**](docs/MonitoringApi.md#monitoring_get_droplet_cpu_metrics) | **GET** /v2/monitoring/metrics/droplet/cpu | Get Droplet CPU Metrics
*MonitoringApi* | [**monitoring_get_droplet_filesystem_free_metrics**](docs/MonitoringApi.md#monitoring_get_droplet_filesystem_free_metrics) | **GET** /v2/monitoring/metrics/droplet/filesystem_free | Get Droplet Filesystem Free Metrics
*MonitoringApi* | [**monitoring_get_droplet_filesystem_size_metrics**](docs/MonitoringApi.md#monitoring_get_droplet_filesystem_size_metrics) | **GET** /v2/monitoring/metrics/droplet/filesystem_size | Get Droplet Filesystem Size Metrics
*MonitoringApi* | [**monitoring_get_droplet_load15_metrics**](docs/MonitoringApi.md#monitoring_get_droplet_load15_metrics) | **GET** /v2/monitoring/metrics/droplet/load_15 | Get Droplet Load15 Metrics
*MonitoringApi* | [**monitoring_get_droplet_load1_metrics**](docs/MonitoringApi.md#monitoring_get_droplet_load1_metrics) | **GET** /v2/monitoring/metrics/droplet/load_1 | Get Droplet Load1 Metrics
*MonitoringApi* | [**monitoring_get_droplet_load5_metrics**](docs/MonitoringApi.md#monitoring_get_droplet_load5_metrics) | **GET** /v2/monitoring/metrics/droplet/load_5 | Get Droplet Load5 Metrics
*MonitoringApi* | [**monitoring_get_droplet_memory_available_metrics**](docs/MonitoringApi.md#monitoring_get_droplet_memory_available_metrics) | **GET** /v2/monitoring/metrics/droplet/memory_available | Get Droplet Available Memory Metrics
*MonitoringApi* | [**monitoring_get_droplet_memory_cached_metrics**](docs/MonitoringApi.md#monitoring_get_droplet_memory_cached_metrics) | **GET** /v2/monitoring/metrics/droplet/memory_cached | Get Droplet Cached Memory Metrics
*MonitoringApi* | [**monitoring_get_droplet_memory_free_metrics**](docs/MonitoringApi.md#monitoring_get_droplet_memory_free_metrics) | **GET** /v2/monitoring/metrics/droplet/memory_free | Get Droplet Free Memory Metrics
*MonitoringApi* | [**monitoring_get_droplet_memory_total_metrics**](docs/MonitoringApi.md#monitoring_get_droplet_memory_total_metrics) | **GET** /v2/monitoring/metrics/droplet/memory_total | Get Droplet Total Memory Metrics
*MonitoringApi* | [**monitoring_get_lb_droplets_connections**](docs/MonitoringApi.md#monitoring_get_lb_droplets_connections) | **GET** /v2/monitoring/metrics/load_balancer/droplets_connections | Get Load Balancer Droplets Active Connections Metrics
*MonitoringApi* | [**monitoring_get_lb_droplets_downtime**](docs/MonitoringApi.md#monitoring_get_lb_droplets_downtime) | **GET** /v2/monitoring/metrics/load_balancer/droplets_downtime | Get Load Balancer Droplets Downtime Status Metrics
*MonitoringApi* | [**monitoring_get_lb_droplets_health_checks**](docs/MonitoringApi.md#monitoring_get_lb_droplets_health_checks) | **GET** /v2/monitoring/metrics/load_balancer/droplets_health_checks | Get Load Balancer Droplets Health Check Status Metrics
*MonitoringApi* | [**monitoring_get_lb_droplets_http_response_time50p**](docs/MonitoringApi.md#monitoring_get_lb_droplets_http_response_time50p) | **GET** /v2/monitoring/metrics/load_balancer/droplets_http_response_time_50p | Get Load Balancer Droplets 50th Percentile HTTP Response Time Metrics
*MonitoringApi* | [**monitoring_get_lb_droplets_http_response_time95p**](docs/MonitoringApi.md#monitoring_get_lb_droplets_http_response_time95p) | **GET** /v2/monitoring/metrics/load_balancer/droplets_http_response_time_95p | Get Load Balancer Droplets 95th Percentile HTTP Response Time Metrics
*MonitoringApi* | [**monitoring_get_lb_droplets_http_response_time99p**](docs/MonitoringApi.md#monitoring_get_lb_droplets_http_response_time99p) | **GET** /v2/monitoring/metrics/load_balancer/droplets_http_response_time_99p | Get Load Balancer Droplets 99th Percentile HTTP Response Time Metrics
*MonitoringApi* | [**monitoring_get_lb_droplets_http_response_time_avg**](docs/MonitoringApi.md#monitoring_get_lb_droplets_http_response_time_avg) | **GET** /v2/monitoring/metrics/load_balancer/droplets_http_response_time_avg | Get Load Balancer Droplets Average HTTP Response Time Metrics
*MonitoringApi* | [**monitoring_get_lb_droplets_http_responses**](docs/MonitoringApi.md#monitoring_get_lb_droplets_http_responses) | **GET** /v2/monitoring/metrics/load_balancer/droplets_http_responses | Get Load Balancer Droplets HTTP Rate Of Response Code Metrics
*MonitoringApi* | [**monitoring_get_lb_droplets_http_session_duration50p**](docs/MonitoringApi.md#monitoring_get_lb_droplets_http_session_duration50p) | **GET** /v2/monitoring/metrics/load_balancer/droplets_http_session_duration_50p | Get Load Balancer Droplets 50th Percentile HTTP Session Duration Metrics
*MonitoringApi* | [**monitoring_get_lb_droplets_http_session_duration95p**](docs/MonitoringApi.md#monitoring_get_lb_droplets_http_session_duration95p) | **GET** /v2/monitoring/metrics/load_balancer/droplets_http_session_duration_95p | Get Load Balancer Droplets 95th Percentile HTTP Session Duration Metrics
*MonitoringApi* | [**monitoring_get_lb_droplets_http_session_duration_avg**](docs/MonitoringApi.md#monitoring_get_lb_droplets_http_session_duration_avg) | **GET** /v2/monitoring/metrics/load_balancer/droplets_http_session_duration_avg | Get Load Balancer Droplets Average HTTP Session Duration Metrics
*MonitoringApi* | [**monitoring_get_lb_droplets_queue_size**](docs/MonitoringApi.md#monitoring_get_lb_droplets_queue_size) | **GET** /v2/monitoring/metrics/load_balancer/droplets_queue_size | Get Load Balancer Droplets Queue Size Metrics
*MonitoringApi* | [**monitoring_get_lb_frontend_connections_current**](docs/MonitoringApi.md#monitoring_get_lb_frontend_connections_current) | **GET** /v2/monitoring/metrics/load_balancer/frontend_connections_current | Get Load Balancer Frontend Total Current Active Connections Metrics
*MonitoringApi* | [**monitoring_get_lb_frontend_connections_limit**](docs/MonitoringApi.md#monitoring_get_lb_frontend_connections_limit) | **GET** /v2/monitoring/metrics/load_balancer/frontend_connections_limit | Get Load Balancer Frontend Max Connections Limit Metrics
*MonitoringApi* | [**monitoring_get_lb_frontend_cpu_utilization**](docs/MonitoringApi.md#monitoring_get_lb_frontend_cpu_utilization) | **GET** /v2/monitoring/metrics/load_balancer/frontend_cpu_utilization | Get Load Balancer Frontend Average Percentage CPU Utilization Metrics
*MonitoringApi* | [**monitoring_get_lb_frontend_firewall_dropped_bytes**](docs/MonitoringApi.md#monitoring_get_lb_frontend_firewall_dropped_bytes) | **GET** /v2/monitoring/metrics/load_balancer/frontend_firewall_dropped_bytes | Get Load Balancer Frontend Firewall Dropped Bytes Metrics
*MonitoringApi* | [**monitoring_get_lb_frontend_firewall_dropped_packets**](docs/MonitoringApi.md#monitoring_get_lb_frontend_firewall_dropped_packets) | **GET** /v2/monitoring/metrics/load_balancer/frontend_firewall_dropped_packets | Get Load Balancer Frontend Firewall Dropped Packets Metrics
*MonitoringApi* | [**monitoring_get_lb_frontend_http_requests_per_second**](docs/MonitoringApi.md#monitoring_get_lb_frontend_http_requests_per_second) | **GET** /v2/monitoring/metrics/load_balancer/frontend_http_requests_per_second | Get Load Balancer Frontend HTTP Requests Metrics
*MonitoringApi* | [**monitoring_get_lb_frontend_http_responses**](docs/MonitoringApi.md#monitoring_get_lb_frontend_http_responses) | **GET** /v2/monitoring/metrics/load_balancer/frontend_http_responses | Get Load Balancer Frontend HTTP Rate Of Response Code Metrics
*MonitoringApi* | [**monitoring_get_lb_frontend_network_throughput_http**](docs/MonitoringApi.md#monitoring_get_lb_frontend_network_throughput_http) | **GET** /v2/monitoring/metrics/load_balancer/frontend_network_throughput_http | Get Load Balancer Frontend HTTP Throughput Metrics
*MonitoringApi* | [**monitoring_get_lb_frontend_network_throughput_tcp**](docs/MonitoringApi.md#monitoring_get_lb_frontend_network_throughput_tcp) | **GET** /v2/monitoring/metrics/load_balancer/frontend_network_throughput_tcp | Get Load Balancer Frontend TCP Throughput Metrics
*MonitoringApi* | [**monitoring_get_lb_frontend_network_throughput_udp**](docs/MonitoringApi.md#monitoring_get_lb_frontend_network_throughput_udp) | **GET** /v2/monitoring/metrics/load_balancer/frontend_network_throughput_udp | Get Load Balancer Frontend UDP Throughput Metrics
*MonitoringApi* | [**monitoring_get_lb_frontend_nlb_tcp_network_throughput**](docs/MonitoringApi.md#monitoring_get_lb_frontend_nlb_tcp_network_throughput) | **GET** /v2/monitoring/metrics/load_balancer/frontend_nlb_tcp_network_throughput | Get Network Load Balancer Frontend TCP Throughput Metrics
*MonitoringApi* | [**monitoring_get_lb_frontend_nlb_udp_network_throughput**](docs/MonitoringApi.md#monitoring_get_lb_frontend_nlb_udp_network_throughput) | **GET** /v2/monitoring/metrics/load_balancer/frontend_nlb_udp_network_throughput | Get Network Load Balancer Frontend UDP Throughput Metrics
*MonitoringApi* | [**monitoring_get_lb_frontend_tls_connections_current**](docs/MonitoringApi.md#monitoring_get_lb_frontend_tls_connections_current) | **GET** /v2/monitoring/metrics/load_balancer/frontend_tls_connections_current | Get Load Balancer Frontend Current TLS Connections Rate Metrics
*MonitoringApi* | [**monitoring_get_lb_frontend_tls_connections_exceeding_rate_limit**](docs/MonitoringApi.md#monitoring_get_lb_frontend_tls_connections_exceeding_rate_limit) | **GET** /v2/monitoring/metrics/load_balancer/frontend_tls_connections_exceeding_rate_limit | Get Load Balancer Frontend Closed TLS Connections For Exceeded Rate Limit Metrics
*MonitoringApi* | [**monitoring_get_lb_frontend_tls_connections_limit**](docs/MonitoringApi.md#monitoring_get_lb_frontend_tls_connections_limit) | **GET** /v2/monitoring/metrics/load_balancer/frontend_tls_connections_limit | Get Load Balancer Frontend Max TLS Connections Limit Metrics
*MonitoringApi* | [**monitoring_get_sink**](docs/MonitoringApi.md#monitoring_get_sink) | **GET** /v2/monitoring/sinks/{sink_uuid} | Get Sink
*MonitoringApi* | [**monitoring_list_alert_policy**](docs/MonitoringApi.md#monitoring_list_alert_policy) | **GET** /v2/monitoring/alerts | List Alert Policies
*MonitoringApi* | [**monitoring_list_destinations**](docs/MonitoringApi.md#monitoring_list_destinations) | **GET** /v2/monitoring/sinks/destinations | List Logging Destinations
*MonitoringApi* | [**monitoring_list_sinks**](docs/MonitoringApi.md#monitoring_list_sinks) | **GET** /v2/monitoring/sinks | Lists all sinks
*MonitoringApi* | [**monitoring_update_alert_policy**](docs/MonitoringApi.md#monitoring_update_alert_policy) | **PUT** /v2/monitoring/alerts/{alert_uuid} | Update an Alert Policy
*MonitoringApi* | [**monitoring_update_destination**](docs/MonitoringApi.md#monitoring_update_destination) | **POST** /v2/monitoring/sinks/destinations/{destination_uuid} | Update Logging Destination
*NfsApi* | [**nfs_create**](docs/NfsApi.md#nfs_create) | **POST** /v2/nfs | Create a new NFS share
*NfsApi* | [**nfs_delete**](docs/NfsApi.md#nfs_delete) | **DELETE** /v2/nfs/{nfs_id} | Delete an NFS share
*NfsApi* | [**nfs_delete_snapshot**](docs/NfsApi.md#nfs_delete_snapshot) | **DELETE** /v2/nfs/snapshots/{nfs_snapshot_id} | Delete an NFS snapshot
*NfsApi* | [**nfs_get**](docs/NfsApi.md#nfs_get) | **GET** /v2/nfs/{nfs_id} | Get an NFS share
*NfsApi* | [**nfs_get_snapshot**](docs/NfsApi.md#nfs_get_snapshot) | **GET** /v2/nfs/snapshots/{nfs_snapshot_id} | Get an NFS snapshot by ID
*NfsApi* | [**nfs_list**](docs/NfsApi.md#nfs_list) | **GET** /v2/nfs | List NFS shares per region
*NfsApi* | [**nfs_list_snapshot**](docs/NfsApi.md#nfs_list_snapshot) | **GET** /v2/nfs/snapshots | List NFS snapshots per region
*NfsActionsApi* | [**nfs_create_action**](docs/NfsActionsApi.md#nfs_create_action) | **POST** /v2/nfs/{nfs_id}/actions | Initiate an NFS action
*PartnerNetworkConnectApi* | [**partner_attachments_create**](docs/PartnerNetworkConnectApi.md#partner_attachments_create) | **POST** /v2/partner_network_connect/attachments | Create a new partner attachment
*PartnerNetworkConnectApi* | [**partner_attachments_create_service_key**](docs/PartnerNetworkConnectApi.md#partner_attachments_create_service_key) | **POST** /v2/partner_network_connect/attachments/{pa_id}/service_key | Regenerate the service key for the partner attachment
*PartnerNetworkConnectApi* | [**partner_attachments_delete**](docs/PartnerNetworkConnectApi.md#partner_attachments_delete) | **DELETE** /v2/partner_network_connect/attachments/{pa_id} | Delete an existing partner attachment
*PartnerNetworkConnectApi* | [**partner_attachments_get**](docs/PartnerNetworkConnectApi.md#partner_attachments_get) | **GET** /v2/partner_network_connect/attachments/{pa_id} | Retrieve an existing partner attachment
*PartnerNetworkConnectApi* | [**partner_attachments_get_bgp_auth_key**](docs/PartnerNetworkConnectApi.md#partner_attachments_get_bgp_auth_key) | **GET** /v2/partner_network_connect/attachments/{pa_id}/bgp_auth_key | Get current BGP auth key for the partner attachment
*PartnerNetworkConnectApi* | [**partner_attachments_get_service_key**](docs/PartnerNetworkConnectApi.md#partner_attachments_get_service_key) | **GET** /v2/partner_network_connect/attachments/{pa_id}/service_key | Get the current service key for the partner attachment
*PartnerNetworkConnectApi* | [**partner_attachments_list**](docs/PartnerNetworkConnectApi.md#partner_attachments_list) | **GET** /v2/partner_network_connect/attachments | List all partner attachments
*PartnerNetworkConnectApi* | [**partner_attachments_list_remote_routes**](docs/PartnerNetworkConnectApi.md#partner_attachments_list_remote_routes) | **GET** /v2/partner_network_connect/attachments/{pa_id}/remote_routes | List remote routes for a partner attachment
*PartnerNetworkConnectApi* | [**partner_attachments_patch**](docs/PartnerNetworkConnectApi.md#partner_attachments_patch) | **PATCH** /v2/partner_network_connect/attachments/{pa_id} | Update an existing partner attachment
*ProjectResourcesApi* | [**projects_assign_resources**](docs/ProjectResourcesApi.md#projects_assign_resources) | **POST** /v2/projects/{project_id}/resources | Assign Resources to a Project
*ProjectResourcesApi* | [**projects_assign_resources_default**](docs/ProjectResourcesApi.md#projects_assign_resources_default) | **POST** /v2/projects/default/resources | Assign Resources to Default Project
*ProjectResourcesApi* | [**projects_list_resources**](docs/ProjectResourcesApi.md#projects_list_resources) | **GET** /v2/projects/{project_id}/resources | List Project Resources
*ProjectResourcesApi* | [**projects_list_resources_default**](docs/ProjectResourcesApi.md#projects_list_resources_default) | **GET** /v2/projects/default/resources | List Default Project Resources
*ProjectsApi* | [**projects_create**](docs/ProjectsApi.md#projects_create) | **POST** /v2/projects | Create a Project
*ProjectsApi* | [**projects_delete**](docs/ProjectsApi.md#projects_delete) | **DELETE** /v2/projects/{project_id} | Delete an Existing Project
*ProjectsApi* | [**projects_get**](docs/ProjectsApi.md#projects_get) | **GET** /v2/projects/{project_id} | Retrieve an Existing Project
*ProjectsApi* | [**projects_get_default**](docs/ProjectsApi.md#projects_get_default) | **GET** /v2/projects/default | Retrieve the Default Project
*ProjectsApi* | [**projects_list**](docs/ProjectsApi.md#projects_list) | **GET** /v2/projects | List All Projects
*ProjectsApi* | [**projects_patch**](docs/ProjectsApi.md#projects_patch) | **PATCH** /v2/projects/{project_id} | Patch a Project
*ProjectsApi* | [**projects_patch_default**](docs/ProjectsApi.md#projects_patch_default) | **PATCH** /v2/projects/default | Patch the Default Project
*ProjectsApi* | [**projects_update**](docs/ProjectsApi.md#projects_update) | **PUT** /v2/projects/{project_id} | Update a Project
*ProjectsApi* | [**projects_update_default**](docs/ProjectsApi.md#projects_update_default) | **PUT** /v2/projects/default | Update the Default Project
*RegionsApi* | [**regions_list**](docs/RegionsApi.md#regions_list) | **GET** /v2/regions | List All Data Center Regions
*ReservedIpActionsApi* | [**reserved_ips_actions_get**](docs/ReservedIpActionsApi.md#reserved_ips_actions_get) | **GET** /v2/reserved_ips/{reserved_ip}/actions/{action_id} | Retrieve an Existing Reserved IP Action
*ReservedIpActionsApi* | [**reserved_ips_actions_list**](docs/ReservedIpActionsApi.md#reserved_ips_actions_list) | **GET** /v2/reserved_ips/{reserved_ip}/actions | List All Actions for a Reserved IP
*ReservedIpActionsApi* | [**reserved_ips_actions_post**](docs/ReservedIpActionsApi.md#reserved_ips_actions_post) | **POST** /v2/reserved_ips/{reserved_ip}/actions | Initiate a Reserved IP Action
*ReservedIpsApi* | [**reserved_ips_create**](docs/ReservedIpsApi.md#reserved_ips_create) | **POST** /v2/reserved_ips | Create a New Reserved IP
*ReservedIpsApi* | [**reserved_ips_delete**](docs/ReservedIpsApi.md#reserved_ips_delete) | **DELETE** /v2/reserved_ips/{reserved_ip} | Delete a Reserved IP
*ReservedIpsApi* | [**reserved_ips_get**](docs/ReservedIpsApi.md#reserved_ips_get) | **GET** /v2/reserved_ips/{reserved_ip} | Retrieve an Existing Reserved IP
*ReservedIpsApi* | [**reserved_ips_list**](docs/ReservedIpsApi.md#reserved_ips_list) | **GET** /v2/reserved_ips | List All Reserved IPs
*ReservedIpv6Api* | [**reserved_ipv6_create**](docs/ReservedIpv6Api.md#reserved_ipv6_create) | **POST** /v2/reserved_ipv6 | Create a New Reserved IPv6
*ReservedIpv6Api* | [**reserved_ipv6_delete**](docs/ReservedIpv6Api.md#reserved_ipv6_delete) | **DELETE** /v2/reserved_ipv6/{reserved_ipv6} | Delete a Reserved IPv6
*ReservedIpv6Api* | [**reserved_ipv6_get**](docs/ReservedIpv6Api.md#reserved_ipv6_get) | **GET** /v2/reserved_ipv6/{reserved_ipv6} | Retrieve an Existing Reserved IPv6
*ReservedIpv6Api* | [**reserved_ipv6_list**](docs/ReservedIpv6Api.md#reserved_ipv6_list) | **GET** /v2/reserved_ipv6 | List All Reserved IPv6s
*ReservedIpv6ActionsApi* | [**reserved_ipv6_actions_post**](docs/ReservedIpv6ActionsApi.md#reserved_ipv6_actions_post) | **POST** /v2/reserved_ipv6/{reserved_ipv6}/actions | Initiate a Reserved IPv6 Action
*SshKeysApi* | [**ssh_keys_create**](docs/SshKeysApi.md#ssh_keys_create) | **POST** /v2/account/keys | Create a New SSH Key
*SshKeysApi* | [**ssh_keys_delete**](docs/SshKeysApi.md#ssh_keys_delete) | **DELETE** /v2/account/keys/{ssh_key_identifier} | Delete an SSH Key
*SshKeysApi* | [**ssh_keys_get**](docs/SshKeysApi.md#ssh_keys_get) | **GET** /v2/account/keys/{ssh_key_identifier} | Retrieve an Existing SSH Key
*SshKeysApi* | [**ssh_keys_list**](docs/SshKeysApi.md#ssh_keys_list) | **GET** /v2/account/keys | List All SSH Keys
*SshKeysApi* | [**ssh_keys_update**](docs/SshKeysApi.md#ssh_keys_update) | **PUT** /v2/account/keys/{ssh_key_identifier} | Update an SSH Key's Name
*SizesApi* | [**sizes_list**](docs/SizesApi.md#sizes_list) | **GET** /v2/sizes | List All Droplet Sizes
*SnapshotsApi* | [**snapshots_delete**](docs/SnapshotsApi.md#snapshots_delete) | **DELETE** /v2/snapshots/{snapshot_id} | Delete a Snapshot
*SnapshotsApi* | [**snapshots_get**](docs/SnapshotsApi.md#snapshots_get) | **GET** /v2/snapshots/{snapshot_id} | Retrieve an Existing Snapshot
*SnapshotsApi* | [**snapshots_list**](docs/SnapshotsApi.md#snapshots_list) | **GET** /v2/snapshots | List All Snapshots
*SpacesKeysApi* | [**spaces_key_create**](docs/SpacesKeysApi.md#spaces_key_create) | **POST** /v2/spaces/keys | Create a New Spaces Access Key
*SpacesKeysApi* | [**spaces_key_delete**](docs/SpacesKeysApi.md#spaces_key_delete) | **DELETE** /v2/spaces/keys/{access_key} | Delete a Spaces Access Key
*SpacesKeysApi* | [**spaces_key_get**](docs/SpacesKeysApi.md#spaces_key_get) | **GET** /v2/spaces/keys/{access_key} | Get a Spaces Access Key
*SpacesKeysApi* | [**spaces_key_list**](docs/SpacesKeysApi.md#spaces_key_list) | **GET** /v2/spaces/keys | List Spaces Access Keys
*SpacesKeysApi* | [**spaces_key_patch**](docs/SpacesKeysApi.md#spaces_key_patch) | **PATCH** /v2/spaces/keys/{access_key} | Update Spaces Access Keys
*SpacesKeysApi* | [**spaces_key_update**](docs/SpacesKeysApi.md#spaces_key_update) | **PUT** /v2/spaces/keys/{access_key} | Update Spaces Access Keys
*TagsApi* | [**tags_assign_resources**](docs/TagsApi.md#tags_assign_resources) | **POST** /v2/tags/{tag_id}/resources | Tag a Resource
*TagsApi* | [**tags_create**](docs/TagsApi.md#tags_create) | **POST** /v2/tags | Create a New Tag
*TagsApi* | [**tags_delete**](docs/TagsApi.md#tags_delete) | **DELETE** /v2/tags/{tag_id} | Delete a Tag
*TagsApi* | [**tags_get**](docs/TagsApi.md#tags_get) | **GET** /v2/tags/{tag_id} | Retrieve a Tag
*TagsApi* | [**tags_list**](docs/TagsApi.md#tags_list) | **GET** /v2/tags | List All Tags
*TagsApi* | [**tags_unassign_resources**](docs/TagsApi.md#tags_unassign_resources) | **DELETE** /v2/tags/{tag_id}/resources | Untag a Resource
*UptimeApi* | [**uptime_create_alert**](docs/UptimeApi.md#uptime_create_alert) | **POST** /v2/uptime/checks/{check_id}/alerts | Create a New Alert
*UptimeApi* | [**uptime_create_check**](docs/UptimeApi.md#uptime_create_check) | **POST** /v2/uptime/checks | Create a New Check
*UptimeApi* | [**uptime_delete_alert**](docs/UptimeApi.md#uptime_delete_alert) | **DELETE** /v2/uptime/checks/{check_id}/alerts/{alert_id} | Delete an Alert
*UptimeApi* | [**uptime_delete_check**](docs/UptimeApi.md#uptime_delete_check) | **DELETE** /v2/uptime/checks/{check_id} | Delete a Check
*UptimeApi* | [**uptime_get_alert**](docs/UptimeApi.md#uptime_get_alert) | **GET** /v2/uptime/checks/{check_id}/alerts/{alert_id} | Retrieve an Existing Alert
*UptimeApi* | [**uptime_get_check**](docs/UptimeApi.md#uptime_get_check) | **GET** /v2/uptime/checks/{check_id} | Retrieve an Existing Check
*UptimeApi* | [**uptime_get_check_state**](docs/UptimeApi.md#uptime_get_check_state) | **GET** /v2/uptime/checks/{check_id}/state | Retrieve Check State
*UptimeApi* | [**uptime_list_alerts**](docs/UptimeApi.md#uptime_list_alerts) | **GET** /v2/uptime/checks/{check_id}/alerts | List All Alerts
*UptimeApi* | [**uptime_list_checks**](docs/UptimeApi.md#uptime_list_checks) | **GET** /v2/uptime/checks | List All Checks
*UptimeApi* | [**uptime_update_alert**](docs/UptimeApi.md#uptime_update_alert) | **PUT** /v2/uptime/checks/{check_id}/alerts/{alert_id} | Update an Alert
*UptimeApi* | [**uptime_update_check**](docs/UptimeApi.md#uptime_update_check) | **PUT** /v2/uptime/checks/{check_id} | Update a Check
*VpcnatGatewaysApi* | [**vpcnatgateways_create**](docs/VpcnatGatewaysApi.md#vpcnatgateways_create) | **POST** /v2/vpc_nat_gateways | Create a New VPC NAT Gateway
*VpcnatGatewaysApi* | [**vpcnatgateways_delete**](docs/VpcnatGatewaysApi.md#vpcnatgateways_delete) | **DELETE** /v2/vpc_nat_gateways/{id} | Delete VPC NAT Gateway
*VpcnatGatewaysApi* | [**vpcnatgateways_get**](docs/VpcnatGatewaysApi.md#vpcnatgateways_get) | **GET** /v2/vpc_nat_gateways/{id} | Retrieve an Existing VPC NAT Gateway
*VpcnatGatewaysApi* | [**vpcnatgateways_list**](docs/VpcnatGatewaysApi.md#vpcnatgateways_list) | **GET** /v2/vpc_nat_gateways | List All VPC NAT Gateways
*VpcnatGatewaysApi* | [**vpcnatgateways_update**](docs/VpcnatGatewaysApi.md#vpcnatgateways_update) | **PUT** /v2/vpc_nat_gateways/{id} | Update VPC NAT Gateway
*VpcPeeringsApi* | [**vpc_peerings_create**](docs/VpcPeeringsApi.md#vpc_peerings_create) | **POST** /v2/vpc_peerings | Create a New VPC Peering
*VpcPeeringsApi* | [**vpc_peerings_delete**](docs/VpcPeeringsApi.md#vpc_peerings_delete) | **DELETE** /v2/vpc_peerings/{vpc_peering_id} | Delete a VPC peering
*VpcPeeringsApi* | [**vpc_peerings_get**](docs/VpcPeeringsApi.md#vpc_peerings_get) | **GET** /v2/vpc_peerings/{vpc_peering_id} | Retrieve an Existing VPC Peering
*VpcPeeringsApi* | [**vpc_peerings_list**](docs/VpcPeeringsApi.md#vpc_peerings_list) | **GET** /v2/vpc_peerings | List All VPC Peerings
*VpcPeeringsApi* | [**vpc_peerings_patch**](docs/VpcPeeringsApi.md#vpc_peerings_patch) | **PATCH** /v2/vpc_peerings/{vpc_peering_id} | Update a VPC peering
*VpcsApi* | [**vpcs_create**](docs/VpcsApi.md#vpcs_create) | **POST** /v2/vpcs | Create a New VPC
*VpcsApi* | [**vpcs_create_peerings**](docs/VpcsApi.md#vpcs_create_peerings) | **POST** /v2/vpcs/{vpc_id}/peerings | Create a Peering with a VPC
*VpcsApi* | [**vpcs_delete**](docs/VpcsApi.md#vpcs_delete) | **DELETE** /v2/vpcs/{vpc_id} | Delete a VPC
*VpcsApi* | [**vpcs_get**](docs/VpcsApi.md#vpcs_get) | **GET** /v2/vpcs/{vpc_id} | Retrieve an Existing VPC
*VpcsApi* | [**vpcs_list**](docs/VpcsApi.md#vpcs_list) | **GET** /v2/vpcs | List All VPCs
*VpcsApi* | [**vpcs_list_members**](docs/VpcsApi.md#vpcs_list_members) | **GET** /v2/vpcs/{vpc_id}/members | List the Member Resources of a VPC
*VpcsApi* | [**vpcs_list_peerings**](docs/VpcsApi.md#vpcs_list_peerings) | **GET** /v2/vpcs/{vpc_id}/peerings | List the Peerings of a VPC
*VpcsApi* | [**vpcs_patch**](docs/VpcsApi.md#vpcs_patch) | **PATCH** /v2/vpcs/{vpc_id} | Partially Update a VPC
*VpcsApi* | [**vpcs_patch_peerings**](docs/VpcsApi.md#vpcs_patch_peerings) | **PATCH** /v2/vpcs/{vpc_id}/peerings/{vpc_peering_id} | Update a VPC Peering
*VpcsApi* | [**vpcs_update**](docs/VpcsApi.md#vpcs_update) | **PUT** /v2/vpcs/{vpc_id} | Update a VPC


## Documentation For Models

 - [Account](docs/Account.md)
 - [AccountGet200Response](docs/AccountGet200Response.md)
 - [AccountTeam](docs/AccountTeam.md)
 - [Action](docs/Action.md)
 - [ActionLink](docs/ActionLink.md)
 - [ActionsGet200Response](docs/ActionsGet200Response.md)
 - [ActionsList200Response](docs/ActionsList200Response.md)
 - [AddonsAppInfo](docs/AddonsAppInfo.md)
 - [AddonsAppMetadata](docs/AddonsAppMetadata.md)
 - [AddonsCreate200Response](docs/AddonsCreate200Response.md)
 - [AddonsDimensionVolumeWithPrice](docs/AddonsDimensionVolumeWithPrice.md)
 - [AddonsDimensionWithPrice](docs/AddonsDimensionWithPrice.md)
 - [AddonsFeature](docs/AddonsFeature.md)
 - [AddonsFeatureValue](docs/AddonsFeatureValue.md)
 - [AddonsGetApp200Response](docs/AddonsGetApp200Response.md)
 - [AddonsGetAppMetadata200Response](docs/AddonsGetAppMetadata200Response.md)
 - [AddonsList200Response](docs/AddonsList200Response.md)
 - [AddonsPatchPlanRequest](docs/AddonsPatchPlanRequest.md)
 - [AddonsPatchRequest](docs/AddonsPatchRequest.md)
 - [AddonsPlan](docs/AddonsPlan.md)
 - [AddonsResource](docs/AddonsResource.md)
 - [AddonsResourceMetadata](docs/AddonsResourceMetadata.md)
 - [AddonsResourceMetadataValue](docs/AddonsResourceMetadataValue.md)
 - [AddonsResourceNew](docs/AddonsResourceNew.md)
 - [Alert](docs/Alert.md)
 - [AlertBase](docs/AlertBase.md)
 - [AlertPolicy](docs/AlertPolicy.md)
 - [AlertPolicyRequest](docs/AlertPolicyRequest.md)
 - [AlertUpdatable](docs/AlertUpdatable.md)
 - [Alerts](docs/Alerts.md)
 - [AmdGpuDeviceMetricsExporterPlugin](docs/AmdGpuDeviceMetricsExporterPlugin.md)
 - [AmdGpuDevicePlugin](docs/AmdGpuDevicePlugin.md)
 - [ApiAgent](docs/ApiAgent.md)
 - [ApiAgentApiKey](docs/ApiAgentApiKey.md)
 - [ApiAgentApiKeyInfo](docs/ApiAgentApiKeyInfo.md)
 - [ApiAgentChatbotIdentifier](docs/ApiAgentChatbotIdentifier.md)
 - [ApiAgentChildRelationshipVerion](docs/ApiAgentChildRelationshipVerion.md)
 - [ApiAgentFunction](docs/ApiAgentFunction.md)
 - [ApiAgentFunctionVersion](docs/ApiAgentFunctionVersion.md)
 - [ApiAgentGuardrail](docs/ApiAgentGuardrail.md)
 - [ApiAgentGuardrailVersion](docs/ApiAgentGuardrailVersion.md)
 - [ApiAgentKnowledgeBaseVersion](docs/ApiAgentKnowledgeBaseVersion.md)
 - [ApiAgentLoggingConfig](docs/ApiAgentLoggingConfig.md)
 - [ApiAgentPublic](docs/ApiAgentPublic.md)
 - [ApiAgentTemplate](docs/ApiAgentTemplate.md)
 - [ApiAgentTemplateGuardrail](docs/ApiAgentTemplateGuardrail.md)
 - [ApiAgentTemplateType](docs/ApiAgentTemplateType.md)
 - [ApiAgentVersion](docs/ApiAgentVersion.md)
 - [ApiAgreement](docs/ApiAgreement.md)
 - [ApiAnthropicApiKeyInfo](docs/ApiAnthropicApiKeyInfo.md)
 - [ApiAuditHeader](docs/ApiAuditHeader.md)
 - [ApiAwsDataSource](docs/ApiAwsDataSource.md)
 - [ApiAwsDataSourceDisplay](docs/ApiAwsDataSourceDisplay.md)
 - [ApiBatchJobPhase](docs/ApiBatchJobPhase.md)
 - [ApiCancelKnowledgeBaseIndexingJobInputPublic](docs/ApiCancelKnowledgeBaseIndexingJobInputPublic.md)
 - [ApiCancelKnowledgeBaseIndexingJobOutput](docs/ApiCancelKnowledgeBaseIndexingJobOutput.md)
 - [ApiChatbot](docs/ApiChatbot.md)
 - [ApiChunkingAlgorithm](docs/ApiChunkingAlgorithm.md)
 - [ApiChunkingOptions](docs/ApiChunkingOptions.md)
 - [ApiCrawlingOption](docs/ApiCrawlingOption.md)
 - [ApiCreateAgentApiKeyInputPublic](docs/ApiCreateAgentApiKeyInputPublic.md)
 - [ApiCreateAgentApiKeyOutput](docs/ApiCreateAgentApiKeyOutput.md)
 - [ApiCreateAgentInputPublic](docs/ApiCreateAgentInputPublic.md)
 - [ApiCreateAgentOutput](docs/ApiCreateAgentOutput.md)
 - [ApiCreateAnthropicApiKeyInputPublic](docs/ApiCreateAnthropicApiKeyInputPublic.md)
 - [ApiCreateAnthropicApiKeyOutput](docs/ApiCreateAnthropicApiKeyOutput.md)
 - [ApiCreateDataSourceFileUploadPresignedUrlsInputPublic](docs/ApiCreateDataSourceFileUploadPresignedUrlsInputPublic.md)
 - [ApiCreateDataSourceFileUploadPresignedUrlsOutput](docs/ApiCreateDataSourceFileUploadPresignedUrlsOutput.md)
 - [ApiCreateEvaluationDatasetInputPublic](docs/ApiCreateEvaluationDatasetInputPublic.md)
 - [ApiCreateEvaluationDatasetOutput](docs/ApiCreateEvaluationDatasetOutput.md)
 - [ApiCreateEvaluationTestCaseInputPublic](docs/ApiCreateEvaluationTestCaseInputPublic.md)
 - [ApiCreateEvaluationTestCaseOutput](docs/ApiCreateEvaluationTestCaseOutput.md)
 - [ApiCreateKnowledgeBaseDataSourceInputPublic](docs/ApiCreateKnowledgeBaseDataSourceInputPublic.md)
 - [ApiCreateKnowledgeBaseDataSourceOutput](docs/ApiCreateKnowledgeBaseDataSourceOutput.md)
 - [ApiCreateKnowledgeBaseInputPublic](docs/ApiCreateKnowledgeBaseInputPublic.md)
 - [ApiCreateKnowledgeBaseOutput](docs/ApiCreateKnowledgeBaseOutput.md)
 - [ApiCreateModelApiKeyInputPublic](docs/ApiCreateModelApiKeyInputPublic.md)
 - [ApiCreateModelApiKeyOutput](docs/ApiCreateModelApiKeyOutput.md)
 - [ApiCreateOpenAiapiKeyInputPublic](docs/ApiCreateOpenAiapiKeyInputPublic.md)
 - [ApiCreateOpenAiapiKeyOutput](docs/ApiCreateOpenAiapiKeyOutput.md)
 - [ApiCreateScheduledIndexingInputPublic](docs/ApiCreateScheduledIndexingInputPublic.md)
 - [ApiCreateScheduledIndexingOutput](docs/ApiCreateScheduledIndexingOutput.md)
 - [ApiCreateWorkspaceInputPublic](docs/ApiCreateWorkspaceInputPublic.md)
 - [ApiCreateWorkspaceOutput](docs/ApiCreateWorkspaceOutput.md)
 - [ApiDeleteAgentApiKeyOutput](docs/ApiDeleteAgentApiKeyOutput.md)
 - [ApiDeleteAgentOutput](docs/ApiDeleteAgentOutput.md)
 - [ApiDeleteAnthropicApiKeyOutput](docs/ApiDeleteAnthropicApiKeyOutput.md)
 - [ApiDeleteKnowledgeBaseDataSourceOutput](docs/ApiDeleteKnowledgeBaseDataSourceOutput.md)
 - [ApiDeleteKnowledgeBaseOutput](docs/ApiDeleteKnowledgeBaseOutput.md)
 - [ApiDeleteModelApiKeyOutput](docs/ApiDeleteModelApiKeyOutput.md)
 - [ApiDeleteOpenAiapiKeyOutput](docs/ApiDeleteOpenAiapiKeyOutput.md)
 - [ApiDeleteScheduledIndexingOutput](docs/ApiDeleteScheduledIndexingOutput.md)
 - [ApiDeleteWorkspaceOutput](docs/ApiDeleteWorkspaceOutput.md)
 - [ApiDeployment](docs/ApiDeployment.md)
 - [ApiDeploymentStatus](docs/ApiDeploymentStatus.md)
 - [ApiDeploymentVisibility](docs/ApiDeploymentVisibility.md)
 - [ApiDropboxDataSource](docs/ApiDropboxDataSource.md)
 - [ApiDropboxDataSourceDisplay](docs/ApiDropboxDataSourceDisplay.md)
 - [ApiDropboxOauth2GetTokensInput](docs/ApiDropboxOauth2GetTokensInput.md)
 - [ApiDropboxOauth2GetTokensOutput](docs/ApiDropboxOauth2GetTokensOutput.md)
 - [ApiEvaluationDataset](docs/ApiEvaluationDataset.md)
 - [ApiEvaluationDatasetType](docs/ApiEvaluationDatasetType.md)
 - [ApiEvaluationMetric](docs/ApiEvaluationMetric.md)
 - [ApiEvaluationMetricCategory](docs/ApiEvaluationMetricCategory.md)
 - [ApiEvaluationMetricResult](docs/ApiEvaluationMetricResult.md)
 - [ApiEvaluationMetricType](docs/ApiEvaluationMetricType.md)
 - [ApiEvaluationMetricValueType](docs/ApiEvaluationMetricValueType.md)
 - [ApiEvaluationRun](docs/ApiEvaluationRun.md)
 - [ApiEvaluationRunStatus](docs/ApiEvaluationRunStatus.md)
 - [ApiEvaluationTestCase](docs/ApiEvaluationTestCase.md)
 - [ApiEvaluationTestCaseMetricList](docs/ApiEvaluationTestCaseMetricList.md)
 - [ApiEvaluationTraceSpan](docs/ApiEvaluationTraceSpan.md)
 - [ApiFilePresignedUrlResponse](docs/ApiFilePresignedUrlResponse.md)
 - [ApiFileUploadDataSource](docs/ApiFileUploadDataSource.md)
 - [ApiGenerateOauth2UrlOutput](docs/ApiGenerateOauth2UrlOutput.md)
 - [ApiGetAgentOutput](docs/ApiGetAgentOutput.md)
 - [ApiGetAgentUsageOutput](docs/ApiGetAgentUsageOutput.md)
 - [ApiGetAnthropicApiKeyOutput](docs/ApiGetAnthropicApiKeyOutput.md)
 - [ApiGetChildrenOutput](docs/ApiGetChildrenOutput.md)
 - [ApiGetEvaluationRunOutput](docs/ApiGetEvaluationRunOutput.md)
 - [ApiGetEvaluationRunPromptResultsOutput](docs/ApiGetEvaluationRunPromptResultsOutput.md)
 - [ApiGetEvaluationRunResultsOutput](docs/ApiGetEvaluationRunResultsOutput.md)
 - [ApiGetEvaluationTestCaseOutput](docs/ApiGetEvaluationTestCaseOutput.md)
 - [ApiGetIndexingJobDetailsSignedUrlOutput](docs/ApiGetIndexingJobDetailsSignedUrlOutput.md)
 - [ApiGetKnowledgeBaseIndexingJobOutput](docs/ApiGetKnowledgeBaseIndexingJobOutput.md)
 - [ApiGetKnowledgeBaseOutput](docs/ApiGetKnowledgeBaseOutput.md)
 - [ApiGetOpenAiapiKeyOutput](docs/ApiGetOpenAiapiKeyOutput.md)
 - [ApiGetScheduledIndexingOutput](docs/ApiGetScheduledIndexingOutput.md)
 - [ApiGetWorkspaceOutput](docs/ApiGetWorkspaceOutput.md)
 - [ApiGoogleDriveDataSource](docs/ApiGoogleDriveDataSource.md)
 - [ApiGoogleDriveDataSourceDisplay](docs/ApiGoogleDriveDataSourceDisplay.md)
 - [ApiGuardrailType](docs/ApiGuardrailType.md)
 - [ApiIndexJobStatus](docs/ApiIndexJobStatus.md)
 - [ApiIndexedDataSource](docs/ApiIndexedDataSource.md)
 - [ApiIndexedDataSourceStatus](docs/ApiIndexedDataSourceStatus.md)
 - [ApiIndexingJob](docs/ApiIndexingJob.md)
 - [ApiKbDataSource](docs/ApiKbDataSource.md)
 - [ApiKnowledgeBase](docs/ApiKnowledgeBase.md)
 - [ApiKnowledgeBaseDataSource](docs/ApiKnowledgeBaseDataSource.md)
 - [ApiLinkAgentFunctionInputPublic](docs/ApiLinkAgentFunctionInputPublic.md)
 - [ApiLinkAgentFunctionOutput](docs/ApiLinkAgentFunctionOutput.md)
 - [ApiLinkAgentInputPublic](docs/ApiLinkAgentInputPublic.md)
 - [ApiLinkAgentOutput](docs/ApiLinkAgentOutput.md)
 - [ApiLinkKnowledgeBaseOutput](docs/ApiLinkKnowledgeBaseOutput.md)
 - [ApiLinks](docs/ApiLinks.md)
 - [ApiListAgentApiKeysOutput](docs/ApiListAgentApiKeysOutput.md)
 - [ApiListAgentVersionsOutput](docs/ApiListAgentVersionsOutput.md)
 - [ApiListAgentsByAnthropicKeyOutput](docs/ApiListAgentsByAnthropicKeyOutput.md)
 - [ApiListAgentsByOpenAiKeyOutput](docs/ApiListAgentsByOpenAiKeyOutput.md)
 - [ApiListAgentsByWorkspaceOutput](docs/ApiListAgentsByWorkspaceOutput.md)
 - [ApiListAgentsOutputPublic](docs/ApiListAgentsOutputPublic.md)
 - [ApiListAnthropicApiKeysOutput](docs/ApiListAnthropicApiKeysOutput.md)
 - [ApiListEvaluationMetricsOutput](docs/ApiListEvaluationMetricsOutput.md)
 - [ApiListEvaluationRunsByTestCaseOutput](docs/ApiListEvaluationRunsByTestCaseOutput.md)
 - [ApiListEvaluationTestCasesByWorkspaceOutput](docs/ApiListEvaluationTestCasesByWorkspaceOutput.md)
 - [ApiListEvaluationTestCasesOutput](docs/ApiListEvaluationTestCasesOutput.md)
 - [ApiListIndexingJobDataSourcesOutput](docs/ApiListIndexingJobDataSourcesOutput.md)
 - [ApiListKnowledgeBaseDataSourcesOutput](docs/ApiListKnowledgeBaseDataSourcesOutput.md)
 - [ApiListKnowledgeBaseIndexingJobsOutput](docs/ApiListKnowledgeBaseIndexingJobsOutput.md)
 - [ApiListKnowledgeBasesOutput](docs/ApiListKnowledgeBasesOutput.md)
 - [ApiListModelApiKeysOutput](docs/ApiListModelApiKeysOutput.md)
 - [ApiListModelsOutputPublic](docs/ApiListModelsOutputPublic.md)
 - [ApiListOpenAiapiKeysOutput](docs/ApiListOpenAiapiKeysOutput.md)
 - [ApiListRegionsOutput](docs/ApiListRegionsOutput.md)
 - [ApiListWorkspacesOutput](docs/ApiListWorkspacesOutput.md)
 - [ApiMeta](docs/ApiMeta.md)
 - [ApiModel](docs/ApiModel.md)
 - [ApiModelApiKeyInfo](docs/ApiModelApiKeyInfo.md)
 - [ApiModelProvider](docs/ApiModelProvider.md)
 - [ApiModelProviderKeyInfo](docs/ApiModelProviderKeyInfo.md)
 - [ApiModelPublic](docs/ApiModelPublic.md)
 - [ApiModelUsecase](docs/ApiModelUsecase.md)
 - [ApiModelVersion](docs/ApiModelVersion.md)
 - [ApiMoveAgentsToWorkspaceInputPublic](docs/ApiMoveAgentsToWorkspaceInputPublic.md)
 - [ApiMoveAgentsToWorkspaceOutput](docs/ApiMoveAgentsToWorkspaceOutput.md)
 - [ApiOpenAiapiKeyInfo](docs/ApiOpenAiapiKeyInfo.md)
 - [ApiPages](docs/ApiPages.md)
 - [ApiPresignedUrlFile](docs/ApiPresignedUrlFile.md)
 - [ApiPrompt](docs/ApiPrompt.md)
 - [ApiPromptChunk](docs/ApiPromptChunk.md)
 - [ApiRegenerateAgentApiKeyOutput](docs/ApiRegenerateAgentApiKeyOutput.md)
 - [ApiRegenerateModelApiKeyOutput](docs/ApiRegenerateModelApiKeyOutput.md)
 - [ApiResourceUsage](docs/ApiResourceUsage.md)
 - [ApiRetrievalMethod](docs/ApiRetrievalMethod.md)
 - [ApiRollbackToAgentVersionInputPublic](docs/ApiRollbackToAgentVersionInputPublic.md)
 - [ApiRollbackToAgentVersionOutput](docs/ApiRollbackToAgentVersionOutput.md)
 - [ApiRunEvaluationTestCaseInputPublic](docs/ApiRunEvaluationTestCaseInputPublic.md)
 - [ApiRunEvaluationTestCaseOutput](docs/ApiRunEvaluationTestCaseOutput.md)
 - [ApiScheduledIndexingInfo](docs/ApiScheduledIndexingInfo.md)
 - [ApiSpacesDataSource](docs/ApiSpacesDataSource.md)
 - [ApiStarMetric](docs/ApiStarMetric.md)
 - [ApiStartKnowledgeBaseIndexingJobInputPublic](docs/ApiStartKnowledgeBaseIndexingJobInputPublic.md)
 - [ApiStartKnowledgeBaseIndexingJobOutput](docs/ApiStartKnowledgeBaseIndexingJobOutput.md)
 - [ApiTraceSpanType](docs/ApiTraceSpanType.md)
 - [ApiUnlinkAgentFunctionOutput](docs/ApiUnlinkAgentFunctionOutput.md)
 - [ApiUnlinkAgentOutput](docs/ApiUnlinkAgentOutput.md)
 - [ApiUnlinkKnowledgeBaseOutput](docs/ApiUnlinkKnowledgeBaseOutput.md)
 - [ApiUpdateAgentApiKeyInputPublic](docs/ApiUpdateAgentApiKeyInputPublic.md)
 - [ApiUpdateAgentApiKeyOutput](docs/ApiUpdateAgentApiKeyOutput.md)
 - [ApiUpdateAgentDeploymentVisbilityOutput](docs/ApiUpdateAgentDeploymentVisbilityOutput.md)
 - [ApiUpdateAgentDeploymentVisibilityInputPublic](docs/ApiUpdateAgentDeploymentVisibilityInputPublic.md)
 - [ApiUpdateAgentFunctionInputPublic](docs/ApiUpdateAgentFunctionInputPublic.md)
 - [ApiUpdateAgentFunctionOutput](docs/ApiUpdateAgentFunctionOutput.md)
 - [ApiUpdateAgentInputPublic](docs/ApiUpdateAgentInputPublic.md)
 - [ApiUpdateAgentOutput](docs/ApiUpdateAgentOutput.md)
 - [ApiUpdateAnthropicApiKeyInputPublic](docs/ApiUpdateAnthropicApiKeyInputPublic.md)
 - [ApiUpdateAnthropicApiKeyOutput](docs/ApiUpdateAnthropicApiKeyOutput.md)
 - [ApiUpdateEvaluationTestCaseInputPublic](docs/ApiUpdateEvaluationTestCaseInputPublic.md)
 - [ApiUpdateEvaluationTestCaseOutput](docs/ApiUpdateEvaluationTestCaseOutput.md)
 - [ApiUpdateKnowledgeBaseInputPublic](docs/ApiUpdateKnowledgeBaseInputPublic.md)
 - [ApiUpdateKnowledgeBaseOutput](docs/ApiUpdateKnowledgeBaseOutput.md)
 - [ApiUpdateLinkedAgentInputPublic](docs/ApiUpdateLinkedAgentInputPublic.md)
 - [ApiUpdateLinkedAgentOutput](docs/ApiUpdateLinkedAgentOutput.md)
 - [ApiUpdateModelApiKeyInputPublic](docs/ApiUpdateModelApiKeyInputPublic.md)
 - [ApiUpdateModelApiKeyOutput](docs/ApiUpdateModelApiKeyOutput.md)
 - [ApiUpdateOpenAiapiKeyInputPublic](docs/ApiUpdateOpenAiapiKeyInputPublic.md)
 - [ApiUpdateOpenAiapiKeyOutput](docs/ApiUpdateOpenAiapiKeyOutput.md)
 - [ApiUpdateWorkspaceInputPublic](docs/ApiUpdateWorkspaceInputPublic.md)
 - [ApiUpdateWorkspaceOutput](docs/ApiUpdateWorkspaceOutput.md)
 - [ApiUsageMeasurement](docs/ApiUsageMeasurement.md)
 - [ApiWebCrawlerDataSource](docs/ApiWebCrawlerDataSource.md)
 - [ApiWorkspace](docs/ApiWorkspace.md)
 - [App](docs/App.md)
 - [AppAlert](docs/AppAlert.md)
 - [AppAlertPhase](docs/AppAlertPhase.md)
 - [AppAlertProgress](docs/AppAlertProgress.md)
 - [AppAlertProgressStep](docs/AppAlertProgressStep.md)
 - [AppAlertProgressStepReason](docs/AppAlertProgressStepReason.md)
 - [AppAlertProgressStepStatus](docs/AppAlertProgressStepStatus.md)
 - [AppAlertSlackWebhook](docs/AppAlertSlackWebhook.md)
 - [AppAlertSpec](docs/AppAlertSpec.md)
 - [AppAlertSpecOperator](docs/AppAlertSpecOperator.md)
 - [AppAlertSpecRule](docs/AppAlertSpecRule.md)
 - [AppAlertSpecWindow](docs/AppAlertSpecWindow.md)
 - [AppComponentBase](docs/AppComponentBase.md)
 - [AppComponentHealth](docs/AppComponentHealth.md)
 - [AppComponentInstanceBase](docs/AppComponentInstanceBase.md)
 - [AppComponentInstanceBaseAutoscaling](docs/AppComponentInstanceBaseAutoscaling.md)
 - [AppComponentInstanceBaseAutoscalingMetrics](docs/AppComponentInstanceBaseAutoscalingMetrics.md)
 - [AppComponentInstanceBaseAutoscalingMetricsCpu](docs/AppComponentInstanceBaseAutoscalingMetricsCpu.md)
 - [AppDatabaseSpec](docs/AppDatabaseSpec.md)
 - [AppDomainSpec](docs/AppDomainSpec.md)
 - [AppDomainValidation](docs/AppDomainValidation.md)
 - [AppEgressSpec](docs/AppEgressSpec.md)
 - [AppEgressTypeSpec](docs/AppEgressTypeSpec.md)
 - [AppFunctionsComponentHealth](docs/AppFunctionsComponentHealth.md)
 - [AppFunctionsComponentHealthFunctionsComponentHealthMetricsInner](docs/AppFunctionsComponentHealthFunctionsComponentHealthMetricsInner.md)
 - [AppFunctionsSpec](docs/AppFunctionsSpec.md)
 - [AppHealth](docs/AppHealth.md)
 - [AppHealthCheckSpec](docs/AppHealthCheckSpec.md)
 - [AppHealthResponse](docs/AppHealthResponse.md)
 - [AppIngressSpec](docs/AppIngressSpec.md)
 - [AppIngressSpecRule](docs/AppIngressSpecRule.md)
 - [AppIngressSpecRuleMatch](docs/AppIngressSpecRuleMatch.md)
 - [AppIngressSpecRuleRoutingComponent](docs/AppIngressSpecRuleRoutingComponent.md)
 - [AppIngressSpecRuleRoutingRedirect](docs/AppIngressSpecRuleRoutingRedirect.md)
 - [AppIngressSpecRuleStringMatchExact](docs/AppIngressSpecRuleStringMatchExact.md)
 - [AppIngressSpecRuleStringMatchPrefix](docs/AppIngressSpecRuleStringMatchPrefix.md)
 - [AppInstance](docs/AppInstance.md)
 - [AppInstances](docs/AppInstances.md)
 - [AppJobInvocation](docs/AppJobInvocation.md)
 - [AppJobInvocations](docs/AppJobInvocations.md)
 - [AppJobSpec](docs/AppJobSpec.md)
 - [AppJobSpecTermination](docs/AppJobSpecTermination.md)
 - [AppLogDestinationDatadogSpec](docs/AppLogDestinationDatadogSpec.md)
 - [AppLogDestinationDefinition](docs/AppLogDestinationDefinition.md)
 - [AppLogDestinationLogtailSpec](docs/AppLogDestinationLogtailSpec.md)
 - [AppLogDestinationOpenSearchSpec](docs/AppLogDestinationOpenSearchSpec.md)
 - [AppLogDestinationOpenSearchSpecBasicAuth](docs/AppLogDestinationOpenSearchSpecBasicAuth.md)
 - [AppLogDestinationPapertrailSpec](docs/AppLogDestinationPapertrailSpec.md)
 - [AppMaintenanceSpec](docs/AppMaintenanceSpec.md)
 - [AppMetricsBandwidthUsage](docs/AppMetricsBandwidthUsage.md)
 - [AppMetricsBandwidthUsageDetails](docs/AppMetricsBandwidthUsageDetails.md)
 - [AppMetricsBandwidthUsageRequest](docs/AppMetricsBandwidthUsageRequest.md)
 - [AppPropose](docs/AppPropose.md)
 - [AppProposeResponse](docs/AppProposeResponse.md)
 - [AppResponse](docs/AppResponse.md)
 - [AppRollbackValidationCondition](docs/AppRollbackValidationCondition.md)
 - [AppRouteSpec](docs/AppRouteSpec.md)
 - [AppServiceSpec](docs/AppServiceSpec.md)
 - [AppServiceSpecHealthCheck](docs/AppServiceSpecHealthCheck.md)
 - [AppServiceSpecTermination](docs/AppServiceSpecTermination.md)
 - [AppSpec](docs/AppSpec.md)
 - [AppStaticSiteSpec](docs/AppStaticSiteSpec.md)
 - [AppVariableDefinition](docs/AppVariableDefinition.md)
 - [AppWorkerSpec](docs/AppWorkerSpec.md)
 - [AppWorkerSpecTermination](docs/AppWorkerSpecTermination.md)
 - [AppsAlertResponse](docs/AppsAlertResponse.md)
 - [AppsAssignAppAlertDestinationsRequest](docs/AppsAssignAppAlertDestinationsRequest.md)
 - [AppsBitbucketSourceSpec](docs/AppsBitbucketSourceSpec.md)
 - [AppsCorsPolicy](docs/AppsCorsPolicy.md)
 - [AppsCreateAppRequest](docs/AppsCreateAppRequest.md)
 - [AppsCreateDeploymentRequest](docs/AppsCreateDeploymentRequest.md)
 - [AppsDedicatedEgressIp](docs/AppsDedicatedEgressIp.md)
 - [AppsDedicatedEgressIpStatus](docs/AppsDedicatedEgressIpStatus.md)
 - [AppsDeleteAppResponse](docs/AppsDeleteAppResponse.md)
 - [AppsDeployment](docs/AppsDeployment.md)
 - [AppsDeploymentFunctions](docs/AppsDeploymentFunctions.md)
 - [AppsDeploymentJob](docs/AppsDeploymentJob.md)
 - [AppsDeploymentPhase](docs/AppsDeploymentPhase.md)
 - [AppsDeploymentProgress](docs/AppsDeploymentProgress.md)
 - [AppsDeploymentProgressStep](docs/AppsDeploymentProgressStep.md)
 - [AppsDeploymentProgressStepReason](docs/AppsDeploymentProgressStepReason.md)
 - [AppsDeploymentProgressStepStatus](docs/AppsDeploymentProgressStepStatus.md)
 - [AppsDeploymentResponse](docs/AppsDeploymentResponse.md)
 - [AppsDeploymentService](docs/AppsDeploymentService.md)
 - [AppsDeploymentStaticSite](docs/AppsDeploymentStaticSite.md)
 - [AppsDeploymentWorker](docs/AppsDeploymentWorker.md)
 - [AppsDeploymentsResponse](docs/AppsDeploymentsResponse.md)
 - [AppsDomain](docs/AppsDomain.md)
 - [AppsDomainPhase](docs/AppsDomainPhase.md)
 - [AppsDomainProgress](docs/AppsDomainProgress.md)
 - [AppsGetExecResponse](docs/AppsGetExecResponse.md)
 - [AppsGetInstanceSizeResponse](docs/AppsGetInstanceSizeResponse.md)
 - [AppsGetLogsResponse](docs/AppsGetLogsResponse.md)
 - [AppsGitSourceSpec](docs/AppsGitSourceSpec.md)
 - [AppsGithubSourceSpec](docs/AppsGithubSourceSpec.md)
 - [AppsGitlabSourceSpec](docs/AppsGitlabSourceSpec.md)
 - [AppsImageSourceSpec](docs/AppsImageSourceSpec.md)
 - [AppsImageSourceSpecDeployOnPush](docs/AppsImageSourceSpecDeployOnPush.md)
 - [AppsInstanceSize](docs/AppsInstanceSize.md)
 - [AppsListAlertsResponse](docs/AppsListAlertsResponse.md)
 - [AppsListInstanceSizesResponse](docs/AppsListInstanceSizesResponse.md)
 - [AppsListRegionsResponse](docs/AppsListRegionsResponse.md)
 - [AppsRegion](docs/AppsRegion.md)
 - [AppsResponse](docs/AppsResponse.md)
 - [AppsRestartRequest](docs/AppsRestartRequest.md)
 - [AppsRollbackAppRequest](docs/AppsRollbackAppRequest.md)
 - [AppsStringMatch](docs/AppsStringMatch.md)
 - [AppsUpdateAppRequest](docs/AppsUpdateAppRequest.md)
 - [AppsValidateRollback200Response](docs/AppsValidateRollback200Response.md)
 - [AppsVpc](docs/AppsVpc.md)
 - [AppsVpcEgressIp](docs/AppsVpcEgressIp.md)
 - [AssignDropletsById](docs/AssignDropletsById.md)
 - [AssignDropletsByTag](docs/AssignDropletsByTag.md)
 - [AssignToDroplet](docs/AssignToDroplet.md)
 - [AssignToDroplet1](docs/AssignToDroplet1.md)
 - [AssociatedKubernetesResource](docs/AssociatedKubernetesResource.md)
 - [AssociatedKubernetesResources](docs/AssociatedKubernetesResources.md)
 - [AssociatedResource](docs/AssociatedResource.md)
 - [AssociatedResourceStatus](docs/AssociatedResourceStatus.md)
 - [AssociatedResourceStatusResources](docs/AssociatedResourceStatusResources.md)
 - [AutoscalePool](docs/AutoscalePool.md)
 - [AutoscalePoolConfig](docs/AutoscalePoolConfig.md)
 - [AutoscalePoolCreate](docs/AutoscalePoolCreate.md)
 - [AutoscalePoolDropletTemplate](docs/AutoscalePoolDropletTemplate.md)
 - [AutoscalePoolDynamicConfig](docs/AutoscalePoolDynamicConfig.md)
 - [AutoscalePoolStaticConfig](docs/AutoscalePoolStaticConfig.md)
 - [AutoscalepoolsCreate202Response](docs/AutoscalepoolsCreate202Response.md)
 - [AutoscalepoolsList200Response](docs/AutoscalepoolsList200Response.md)
 - [AutoscalepoolsListHistory200Response](docs/AutoscalepoolsListHistory200Response.md)
 - [AutoscalepoolsListMembers200Response](docs/AutoscalepoolsListMembers200Response.md)
 - [Backup](docs/Backup.md)
 - [BackwardLinks](docs/BackwardLinks.md)
 - [Balance](docs/Balance.md)
 - [Bgp](docs/Bgp.md)
 - [BgpBgp](docs/BgpBgp.md)
 - [BillingAddress](docs/BillingAddress.md)
 - [BillingDataPoint](docs/BillingDataPoint.md)
 - [BillingHistory](docs/BillingHistory.md)
 - [BillingHistoryList200Response](docs/BillingHistoryList200Response.md)
 - [BillingInsightsList200Response](docs/BillingInsightsList200Response.md)
 - [ByoipPrefix](docs/ByoipPrefix.md)
 - [ByoipPrefixCreate](docs/ByoipPrefixCreate.md)
 - [ByoipPrefixResource](docs/ByoipPrefixResource.md)
 - [ByoipPrefixUpdate](docs/ByoipPrefixUpdate.md)
 - [ByoipPrefixValidationsInner](docs/ByoipPrefixValidationsInner.md)
 - [ByoipPrefixesCreate202Response](docs/ByoipPrefixesCreate202Response.md)
 - [ByoipPrefixesGet200Response](docs/ByoipPrefixesGet200Response.md)
 - [ByoipPrefixesList200Response](docs/ByoipPrefixesList200Response.md)
 - [ByoipPrefixesListResources200Response](docs/ByoipPrefixesListResources200Response.md)
 - [Ca](docs/Ca.md)
 - [CdnCreateEndpoint201Response](docs/CdnCreateEndpoint201Response.md)
 - [CdnEndpoint](docs/CdnEndpoint.md)
 - [CdnListEndpoints200Response](docs/CdnListEndpoints200Response.md)
 - [Certificate](docs/Certificate.md)
 - [CertificateCreateBase](docs/CertificateCreateBase.md)
 - [CertificateRequestCustom](docs/CertificateRequestCustom.md)
 - [CertificateRequestLetsEncrypt](docs/CertificateRequestLetsEncrypt.md)
 - [CertificatesCreate201Response](docs/CertificatesCreate201Response.md)
 - [CertificatesCreateRequest](docs/CertificatesCreateRequest.md)
 - [CertificatesList200Response](docs/CertificatesList200Response.md)
 - [Check](docs/Check.md)
 - [CheckBase](docs/CheckBase.md)
 - [CheckUpdatable](docs/CheckUpdatable.md)
 - [Cluster](docs/Cluster.md)
 - [ClusterAutoscalerConfiguration](docs/ClusterAutoscalerConfiguration.md)
 - [ClusterRead](docs/ClusterRead.md)
 - [ClusterReadStatus](docs/ClusterReadStatus.md)
 - [ClusterRegistries](docs/ClusterRegistries.md)
 - [ClusterRegistry](docs/ClusterRegistry.md)
 - [ClusterUpdate](docs/ClusterUpdate.md)
 - [ClusterlintRequest](docs/ClusterlintRequest.md)
 - [ClusterlintResults](docs/ClusterlintResults.md)
 - [ClusterlintResultsDiagnosticsInner](docs/ClusterlintResultsDiagnosticsInner.md)
 - [ClusterlintResultsDiagnosticsInnerObject](docs/ClusterlintResultsDiagnosticsInnerObject.md)
 - [ConnectionPool](docs/ConnectionPool.md)
 - [ConnectionPoolUpdate](docs/ConnectionPoolUpdate.md)
 - [ConnectionPools](docs/ConnectionPools.md)
 - [ControlPlaneFirewall](docs/ControlPlaneFirewall.md)
 - [CreateNamespace](docs/CreateNamespace.md)
 - [CreateTrigger](docs/CreateTrigger.md)
 - [Credentials](docs/Credentials.md)
 - [CurrentUtilization](docs/CurrentUtilization.md)
 - [Database](docs/Database.md)
 - [DatabaseAutoscaleParams](docs/DatabaseAutoscaleParams.md)
 - [DatabaseBackup](docs/DatabaseBackup.md)
 - [DatabaseCluster](docs/DatabaseCluster.md)
 - [DatabaseClusterRead](docs/DatabaseClusterRead.md)
 - [DatabaseClusterResize](docs/DatabaseClusterResize.md)
 - [DatabaseConfig](docs/DatabaseConfig.md)
 - [DatabaseConfigConfig](docs/DatabaseConfigConfig.md)
 - [DatabaseConnection](docs/DatabaseConnection.md)
 - [DatabaseKafkaSchemaCreate](docs/DatabaseKafkaSchemaCreate.md)
 - [DatabaseLayoutOption](docs/DatabaseLayoutOption.md)
 - [DatabaseLayoutOptions](docs/DatabaseLayoutOptions.md)
 - [DatabaseMaintenanceWindow](docs/DatabaseMaintenanceWindow.md)
 - [DatabaseMetricsCredentials](docs/DatabaseMetricsCredentials.md)
 - [DatabaseRegionOptions](docs/DatabaseRegionOptions.md)
 - [DatabaseReplica](docs/DatabaseReplica.md)
 - [DatabaseReplicaRead](docs/DatabaseReplicaRead.md)
 - [DatabaseServiceEndpoint](docs/DatabaseServiceEndpoint.md)
 - [DatabaseStorageAutoscaleParams](docs/DatabaseStorageAutoscaleParams.md)
 - [DatabaseUser](docs/DatabaseUser.md)
 - [DatabaseVersionAvailability](docs/DatabaseVersionAvailability.md)
 - [DatabaseVersionOptions](docs/DatabaseVersionOptions.md)
 - [DatabasesAdd201Response](docs/DatabasesAdd201Response.md)
 - [DatabasesAddConnectionPool201Response](docs/DatabasesAddConnectionPool201Response.md)
 - [DatabasesAddUser201Response](docs/DatabasesAddUser201Response.md)
 - [DatabasesAddUserRequest](docs/DatabasesAddUserRequest.md)
 - [DatabasesBasicAuthCredentials](docs/DatabasesBasicAuthCredentials.md)
 - [DatabasesCreateCluster201Response](docs/DatabasesCreateCluster201Response.md)
 - [DatabasesCreateClusterRequest](docs/DatabasesCreateClusterRequest.md)
 - [DatabasesCreateKafkaTopic201Response](docs/DatabasesCreateKafkaTopic201Response.md)
 - [DatabasesCreateLogsink201Response](docs/DatabasesCreateLogsink201Response.md)
 - [DatabasesCreateReplica201Response](docs/DatabasesCreateReplica201Response.md)
 - [DatabasesGetAutoscale200Response](docs/DatabasesGetAutoscale200Response.md)
 - [DatabasesGetCa200Response](docs/DatabasesGetCa200Response.md)
 - [DatabasesGetClusterMetricsCredentials200Response](docs/DatabasesGetClusterMetricsCredentials200Response.md)
 - [DatabasesGetConfig200Response](docs/DatabasesGetConfig200Response.md)
 - [DatabasesGetConfig200ResponseConfig](docs/DatabasesGetConfig200ResponseConfig.md)
 - [DatabasesGetEvictionPolicy200Response](docs/DatabasesGetEvictionPolicy200Response.md)
 - [DatabasesGetKafkaSchemaConfig200Response](docs/DatabasesGetKafkaSchemaConfig200Response.md)
 - [DatabasesGetKafkaSchemaSubjectConfig200Response](docs/DatabasesGetKafkaSchemaSubjectConfig200Response.md)
 - [DatabasesList200Response](docs/DatabasesList200Response.md)
 - [DatabasesListBackups200Response](docs/DatabasesListBackups200Response.md)
 - [DatabasesListBackups200ResponseScheduledBackupTime](docs/DatabasesListBackups200ResponseScheduledBackupTime.md)
 - [DatabasesListClusters200Response](docs/DatabasesListClusters200Response.md)
 - [DatabasesListEventsLogs200Response](docs/DatabasesListEventsLogs200Response.md)
 - [DatabasesListFirewallRules200Response](docs/DatabasesListFirewallRules200Response.md)
 - [DatabasesListKafkaSchemas200Response](docs/DatabasesListKafkaSchemas200Response.md)
 - [DatabasesListKafkaTopics200Response](docs/DatabasesListKafkaTopics200Response.md)
 - [DatabasesListLogsink200Response](docs/DatabasesListLogsink200Response.md)
 - [DatabasesListOpeasearchIndexes200Response](docs/DatabasesListOpeasearchIndexes200Response.md)
 - [DatabasesListReplicas200Response](docs/DatabasesListReplicas200Response.md)
 - [DatabasesListUsers200Response](docs/DatabasesListUsers200Response.md)
 - [DatabasesResetAuthRequest](docs/DatabasesResetAuthRequest.md)
 - [DatabasesUpdateEvictionPolicyRequest](docs/DatabasesUpdateEvictionPolicyRequest.md)
 - [DatabasesUpdateFirewallRulesRequest](docs/DatabasesUpdateFirewallRulesRequest.md)
 - [DatabasesUpdateRegionRequest](docs/DatabasesUpdateRegionRequest.md)
 - [DatabasesUpdateUserRequest](docs/DatabasesUpdateUserRequest.md)
 - [DatadogLogsink](docs/DatadogLogsink.md)
 - [DbaasClusterStatus](docs/DbaasClusterStatus.md)
 - [Destination](docs/Destination.md)
 - [DestinationOmitCredentials](docs/DestinationOmitCredentials.md)
 - [DestinationRequest](docs/DestinationRequest.md)
 - [DestroyAssociatedKubernetesResources](docs/DestroyAssociatedKubernetesResources.md)
 - [DestroyedAssociatedResource](docs/DestroyedAssociatedResource.md)
 - [DiskInfo](docs/DiskInfo.md)
 - [DiskInfoSize](docs/DiskInfoSize.md)
 - [Distribution](docs/Distribution.md)
 - [DockerCredentials](docs/DockerCredentials.md)
 - [DockerCredentialsAuths](docs/DockerCredentialsAuths.md)
 - [DockerCredentialsAuthsRegistryDigitaloceanCom](docs/DockerCredentialsAuthsRegistryDigitaloceanCom.md)
 - [Domain](docs/Domain.md)
 - [DomainRecord](docs/DomainRecord.md)
 - [DomainRecordA](docs/DomainRecordA.md)
 - [DomainRecordAaaa](docs/DomainRecordAaaa.md)
 - [DomainRecordCaa](docs/DomainRecordCaa.md)
 - [DomainRecordCname](docs/DomainRecordCname.md)
 - [DomainRecordMx](docs/DomainRecordMx.md)
 - [DomainRecordNs](docs/DomainRecordNs.md)
 - [DomainRecordSoa](docs/DomainRecordSoa.md)
 - [DomainRecordSrv](docs/DomainRecordSrv.md)
 - [DomainRecordTxt](docs/DomainRecordTxt.md)
 - [Domains](docs/Domains.md)
 - [DomainsCreate201Response](docs/DomainsCreate201Response.md)
 - [DomainsCreateRecord201Response](docs/DomainsCreateRecord201Response.md)
 - [DomainsCreateRecordRequest](docs/DomainsCreateRecordRequest.md)
 - [DomainsGet200Response](docs/DomainsGet200Response.md)
 - [DomainsGetRecord200Response](docs/DomainsGetRecord200Response.md)
 - [DomainsList200Response](docs/DomainsList200Response.md)
 - [DomainsListRecords200Response](docs/DomainsListRecords200Response.md)
 - [Droplet](docs/Droplet.md)
 - [DropletAction](docs/DropletAction.md)
 - [DropletActionChangeBackupPolicy](docs/DropletActionChangeBackupPolicy.md)
 - [DropletActionChangeKernel](docs/DropletActionChangeKernel.md)
 - [DropletActionEnableBackups](docs/DropletActionEnableBackups.md)
 - [DropletActionRebuild](docs/DropletActionRebuild.md)
 - [DropletActionRebuildAllOfImage](docs/DropletActionRebuildAllOfImage.md)
 - [DropletActionRename](docs/DropletActionRename.md)
 - [DropletActionResize](docs/DropletActionResize.md)
 - [DropletActionRestore](docs/DropletActionRestore.md)
 - [DropletActionSnapshot](docs/DropletActionSnapshot.md)
 - [DropletActionsPostByTag201Response](docs/DropletActionsPostByTag201Response.md)
 - [DropletActionsPostByTagRequest](docs/DropletActionsPostByTagRequest.md)
 - [DropletActionsPostRequest](docs/DropletActionsPostRequest.md)
 - [DropletBackupPolicy](docs/DropletBackupPolicy.md)
 - [DropletBackupPolicyRecord](docs/DropletBackupPolicyRecord.md)
 - [DropletCreate](docs/DropletCreate.md)
 - [DropletCreateImage](docs/DropletCreateImage.md)
 - [DropletCreateSshKeysInner](docs/DropletCreateSshKeysInner.md)
 - [DropletMultiCreate](docs/DropletMultiCreate.md)
 - [DropletNetworks](docs/DropletNetworks.md)
 - [DropletNextBackupWindow](docs/DropletNextBackupWindow.md)
 - [DropletSingleCreate](docs/DropletSingleCreate.md)
 - [DropletSnapshot](docs/DropletSnapshot.md)
 - [DropletsCreate202Response](docs/DropletsCreate202Response.md)
 - [DropletsCreateRequest](docs/DropletsCreateRequest.md)
 - [DropletsGet200Response](docs/DropletsGet200Response.md)
 - [DropletsGetBackupPolicy200Response](docs/DropletsGetBackupPolicy200Response.md)
 - [DropletsList200Response](docs/DropletsList200Response.md)
 - [DropletsListAssociatedResources200Response](docs/DropletsListAssociatedResources200Response.md)
 - [DropletsListBackupPolicies200Response](docs/DropletsListBackupPolicies200Response.md)
 - [DropletsListBackups200Response](docs/DropletsListBackups200Response.md)
 - [DropletsListFirewalls200Response](docs/DropletsListFirewalls200Response.md)
 - [DropletsListKernels200Response](docs/DropletsListKernels200Response.md)
 - [DropletsListNeighbors200Response](docs/DropletsListNeighbors200Response.md)
 - [DropletsListSnapshots200Response](docs/DropletsListSnapshots200Response.md)
 - [DropletsListSupportedBackupPolicies200Response](docs/DropletsListSupportedBackupPolicies200Response.md)
 - [ElasticsearchLogsink](docs/ElasticsearchLogsink.md)
 - [Error](docs/Error.md)
 - [ErrorWithRootCauses](docs/ErrorWithRootCauses.md)
 - [EventsLogs](docs/EventsLogs.md)
 - [EvictionPolicyModel](docs/EvictionPolicyModel.md)
 - [Firewall](docs/Firewall.md)
 - [FirewallAllOfPendingChanges](docs/FirewallAllOfPendingChanges.md)
 - [FirewallRule](docs/FirewallRule.md)
 - [FirewallRuleBase](docs/FirewallRuleBase.md)
 - [FirewallRuleTarget](docs/FirewallRuleTarget.md)
 - [FirewallRules](docs/FirewallRules.md)
 - [FirewallRulesInboundRulesInner](docs/FirewallRulesInboundRulesInner.md)
 - [FirewallRulesOutboundRulesInner](docs/FirewallRulesOutboundRulesInner.md)
 - [FirewallsAddRulesRequest](docs/FirewallsAddRulesRequest.md)
 - [FirewallsAddTagsRequest](docs/FirewallsAddTagsRequest.md)
 - [FirewallsAssignDropletsRequest](docs/FirewallsAssignDropletsRequest.md)
 - [FirewallsCreate202Response](docs/FirewallsCreate202Response.md)
 - [FirewallsCreateRequest](docs/FirewallsCreateRequest.md)
 - [FirewallsDeleteDropletsRequest](docs/FirewallsDeleteDropletsRequest.md)
 - [FirewallsDeleteTagsRequest](docs/FirewallsDeleteTagsRequest.md)
 - [FirewallsUpdateRequest](docs/FirewallsUpdateRequest.md)
 - [FloatingIp](docs/FloatingIp.md)
 - [FloatingIpActionAssign](docs/FloatingIpActionAssign.md)
 - [FloatingIpActionUnassign](docs/FloatingIpActionUnassign.md)
 - [FloatingIpCreate](docs/FloatingIpCreate.md)
 - [FloatingIpDroplet](docs/FloatingIpDroplet.md)
 - [FloatingIpRegion](docs/FloatingIpRegion.md)
 - [FloatingIpsAction](docs/FloatingIpsAction.md)
 - [FloatingIpsActionList200Response](docs/FloatingIpsActionList200Response.md)
 - [FloatingIpsActionPost201Response](docs/FloatingIpsActionPost201Response.md)
 - [FloatingIpsActionPost201ResponseAction](docs/FloatingIpsActionPost201ResponseAction.md)
 - [FloatingIpsActionPostRequest](docs/FloatingIpsActionPostRequest.md)
 - [FloatingIpsCreate202Response](docs/FloatingIpsCreate202Response.md)
 - [FloatingIpsCreate202ResponseLinks](docs/FloatingIpsCreate202ResponseLinks.md)
 - [FloatingIpsGet200Response](docs/FloatingIpsGet200Response.md)
 - [FloatingIpsList200Response](docs/FloatingIpsList200Response.md)
 - [ForwardLinks](docs/ForwardLinks.md)
 - [ForwardingRule](docs/ForwardingRule.md)
 - [FunctionsCreateNamespace200Response](docs/FunctionsCreateNamespace200Response.md)
 - [FunctionsCreateTrigger200Response](docs/FunctionsCreateTrigger200Response.md)
 - [FunctionsListNamespaces200Response](docs/FunctionsListNamespaces200Response.md)
 - [FunctionsListTriggers200Response](docs/FunctionsListTriggers200Response.md)
 - [GarbageCollection](docs/GarbageCollection.md)
 - [GenaiapiRegion](docs/GenaiapiRegion.md)
 - [GlbSettings](docs/GlbSettings.md)
 - [GlbSettingsCdn](docs/GlbSettingsCdn.md)
 - [GpuInfo](docs/GpuInfo.md)
 - [GpuInfoVram](docs/GpuInfoVram.md)
 - [Grant](docs/Grant.md)
 - [HealthCheck](docs/HealthCheck.md)
 - [History](docs/History.md)
 - [Image](docs/Image.md)
 - [ImageActionBase](docs/ImageActionBase.md)
 - [ImageActionTransfer](docs/ImageActionTransfer.md)
 - [ImageActionsPostRequest](docs/ImageActionsPostRequest.md)
 - [ImageNewCustom](docs/ImageNewCustom.md)
 - [ImageUpdate](docs/ImageUpdate.md)
 - [ImagesCreateCustom202Response](docs/ImagesCreateCustom202Response.md)
 - [ImagesGet200Response](docs/ImagesGet200Response.md)
 - [ImagesGetImageIdParameter](docs/ImagesGetImageIdParameter.md)
 - [ImagesList200Response](docs/ImagesList200Response.md)
 - [InstanceSizeCpuType](docs/InstanceSizeCpuType.md)
 - [InvoiceItem](docs/InvoiceItem.md)
 - [InvoicePreview](docs/InvoicePreview.md)
 - [InvoiceSummary](docs/InvoiceSummary.md)
 - [InvoicesGetByUuid200Response](docs/InvoicesGetByUuid200Response.md)
 - [InvoicesList200Response](docs/InvoicesList200Response.md)
 - [KafkaAdvancedConfig](docs/KafkaAdvancedConfig.md)
 - [KafkaSchemaVerbose](docs/KafkaSchemaVerbose.md)
 - [KafkaSchemaVersionVerbose](docs/KafkaSchemaVersionVerbose.md)
 - [KafkaTopic](docs/KafkaTopic.md)
 - [KafkaTopicBase](docs/KafkaTopicBase.md)
 - [KafkaTopicConfig](docs/KafkaTopicConfig.md)
 - [KafkaTopicCreate](docs/KafkaTopicCreate.md)
 - [KafkaTopicPartition](docs/KafkaTopicPartition.md)
 - [KafkaTopicPartitionConsumerGroupsInner](docs/KafkaTopicPartitionConsumerGroupsInner.md)
 - [KafkaTopicUpdate](docs/KafkaTopicUpdate.md)
 - [KafkaTopicVerbose](docs/KafkaTopicVerbose.md)
 - [Kernel](docs/Kernel.md)
 - [Key](docs/Key.md)
 - [KeyCreateResponse](docs/KeyCreateResponse.md)
 - [KubernetesAddNodePool201Response](docs/KubernetesAddNodePool201Response.md)
 - [KubernetesCreateCluster201Response](docs/KubernetesCreateCluster201Response.md)
 - [KubernetesGetAvailableUpgrades200Response](docs/KubernetesGetAvailableUpgrades200Response.md)
 - [KubernetesGetCluster200Response](docs/KubernetesGetCluster200Response.md)
 - [KubernetesGetNodePool200Response](docs/KubernetesGetNodePool200Response.md)
 - [KubernetesGetStatusMessages200Response](docs/KubernetesGetStatusMessages200Response.md)
 - [KubernetesListClusters200Response](docs/KubernetesListClusters200Response.md)
 - [KubernetesListNodePools200Response](docs/KubernetesListNodePools200Response.md)
 - [KubernetesNodePool](docs/KubernetesNodePool.md)
 - [KubernetesNodePoolBase](docs/KubernetesNodePoolBase.md)
 - [KubernetesNodePoolSize](docs/KubernetesNodePoolSize.md)
 - [KubernetesNodePoolTaint](docs/KubernetesNodePoolTaint.md)
 - [KubernetesNodePoolUpdate](docs/KubernetesNodePoolUpdate.md)
 - [KubernetesOptions](docs/KubernetesOptions.md)
 - [KubernetesOptionsOptions](docs/KubernetesOptionsOptions.md)
 - [KubernetesRecycleNodePoolRequest](docs/KubernetesRecycleNodePoolRequest.md)
 - [KubernetesRegion](docs/KubernetesRegion.md)
 - [KubernetesRunClusterLint202Response](docs/KubernetesRunClusterLint202Response.md)
 - [KubernetesSize](docs/KubernetesSize.md)
 - [KubernetesUpdateNodePool202Response](docs/KubernetesUpdateNodePool202Response.md)
 - [KubernetesUpgradeClusterRequest](docs/KubernetesUpgradeClusterRequest.md)
 - [KubernetesVersion](docs/KubernetesVersion.md)
 - [LbFirewall](docs/LbFirewall.md)
 - [LinkToFirstPage](docs/LinkToFirstPage.md)
 - [LinkToLastPage](docs/LinkToLastPage.md)
 - [LinkToNextPage](docs/LinkToNextPage.md)
 - [LinkToPrevPage](docs/LinkToPrevPage.md)
 - [ListAlertPolicy](docs/ListAlertPolicy.md)
 - [LoadBalancer](docs/LoadBalancer.md)
 - [LoadBalancerBase](docs/LoadBalancerBase.md)
 - [LoadBalancerCreate](docs/LoadBalancerCreate.md)
 - [LoadBalancersAddDropletsRequest](docs/LoadBalancersAddDropletsRequest.md)
 - [LoadBalancersAddForwardingRulesRequest](docs/LoadBalancersAddForwardingRulesRequest.md)
 - [LoadBalancersCreate202Response](docs/LoadBalancersCreate202Response.md)
 - [LoadBalancersList200Response](docs/LoadBalancersList200Response.md)
 - [LogsinkBase](docs/LogsinkBase.md)
 - [LogsinkBaseVerbose](docs/LogsinkBaseVerbose.md)
 - [LogsinkCreate](docs/LogsinkCreate.md)
 - [LogsinkCreateAllOfConfig](docs/LogsinkCreateAllOfConfig.md)
 - [LogsinkSchema](docs/LogsinkSchema.md)
 - [LogsinkUpdate](docs/LogsinkUpdate.md)
 - [LogsinkVerbose](docs/LogsinkVerbose.md)
 - [LogsinkVerboseAllOfConfig](docs/LogsinkVerboseAllOfConfig.md)
 - [MaintenancePolicy](docs/MaintenancePolicy.md)
 - [Member](docs/Member.md)
 - [MemberCurrentUtilization](docs/MemberCurrentUtilization.md)
 - [Meta](docs/Meta.md)
 - [MetaOptionalTotal](docs/MetaOptionalTotal.md)
 - [MetaProperties](docs/MetaProperties.md)
 - [Metrics](docs/Metrics.md)
 - [MetricsData](docs/MetricsData.md)
 - [MetricsResult](docs/MetricsResult.md)
 - [MetricsResultValuesInnerInner](docs/MetricsResultValuesInnerInner.md)
 - [MongoAdvancedConfig](docs/MongoAdvancedConfig.md)
 - [MonitoringCreateAlertPolicy200Response](docs/MonitoringCreateAlertPolicy200Response.md)
 - [MonitoringCreateDestination200Response](docs/MonitoringCreateDestination200Response.md)
 - [MonitoringCreateSinkRequest](docs/MonitoringCreateSinkRequest.md)
 - [MonitoringGetSink200Response](docs/MonitoringGetSink200Response.md)
 - [MonitoringListAlertPolicy200Response](docs/MonitoringListAlertPolicy200Response.md)
 - [MonitoringListDestinations200Response](docs/MonitoringListDestinations200Response.md)
 - [MonitoringListSinks200Response](docs/MonitoringListSinks200Response.md)
 - [MultipleDropletResponse](docs/MultipleDropletResponse.md)
 - [Multiregistry](docs/Multiregistry.md)
 - [MultiregistryCreate](docs/MultiregistryCreate.md)
 - [MysqlAdvancedConfig](docs/MysqlAdvancedConfig.md)
 - [MysqlIncrementalBackup](docs/MysqlIncrementalBackup.md)
 - [MysqlSettings](docs/MysqlSettings.md)
 - [Name](docs/Name.md)
 - [NamespaceInfo](docs/NamespaceInfo.md)
 - [NeighborIds](docs/NeighborIds.md)
 - [NetworkV4](docs/NetworkV4.md)
 - [NetworkV6](docs/NetworkV6.md)
 - [NfsAction](docs/NfsAction.md)
 - [NfsActionAttach](docs/NfsActionAttach.md)
 - [NfsActionAttachAllOfParams](docs/NfsActionAttachAllOfParams.md)
 - [NfsActionDetach](docs/NfsActionDetach.md)
 - [NfsActionDetachAllOfParams](docs/NfsActionDetachAllOfParams.md)
 - [NfsActionResize](docs/NfsActionResize.md)
 - [NfsActionResizeAllOfParams](docs/NfsActionResizeAllOfParams.md)
 - [NfsActionSnapshot](docs/NfsActionSnapshot.md)
 - [NfsActionSnapshotAllOfParams](docs/NfsActionSnapshotAllOfParams.md)
 - [NfsActionsResponse](docs/NfsActionsResponse.md)
 - [NfsActionsResponseAction](docs/NfsActionsResponseAction.md)
 - [NfsCreateActionRequest](docs/NfsCreateActionRequest.md)
 - [NfsCreateResponse](docs/NfsCreateResponse.md)
 - [NfsGetResponse](docs/NfsGetResponse.md)
 - [NfsListResponse](docs/NfsListResponse.md)
 - [NfsRequest](docs/NfsRequest.md)
 - [NfsResponse](docs/NfsResponse.md)
 - [NfsSnapshotGetResponse](docs/NfsSnapshotGetResponse.md)
 - [NfsSnapshotListResponse](docs/NfsSnapshotListResponse.md)
 - [NfsSnapshotResponse](docs/NfsSnapshotResponse.md)
 - [Node](docs/Node.md)
 - [NodeStatus](docs/NodeStatus.md)
 - [Notification](docs/Notification.md)
 - [NotificationSlackInner](docs/NotificationSlackInner.md)
 - [NvidiaGpuDevicePlugin](docs/NvidiaGpuDevicePlugin.md)
 - [OneClicks](docs/OneClicks.md)
 - [OneClicksCreate](docs/OneClicksCreate.md)
 - [OneClicksInstallKubernetes200Response](docs/OneClicksInstallKubernetes200Response.md)
 - [OneClicksList200Response](docs/OneClicksList200Response.md)
 - [OnlineMigration](docs/OnlineMigration.md)
 - [OpensearchAdvancedConfig](docs/OpensearchAdvancedConfig.md)
 - [OpensearchConfig](docs/OpensearchConfig.md)
 - [OpensearchConfigOmitCredentials](docs/OpensearchConfigOmitCredentials.md)
 - [OpensearchConfigRequest](docs/OpensearchConfigRequest.md)
 - [OpensearchConfigRequestCredentials](docs/OpensearchConfigRequestCredentials.md)
 - [OpensearchConnection](docs/OpensearchConnection.md)
 - [OpensearchIndex](docs/OpensearchIndex.md)
 - [OpensearchIndexBase](docs/OpensearchIndexBase.md)
 - [OpensearchLogsink](docs/OpensearchLogsink.md)
 - [Options](docs/Options.md)
 - [OptionsOptions](docs/OptionsOptions.md)
 - [OptionsOptionsKafka](docs/OptionsOptionsKafka.md)
 - [OptionsVersionAvailability](docs/OptionsVersionAvailability.md)
 - [PageLinks](docs/PageLinks.md)
 - [PageLinksPages](docs/PageLinksPages.md)
 - [Pagination](docs/Pagination.md)
 - [PartnerAttachment](docs/PartnerAttachment.md)
 - [PartnerAttachmentBgp](docs/PartnerAttachmentBgp.md)
 - [PartnerAttachmentRemoteRoute](docs/PartnerAttachmentRemoteRoute.md)
 - [PartnerAttachmentServiceKey](docs/PartnerAttachmentServiceKey.md)
 - [PartnerAttachmentUpdatable](docs/PartnerAttachmentUpdatable.md)
 - [PartnerAttachmentWritable](docs/PartnerAttachmentWritable.md)
 - [PartnerAttachmentWritableBgp](docs/PartnerAttachmentWritableBgp.md)
 - [PartnerAttachmentsCreate202Response](docs/PartnerAttachmentsCreate202Response.md)
 - [PartnerAttachmentsDelete202Response](docs/PartnerAttachmentsDelete202Response.md)
 - [PartnerAttachmentsGetBgpAuthKey200Response](docs/PartnerAttachmentsGetBgpAuthKey200Response.md)
 - [PartnerAttachmentsGetServiceKey200Response](docs/PartnerAttachmentsGetServiceKey200Response.md)
 - [PartnerAttachmentsList200Response](docs/PartnerAttachmentsList200Response.md)
 - [PartnerAttachmentsListRemoteRoutes200Response](docs/PartnerAttachmentsListRemoteRoutes200Response.md)
 - [PgbouncerAdvancedConfig](docs/PgbouncerAdvancedConfig.md)
 - [PostgresAdvancedConfig](docs/PostgresAdvancedConfig.md)
 - [PreviousOutage](docs/PreviousOutage.md)
 - [ProductChargeItem](docs/ProductChargeItem.md)
 - [ProductUsageCharges](docs/ProductUsageCharges.md)
 - [Project](docs/Project.md)
 - [ProjectAssignment](docs/ProjectAssignment.md)
 - [ProjectBase](docs/ProjectBase.md)
 - [ProjectsAssignResources200Response](docs/ProjectsAssignResources200Response.md)
 - [ProjectsCreate201Response](docs/ProjectsCreate201Response.md)
 - [ProjectsGetDefault200Response](docs/ProjectsGetDefault200Response.md)
 - [ProjectsList200Response](docs/ProjectsList200Response.md)
 - [ProjectsListResources200Response](docs/ProjectsListResources200Response.md)
 - [PurgeCache](docs/PurgeCache.md)
 - [RdmaSharedDevPlugin](docs/RdmaSharedDevPlugin.md)
 - [RedisAdvancedConfig](docs/RedisAdvancedConfig.md)
 - [Region](docs/Region.md)
 - [RegionSlug](docs/RegionSlug.md)
 - [RegionState](docs/RegionState.md)
 - [RegionalState](docs/RegionalState.md)
 - [RegionsList200Response](docs/RegionsList200Response.md)
 - [RegistriesCreate201Response](docs/RegistriesCreate201Response.md)
 - [RegistriesGetGarbageCollection200Response](docs/RegistriesGetGarbageCollection200Response.md)
 - [RegistriesGetOptions200Response](docs/RegistriesGetOptions200Response.md)
 - [RegistriesGetOptions200ResponseOptions](docs/RegistriesGetOptions200ResponseOptions.md)
 - [RegistriesGetOptions200ResponseOptionsSubscriptionTiersInner](docs/RegistriesGetOptions200ResponseOptionsSubscriptionTiersInner.md)
 - [RegistriesGetSubscription200Response](docs/RegistriesGetSubscription200Response.md)
 - [RegistriesList200Response](docs/RegistriesList200Response.md)
 - [RegistriesList200ResponseRegistriesInner](docs/RegistriesList200ResponseRegistriesInner.md)
 - [RegistriesListGarbageCollections200Response](docs/RegistriesListGarbageCollections200Response.md)
 - [RegistriesListRepositoriesV2200Response](docs/RegistriesListRepositoriesV2200Response.md)
 - [RegistriesListRepositoryManifests200Response](docs/RegistriesListRepositoryManifests200Response.md)
 - [RegistriesListRepositoryTags200Response](docs/RegistriesListRepositoryTags200Response.md)
 - [RegistriesUpdateSubscriptionRequest](docs/RegistriesUpdateSubscriptionRequest.md)
 - [Registry](docs/Registry.md)
 - [RegistryBase](docs/RegistryBase.md)
 - [RegistryCreate](docs/RegistryCreate.md)
 - [RegistryGet200Response](docs/RegistryGet200Response.md)
 - [RegistryListRepositories200Response](docs/RegistryListRepositories200Response.md)
 - [RegistryRunGc](docs/RegistryRunGc.md)
 - [Repository](docs/Repository.md)
 - [RepositoryBlob](docs/RepositoryBlob.md)
 - [RepositoryManifest](docs/RepositoryManifest.md)
 - [RepositoryTag](docs/RepositoryTag.md)
 - [RepositoryV2](docs/RepositoryV2.md)
 - [ReserveToRegion](docs/ReserveToRegion.md)
 - [ReserveToRegion1](docs/ReserveToRegion1.md)
 - [ReservedIp](docs/ReservedIp.md)
 - [ReservedIpActionAssign](docs/ReservedIpActionAssign.md)
 - [ReservedIpActionType](docs/ReservedIpActionType.md)
 - [ReservedIpActionUnassign](docs/ReservedIpActionUnassign.md)
 - [ReservedIpCreate](docs/ReservedIpCreate.md)
 - [ReservedIpDroplet](docs/ReservedIpDroplet.md)
 - [ReservedIpRegion](docs/ReservedIpRegion.md)
 - [ReservedIpsActionsList200Response](docs/ReservedIpsActionsList200Response.md)
 - [ReservedIpsActionsPost201Response](docs/ReservedIpsActionsPost201Response.md)
 - [ReservedIpsActionsPostRequest](docs/ReservedIpsActionsPostRequest.md)
 - [ReservedIpsCreate202Response](docs/ReservedIpsCreate202Response.md)
 - [ReservedIpsGet200Response](docs/ReservedIpsGet200Response.md)
 - [ReservedIpsList200Response](docs/ReservedIpsList200Response.md)
 - [ReservedIpv6](docs/ReservedIpv6.md)
 - [ReservedIpv6ActionAssign](docs/ReservedIpv6ActionAssign.md)
 - [ReservedIpv6ActionType](docs/ReservedIpv6ActionType.md)
 - [ReservedIpv6ActionUnassign](docs/ReservedIpv6ActionUnassign.md)
 - [ReservedIpv6ActionsPost201Response](docs/ReservedIpv6ActionsPost201Response.md)
 - [ReservedIpv6ActionsPost201ResponseAction](docs/ReservedIpv6ActionsPost201ResponseAction.md)
 - [ReservedIpv6ActionsPostRequest](docs/ReservedIpv6ActionsPostRequest.md)
 - [ReservedIpv6Create](docs/ReservedIpv6Create.md)
 - [ReservedIpv6Create201Response](docs/ReservedIpv6Create201Response.md)
 - [ReservedIpv6Create201ResponseReservedIpv6](docs/ReservedIpv6Create201ResponseReservedIpv6.md)
 - [ReservedIpv6Droplet](docs/ReservedIpv6Droplet.md)
 - [ReservedIpv6Get200Response](docs/ReservedIpv6Get200Response.md)
 - [ReservedIpv6List](docs/ReservedIpv6List.md)
 - [ReservedIpv6List200Response](docs/ReservedIpv6List200Response.md)
 - [ReservedIpv6ListReservedIpv6sInner](docs/ReservedIpv6ListReservedIpv6sInner.md)
 - [ReservedIpv6ListReservedIpv6sInnerDroplet](docs/ReservedIpv6ListReservedIpv6sInnerDroplet.md)
 - [Resource](docs/Resource.md)
 - [ResourceLinks](docs/ResourceLinks.md)
 - [RoutingAgent](docs/RoutingAgent.md)
 - [RsyslogLogsink](docs/RsyslogLogsink.md)
 - [ScheduledDetails](docs/ScheduledDetails.md)
 - [ScheduledDetailsBody](docs/ScheduledDetailsBody.md)
 - [SchemaRegistryConnection](docs/SchemaRegistryConnection.md)
 - [SelectiveDestroyAssociatedResource](docs/SelectiveDestroyAssociatedResource.md)
 - [SimpleCharge](docs/SimpleCharge.md)
 - [SingleDropletResponse](docs/SingleDropletResponse.md)
 - [SingleDropletResponseLinks](docs/SingleDropletResponseLinks.md)
 - [SinkResource](docs/SinkResource.md)
 - [SinksResponse](docs/SinksResponse.md)
 - [Size](docs/Size.md)
 - [SizesList200Response](docs/SizesList200Response.md)
 - [SlackDetails](docs/SlackDetails.md)
 - [Snapshots](docs/Snapshots.md)
 - [SnapshotsBase](docs/SnapshotsBase.md)
 - [SnapshotsGet200Response](docs/SnapshotsGet200Response.md)
 - [SnapshotsGetSnapshotIdParameter](docs/SnapshotsGetSnapshotIdParameter.md)
 - [SnapshotsList200Response](docs/SnapshotsList200Response.md)
 - [SourceDatabase](docs/SourceDatabase.md)
 - [SourceDatabaseSource](docs/SourceDatabaseSource.md)
 - [SpacesKeyCreate201Response](docs/SpacesKeyCreate201Response.md)
 - [SpacesKeyGet200Response](docs/SpacesKeyGet200Response.md)
 - [SpacesKeyList200Response](docs/SpacesKeyList200Response.md)
 - [SpacesKeyUpdate200Response](docs/SpacesKeyUpdate200Response.md)
 - [SqlMode](docs/SqlMode.md)
 - [SshKeys](docs/SshKeys.md)
 - [SshKeysCreate201Response](docs/SshKeysCreate201Response.md)
 - [SshKeysGetSshKeyIdentifierParameter](docs/SshKeysGetSshKeyIdentifierParameter.md)
 - [SshKeysList200Response](docs/SshKeysList200Response.md)
 - [SshKeysUpdateRequest](docs/SshKeysUpdateRequest.md)
 - [State](docs/State.md)
 - [StatusMessages](docs/StatusMessages.md)
 - [StickySessions](docs/StickySessions.md)
 - [Subscription](docs/Subscription.md)
 - [SubscriptionTierBase](docs/SubscriptionTierBase.md)
 - [SubscriptionTierExtended](docs/SubscriptionTierExtended.md)
 - [SupportedDropletBackupPolicy](docs/SupportedDropletBackupPolicy.md)
 - [Tags](docs/Tags.md)
 - [TagsCreate201Response](docs/TagsCreate201Response.md)
 - [TagsGet200Response](docs/TagsGet200Response.md)
 - [TagsList200Response](docs/TagsList200Response.md)
 - [TagsMetadata](docs/TagsMetadata.md)
 - [TagsResource](docs/TagsResource.md)
 - [TagsResourceResourcesInner](docs/TagsResourceResourcesInner.md)
 - [TagsResources](docs/TagsResources.md)
 - [TheJobInvocationTrigger](docs/TheJobInvocationTrigger.md)
 - [TheJobInvocationTriggerManual](docs/TheJobInvocationTriggerManual.md)
 - [TheJobInvocationTriggerManualUser](docs/TheJobInvocationTriggerManualUser.md)
 - [TheJobInvocationTriggerScheduled](docs/TheJobInvocationTriggerScheduled.md)
 - [TheJobInvocationTriggerScheduledSchedule](docs/TheJobInvocationTriggerScheduledSchedule.md)
 - [TimescaledbAdvancedConfig](docs/TimescaledbAdvancedConfig.md)
 - [TriggerInfo](docs/TriggerInfo.md)
 - [TriggerInfoScheduledRuns](docs/TriggerInfoScheduledRuns.md)
 - [UpdateEndpoint](docs/UpdateEndpoint.md)
 - [UpdateRegistry](docs/UpdateRegistry.md)
 - [UpdateTrigger](docs/UpdateTrigger.md)
 - [UptimeCreateAlert201Response](docs/UptimeCreateAlert201Response.md)
 - [UptimeCreateCheck201Response](docs/UptimeCreateCheck201Response.md)
 - [UptimeGetCheckState200Response](docs/UptimeGetCheckState200Response.md)
 - [UptimeListAlerts200Response](docs/UptimeListAlerts200Response.md)
 - [UptimeListChecks200Response](docs/UptimeListChecks200Response.md)
 - [User](docs/User.md)
 - [UserKubernetesClusterUser](docs/UserKubernetesClusterUser.md)
 - [UserSettings](docs/UserSettings.md)
 - [UserSettingsAclInner](docs/UserSettingsAclInner.md)
 - [UserSettingsMongoUserSettings](docs/UserSettingsMongoUserSettings.md)
 - [UserSettingsOpensearchAclInner](docs/UserSettingsOpensearchAclInner.md)
 - [ValidateRegistry](docs/ValidateRegistry.md)
 - [ValkeyAdvancedConfig](docs/ValkeyAdvancedConfig.md)
 - [Version2](docs/Version2.md)
 - [VolumeAction](docs/VolumeAction.md)
 - [VolumeActionPostAttach](docs/VolumeActionPostAttach.md)
 - [VolumeActionPostBase](docs/VolumeActionPostBase.md)
 - [VolumeActionPostDetach](docs/VolumeActionPostDetach.md)
 - [VolumeActionPostResize](docs/VolumeActionPostResize.md)
 - [VolumeActionsList200Response](docs/VolumeActionsList200Response.md)
 - [VolumeActionsPost202Response](docs/VolumeActionsPost202Response.md)
 - [VolumeActionsPostByIdRequest](docs/VolumeActionsPostByIdRequest.md)
 - [VolumeActionsPostRequest](docs/VolumeActionsPostRequest.md)
 - [VolumeBase](docs/VolumeBase.md)
 - [VolumeBaseRead](docs/VolumeBaseRead.md)
 - [VolumeFull](docs/VolumeFull.md)
 - [VolumeSnapshotId](docs/VolumeSnapshotId.md)
 - [VolumeSnapshotsCreateRequest](docs/VolumeSnapshotsCreateRequest.md)
 - [VolumeSnapshotsGetById200Response](docs/VolumeSnapshotsGetById200Response.md)
 - [VolumeSnapshotsList200Response](docs/VolumeSnapshotsList200Response.md)
 - [VolumeWriteFileSystemType](docs/VolumeWriteFileSystemType.md)
 - [VolumesCreate201Response](docs/VolumesCreate201Response.md)
 - [VolumesCreateRequest](docs/VolumesCreateRequest.md)
 - [VolumesExt4](docs/VolumesExt4.md)
 - [VolumesList200Response](docs/VolumesList200Response.md)
 - [VolumesXfs](docs/VolumesXfs.md)
 - [Vpc](docs/Vpc.md)
 - [VpcBase](docs/VpcBase.md)
 - [VpcCreate](docs/VpcCreate.md)
 - [VpcDefault](docs/VpcDefault.md)
 - [VpcIds](docs/VpcIds.md)
 - [VpcMember](docs/VpcMember.md)
 - [VpcNatGatewayCreate](docs/VpcNatGatewayCreate.md)
 - [VpcNatGatewayCreateVpcsInner](docs/VpcNatGatewayCreateVpcsInner.md)
 - [VpcNatGatewayGet](docs/VpcNatGatewayGet.md)
 - [VpcNatGatewayGetEgresses](docs/VpcNatGatewayGetEgresses.md)
 - [VpcNatGatewayGetEgressesPublicGatewaysInner](docs/VpcNatGatewayGetEgressesPublicGatewaysInner.md)
 - [VpcNatGatewayGetVpcsInner](docs/VpcNatGatewayGetVpcsInner.md)
 - [VpcNatGatewayUpdate](docs/VpcNatGatewayUpdate.md)
 - [VpcNatGatewayUpdateVpcsInner](docs/VpcNatGatewayUpdateVpcsInner.md)
 - [VpcPeering](docs/VpcPeering.md)
 - [VpcPeeringBase](docs/VpcPeeringBase.md)
 - [VpcPeeringCreate](docs/VpcPeeringCreate.md)
 - [VpcPeeringUpdatable](docs/VpcPeeringUpdatable.md)
 - [VpcPeeringsCreate202Response](docs/VpcPeeringsCreate202Response.md)
 - [VpcPeeringsCreateRequest](docs/VpcPeeringsCreateRequest.md)
 - [VpcPeeringsDelete202Response](docs/VpcPeeringsDelete202Response.md)
 - [VpcPeeringsGet200Response](docs/VpcPeeringsGet200Response.md)
 - [VpcPeeringsList200Response](docs/VpcPeeringsList200Response.md)
 - [VpcUpdatable](docs/VpcUpdatable.md)
 - [VpcnatgatewaysCreate202Response](docs/VpcnatgatewaysCreate202Response.md)
 - [VpcnatgatewaysGet200Response](docs/VpcnatgatewaysGet200Response.md)
 - [VpcnatgatewaysList200Response](docs/VpcnatgatewaysList200Response.md)
 - [VpcnatgatewaysUpdate200Response](docs/VpcnatgatewaysUpdate200Response.md)
 - [VpcsCreate201Response](docs/VpcsCreate201Response.md)
 - [VpcsCreatePeerings202Response](docs/VpcsCreatePeerings202Response.md)
 - [VpcsCreatePeeringsRequest](docs/VpcsCreatePeeringsRequest.md)
 - [VpcsCreateRequest](docs/VpcsCreateRequest.md)
 - [VpcsList200Response](docs/VpcsList200Response.md)
 - [VpcsListMembers200Response](docs/VpcsListMembers200Response.md)
 - [VpcsListPeerings200Response](docs/VpcsListPeerings200Response.md)
 - [VpcsPatchRequest](docs/VpcsPatchRequest.md)
 - [VpcsUpdateRequest](docs/VpcsUpdateRequest.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

api-engineering@digitalocean.com

