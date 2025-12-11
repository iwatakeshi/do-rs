/*
 * DigitalOcean API
 *
 * # Introduction  The DigitalOcean API allows you to manage Droplets and resources within the DigitalOcean cloud in a simple, programmatic way using conventional HTTP requests.  All of the functionality that you are familiar with in the DigitalOcean control panel is also available through the API, allowing you to script the complex actions that your situation requires.  The API documentation will start with a general overview about the design and technology that has been implemented, followed by reference information about specific endpoints.  ## Requests  Any tool that is fluent in HTTP can communicate with the API simply by requesting the correct URI. Requests should be made using the HTTPS protocol so that traffic is encrypted. The interface responds to different methods depending on the action required.  |Method|Usage| |--- |--- | |GET|For simple retrieval of information about your account, Droplets, or environment, you should use the GET method.  The information you request will be returned to you as a JSON object. The attributes defined by the JSON object can be used to form additional requests.  Any request using the GET method is read-only and will not affect any of the objects you are querying.| |DELETE|To destroy a resource and remove it from your account and environment, the DELETE method should be used.  This will remove the specified object if it is found.  If it is not found, the operation will return a response indicating that the object was not found. This idempotency means that you do not have to check for a resource's availability prior to issuing a delete command, the final state will be the same regardless of its existence.| |PUT|To update the information about a resource in your account, the PUT method is available. Like the DELETE Method, the PUT method is idempotent.  It sets the state of the target using the provided values, regardless of their current values. Requests using the PUT method do not need to check the current attributes of the object.| |PATCH|Some resources support partial modification. In these cases, the PATCH method is available. Unlike PUT which generally requires a complete representation of a resource, a PATCH request is a set of instructions on how to modify a resource updating only specific attributes.| |POST|To create a new object, your request should specify the POST method. The POST request includes all of the attributes necessary to create a new object.  When you wish to create a new object, send a POST request to the target endpoint.| |HEAD|Finally, to retrieve metadata information, you should use the HEAD method to get the headers.  This returns only the header of what would be returned with an associated GET request. Response headers contain some useful information about your API access and the results that are available for your request. For instance, the headers contain your current rate-limit value and the amount of time available until the limit resets. It also contains metrics about the total number of objects found, pagination information, and the total content length.|   ## HTTP Statuses  Along with the HTTP methods that the API responds to, it will also return standard HTTP statuses, including error codes.  In the event of a problem, the status will contain the error code, while the body of the response will usually contain additional information about the problem that was encountered.  In general, if the status returned is in the 200 range, it indicates that the request was fulfilled successfully and that no error was encountered.  Return codes in the 400 range typically indicate that there was an issue with the request that was sent. Among other things, this could mean that you did not authenticate correctly, that you are requesting an action that you do not have authorization for, that the object you are requesting does not exist, or that your request is malformed.  If you receive a status in the 500 range, this generally indicates a server-side problem. This means that we are having an issue on our end and cannot fulfill your request currently.  400 and 500 level error responses will include a JSON object in their body, including the following attributes:  |Name|Type|Description| |--- |--- |--- | |id|string|A short identifier corresponding to the HTTP status code returned. For example, the ID for a response returning a 404 status code would be \"not_found.\"| |message|string|A message providing additional information about the error, including details to help resolve it when possible.| |request_id|string|Optionally, some endpoints may include a request ID that should be provided when reporting bugs or opening support tickets to help identify the issue.|  ### Example Error Response  ```     HTTP/1.1 403 Forbidden     {       \"id\":       \"forbidden\",       \"message\":  \"You do not have access for the attempted action.\"     } ```  ## Responses  When a request is successful, a response body will typically be sent back in the form of a JSON object. An exception to this is when a DELETE request is processed, which will result in a successful HTTP 204 status and an empty response body.  Inside of this JSON object, the resource root that was the target of the request will be set as the key. This will be the singular form of the word if the request operated on a single object, and the plural form of the word if a collection was processed.  For example, if you send a GET request to `/v2/droplets/$DROPLET_ID` you will get back an object with a key called \"`droplet`\". However, if you send the GET request to the general collection at `/v2/droplets`, you will get back an object with a key called \"`droplets`\".  The value of these keys will generally be a JSON object for a request on a single object and an array of objects for a request on a collection of objects.  ### Response for a Single Object  ```json     {         \"droplet\": {             \"name\": \"example.com\"             . . .         }     } ```  ### Response for an Object Collection  ```json     {         \"droplets\": [             {                 \"name\": \"example.com\"                 . . .             },             {                 \"name\": \"second.com\"                 . . .             }         ]     } ```  ## Meta  In addition to the main resource root, the response may also contain a `meta` object. This object contains information about the response itself.  The `meta` object contains a `total` key that is set to the total number of objects returned by the request. This has implications on the `links` object and pagination.  The `meta` object will only be displayed when it has a value. Currently, the `meta` object will have a value when a request is made on a collection (like `droplets` or `domains`).   ### Sample Meta Object  ```json     {         . . .         \"meta\": {             \"total\": 43         }         . . .     } ```  ## Links & Pagination  The `links` object is returned as part of the response body when pagination is enabled. By default, 20 objects are returned per page. If the response contains 20 objects or fewer, no `links` object will be returned. If the response contains more than 20 objects, the first 20 will be returned along with the `links` object.  You can request a different pagination limit or force pagination by appending `?per_page=` to the request with the number of items you would like per page. For instance, to show only two results per page, you could add `?per_page=2` to the end of your query. The maximum number of results per page is 200.  The `links` object contains a `pages` object. The `pages` object, in turn, contains keys indicating the relationship of additional pages. The values of these are the URLs of the associated pages. The keys will be one of the following:  *   **first**: The URI of the first page of results. *   **prev**: The URI of the previous sequential page of results. *   **next**: The URI of the next sequential page of results. *   **last**: The URI of the last page of results.  The `pages` object will only include the links that make sense. So for the first page of results, no `first` or `prev` links will ever be set. This convention holds true in other situations where a link would not make sense.  ### Sample Links Object  ```json     {         . . .         \"links\": {             \"pages\": {                 \"last\": \"https://api.digitalocean.com/v2/images?page=2\",                 \"next\": \"https://api.digitalocean.com/v2/images?page=2\"             }         }         . . .     } ```  ## Rate Limit  Requests through the API are rate limited per OAuth token. Current rate limits:  *   5,000 requests per hour *   250 requests per minute (5% of the hourly total)  Once you exceed either limit, you will be rate limited until the next cycle starts. Space out any requests that you would otherwise issue in bursts for the best results.  The rate limiting information is contained within the response headers of each request. The relevant headers are:  *   **ratelimit-limit**: The number of requests that can be made per hour. *   **ratelimit-remaining**: The number of requests that remain before you hit your request limit. See the information below for how the request limits expire. *   **ratelimit-reset**: This represents the time when the oldest request will expire. The value is given in [Unix epoch time](http://en.wikipedia.org/wiki/Unix_time). See below for more information about how request limits expire.  More rate limiting information is returned only within burst limit error response headers: *   **retry-after**: The number of seconds to wait before making another request when rate limited.  As long as the `ratelimit-remaining` count is above zero, you will be able to make additional requests.  The way that a request expires and is removed from the current limit count is important to understand. Rather than counting all of the requests for an hour and resetting the `ratelimit-remaining` value at the end of the hour, each request instead has its own timer.  This means that each request contributes toward the `ratelimit-remaining` count for one complete hour after the request is made. When that request's timer runs out, it is no longer counted towards the request limit.  This has implications on the meaning of the `ratelimit-reset` header as well. Because the entire rate limit is not reset at one time, the value of this header is set to the time when the _oldest_ request will expire.  Keep this in mind if you see your `ratelimit-reset` value change, but not move an entire hour into the future.  If the `ratelimit-remaining` reaches zero, subsequent requests will receive a 429 error code until the request reset has been reached.   `ratelimit-remaining` reaching zero can also indicate that the \"burst limit\" of 250  requests per minute limit was met, even if the 5,000 requests per hour limit was not.  In this case, the 429 error response will include a `retry-after` header to indicate how  long to wait (in seconds) until the request may be retried.  You can see the format of the response in the examples.   **Note:** The following endpoints have special rate limit requirements that are independent of the limits defined above.  *   Only 10 `GET` requests to the `/v2/account/keys` endpoint to list SSH keys can be made per 60 seconds. *   Only 5 requests to any and all `v2/cdn/endpoints` can be made per 10 seconds. This includes `v2/cdn/endpoints`,      `v2/cdn/endpoints/$ENDPOINT_ID`, and `v2/cdn/endpoints/$ENDPOINT_ID/cache`. *   Only 50 strings within the `files` json struct in the `v2/cdn/endpoints/$ENDPOINT_ID/cache` [payload](https://docs.digitalocean.com/reference/api/api-reference/#operation/cdn_purge_cache)      can be requested every 20 seconds.  ### Sample Rate Limit Headers  ```     . . .     ratelimit-limit: 1200     ratelimit-remaining: 1193     rateLimit-reset: 1402425459     . . . ```    ### Sample Rate Limit Headers When Burst Limit is Reached:  ```     . . .     ratelimit-limit: 5000     ratelimit-remaining: 0     rateLimit-reset: 1402425459     retry-after: 29     . . . ```  ### Sample Rate Exceeded Response  ```     429 Too Many Requests     {             id: \"too_many_requests\",             message: \"API Rate limit exceeded.\"     } ```  ## Curl Examples  Throughout this document, some example API requests will be given using the `curl` command. This will allow us to demonstrate the various endpoints in a simple, textual format.      These examples assume that you are using a Linux or macOS command line. To run these commands on a Windows machine, you can either use cmd.exe, PowerShell, or WSL:  * For cmd.exe, use the `set VAR=VALUE` [syntax](https://docs.microsoft.com/en-us/windows-server/administration/windows-commands/set_1) to define environment variables, call them with `%VAR%`, then replace all backslashes (`\\`) in the examples with carets (`^`).  * For PowerShell, use the `$Env:VAR = \"VALUE\"` [syntax](https://docs.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_environment_variables?view=powershell-7.2) to define environment variables, call them with `$Env:VAR`, then replace `curl` with `curl.exe` and all backslashes (`\\`) in the examples with backticks (`` ` ``).  * WSL is a compatibility layer that allows you to emulate a Linux terminal on a Windows machine. Install WSL with our [community tutorial](https://www.digitalocean.com/community/tutorials/how-to-install-the-windows-subsystem-for-linux-2-on-microsoft-windows-10),  then follow this API documentation normally.  The names of account-specific references (like Droplet IDs, for instance) will be represented by variables. For instance, a Droplet ID may be represented by a variable called `$DROPLET_ID`. You can set the associated variables in your environment if you wish to use the examples without modification.  The first variable that you should set to get started is your OAuth authorization token. The next section will go over the details of this, but you can set an environmental variable for it now.  Generate a token by going to the [Apps & API](https://cloud.digitalocean.com/settings/applications) section of the DigitalOcean control panel. Use an existing token if you have saved one, or generate a new token with the \"Generate new token\" button. Copy the generated token and use it to set and export the TOKEN variable in your environment as the example shows.  You may also wish to set some other variables now or as you go along. For example, you may wish to set the `DROPLET_ID` variable to one of your Droplet IDs since this will be used frequently in the API.  If you are following along, make sure you use a Droplet ID that you control so that your commands will execute correctly.  If you need access to the headers of a response through `curl`, you can pass the `-i` flag to display the header information along with the body. If you are only interested in the header, you can instead pass the `-I` flag, which will exclude the response body entirely.   ### Set and Export your OAuth Token  ``` export DIGITALOCEAN_TOKEN=your_token_here ```  ### Set and Export a Variable  ``` export DROPLET_ID=1111111 ```  ## Parameters  There are two different ways to pass parameters in a request with the API.  When passing parameters to create or update an object, parameters should be passed as a JSON object containing the appropriate attribute names and values as key-value pairs. When you use this format, you should specify that you are sending a JSON object in the header. This is done by setting the `Content-Type` header to `application/json`. This ensures that your request is interpreted correctly.  When passing parameters to filter a response on GET requests, parameters can be passed using standard query attributes. In this case, the parameters would be embedded into the URI itself by appending a `?` to the end of the URI and then setting each attribute with an equal sign. Attributes can be separated with a `&`. Tools like `curl` can create the appropriate URI when given parameters and values; this can also be done using the `-F` flag and then passing the key and value as an argument. The argument should take the form of a quoted string with the attribute being set to a value with an equal sign.  ### Pass Parameters as a JSON Object  ```     curl -H \"Authorization: Bearer $DIGITALOCEAN_TOKEN\" \\         -H \"Content-Type: application/json\" \\         -d '{\"name\": \"example.com\", \"ip_address\": \"127.0.0.1\"}' \\         -X POST \"https://api.digitalocean.com/v2/domains\" ```  ### Pass Filter Parameters as a Query String  ```      curl -H \"Authorization: Bearer $DIGITALOCEAN_TOKEN\" \\          -X GET \\          \"https://api.digitalocean.com/v2/images?private=true\" ```  ## Cross Origin Resource Sharing  In order to make requests to the API from other domains, the API implements Cross Origin Resource Sharing (CORS) support.  CORS support is generally used to create AJAX requests outside of the domain that the request originated from. This is necessary to implement projects like control panels utilizing the API. This tells the browser that it can send requests to an outside domain.  The procedure that the browser initiates in order to perform these actions (other than GET requests) begins by sending a \"preflight\" request. This sets the `Origin` header and uses the `OPTIONS` method. The server will reply back with the methods it allows and some of the limits it imposes. The client then sends the actual request if it falls within the allowed constraints.  This process is usually done in the background by the browser, but you can use curl to emulate this process using the example provided. The headers that will be set to show the constraints are:  *   **Access-Control-Allow-Origin**: This is the domain that is sent by the client or browser as the origin of the request. It is set through an `Origin` header. *   **Access-Control-Allow-Methods**: This specifies the allowed options for requests from that domain. This will generally be all available methods. *   **Access-Control-Expose-Headers**: This will contain the headers that will be available to requests from the origin domain. *   **Access-Control-Max-Age**: This is the length of time that the access is considered valid. After this expires, a new preflight should be sent. *   **Access-Control-Allow-Credentials**: This will be set to `true`. It basically allows you to send your OAuth token for authentication.  You should not need to be concerned with the details of these headers, because the browser will typically do all of the work for you.
 *
 * The version of the OpenAPI document: 2.0
 * Contact: api-engineering@digitalocean.com
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, ContentType, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{de::Error as _, Deserialize, Serialize};

/// struct for typed errors of method [`apps_assign_alert_destinations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppsAssignAlertDestinationsError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`apps_cancel_deployment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppsCancelDeploymentError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`apps_commit_rollback`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppsCommitRollbackError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`apps_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppsCreateError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`apps_create_deployment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppsCreateDeploymentError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`apps_create_rollback`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppsCreateRollbackError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`apps_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppsDeleteError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`apps_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppsGetError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`apps_get_deployment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppsGetDeploymentError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`apps_get_exec`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppsGetExecError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`apps_get_exec_active_deployment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppsGetExecActiveDeploymentError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`apps_get_health`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppsGetHealthError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`apps_get_instance_size`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppsGetInstanceSizeError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`apps_get_instances`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppsGetInstancesError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`apps_get_job_invocation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppsGetJobInvocationError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`apps_get_job_invocation_logs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppsGetJobInvocationLogsError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`apps_get_logs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppsGetLogsError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`apps_get_logs_active_deployment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppsGetLogsActiveDeploymentError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`apps_get_logs_active_deployment_aggregate`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppsGetLogsActiveDeploymentAggregateError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`apps_get_logs_aggregate`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppsGetLogsAggregateError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`apps_get_metrics_bandwidth_daily`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppsGetMetricsBandwidthDailyError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`apps_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppsListError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`apps_list_alerts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppsListAlertsError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`apps_list_deployments`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppsListDeploymentsError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`apps_list_instance_sizes`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppsListInstanceSizesError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`apps_list_job_invocations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppsListJobInvocationsError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`apps_list_metrics_bandwidth_daily`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppsListMetricsBandwidthDailyError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`apps_list_regions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppsListRegionsError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`apps_restart`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppsRestartError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`apps_revert_rollback`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppsRevertRollbackError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`apps_update`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppsUpdateError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`apps_validate_app_spec`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppsValidateAppSpecError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`apps_validate_rollback`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppsValidateRollbackError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// Updates the emails and slack webhook destinations for app alerts. Emails must be associated to a user with access to the app.
pub async fn apps_assign_alert_destinations(
    configuration: &configuration::Configuration,
    app_id: &str,
    alert_id: &str,
    apps_assign_app_alert_destinations_request: models::AppsAssignAppAlertDestinationsRequest,
) -> Result<models::AppsAlertResponse, Error<AppsAssignAlertDestinationsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_app_id = app_id;
    let p_path_alert_id = alert_id;
    let p_body_apps_assign_app_alert_destinations_request =
        apps_assign_app_alert_destinations_request;

    let uri_str = format!(
        "{}/v2/apps/{app_id}/alerts/{alert_id}/destinations",
        configuration.base_path,
        app_id = crate::apis::urlencode(p_path_app_id),
        alert_id = crate::apis::urlencode(p_path_alert_id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_apps_assign_app_alert_destinations_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AppsAlertResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AppsAlertResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AppsAssignAlertDestinationsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Immediately cancel an in-progress deployment.
pub async fn apps_cancel_deployment(
    configuration: &configuration::Configuration,
    app_id: &str,
    deployment_id: &str,
) -> Result<models::AppsDeploymentResponse, Error<AppsCancelDeploymentError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_app_id = app_id;
    let p_path_deployment_id = deployment_id;

    let uri_str = format!(
        "{}/v2/apps/{app_id}/deployments/{deployment_id}/cancel",
        configuration.base_path,
        app_id = crate::apis::urlencode(p_path_app_id),
        deployment_id = crate::apis::urlencode(p_path_deployment_id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AppsDeploymentResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AppsDeploymentResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AppsCancelDeploymentError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Commit an app rollback. This action permanently applies the rollback and unpins the app to resume new deployments.
pub async fn apps_commit_rollback(
    configuration: &configuration::Configuration,
    app_id: &str,
) -> Result<(), Error<AppsCommitRollbackError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_app_id = app_id;

    let uri_str = format!(
        "{}/v2/apps/{app_id}/rollback/commit",
        configuration.base_path,
        app_id = crate::apis::urlencode(p_path_app_id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<AppsCommitRollbackError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Create a new app by submitting an app specification. For documentation on app specifications (`AppSpec` objects), please refer to [the product documentation](https://docs.digitalocean.com/products/app-platform/reference/app-spec/).
pub async fn apps_create(
    configuration: &configuration::Configuration,
    apps_create_app_request: models::AppsCreateAppRequest,
    accept: Option<&str>,
    content_type: Option<&str>,
) -> Result<models::AppResponse, Error<AppsCreateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_apps_create_app_request = apps_create_app_request;
    let p_header_accept = accept;
    let p_header_content_type = content_type;

    let uri_str = format!("{}/v2/apps", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_header_accept {
        req_builder = req_builder.header("Accept", param_value.to_string());
    }
    if let Some(param_value) = p_header_content_type {
        req_builder = req_builder.header("Content-Type", param_value.to_string());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_apps_create_app_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AppResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AppResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AppsCreateError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Creating an app deployment will pull the latest changes from your repository and schedule a new deployment for your app.
pub async fn apps_create_deployment(
    configuration: &configuration::Configuration,
    app_id: &str,
    apps_create_deployment_request: models::AppsCreateDeploymentRequest,
) -> Result<models::AppsDeploymentResponse, Error<AppsCreateDeploymentError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_app_id = app_id;
    let p_body_apps_create_deployment_request = apps_create_deployment_request;

    let uri_str = format!(
        "{}/v2/apps/{app_id}/deployments",
        configuration.base_path,
        app_id = crate::apis::urlencode(p_path_app_id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_apps_create_deployment_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AppsDeploymentResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AppsDeploymentResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AppsCreateDeploymentError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Rollback an app to a previous deployment. A new deployment will be created to perform the rollback. The app will be pinned to the rollback deployment preventing any new deployments from being created, either manually or through Auto Deploy on Push webhooks. To resume deployments, the rollback must be either committed or reverted.  It is recommended to use the Validate App Rollback endpoint to double check if the rollback is valid and if there are any warnings.
pub async fn apps_create_rollback(
    configuration: &configuration::Configuration,
    app_id: &str,
    apps_rollback_app_request: models::AppsRollbackAppRequest,
) -> Result<models::AppsDeploymentResponse, Error<AppsCreateRollbackError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_app_id = app_id;
    let p_body_apps_rollback_app_request = apps_rollback_app_request;

    let uri_str = format!(
        "{}/v2/apps/{app_id}/rollback",
        configuration.base_path,
        app_id = crate::apis::urlencode(p_path_app_id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_apps_rollback_app_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AppsDeploymentResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AppsDeploymentResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AppsCreateRollbackError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Delete an existing app. Once deleted, all active deployments will be permanently shut down and the app deleted. If needed, be sure to back up your app specification so that you may re-create it at a later time.
pub async fn apps_delete(
    configuration: &configuration::Configuration,
    id: &str,
) -> Result<models::AppsDeleteAppResponse, Error<AppsDeleteError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;

    let uri_str = format!(
        "{}/v2/apps/{id}",
        configuration.base_path,
        id = crate::apis::urlencode(p_path_id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AppsDeleteAppResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AppsDeleteAppResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AppsDeleteError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Retrieve details about an existing app by either its ID or name. To retrieve an app by its name, do not include an ID in the request path. Information about the current active deployment as well as any in progress ones will also be included in the response.
pub async fn apps_get(
    configuration: &configuration::Configuration,
    id: &str,
    name: Option<&str>,
) -> Result<models::AppResponse, Error<AppsGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_query_name = name;

    let uri_str = format!(
        "{}/v2/apps/{id}",
        configuration.base_path,
        id = crate::apis::urlencode(p_path_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_name {
        req_builder = req_builder.query(&[("name", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AppResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AppResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AppsGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Retrieve information about an app deployment.
pub async fn apps_get_deployment(
    configuration: &configuration::Configuration,
    app_id: &str,
    deployment_id: &str,
) -> Result<models::AppsDeploymentResponse, Error<AppsGetDeploymentError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_app_id = app_id;
    let p_path_deployment_id = deployment_id;

    let uri_str = format!(
        "{}/v2/apps/{app_id}/deployments/{deployment_id}",
        configuration.base_path,
        app_id = crate::apis::urlencode(p_path_app_id),
        deployment_id = crate::apis::urlencode(p_path_deployment_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AppsDeploymentResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AppsDeploymentResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AppsGetDeploymentError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Returns a websocket URL that allows sending/receiving console input and output to a component of the specified deployment if one exists. Optionally, the instance_name parameter can be provided to retrieve the exec URL for a specific instance. Note that instances are ephemeral; therefore, we recommended to avoid making persistent changes or such scripting around them.
pub async fn apps_get_exec(
    configuration: &configuration::Configuration,
    app_id: &str,
    deployment_id: &str,
    component_name: &str,
    instance_name: Option<&str>,
) -> Result<models::AppsGetExecResponse, Error<AppsGetExecError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_app_id = app_id;
    let p_path_deployment_id = deployment_id;
    let p_path_component_name = component_name;
    let p_query_instance_name = instance_name;

    let uri_str = format!(
        "{}/v2/apps/{app_id}/deployments/{deployment_id}/components/{component_name}/exec",
        configuration.base_path,
        app_id = crate::apis::urlencode(p_path_app_id),
        deployment_id = crate::apis::urlencode(p_path_deployment_id),
        component_name = crate::apis::urlencode(p_path_component_name)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_instance_name {
        req_builder = req_builder.query(&[("instance_name", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AppsGetExecResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AppsGetExecResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AppsGetExecError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Returns a websocket URL that allows sending/receiving console input and output to a component of the active deployment if one exists.
pub async fn apps_get_exec_active_deployment(
    configuration: &configuration::Configuration,
    app_id: &str,
    component_name: &str,
    instance_name: Option<&str>,
) -> Result<models::AppsGetExecResponse, Error<AppsGetExecActiveDeploymentError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_app_id = app_id;
    let p_path_component_name = component_name;
    let p_query_instance_name = instance_name;

    let uri_str = format!(
        "{}/v2/apps/{app_id}/components/{component_name}/exec",
        configuration.base_path,
        app_id = crate::apis::urlencode(p_path_app_id),
        component_name = crate::apis::urlencode(p_path_component_name)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_instance_name {
        req_builder = req_builder.query(&[("instance_name", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AppsGetExecResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AppsGetExecResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AppsGetExecActiveDeploymentError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Retrieve information like health status, cpu and memory utilization of app components.
pub async fn apps_get_health(
    configuration: &configuration::Configuration,
    app_id: &str,
) -> Result<models::AppHealthResponse, Error<AppsGetHealthError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_app_id = app_id;

    let uri_str = format!(
        "{}/v2/apps/{app_id}/health",
        configuration.base_path,
        app_id = crate::apis::urlencode(p_path_app_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AppHealthResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AppHealthResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AppsGetHealthError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Retrieve information about a specific instance size for `service`, `worker`, and `job` components.
pub async fn apps_get_instance_size(
    configuration: &configuration::Configuration,
    slug: &str,
) -> Result<models::AppsGetInstanceSizeResponse, Error<AppsGetInstanceSizeError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_slug = slug;

    let uri_str = format!(
        "{}/v2/apps/tiers/instance_sizes/{slug}",
        configuration.base_path,
        slug = crate::apis::urlencode(p_path_slug)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AppsGetInstanceSizeResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AppsGetInstanceSizeResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AppsGetInstanceSizeError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Retrieve the list of running instances for a given application, including instance names and component types. Please note that these instances are ephemeral and may change over time. It is recommended not to make persistent changes or develop scripts that rely on their persistence.
pub async fn apps_get_instances(
    configuration: &configuration::Configuration,
    app_id: &str,
) -> Result<models::AppInstances, Error<AppsGetInstancesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_app_id = app_id;

    let uri_str = format!(
        "{}/v2/apps/{app_id}/instances",
        configuration.base_path,
        app_id = crate::apis::urlencode(p_path_app_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AppInstances`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AppInstances`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AppsGetInstancesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Get a specific job invocation for an app.
pub async fn apps_get_job_invocation(
    configuration: &configuration::Configuration,
    app_id: &str,
    job_invocation_id: &str,
    job_name: Option<&str>,
) -> Result<models::AppJobInvocation, Error<AppsGetJobInvocationError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_app_id = app_id;
    let p_path_job_invocation_id = job_invocation_id;
    let p_query_job_name = job_name;

    let uri_str = format!(
        "{}/v2/apps/{app_id}/job-invocations/{job_invocation_id}",
        configuration.base_path,
        app_id = crate::apis::urlencode(p_path_app_id),
        job_invocation_id = crate::apis::urlencode(p_path_job_invocation_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_job_name {
        req_builder = req_builder.query(&[("job_name", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AppJobInvocation`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AppJobInvocation`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AppsGetJobInvocationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Retrieve the logs of a past, in-progress, or active deployment. If a component name is specified, the logs will be limited to only that component. If deployment is omitted the active deployment will be selected (if available). The response will include links to either real-time logs of an in-progress or active deployment or archived logs of a past deployment.
pub async fn apps_get_job_invocation_logs(
    configuration: &configuration::Configuration,
    app_id: &str,
    job_name: &str,
    job_invocation_id: &str,
    r#type: &str,
    deployment_id: Option<&str>,
    follow: Option<bool>,
    pod_connection_timeout: Option<&str>,
    tail_lines: Option<&str>,
) -> Result<models::AppsGetLogsResponse, Error<AppsGetJobInvocationLogsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_app_id = app_id;
    let p_path_job_name = job_name;
    let p_path_job_invocation_id = job_invocation_id;
    let p_query_type = r#type;
    let p_query_deployment_id = deployment_id;
    let p_query_follow = follow;
    let p_query_pod_connection_timeout = pod_connection_timeout;
    let p_query_tail_lines = tail_lines;

    let uri_str = format!(
        "{}/v2/apps/{app_id}/jobs/{job_name}/invocations/{job_invocation_id}/logs",
        configuration.base_path,
        app_id = crate::apis::urlencode(p_path_app_id),
        job_name = crate::apis::urlencode(p_path_job_name),
        job_invocation_id = crate::apis::urlencode(p_path_job_invocation_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_deployment_id {
        req_builder = req_builder.query(&[("deployment_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_follow {
        req_builder = req_builder.query(&[("follow", &param_value.to_string())]);
    }
    req_builder = req_builder.query(&[("type", &p_query_type.to_string())]);
    if let Some(ref param_value) = p_query_pod_connection_timeout {
        req_builder = req_builder.query(&[("pod_connection_timeout", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_tail_lines {
        req_builder = req_builder.query(&[("tail_lines", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AppsGetLogsResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AppsGetLogsResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AppsGetJobInvocationLogsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Retrieve the logs of a past, in-progress, or active deployment. The response will include links to either real-time logs of an in-progress or active deployment or archived logs of a past deployment.
pub async fn apps_get_logs(
    configuration: &configuration::Configuration,
    app_id: &str,
    deployment_id: &str,
    component_name: &str,
    r#type: &str,
    follow: Option<bool>,
    pod_connection_timeout: Option<&str>,
) -> Result<models::AppsGetLogsResponse, Error<AppsGetLogsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_app_id = app_id;
    let p_path_deployment_id = deployment_id;
    let p_path_component_name = component_name;
    let p_query_type = r#type;
    let p_query_follow = follow;
    let p_query_pod_connection_timeout = pod_connection_timeout;

    let uri_str = format!(
        "{}/v2/apps/{app_id}/deployments/{deployment_id}/components/{component_name}/logs",
        configuration.base_path,
        app_id = crate::apis::urlencode(p_path_app_id),
        deployment_id = crate::apis::urlencode(p_path_deployment_id),
        component_name = crate::apis::urlencode(p_path_component_name)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_follow {
        req_builder = req_builder.query(&[("follow", &param_value.to_string())]);
    }
    req_builder = req_builder.query(&[("type", &p_query_type.to_string())]);
    if let Some(ref param_value) = p_query_pod_connection_timeout {
        req_builder = req_builder.query(&[("pod_connection_timeout", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AppsGetLogsResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AppsGetLogsResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AppsGetLogsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Retrieve the logs of the active deployment if one exists. The response will include links to either real-time logs of an in-progress or active deployment or archived logs of a past deployment. Note log_type=BUILD logs will return logs associated with the current active deployment (being served). To view build logs associated with in-progress build, the query must explicitly reference the deployment id.
pub async fn apps_get_logs_active_deployment(
    configuration: &configuration::Configuration,
    app_id: &str,
    component_name: &str,
    r#type: &str,
    follow: Option<bool>,
    pod_connection_timeout: Option<&str>,
) -> Result<models::AppsGetLogsResponse, Error<AppsGetLogsActiveDeploymentError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_app_id = app_id;
    let p_path_component_name = component_name;
    let p_query_type = r#type;
    let p_query_follow = follow;
    let p_query_pod_connection_timeout = pod_connection_timeout;

    let uri_str = format!(
        "{}/v2/apps/{app_id}/components/{component_name}/logs",
        configuration.base_path,
        app_id = crate::apis::urlencode(p_path_app_id),
        component_name = crate::apis::urlencode(p_path_component_name)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_follow {
        req_builder = req_builder.query(&[("follow", &param_value.to_string())]);
    }
    req_builder = req_builder.query(&[("type", &p_query_type.to_string())]);
    if let Some(ref param_value) = p_query_pod_connection_timeout {
        req_builder = req_builder.query(&[("pod_connection_timeout", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AppsGetLogsResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AppsGetLogsResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AppsGetLogsActiveDeploymentError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Retrieve the logs of the active deployment if one exists. The response will include links to either real-time logs of an in-progress or active deployment or archived logs of a past deployment. Note log_type=BUILD logs will return logs associated with the current active deployment (being served). To view build logs associated with in-progress build, the query must explicitly reference the deployment id.
pub async fn apps_get_logs_active_deployment_aggregate(
    configuration: &configuration::Configuration,
    app_id: &str,
    r#type: &str,
    follow: Option<bool>,
    pod_connection_timeout: Option<&str>,
) -> Result<models::AppsGetLogsResponse, Error<AppsGetLogsActiveDeploymentAggregateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_app_id = app_id;
    let p_query_type = r#type;
    let p_query_follow = follow;
    let p_query_pod_connection_timeout = pod_connection_timeout;

    let uri_str = format!(
        "{}/v2/apps/{app_id}/logs",
        configuration.base_path,
        app_id = crate::apis::urlencode(p_path_app_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_follow {
        req_builder = req_builder.query(&[("follow", &param_value.to_string())]);
    }
    req_builder = req_builder.query(&[("type", &p_query_type.to_string())]);
    if let Some(ref param_value) = p_query_pod_connection_timeout {
        req_builder = req_builder.query(&[("pod_connection_timeout", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AppsGetLogsResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AppsGetLogsResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AppsGetLogsActiveDeploymentAggregateError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Retrieve the logs of a past, in-progress, or active deployment. If a component name is specified, the logs will be limited to only that component. The response will include links to either real-time logs of an in-progress or active deployment or archived logs of a past deployment.
pub async fn apps_get_logs_aggregate(
    configuration: &configuration::Configuration,
    app_id: &str,
    deployment_id: &str,
    r#type: &str,
    follow: Option<bool>,
    pod_connection_timeout: Option<&str>,
) -> Result<models::AppsGetLogsResponse, Error<AppsGetLogsAggregateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_app_id = app_id;
    let p_path_deployment_id = deployment_id;
    let p_query_type = r#type;
    let p_query_follow = follow;
    let p_query_pod_connection_timeout = pod_connection_timeout;

    let uri_str = format!(
        "{}/v2/apps/{app_id}/deployments/{deployment_id}/logs",
        configuration.base_path,
        app_id = crate::apis::urlencode(p_path_app_id),
        deployment_id = crate::apis::urlencode(p_path_deployment_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_follow {
        req_builder = req_builder.query(&[("follow", &param_value.to_string())]);
    }
    req_builder = req_builder.query(&[("type", &p_query_type.to_string())]);
    if let Some(ref param_value) = p_query_pod_connection_timeout {
        req_builder = req_builder.query(&[("pod_connection_timeout", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AppsGetLogsResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AppsGetLogsResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AppsGetLogsAggregateError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Retrieve daily bandwidth usage metrics for a single app.
pub async fn apps_get_metrics_bandwidth_daily(
    configuration: &configuration::Configuration,
    app_id: &str,
    date: Option<String>,
) -> Result<models::AppMetricsBandwidthUsage, Error<AppsGetMetricsBandwidthDailyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_app_id = app_id;
    let p_query_date = date;

    let uri_str = format!(
        "{}/v2/apps/{app_id}/metrics/bandwidth_daily",
        configuration.base_path,
        app_id = crate::apis::urlencode(p_path_app_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_date {
        req_builder = req_builder.query(&[("date", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AppMetricsBandwidthUsage`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AppMetricsBandwidthUsage`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AppsGetMetricsBandwidthDailyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// List all apps on your account. Information about the current active deployment as well as any in progress ones will also be included for each app.
pub async fn apps_list(
    configuration: &configuration::Configuration,
    page: Option<i32>,
    per_page: Option<i32>,
    with_projects: Option<bool>,
) -> Result<models::AppsResponse, Error<AppsListError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_page = page;
    let p_query_per_page = per_page;
    let p_query_with_projects = with_projects;

    let uri_str = format!("{}/v2/apps", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_per_page {
        req_builder = req_builder.query(&[("per_page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_with_projects {
        req_builder = req_builder.query(&[("with_projects", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AppsResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AppsResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AppsListError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// List alerts associated to the app and any components. This includes configuration information about the alerts including emails, slack webhooks, and triggering events or conditions.
pub async fn apps_list_alerts(
    configuration: &configuration::Configuration,
    app_id: &str,
) -> Result<models::AppsListAlertsResponse, Error<AppsListAlertsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_app_id = app_id;

    let uri_str = format!(
        "{}/v2/apps/{app_id}/alerts",
        configuration.base_path,
        app_id = crate::apis::urlencode(p_path_app_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AppsListAlertsResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AppsListAlertsResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AppsListAlertsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// List all deployments of an app.
pub async fn apps_list_deployments(
    configuration: &configuration::Configuration,
    app_id: &str,
    page: Option<i32>,
    per_page: Option<i32>,
    deployment_types: Option<Vec<String>>,
) -> Result<models::AppsDeploymentsResponse, Error<AppsListDeploymentsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_app_id = app_id;
    let p_query_page = page;
    let p_query_per_page = per_page;
    let p_query_deployment_types = deployment_types;

    let uri_str = format!(
        "{}/v2/apps/{app_id}/deployments",
        configuration.base_path,
        app_id = crate::apis::urlencode(p_path_app_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_per_page {
        req_builder = req_builder.query(&[("per_page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_deployment_types {
        req_builder = match "multi" {
            "multi" => req_builder.query(
                &param_value
                    .into_iter()
                    .map(|p| ("deployment_types".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "deployment_types",
                &param_value
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AppsDeploymentsResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AppsDeploymentsResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AppsListDeploymentsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// List all instance sizes for `service`, `worker`, and `job` components.
pub async fn apps_list_instance_sizes(
    configuration: &configuration::Configuration,
) -> Result<models::AppsListInstanceSizesResponse, Error<AppsListInstanceSizesError>> {
    let uri_str = format!("{}/v2/apps/tiers/instance_sizes", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AppsListInstanceSizesResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AppsListInstanceSizesResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AppsListInstanceSizesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// List all job invocations for an app.
pub async fn apps_list_job_invocations(
    configuration: &configuration::Configuration,
    app_id: &str,
    job_names: Option<Vec<String>>,
    deployment_id: Option<&str>,
    page: Option<i32>,
    per_page: Option<i32>,
) -> Result<models::AppJobInvocations, Error<AppsListJobInvocationsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_app_id = app_id;
    let p_query_job_names = job_names;
    let p_query_deployment_id = deployment_id;
    let p_query_page = page;
    let p_query_per_page = per_page;

    let uri_str = format!(
        "{}/v2/apps/{app_id}/job-invocations",
        configuration.base_path,
        app_id = crate::apis::urlencode(p_path_app_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_job_names {
        req_builder = match "multi" {
            "multi" => req_builder.query(
                &param_value
                    .into_iter()
                    .map(|p| ("job_names".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "job_names",
                &param_value
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref param_value) = p_query_deployment_id {
        req_builder = req_builder.query(&[("deployment_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_per_page {
        req_builder = req_builder.query(&[("per_page", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AppJobInvocations`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AppJobInvocations`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AppsListJobInvocationsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Retrieve daily bandwidth usage metrics for multiple apps.
pub async fn apps_list_metrics_bandwidth_daily(
    configuration: &configuration::Configuration,
    app_metrics_bandwidth_usage_request: models::AppMetricsBandwidthUsageRequest,
) -> Result<models::AppMetricsBandwidthUsage, Error<AppsListMetricsBandwidthDailyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_app_metrics_bandwidth_usage_request = app_metrics_bandwidth_usage_request;

    let uri_str = format!(
        "{}/v2/apps/metrics/bandwidth_daily",
        configuration.base_path
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_app_metrics_bandwidth_usage_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AppMetricsBandwidthUsage`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AppMetricsBandwidthUsage`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AppsListMetricsBandwidthDailyError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// List all regions supported by App Platform.
pub async fn apps_list_regions(
    configuration: &configuration::Configuration,
) -> Result<models::AppsListRegionsResponse, Error<AppsListRegionsError>> {
    let uri_str = format!("{}/v2/apps/regions", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AppsListRegionsResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AppsListRegionsResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AppsListRegionsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Perform a rolling restart of all or specific components in an app.
pub async fn apps_restart(
    configuration: &configuration::Configuration,
    app_id: &str,
    apps_restart_request: Option<models::AppsRestartRequest>,
) -> Result<models::AppsDeploymentResponse, Error<AppsRestartError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_app_id = app_id;
    let p_body_apps_restart_request = apps_restart_request;

    let uri_str = format!(
        "{}/v2/apps/{app_id}/restart",
        configuration.base_path,
        app_id = crate::apis::urlencode(p_path_app_id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_apps_restart_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AppsDeploymentResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AppsDeploymentResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AppsRestartError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Revert an app rollback. This action reverts the active rollback by creating a new deployment from the latest app spec prior to the rollback and unpins the app to resume new deployments.
pub async fn apps_revert_rollback(
    configuration: &configuration::Configuration,
    app_id: &str,
) -> Result<models::AppsDeploymentResponse, Error<AppsRevertRollbackError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_app_id = app_id;

    let uri_str = format!(
        "{}/v2/apps/{app_id}/rollback/revert",
        configuration.base_path,
        app_id = crate::apis::urlencode(p_path_app_id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AppsDeploymentResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AppsDeploymentResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AppsRevertRollbackError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Update an existing app by submitting a new app specification. For documentation on app specifications (`AppSpec` objects), please refer to [the product documentation](https://docs.digitalocean.com/products/app-platform/reference/app-spec/).
pub async fn apps_update(
    configuration: &configuration::Configuration,
    id: &str,
    apps_update_app_request: models::AppsUpdateAppRequest,
) -> Result<models::AppResponse, Error<AppsUpdateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_body_apps_update_app_request = apps_update_app_request;

    let uri_str = format!(
        "{}/v2/apps/{id}",
        configuration.base_path,
        id = crate::apis::urlencode(p_path_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_apps_update_app_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AppResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AppResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AppsUpdateError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To propose and validate a spec for a new or existing app, send a POST request to the `/v2/apps/propose` endpoint. The request returns some information about the proposed app, including app cost and upgrade cost. If an existing app ID is specified, the app spec is treated as a proposed update to the existing app.
pub async fn apps_validate_app_spec(
    configuration: &configuration::Configuration,
    app_propose: models::AppPropose,
) -> Result<models::AppProposeResponse, Error<AppsValidateAppSpecError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_app_propose = app_propose;

    let uri_str = format!("{}/v2/apps/propose", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_app_propose);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AppProposeResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AppProposeResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AppsValidateAppSpecError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Check whether an app can be rolled back to a specific deployment. This endpoint can also be used to check if there are any warnings or validation conditions that will cause the rollback to proceed under unideal circumstances. For example, if a component must be rebuilt as part of the rollback causing it to take longer than usual.
pub async fn apps_validate_rollback(
    configuration: &configuration::Configuration,
    app_id: &str,
    apps_rollback_app_request: models::AppsRollbackAppRequest,
) -> Result<models::AppsValidateRollback200Response, Error<AppsValidateRollbackError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_app_id = app_id;
    let p_body_apps_rollback_app_request = apps_rollback_app_request;

    let uri_str = format!(
        "{}/v2/apps/{app_id}/rollback/validate",
        configuration.base_path,
        app_id = crate::apis::urlencode(p_path_app_id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_apps_rollback_app_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AppsValidateRollback200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AppsValidateRollback200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AppsValidateRollbackError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
