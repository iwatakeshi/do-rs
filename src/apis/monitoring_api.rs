/*
 * DigitalOcean API
 *
 * # Introduction  The DigitalOcean API allows you to manage Droplets and resources within the DigitalOcean cloud in a simple, programmatic way using conventional HTTP requests.  All of the functionality that you are familiar with in the DigitalOcean control panel is also available through the API, allowing you to script the complex actions that your situation requires.  The API documentation will start with a general overview about the design and technology that has been implemented, followed by reference information about specific endpoints.  ## Requests  Any tool that is fluent in HTTP can communicate with the API simply by requesting the correct URI. Requests should be made using the HTTPS protocol so that traffic is encrypted. The interface responds to different methods depending on the action required.  |Method|Usage| |--- |--- | |GET|For simple retrieval of information about your account, Droplets, or environment, you should use the GET method.  The information you request will be returned to you as a JSON object. The attributes defined by the JSON object can be used to form additional requests.  Any request using the GET method is read-only and will not affect any of the objects you are querying.| |DELETE|To destroy a resource and remove it from your account and environment, the DELETE method should be used.  This will remove the specified object if it is found.  If it is not found, the operation will return a response indicating that the object was not found. This idempotency means that you do not have to check for a resource's availability prior to issuing a delete command, the final state will be the same regardless of its existence.| |PUT|To update the information about a resource in your account, the PUT method is available. Like the DELETE Method, the PUT method is idempotent.  It sets the state of the target using the provided values, regardless of their current values. Requests using the PUT method do not need to check the current attributes of the object.| |PATCH|Some resources support partial modification. In these cases, the PATCH method is available. Unlike PUT which generally requires a complete representation of a resource, a PATCH request is a set of instructions on how to modify a resource updating only specific attributes.| |POST|To create a new object, your request should specify the POST method. The POST request includes all of the attributes necessary to create a new object.  When you wish to create a new object, send a POST request to the target endpoint.| |HEAD|Finally, to retrieve metadata information, you should use the HEAD method to get the headers.  This returns only the header of what would be returned with an associated GET request. Response headers contain some useful information about your API access and the results that are available for your request. For instance, the headers contain your current rate-limit value and the amount of time available until the limit resets. It also contains metrics about the total number of objects found, pagination information, and the total content length.|   ## HTTP Statuses  Along with the HTTP methods that the API responds to, it will also return standard HTTP statuses, including error codes.  In the event of a problem, the status will contain the error code, while the body of the response will usually contain additional information about the problem that was encountered.  In general, if the status returned is in the 200 range, it indicates that the request was fulfilled successfully and that no error was encountered.  Return codes in the 400 range typically indicate that there was an issue with the request that was sent. Among other things, this could mean that you did not authenticate correctly, that you are requesting an action that you do not have authorization for, that the object you are requesting does not exist, or that your request is malformed.  If you receive a status in the 500 range, this generally indicates a server-side problem. This means that we are having an issue on our end and cannot fulfill your request currently.  400 and 500 level error responses will include a JSON object in their body, including the following attributes:  |Name|Type|Description| |--- |--- |--- | |id|string|A short identifier corresponding to the HTTP status code returned. For example, the ID for a response returning a 404 status code would be \"not_found.\"| |message|string|A message providing additional information about the error, including details to help resolve it when possible.| |request_id|string|Optionally, some endpoints may include a request ID that should be provided when reporting bugs or opening support tickets to help identify the issue.|  ### Example Error Response  ```     HTTP/1.1 403 Forbidden     {       \"id\":       \"forbidden\",       \"message\":  \"You do not have access for the attempted action.\"     } ```  ## Responses  When a request is successful, a response body will typically be sent back in the form of a JSON object. An exception to this is when a DELETE request is processed, which will result in a successful HTTP 204 status and an empty response body.  Inside of this JSON object, the resource root that was the target of the request will be set as the key. This will be the singular form of the word if the request operated on a single object, and the plural form of the word if a collection was processed.  For example, if you send a GET request to `/v2/droplets/$DROPLET_ID` you will get back an object with a key called \"`droplet`\". However, if you send the GET request to the general collection at `/v2/droplets`, you will get back an object with a key called \"`droplets`\".  The value of these keys will generally be a JSON object for a request on a single object and an array of objects for a request on a collection of objects.  ### Response for a Single Object  ```json     {         \"droplet\": {             \"name\": \"example.com\"             . . .         }     } ```  ### Response for an Object Collection  ```json     {         \"droplets\": [             {                 \"name\": \"example.com\"                 . . .             },             {                 \"name\": \"second.com\"                 . . .             }         ]     } ```  ## Meta  In addition to the main resource root, the response may also contain a `meta` object. This object contains information about the response itself.  The `meta` object contains a `total` key that is set to the total number of objects returned by the request. This has implications on the `links` object and pagination.  The `meta` object will only be displayed when it has a value. Currently, the `meta` object will have a value when a request is made on a collection (like `droplets` or `domains`).   ### Sample Meta Object  ```json     {         . . .         \"meta\": {             \"total\": 43         }         . . .     } ```  ## Links & Pagination  The `links` object is returned as part of the response body when pagination is enabled. By default, 20 objects are returned per page. If the response contains 20 objects or fewer, no `links` object will be returned. If the response contains more than 20 objects, the first 20 will be returned along with the `links` object.  You can request a different pagination limit or force pagination by appending `?per_page=` to the request with the number of items you would like per page. For instance, to show only two results per page, you could add `?per_page=2` to the end of your query. The maximum number of results per page is 200.  The `links` object contains a `pages` object. The `pages` object, in turn, contains keys indicating the relationship of additional pages. The values of these are the URLs of the associated pages. The keys will be one of the following:  *   **first**: The URI of the first page of results. *   **prev**: The URI of the previous sequential page of results. *   **next**: The URI of the next sequential page of results. *   **last**: The URI of the last page of results.  The `pages` object will only include the links that make sense. So for the first page of results, no `first` or `prev` links will ever be set. This convention holds true in other situations where a link would not make sense.  ### Sample Links Object  ```json     {         . . .         \"links\": {             \"pages\": {                 \"last\": \"https://api.digitalocean.com/v2/images?page=2\",                 \"next\": \"https://api.digitalocean.com/v2/images?page=2\"             }         }         . . .     } ```  ## Rate Limit  Requests through the API are rate limited per OAuth token. Current rate limits:  *   5,000 requests per hour *   250 requests per minute (5% of the hourly total)  Once you exceed either limit, you will be rate limited until the next cycle starts. Space out any requests that you would otherwise issue in bursts for the best results.  The rate limiting information is contained within the response headers of each request. The relevant headers are:  *   **ratelimit-limit**: The number of requests that can be made per hour. *   **ratelimit-remaining**: The number of requests that remain before you hit your request limit. See the information below for how the request limits expire. *   **ratelimit-reset**: This represents the time when the oldest request will expire. The value is given in [Unix epoch time](http://en.wikipedia.org/wiki/Unix_time). See below for more information about how request limits expire.  More rate limiting information is returned only within burst limit error response headers: *   **retry-after**: The number of seconds to wait before making another request when rate limited.  As long as the `ratelimit-remaining` count is above zero, you will be able to make additional requests.  The way that a request expires and is removed from the current limit count is important to understand. Rather than counting all of the requests for an hour and resetting the `ratelimit-remaining` value at the end of the hour, each request instead has its own timer.  This means that each request contributes toward the `ratelimit-remaining` count for one complete hour after the request is made. When that request's timer runs out, it is no longer counted towards the request limit.  This has implications on the meaning of the `ratelimit-reset` header as well. Because the entire rate limit is not reset at one time, the value of this header is set to the time when the _oldest_ request will expire.  Keep this in mind if you see your `ratelimit-reset` value change, but not move an entire hour into the future.  If the `ratelimit-remaining` reaches zero, subsequent requests will receive a 429 error code until the request reset has been reached.   `ratelimit-remaining` reaching zero can also indicate that the \"burst limit\" of 250  requests per minute limit was met, even if the 5,000 requests per hour limit was not.  In this case, the 429 error response will include a `retry-after` header to indicate how  long to wait (in seconds) until the request may be retried.  You can see the format of the response in the examples.   **Note:** The following endpoints have special rate limit requirements that are independent of the limits defined above.  *   Only 10 `GET` requests to the `/v2/account/keys` endpoint to list SSH keys can be made per 60 seconds. *   Only 5 requests to any and all `v2/cdn/endpoints` can be made per 10 seconds. This includes `v2/cdn/endpoints`,      `v2/cdn/endpoints/$ENDPOINT_ID`, and `v2/cdn/endpoints/$ENDPOINT_ID/cache`. *   Only 50 strings within the `files` json struct in the `v2/cdn/endpoints/$ENDPOINT_ID/cache` [payload](https://docs.digitalocean.com/reference/api/api-reference/#operation/cdn_purge_cache)      can be requested every 20 seconds.  ### Sample Rate Limit Headers  ```     . . .     ratelimit-limit: 1200     ratelimit-remaining: 1193     rateLimit-reset: 1402425459     . . . ```    ### Sample Rate Limit Headers When Burst Limit is Reached:  ```     . . .     ratelimit-limit: 5000     ratelimit-remaining: 0     rateLimit-reset: 1402425459     retry-after: 29     . . . ```  ### Sample Rate Exceeded Response  ```     429 Too Many Requests     {             id: \"too_many_requests\",             message: \"API Rate limit exceeded.\"     } ```  ## Curl Examples  Throughout this document, some example API requests will be given using the `curl` command. This will allow us to demonstrate the various endpoints in a simple, textual format.      These examples assume that you are using a Linux or macOS command line. To run these commands on a Windows machine, you can either use cmd.exe, PowerShell, or WSL:  * For cmd.exe, use the `set VAR=VALUE` [syntax](https://docs.microsoft.com/en-us/windows-server/administration/windows-commands/set_1) to define environment variables, call them with `%VAR%`, then replace all backslashes (`\\`) in the examples with carets (`^`).  * For PowerShell, use the `$Env:VAR = \"VALUE\"` [syntax](https://docs.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_environment_variables?view=powershell-7.2) to define environment variables, call them with `$Env:VAR`, then replace `curl` with `curl.exe` and all backslashes (`\\`) in the examples with backticks (`` ` ``).  * WSL is a compatibility layer that allows you to emulate a Linux terminal on a Windows machine. Install WSL with our [community tutorial](https://www.digitalocean.com/community/tutorials/how-to-install-the-windows-subsystem-for-linux-2-on-microsoft-windows-10),  then follow this API documentation normally.  The names of account-specific references (like Droplet IDs, for instance) will be represented by variables. For instance, a Droplet ID may be represented by a variable called `$DROPLET_ID`. You can set the associated variables in your environment if you wish to use the examples without modification.  The first variable that you should set to get started is your OAuth authorization token. The next section will go over the details of this, but you can set an environmental variable for it now.  Generate a token by going to the [Apps & API](https://cloud.digitalocean.com/settings/applications) section of the DigitalOcean control panel. Use an existing token if you have saved one, or generate a new token with the \"Generate new token\" button. Copy the generated token and use it to set and export the TOKEN variable in your environment as the example shows.  You may also wish to set some other variables now or as you go along. For example, you may wish to set the `DROPLET_ID` variable to one of your Droplet IDs since this will be used frequently in the API.  If you are following along, make sure you use a Droplet ID that you control so that your commands will execute correctly.  If you need access to the headers of a response through `curl`, you can pass the `-i` flag to display the header information along with the body. If you are only interested in the header, you can instead pass the `-I` flag, which will exclude the response body entirely.   ### Set and Export your OAuth Token  ``` export DIGITALOCEAN_TOKEN=your_token_here ```  ### Set and Export a Variable  ``` export DROPLET_ID=1111111 ```  ## Parameters  There are two different ways to pass parameters in a request with the API.  When passing parameters to create or update an object, parameters should be passed as a JSON object containing the appropriate attribute names and values as key-value pairs. When you use this format, you should specify that you are sending a JSON object in the header. This is done by setting the `Content-Type` header to `application/json`. This ensures that your request is interpreted correctly.  When passing parameters to filter a response on GET requests, parameters can be passed using standard query attributes. In this case, the parameters would be embedded into the URI itself by appending a `?` to the end of the URI and then setting each attribute with an equal sign. Attributes can be separated with a `&`. Tools like `curl` can create the appropriate URI when given parameters and values; this can also be done using the `-F` flag and then passing the key and value as an argument. The argument should take the form of a quoted string with the attribute being set to a value with an equal sign.  ### Pass Parameters as a JSON Object  ```     curl -H \"Authorization: Bearer $DIGITALOCEAN_TOKEN\" \\         -H \"Content-Type: application/json\" \\         -d '{\"name\": \"example.com\", \"ip_address\": \"127.0.0.1\"}' \\         -X POST \"https://api.digitalocean.com/v2/domains\" ```  ### Pass Filter Parameters as a Query String  ```      curl -H \"Authorization: Bearer $DIGITALOCEAN_TOKEN\" \\          -X GET \\          \"https://api.digitalocean.com/v2/images?private=true\" ```  ## Cross Origin Resource Sharing  In order to make requests to the API from other domains, the API implements Cross Origin Resource Sharing (CORS) support.  CORS support is generally used to create AJAX requests outside of the domain that the request originated from. This is necessary to implement projects like control panels utilizing the API. This tells the browser that it can send requests to an outside domain.  The procedure that the browser initiates in order to perform these actions (other than GET requests) begins by sending a \"preflight\" request. This sets the `Origin` header and uses the `OPTIONS` method. The server will reply back with the methods it allows and some of the limits it imposes. The client then sends the actual request if it falls within the allowed constraints.  This process is usually done in the background by the browser, but you can use curl to emulate this process using the example provided. The headers that will be set to show the constraints are:  *   **Access-Control-Allow-Origin**: This is the domain that is sent by the client or browser as the origin of the request. It is set through an `Origin` header. *   **Access-Control-Allow-Methods**: This specifies the allowed options for requests from that domain. This will generally be all available methods. *   **Access-Control-Expose-Headers**: This will contain the headers that will be available to requests from the origin domain. *   **Access-Control-Max-Age**: This is the length of time that the access is considered valid. After this expires, a new preflight should be sent. *   **Access-Control-Allow-Credentials**: This will be set to `true`. It basically allows you to send your OAuth token for authentication.  You should not need to be concerned with the details of these headers, because the browser will typically do all of the work for you. 
 *
 * The version of the OpenAPI document: 2.0
 * Contact: api-engineering@digitalocean.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`monitoring_create_alert_policy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringCreateAlertPolicyError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_create_destination`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringCreateDestinationError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_create_sink`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringCreateSinkError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_delete_alert_policy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringDeleteAlertPolicyError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_delete_destination`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringDeleteDestinationError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_delete_sink`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringDeleteSinkError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_alert_policy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetAlertPolicyError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_app_cpu_percentage_metrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetAppCpuPercentageMetricsError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_app_memory_percentage_metrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetAppMemoryPercentageMetricsError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_app_restart_count_metrics_yml`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetAppRestartCountMetricsYmlError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_destination`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetDestinationError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_droplet_autoscale_current_cpu_utilization_yml`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetDropletAutoscaleCurrentCpuUtilizationYmlError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_droplet_autoscale_current_instances`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetDropletAutoscaleCurrentInstancesError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_droplet_autoscale_current_memory_utilization`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetDropletAutoscaleCurrentMemoryUtilizationError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_droplet_autoscale_target_cpu_utilization`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetDropletAutoscaleTargetCpuUtilizationError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_droplet_autoscale_target_instances`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetDropletAutoscaleTargetInstancesError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_droplet_autoscale_target_memory_utilization`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetDropletAutoscaleTargetMemoryUtilizationError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_droplet_bandwidth_metrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetDropletBandwidthMetricsError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_droplet_cpu_metrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetDropletCpuMetricsError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_droplet_filesystem_free_metrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetDropletFilesystemFreeMetricsError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_droplet_filesystem_size_metrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetDropletFilesystemSizeMetricsError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_droplet_load15_metrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetDropletLoad15MetricsError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_droplet_load1_metrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetDropletLoad1MetricsError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_droplet_load5_metrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetDropletLoad5MetricsError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_droplet_memory_available_metrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetDropletMemoryAvailableMetricsError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_droplet_memory_cached_metrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetDropletMemoryCachedMetricsError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_droplet_memory_free_metrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetDropletMemoryFreeMetricsError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_droplet_memory_total_metrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetDropletMemoryTotalMetricsError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_lb_droplets_connections`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetLbDropletsConnectionsError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_lb_droplets_downtime`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetLbDropletsDowntimeError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_lb_droplets_health_checks`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetLbDropletsHealthChecksError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_lb_droplets_http_response_time50p`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetLbDropletsHttpResponseTime50pError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_lb_droplets_http_response_time95p`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetLbDropletsHttpResponseTime95pError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_lb_droplets_http_response_time99p`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetLbDropletsHttpResponseTime99pError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_lb_droplets_http_response_time_avg`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetLbDropletsHttpResponseTimeAvgError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_lb_droplets_http_responses`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetLbDropletsHttpResponsesError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_lb_droplets_http_session_duration50p`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetLbDropletsHttpSessionDuration50pError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_lb_droplets_http_session_duration95p`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetLbDropletsHttpSessionDuration95pError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_lb_droplets_http_session_duration_avg`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetLbDropletsHttpSessionDurationAvgError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_lb_droplets_queue_size`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetLbDropletsQueueSizeError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_lb_frontend_connections_current`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetLbFrontendConnectionsCurrentError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_lb_frontend_connections_limit`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetLbFrontendConnectionsLimitError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_lb_frontend_cpu_utilization`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetLbFrontendCpuUtilizationError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_lb_frontend_firewall_dropped_bytes`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetLbFrontendFirewallDroppedBytesError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_lb_frontend_firewall_dropped_packets`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetLbFrontendFirewallDroppedPacketsError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_lb_frontend_http_requests_per_second`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetLbFrontendHttpRequestsPerSecondError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_lb_frontend_http_responses`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetLbFrontendHttpResponsesError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_lb_frontend_network_throughput_http`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetLbFrontendNetworkThroughputHttpError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_lb_frontend_network_throughput_tcp`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetLbFrontendNetworkThroughputTcpError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_lb_frontend_network_throughput_udp`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetLbFrontendNetworkThroughputUdpError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_lb_frontend_nlb_tcp_network_throughput`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetLbFrontendNlbTcpNetworkThroughputError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_lb_frontend_nlb_udp_network_throughput`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetLbFrontendNlbUdpNetworkThroughputError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_lb_frontend_tls_connections_current`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetLbFrontendTlsConnectionsCurrentError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_lb_frontend_tls_connections_exceeding_rate_limit`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetLbFrontendTlsConnectionsExceedingRateLimitError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_lb_frontend_tls_connections_limit`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetLbFrontendTlsConnectionsLimitError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_get_sink`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringGetSinkError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_list_alert_policy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringListAlertPolicyError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_list_destinations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringListDestinationsError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_list_sinks`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringListSinksError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_update_alert_policy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringUpdateAlertPolicyError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`monitoring_update_destination`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonitoringUpdateDestinationError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}


/// To create a new alert, send a POST request to `/v2/monitoring/alerts`.
pub async fn monitoring_create_alert_policy(configuration: &configuration::Configuration, alert_policy_request: models::AlertPolicyRequest) -> Result<models::MonitoringCreateAlertPolicy200Response, Error<MonitoringCreateAlertPolicyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_alert_policy_request = alert_policy_request;

    let uri_str = format!("{}/v2/monitoring/alerts", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_alert_policy_request);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::MonitoringCreateAlertPolicy200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::MonitoringCreateAlertPolicy200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringCreateAlertPolicyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To create a new destination, send a POST request to `/v2/monitoring/sinks/destinations`.
pub async fn monitoring_create_destination(configuration: &configuration::Configuration, destination_request: models::DestinationRequest) -> Result<models::MonitoringCreateDestination200Response, Error<MonitoringCreateDestinationError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_destination_request = destination_request;

    let uri_str = format!("{}/v2/monitoring/sinks/destinations", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_destination_request);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::MonitoringCreateDestination200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::MonitoringCreateDestination200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringCreateDestinationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To create a new sink, send a POST request to `/v2/monitoring/sinks`. Forwards logs from the  resources identified in `resources` to the specified pre-existing destination. 
pub async fn monitoring_create_sink(configuration: &configuration::Configuration, monitoring_create_sink_request: models::MonitoringCreateSinkRequest) -> Result<(), Error<MonitoringCreateSinkError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_monitoring_create_sink_request = monitoring_create_sink_request;

    let uri_str = format!("{}/v2/monitoring/sinks", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_monitoring_create_sink_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringCreateSinkError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To delete an alert policy, send a DELETE request to `/v2/monitoring/alerts/{alert_uuid}`
pub async fn monitoring_delete_alert_policy(configuration: &configuration::Configuration, alert_uuid: &str) -> Result<(), Error<MonitoringDeleteAlertPolicyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_alert_uuid = alert_uuid;

    let uri_str = format!("{}/v2/monitoring/alerts/{alert_uuid}", configuration.base_path, alert_uuid=crate::apis::urlencode(p_path_alert_uuid));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

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
        let entity: Option<MonitoringDeleteAlertPolicyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To delete a destination and all associated sinks, send a DELETE request to `/v2/monitoring/sinks/destinations/${destination_uuid}`.
pub async fn monitoring_delete_destination(configuration: &configuration::Configuration, destination_uuid: &str) -> Result<(), Error<MonitoringDeleteDestinationError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_destination_uuid = destination_uuid;

    let uri_str = format!("{}/v2/monitoring/sinks/destinations/{destination_uuid}", configuration.base_path, destination_uuid=crate::apis::urlencode(p_path_destination_uuid));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

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
        let entity: Option<MonitoringDeleteDestinationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To delete a sink, send a DELETE request to `/v2/monitoring/sinks/${sink_uuid}`.
pub async fn monitoring_delete_sink(configuration: &configuration::Configuration, sink_uuid: &str) -> Result<(), Error<MonitoringDeleteSinkError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_sink_uuid = sink_uuid;

    let uri_str = format!("{}/v2/monitoring/sinks/{sink_uuid}", configuration.base_path, sink_uuid=crate::apis::urlencode(p_path_sink_uuid));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

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
        let entity: Option<MonitoringDeleteSinkError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve a given alert policy, send a GET request to `/v2/monitoring/alerts/{alert_uuid}`
pub async fn monitoring_get_alert_policy(configuration: &configuration::Configuration, alert_uuid: &str) -> Result<models::MonitoringCreateAlertPolicy200Response, Error<MonitoringGetAlertPolicyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_alert_uuid = alert_uuid;

    let uri_str = format!("{}/v2/monitoring/alerts/{alert_uuid}", configuration.base_path, alert_uuid=crate::apis::urlencode(p_path_alert_uuid));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::MonitoringCreateAlertPolicy200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::MonitoringCreateAlertPolicy200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetAlertPolicyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve cpu percentage metrics for a given app, send a GET request to `/v2/monitoring/metrics/apps/cpu_percentage`.
pub async fn monitoring_get_app_cpu_percentage_metrics(configuration: &configuration::Configuration, app_id: &str, start: &str, end: &str, app_component: Option<&str>) -> Result<models::Metrics, Error<MonitoringGetAppCpuPercentageMetricsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_app_id = app_id;
    let p_query_start = start;
    let p_query_end = end;
    let p_query_app_component = app_component;

    let uri_str = format!("{}/v2/monitoring/metrics/apps/cpu_percentage", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("app_id", &p_query_app_id.to_string())]);
    if let Some(ref param_value) = p_query_app_component {
        req_builder = req_builder.query(&[("app_component", &param_value.to_string())]);
    }
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetAppCpuPercentageMetricsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve memory percentage metrics for a given app, send a GET request to `/v2/monitoring/metrics/apps/memory_percentage`.
pub async fn monitoring_get_app_memory_percentage_metrics(configuration: &configuration::Configuration, app_id: &str, start: &str, end: &str, app_component: Option<&str>) -> Result<models::Metrics, Error<MonitoringGetAppMemoryPercentageMetricsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_app_id = app_id;
    let p_query_start = start;
    let p_query_end = end;
    let p_query_app_component = app_component;

    let uri_str = format!("{}/v2/monitoring/metrics/apps/memory_percentage", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("app_id", &p_query_app_id.to_string())]);
    if let Some(ref param_value) = p_query_app_component {
        req_builder = req_builder.query(&[("app_component", &param_value.to_string())]);
    }
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetAppMemoryPercentageMetricsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve restart count metrics for a given app, send a GET request to `/v2/monitoring/metrics/apps/restart_count`.
pub async fn monitoring_get_app_restart_count_metrics_yml(configuration: &configuration::Configuration, app_id: &str, start: &str, end: &str, app_component: Option<&str>) -> Result<models::Metrics, Error<MonitoringGetAppRestartCountMetricsYmlError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_app_id = app_id;
    let p_query_start = start;
    let p_query_end = end;
    let p_query_app_component = app_component;

    let uri_str = format!("{}/v2/monitoring/metrics/apps/restart_count", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("app_id", &p_query_app_id.to_string())]);
    if let Some(ref param_value) = p_query_app_component {
        req_builder = req_builder.query(&[("app_component", &param_value.to_string())]);
    }
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetAppRestartCountMetricsYmlError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To get the details of a destination, send a GET request to `/v2/monitoring/sinks/destinations/${destination_uuid}`.
pub async fn monitoring_get_destination(configuration: &configuration::Configuration, destination_uuid: &str) -> Result<models::MonitoringCreateDestination200Response, Error<MonitoringGetDestinationError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_destination_uuid = destination_uuid;

    let uri_str = format!("{}/v2/monitoring/sinks/destinations/{destination_uuid}", configuration.base_path, destination_uuid=crate::apis::urlencode(p_path_destination_uuid));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::MonitoringCreateDestination200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::MonitoringCreateDestination200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetDestinationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve the current average CPU utilization for a given Droplet Autoscale Pool, send a GET request to `/v2/monitoring/metrics/droplet_autoscale/current_cpu_utilization`.
pub async fn monitoring_get_droplet_autoscale_current_cpu_utilization_yml(configuration: &configuration::Configuration, autoscale_pool_id: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetDropletAutoscaleCurrentCpuUtilizationYmlError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_autoscale_pool_id = autoscale_pool_id;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/droplet_autoscale/current_cpu_utilization", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("autoscale_pool_id", &p_query_autoscale_pool_id.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetDropletAutoscaleCurrentCpuUtilizationYmlError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve the current size for a given Droplet Autoscale Pool, send a GET request to `/v2/monitoring/metrics/droplet_autoscale/current_instances`.
pub async fn monitoring_get_droplet_autoscale_current_instances(configuration: &configuration::Configuration, autoscale_pool_id: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetDropletAutoscaleCurrentInstancesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_autoscale_pool_id = autoscale_pool_id;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/droplet_autoscale/current_instances", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("autoscale_pool_id", &p_query_autoscale_pool_id.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetDropletAutoscaleCurrentInstancesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve the current average memory utilization for a given Droplet Autoscale Pool, send a GET request to `/v2/monitoring/metrics/droplet_autoscale/current_memory_utilization`.
pub async fn monitoring_get_droplet_autoscale_current_memory_utilization(configuration: &configuration::Configuration, autoscale_pool_id: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetDropletAutoscaleCurrentMemoryUtilizationError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_autoscale_pool_id = autoscale_pool_id;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/droplet_autoscale/current_memory_utilization", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("autoscale_pool_id", &p_query_autoscale_pool_id.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetDropletAutoscaleCurrentMemoryUtilizationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve the target average CPU utilization for a given Droplet Autoscale Pool, send a GET request to `/v2/monitoring/metrics/droplet_autoscale/target_cpu_utilization`.
pub async fn monitoring_get_droplet_autoscale_target_cpu_utilization(configuration: &configuration::Configuration, autoscale_pool_id: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetDropletAutoscaleTargetCpuUtilizationError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_autoscale_pool_id = autoscale_pool_id;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/droplet_autoscale/target_cpu_utilization", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("autoscale_pool_id", &p_query_autoscale_pool_id.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetDropletAutoscaleTargetCpuUtilizationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve the target size for a given Droplet Autoscale Pool, send a GET request to `/v2/monitoring/metrics/droplet_autoscale/target_instances`.
pub async fn monitoring_get_droplet_autoscale_target_instances(configuration: &configuration::Configuration, autoscale_pool_id: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetDropletAutoscaleTargetInstancesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_autoscale_pool_id = autoscale_pool_id;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/droplet_autoscale/target_instances", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("autoscale_pool_id", &p_query_autoscale_pool_id.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetDropletAutoscaleTargetInstancesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve the target average memory utilization for a given Droplet Autoscale Pool, send a GET request to `/v2/monitoring/metrics/droplet_autoscale/target_memory_utilization`.
pub async fn monitoring_get_droplet_autoscale_target_memory_utilization(configuration: &configuration::Configuration, autoscale_pool_id: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetDropletAutoscaleTargetMemoryUtilizationError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_autoscale_pool_id = autoscale_pool_id;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/droplet_autoscale/target_memory_utilization", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("autoscale_pool_id", &p_query_autoscale_pool_id.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetDropletAutoscaleTargetMemoryUtilizationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve bandwidth metrics for a given Droplet, send a GET request to `/v2/monitoring/metrics/droplet/bandwidth`. Use the `interface` query parameter to specify if the results should be for the `private` or `public` interface. Use the `direction` query parameter to specify if the results should be for `inbound` or `outbound` traffic. The metrics in the response body are in megabits per second (Mbps).
pub async fn monitoring_get_droplet_bandwidth_metrics(configuration: &configuration::Configuration, host_id: &str, interface: &str, direction: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetDropletBandwidthMetricsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_host_id = host_id;
    let p_query_interface = interface;
    let p_query_direction = direction;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/droplet/bandwidth", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("host_id", &p_query_host_id.to_string())]);
    req_builder = req_builder.query(&[("interface", &p_query_interface.to_string())]);
    req_builder = req_builder.query(&[("direction", &p_query_direction.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetDropletBandwidthMetricsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve CPU metrics for a given droplet, send a GET request to `/v2/monitoring/metrics/droplet/cpu`.
pub async fn monitoring_get_droplet_cpu_metrics(configuration: &configuration::Configuration, host_id: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetDropletCpuMetricsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_host_id = host_id;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/droplet/cpu", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("host_id", &p_query_host_id.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetDropletCpuMetricsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve filesystem free metrics for a given droplet, send a GET request to `/v2/monitoring/metrics/droplet/filesystem_free`.
pub async fn monitoring_get_droplet_filesystem_free_metrics(configuration: &configuration::Configuration, host_id: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetDropletFilesystemFreeMetricsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_host_id = host_id;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/droplet/filesystem_free", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("host_id", &p_query_host_id.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetDropletFilesystemFreeMetricsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve filesystem size metrics for a given droplet, send a GET request to `/v2/monitoring/metrics/droplet/filesystem_size`.
pub async fn monitoring_get_droplet_filesystem_size_metrics(configuration: &configuration::Configuration, host_id: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetDropletFilesystemSizeMetricsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_host_id = host_id;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/droplet/filesystem_size", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("host_id", &p_query_host_id.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetDropletFilesystemSizeMetricsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve 15 minute load average metrics for a given droplet, send a GET request to `/v2/monitoring/metrics/droplet/load_15`.
pub async fn monitoring_get_droplet_load15_metrics(configuration: &configuration::Configuration, host_id: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetDropletLoad15MetricsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_host_id = host_id;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/droplet/load_15", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("host_id", &p_query_host_id.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetDropletLoad15MetricsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve 1 minute load average metrics for a given droplet, send a GET request to `/v2/monitoring/metrics/droplet/load_1`.
pub async fn monitoring_get_droplet_load1_metrics(configuration: &configuration::Configuration, host_id: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetDropletLoad1MetricsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_host_id = host_id;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/droplet/load_1", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("host_id", &p_query_host_id.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetDropletLoad1MetricsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve 5 minute load average metrics for a given droplet, send a GET request to `/v2/monitoring/metrics/droplet/load_5`.
pub async fn monitoring_get_droplet_load5_metrics(configuration: &configuration::Configuration, host_id: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetDropletLoad5MetricsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_host_id = host_id;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/droplet/load_5", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("host_id", &p_query_host_id.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetDropletLoad5MetricsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve available memory metrics for a given droplet, send a GET request to `/v2/monitoring/metrics/droplet/memory_available`.
pub async fn monitoring_get_droplet_memory_available_metrics(configuration: &configuration::Configuration, host_id: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetDropletMemoryAvailableMetricsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_host_id = host_id;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/droplet/memory_available", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("host_id", &p_query_host_id.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetDropletMemoryAvailableMetricsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve cached memory metrics for a given droplet, send a GET request to `/v2/monitoring/metrics/droplet/memory_cached`.
pub async fn monitoring_get_droplet_memory_cached_metrics(configuration: &configuration::Configuration, host_id: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetDropletMemoryCachedMetricsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_host_id = host_id;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/droplet/memory_cached", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("host_id", &p_query_host_id.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetDropletMemoryCachedMetricsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve free memory metrics for a given droplet, send a GET request to `/v2/monitoring/metrics/droplet/memory_free`.
pub async fn monitoring_get_droplet_memory_free_metrics(configuration: &configuration::Configuration, host_id: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetDropletMemoryFreeMetricsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_host_id = host_id;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/droplet/memory_free", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("host_id", &p_query_host_id.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetDropletMemoryFreeMetricsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve total memory metrics for a given droplet, send a GET request to `/v2/monitoring/metrics/droplet/memory_total`.
pub async fn monitoring_get_droplet_memory_total_metrics(configuration: &configuration::Configuration, host_id: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetDropletMemoryTotalMetricsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_host_id = host_id;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/droplet/memory_total", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("host_id", &p_query_host_id.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetDropletMemoryTotalMetricsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve Droplets active connections for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/droplets_connections`.
pub async fn monitoring_get_lb_droplets_connections(configuration: &configuration::Configuration, lb_id: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetLbDropletsConnectionsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_lb_id = lb_id;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/load_balancer/droplets_connections", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("lb_id", &p_query_lb_id.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetLbDropletsConnectionsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve Droplets downtime status for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/droplets_downtime`.
pub async fn monitoring_get_lb_droplets_downtime(configuration: &configuration::Configuration, lb_id: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetLbDropletsDowntimeError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_lb_id = lb_id;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/load_balancer/droplets_downtime", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("lb_id", &p_query_lb_id.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetLbDropletsDowntimeError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve Droplets health check status for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/droplets_health_checks`.
pub async fn monitoring_get_lb_droplets_health_checks(configuration: &configuration::Configuration, lb_id: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetLbDropletsHealthChecksError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_lb_id = lb_id;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/load_balancer/droplets_health_checks", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("lb_id", &p_query_lb_id.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetLbDropletsHealthChecksError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve Droplets 50th percentile HTTP response time in seconds for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/droplets_http_response_time_50p`.
pub async fn monitoring_get_lb_droplets_http_response_time50p(configuration: &configuration::Configuration, lb_id: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetLbDropletsHttpResponseTime50pError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_lb_id = lb_id;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/load_balancer/droplets_http_response_time_50p", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("lb_id", &p_query_lb_id.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetLbDropletsHttpResponseTime50pError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve Droplets 95th percentile HTTP response time in seconds for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/droplets_http_response_time_95p`.
pub async fn monitoring_get_lb_droplets_http_response_time95p(configuration: &configuration::Configuration, lb_id: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetLbDropletsHttpResponseTime95pError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_lb_id = lb_id;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/load_balancer/droplets_http_response_time_95p", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("lb_id", &p_query_lb_id.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetLbDropletsHttpResponseTime95pError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve Droplets 99th percentile HTTP response time in seconds for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/droplets_http_response_time_99p`.
pub async fn monitoring_get_lb_droplets_http_response_time99p(configuration: &configuration::Configuration, lb_id: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetLbDropletsHttpResponseTime99pError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_lb_id = lb_id;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/load_balancer/droplets_http_response_time_99p", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("lb_id", &p_query_lb_id.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetLbDropletsHttpResponseTime99pError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve Droplets average HTTP response time in seconds for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/droplets_http_response_time_avg`.
pub async fn monitoring_get_lb_droplets_http_response_time_avg(configuration: &configuration::Configuration, lb_id: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetLbDropletsHttpResponseTimeAvgError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_lb_id = lb_id;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/load_balancer/droplets_http_response_time_avg", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("lb_id", &p_query_lb_id.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetLbDropletsHttpResponseTimeAvgError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve Droplets HTTP rate of response code for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/droplets_http_responses`.
pub async fn monitoring_get_lb_droplets_http_responses(configuration: &configuration::Configuration, lb_id: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetLbDropletsHttpResponsesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_lb_id = lb_id;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/load_balancer/droplets_http_responses", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("lb_id", &p_query_lb_id.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetLbDropletsHttpResponsesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve Droplets 50th percentile HTTP session duration in seconds for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/droplets_http_session_duration_50p`.
pub async fn monitoring_get_lb_droplets_http_session_duration50p(configuration: &configuration::Configuration, lb_id: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetLbDropletsHttpSessionDuration50pError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_lb_id = lb_id;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/load_balancer/droplets_http_session_duration_50p", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("lb_id", &p_query_lb_id.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetLbDropletsHttpSessionDuration50pError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve Droplets 95th percentile HTTP session duration in seconds for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/droplets_http_session_duration_95p`.
pub async fn monitoring_get_lb_droplets_http_session_duration95p(configuration: &configuration::Configuration, lb_id: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetLbDropletsHttpSessionDuration95pError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_lb_id = lb_id;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/load_balancer/droplets_http_session_duration_95p", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("lb_id", &p_query_lb_id.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetLbDropletsHttpSessionDuration95pError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve Droplets average HTTP session duration in seconds for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/droplets_http_session_duration_avg`.
pub async fn monitoring_get_lb_droplets_http_session_duration_avg(configuration: &configuration::Configuration, lb_id: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetLbDropletsHttpSessionDurationAvgError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_lb_id = lb_id;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/load_balancer/droplets_http_session_duration_avg", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("lb_id", &p_query_lb_id.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetLbDropletsHttpSessionDurationAvgError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve Droplets queue size for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/droplets_queue_size`.
pub async fn monitoring_get_lb_droplets_queue_size(configuration: &configuration::Configuration, lb_id: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetLbDropletsQueueSizeError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_lb_id = lb_id;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/load_balancer/droplets_queue_size", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("lb_id", &p_query_lb_id.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetLbDropletsQueueSizeError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve frontend total current active connections for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/frontend_connections_current`.
pub async fn monitoring_get_lb_frontend_connections_current(configuration: &configuration::Configuration, lb_id: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetLbFrontendConnectionsCurrentError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_lb_id = lb_id;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/load_balancer/frontend_connections_current", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("lb_id", &p_query_lb_id.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetLbFrontendConnectionsCurrentError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve frontend max connections limit for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/frontend_connections_limit`.
pub async fn monitoring_get_lb_frontend_connections_limit(configuration: &configuration::Configuration, lb_id: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetLbFrontendConnectionsLimitError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_lb_id = lb_id;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/load_balancer/frontend_connections_limit", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("lb_id", &p_query_lb_id.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetLbFrontendConnectionsLimitError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve frontend average percentage CPU utilization for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/frontend_cpu_utilization`.
pub async fn monitoring_get_lb_frontend_cpu_utilization(configuration: &configuration::Configuration, lb_id: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetLbFrontendCpuUtilizationError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_lb_id = lb_id;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/load_balancer/frontend_cpu_utilization", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("lb_id", &p_query_lb_id.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetLbFrontendCpuUtilizationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve firewall dropped bytes for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/frontend_firewall_dropped_bytes`. This is currently only supported for network load balancers.
pub async fn monitoring_get_lb_frontend_firewall_dropped_bytes(configuration: &configuration::Configuration, lb_id: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetLbFrontendFirewallDroppedBytesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_lb_id = lb_id;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/load_balancer/frontend_firewall_dropped_bytes", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("lb_id", &p_query_lb_id.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetLbFrontendFirewallDroppedBytesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve firewall dropped packets per second for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/frontend_firewall_dropped_packets`. This is currently only supported for network load balancers.
pub async fn monitoring_get_lb_frontend_firewall_dropped_packets(configuration: &configuration::Configuration, lb_id: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetLbFrontendFirewallDroppedPacketsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_lb_id = lb_id;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/load_balancer/frontend_firewall_dropped_packets", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("lb_id", &p_query_lb_id.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetLbFrontendFirewallDroppedPacketsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve frontend HTTP requests per second for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/frontend_http_requests_per_second`.
pub async fn monitoring_get_lb_frontend_http_requests_per_second(configuration: &configuration::Configuration, lb_id: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetLbFrontendHttpRequestsPerSecondError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_lb_id = lb_id;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/load_balancer/frontend_http_requests_per_second", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("lb_id", &p_query_lb_id.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetLbFrontendHttpRequestsPerSecondError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve frontend HTTP rate of response code for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/frontend_http_responses`.
pub async fn monitoring_get_lb_frontend_http_responses(configuration: &configuration::Configuration, lb_id: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetLbFrontendHttpResponsesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_lb_id = lb_id;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/load_balancer/frontend_http_responses", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("lb_id", &p_query_lb_id.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetLbFrontendHttpResponsesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve frontend HTTP throughput in bytes per second for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/frontend_network_throughput_http`.
pub async fn monitoring_get_lb_frontend_network_throughput_http(configuration: &configuration::Configuration, lb_id: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetLbFrontendNetworkThroughputHttpError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_lb_id = lb_id;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/load_balancer/frontend_network_throughput_http", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("lb_id", &p_query_lb_id.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetLbFrontendNetworkThroughputHttpError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve frontend TCP throughput in bytes per second for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/frontend_network_throughput_tcp`.
pub async fn monitoring_get_lb_frontend_network_throughput_tcp(configuration: &configuration::Configuration, lb_id: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetLbFrontendNetworkThroughputTcpError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_lb_id = lb_id;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/load_balancer/frontend_network_throughput_tcp", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("lb_id", &p_query_lb_id.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetLbFrontendNetworkThroughputTcpError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve frontend UDP throughput in bytes per second for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/frontend_network_throughput_udp`.
pub async fn monitoring_get_lb_frontend_network_throughput_udp(configuration: &configuration::Configuration, lb_id: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetLbFrontendNetworkThroughputUdpError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_lb_id = lb_id;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/load_balancer/frontend_network_throughput_udp", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("lb_id", &p_query_lb_id.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetLbFrontendNetworkThroughputUdpError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve frontend TCP throughput in bytes per second for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/frontend_nlb_tcp_network_throughput`.
pub async fn monitoring_get_lb_frontend_nlb_tcp_network_throughput(configuration: &configuration::Configuration, lb_id: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetLbFrontendNlbTcpNetworkThroughputError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_lb_id = lb_id;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/load_balancer/frontend_nlb_tcp_network_throughput", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("lb_id", &p_query_lb_id.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetLbFrontendNlbTcpNetworkThroughputError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve frontend UDP throughput in bytes per second for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/frontend_nlb_udp_network_throughput`.
pub async fn monitoring_get_lb_frontend_nlb_udp_network_throughput(configuration: &configuration::Configuration, lb_id: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetLbFrontendNlbUdpNetworkThroughputError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_lb_id = lb_id;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/load_balancer/frontend_nlb_udp_network_throughput", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("lb_id", &p_query_lb_id.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetLbFrontendNlbUdpNetworkThroughputError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve frontend current TLS connections rate for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/frontend_tls_connections_current`.
pub async fn monitoring_get_lb_frontend_tls_connections_current(configuration: &configuration::Configuration, lb_id: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetLbFrontendTlsConnectionsCurrentError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_lb_id = lb_id;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/load_balancer/frontend_tls_connections_current", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("lb_id", &p_query_lb_id.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetLbFrontendTlsConnectionsCurrentError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve frontend closed TLS connections for exceeded rate limit for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/frontend_tls_connections_exceeding_rate_limit`.
pub async fn monitoring_get_lb_frontend_tls_connections_exceeding_rate_limit(configuration: &configuration::Configuration, lb_id: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetLbFrontendTlsConnectionsExceedingRateLimitError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_lb_id = lb_id;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/load_balancer/frontend_tls_connections_exceeding_rate_limit", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("lb_id", &p_query_lb_id.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetLbFrontendTlsConnectionsExceedingRateLimitError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve frontend max TLS connections limit for a given load balancer, send a GET request to `/v2/monitoring/metrics/load_balancer/frontend_tls_connections_limit`.
pub async fn monitoring_get_lb_frontend_tls_connections_limit(configuration: &configuration::Configuration, lb_id: &str, start: &str, end: &str) -> Result<models::Metrics, Error<MonitoringGetLbFrontendTlsConnectionsLimitError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_lb_id = lb_id;
    let p_query_start = start;
    let p_query_end = end;

    let uri_str = format!("{}/v2/monitoring/metrics/load_balancer/frontend_tls_connections_limit", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("lb_id", &p_query_lb_id.to_string())]);
    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Metrics`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Metrics`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetLbFrontendTlsConnectionsLimitError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To get the details of a sink (resources and destination), send a GET request to `/v2/monitoring/sinks/${sink_uuid}`.
pub async fn monitoring_get_sink(configuration: &configuration::Configuration, sink_uuid: &str) -> Result<models::MonitoringGetSink200Response, Error<MonitoringGetSinkError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_sink_uuid = sink_uuid;

    let uri_str = format!("{}/v2/monitoring/sinks/{sink_uuid}", configuration.base_path, sink_uuid=crate::apis::urlencode(p_path_sink_uuid));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::MonitoringGetSink200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::MonitoringGetSink200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringGetSinkError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns all alert policies that are configured for the given account. To List all alert policies, send a GET request to `/v2/monitoring/alerts`.
pub async fn monitoring_list_alert_policy(configuration: &configuration::Configuration, per_page: Option<i32>, page: Option<i32>) -> Result<models::MonitoringListAlertPolicy200Response, Error<MonitoringListAlertPolicyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_per_page = per_page;
    let p_query_page = page;

    let uri_str = format!("{}/v2/monitoring/alerts", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_per_page {
        req_builder = req_builder.query(&[("per_page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::MonitoringListAlertPolicy200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::MonitoringListAlertPolicy200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringListAlertPolicyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To list all logging destinations, send a GET request to `/v2/monitoring/sinks/destinations`.
pub async fn monitoring_list_destinations(configuration: &configuration::Configuration, ) -> Result<models::MonitoringListDestinations200Response, Error<MonitoringListDestinationsError>> {

    let uri_str = format!("{}/v2/monitoring/sinks/destinations", configuration.base_path);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::MonitoringListDestinations200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::MonitoringListDestinations200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringListDestinationsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To list all sinks, send a GET request to `/v2/monitoring/sinks`.
pub async fn monitoring_list_sinks(configuration: &configuration::Configuration, resource_id: Option<&str>) -> Result<models::MonitoringListSinks200Response, Error<MonitoringListSinksError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_resource_id = resource_id;

    let uri_str = format!("{}/v2/monitoring/sinks", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_resource_id {
        req_builder = req_builder.query(&[("resource_id", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::MonitoringListSinks200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::MonitoringListSinks200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringListSinksError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To update en existing policy, send a PUT request to `v2/monitoring/alerts/{alert_uuid}`.
pub async fn monitoring_update_alert_policy(configuration: &configuration::Configuration, alert_uuid: &str, alert_policy_request: models::AlertPolicyRequest) -> Result<models::MonitoringCreateAlertPolicy200Response, Error<MonitoringUpdateAlertPolicyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_alert_uuid = alert_uuid;
    let p_body_alert_policy_request = alert_policy_request;

    let uri_str = format!("{}/v2/monitoring/alerts/{alert_uuid}", configuration.base_path, alert_uuid=crate::apis::urlencode(p_path_alert_uuid));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_alert_policy_request);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::MonitoringCreateAlertPolicy200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::MonitoringCreateAlertPolicy200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringUpdateAlertPolicyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To update the details of a destination, send a PATCH request to `/v2/monitoring/sinks/destinations/${destination_uuid}`.
pub async fn monitoring_update_destination(configuration: &configuration::Configuration, destination_uuid: &str, destination_request: models::DestinationRequest) -> Result<(), Error<MonitoringUpdateDestinationError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_destination_uuid = destination_uuid;
    let p_body_destination_request = destination_request;

    let uri_str = format!("{}/v2/monitoring/sinks/destinations/{destination_uuid}", configuration.base_path, destination_uuid=crate::apis::urlencode(p_path_destination_uuid));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_destination_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<MonitoringUpdateDestinationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

