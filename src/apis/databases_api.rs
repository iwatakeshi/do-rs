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


/// struct for typed errors of method [`databases_add`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesAddError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_add_connection_pool`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesAddConnectionPoolError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_add_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesAddUserError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_create_cluster`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesCreateClusterError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_create_kafka_schema`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesCreateKafkaSchemaError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_create_kafka_topic`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesCreateKafkaTopicError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_create_logsink`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesCreateLogsinkError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_create_replica`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesCreateReplicaError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesDeleteError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_delete_connection_pool`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesDeleteConnectionPoolError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_delete_kafka_schema`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesDeleteKafkaSchemaError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_delete_kafka_topic`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesDeleteKafkaTopicError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_delete_logsink`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesDeleteLogsinkError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_delete_online_migration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesDeleteOnlineMigrationError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_delete_opensearch_index`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesDeleteOpensearchIndexError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_delete_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesDeleteUserError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_destroy_cluster`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesDestroyClusterError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_destroy_replica`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesDestroyReplicaError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesGetError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_get_autoscale`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesGetAutoscaleError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_get_ca`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesGetCaError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_get_cluster`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesGetClusterError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_get_cluster_metrics_credentials`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesGetClusterMetricsCredentialsError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_get_config`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesGetConfigError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_get_connection_pool`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesGetConnectionPoolError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_get_eviction_policy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesGetEvictionPolicyError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_get_kafka_schema`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesGetKafkaSchemaError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_get_kafka_schema_config`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesGetKafkaSchemaConfigError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_get_kafka_schema_subject_config`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesGetKafkaSchemaSubjectConfigError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_get_kafka_schema_version`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesGetKafkaSchemaVersionError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_get_kafka_topic`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesGetKafkaTopicError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_get_logsink`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesGetLogsinkError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_get_migration_status`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesGetMigrationStatusError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_get_replica`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesGetReplicaError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_get_sql_mode`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesGetSqlModeError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_get_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesGetUserError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_install_update`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesInstallUpdateError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesListError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_list_backups`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesListBackupsError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_list_clusters`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesListClustersError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_list_connection_pools`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesListConnectionPoolsError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_list_events_logs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesListEventsLogsError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_list_firewall_rules`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesListFirewallRulesError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_list_kafka_schemas`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesListKafkaSchemasError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_list_kafka_topics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesListKafkaTopicsError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_list_logsink`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesListLogsinkError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_list_opeasearch_indexes`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesListOpeasearchIndexesError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_list_options`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesListOptionsError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_list_replicas`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesListReplicasError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_list_users`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesListUsersError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_patch_config`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesPatchConfigError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_promote_replica`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesPromoteReplicaError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_reset_auth`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesResetAuthError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_update_autoscale`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesUpdateAutoscaleError {
    Status401(models::Error),
    Status404(models::Error),
    Status422(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_update_cluster_metrics_credentials`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesUpdateClusterMetricsCredentialsError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_update_cluster_size`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesUpdateClusterSizeError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_update_connection_pool`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesUpdateConnectionPoolError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_update_eviction_policy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesUpdateEvictionPolicyError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_update_firewall_rules`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesUpdateFirewallRulesError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_update_kafka_schema_config`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesUpdateKafkaSchemaConfigError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_update_kafka_schema_subject_config`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesUpdateKafkaSchemaSubjectConfigError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_update_kafka_topic`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesUpdateKafkaTopicError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_update_logsink`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesUpdateLogsinkError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_update_maintenance_window`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesUpdateMaintenanceWindowError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_update_major_version`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesUpdateMajorVersionError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_update_online_migration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesUpdateOnlineMigrationError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_update_region`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesUpdateRegionError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_update_sql_mode`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesUpdateSqlModeError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`databases_update_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabasesUpdateUserError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}


/// To add a new database to an existing cluster, send a POST request to `/v2/databases/$DATABASE_ID/dbs`.  Note: Database management is not supported for Caching or Valkey clusters.  The response will be a JSON object with a key called `db`. The value of this will be an object that contains the standard attributes associated with a database. 
pub async fn databases_add(configuration: &configuration::Configuration, database_cluster_uuid: &str, database: models::Database) -> Result<models::DatabasesAdd201Response, Error<DatabasesAddError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;
    let p_body_database = database;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/dbs", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_database);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DatabasesAdd201Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DatabasesAdd201Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesAddError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// For PostgreSQL database clusters, connection pools can be used to allow a database to share its idle connections. The popular PostgreSQL connection pooling utility PgBouncer is used to provide this service. [See here for more information](https://docs.digitalocean.com/products/databases/postgresql/how-to/manage-connection-pools/) about how and why to use PgBouncer connection pooling including details about the available transaction modes.  To add a new connection pool to a PostgreSQL database cluster, send a POST request to `/v2/databases/$DATABASE_ID/pools` specifying a name for the pool, the user to connect with, the database to connect to, as well as its desired size and transaction mode. 
pub async fn databases_add_connection_pool(configuration: &configuration::Configuration, database_cluster_uuid: &str, connection_pool: models::ConnectionPool) -> Result<models::DatabasesAddConnectionPool201Response, Error<DatabasesAddConnectionPoolError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;
    let p_body_connection_pool = connection_pool;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/pools", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_connection_pool);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DatabasesAddConnectionPool201Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DatabasesAddConnectionPool201Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesAddConnectionPoolError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To add a new database user, send a POST request to `/v2/databases/$DATABASE_ID/users` with the desired username.  Note: User management is not supported for Caching or Valkey clusters.  When adding a user to a MySQL cluster, additional options can be configured in the `mysql_settings` object.  When adding a user to a Kafka cluster, additional options can be configured in the `settings` object.   When adding a user to a MongoDB cluster, additional options can be configured in the `settings.mongo_user_settings` object.  The response will be a JSON object with a key called `user`. The value of this will be an object that contains the standard attributes associated with a database user including its randomly generated password. 
pub async fn databases_add_user(configuration: &configuration::Configuration, database_cluster_uuid: &str, databases_add_user_request: models::DatabasesAddUserRequest) -> Result<models::DatabasesAddUser201Response, Error<DatabasesAddUserError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;
    let p_body_databases_add_user_request = databases_add_user_request;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/users", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_databases_add_user_request);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DatabasesAddUser201Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DatabasesAddUser201Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesAddUserError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To create a database cluster, send a POST request to `/v2/databases`. To see a list  of options for each engine, such as available regions, size slugs, and versions, send a GET request to the `/v2/databases/options` endpoint. The available sizes for  the `storage_size_mib` field depends on the cluster's size. To see a list of available sizes, see [Managed Database Pricing](https://www.digitalocean.com/pricing/managed-databases).  The create response returns a JSON object with a key called `database`. The value of this is an object that contains the standard attributes associated with a database cluster. The initial value of the database cluster's `status` attribute is `creating`. When the cluster is ready to receive traffic, this changes to `online`.  The embedded `connection` and `private_connection` objects contains the information needed to access the database cluster. For multi-node clusters, the `standby_connection` and `standby_private_connection` objects contain the information needed to connect to the cluster's standby node(s).  DigitalOcean managed PostgreSQL and MySQL database clusters take automated daily backups. To create a new database cluster based on a backup of an existing cluster, send a POST request to `/v2/databases`. In addition to the standard database cluster attributes, the JSON body must include a key named `backup_restore` with the name of the original database cluster and the timestamp of the backup to be restored. Creating a database from a backup is the same as forking a database in the control panel. Note: Caching cluster creates are no longer supported as of 2025-04-30T00:00:00Z. Backups are also not supported for Caching or Valkey clusters.
pub async fn databases_create_cluster(configuration: &configuration::Configuration, databases_create_cluster_request: models::DatabasesCreateClusterRequest) -> Result<models::DatabasesCreateCluster201Response, Error<DatabasesCreateClusterError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_databases_create_cluster_request = databases_create_cluster_request;

    let uri_str = format!("{}/v2/databases", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_databases_create_cluster_request);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DatabasesCreateCluster201Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DatabasesCreateCluster201Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesCreateClusterError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To create a Kafka schema for a database cluster, send a POST request to `/v2/databases/$DATABASE_ID/schema-registry`. 
pub async fn databases_create_kafka_schema(configuration: &configuration::Configuration, database_cluster_uuid: &str, database_kafka_schema_create: models::DatabaseKafkaSchemaCreate) -> Result<models::KafkaSchemaVerbose, Error<DatabasesCreateKafkaSchemaError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;
    let p_body_database_kafka_schema_create = database_kafka_schema_create;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/schema-registry", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_database_kafka_schema_create);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::KafkaSchemaVerbose`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::KafkaSchemaVerbose`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesCreateKafkaSchemaError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To create a topic attached to a Kafka cluster, send a POST request to `/v2/databases/$DATABASE_ID/topics`.  The result will be a JSON object with a `topic` key. 
pub async fn databases_create_kafka_topic(configuration: &configuration::Configuration, database_cluster_uuid: &str, kafka_topic_create: Option<models::KafkaTopicCreate>) -> Result<models::DatabasesCreateKafkaTopic201Response, Error<DatabasesCreateKafkaTopicError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;
    let p_body_kafka_topic_create = kafka_topic_create;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/topics", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_kafka_topic_create);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DatabasesCreateKafkaTopic201Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DatabasesCreateKafkaTopic201Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesCreateKafkaTopicError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To create logsink for a database cluster, send a POST request to `/v2/databases/$DATABASE_ID/logsink`. 
pub async fn databases_create_logsink(configuration: &configuration::Configuration, database_cluster_uuid: &str, logsink_create: models::LogsinkCreate) -> Result<models::DatabasesCreateLogsink201Response, Error<DatabasesCreateLogsinkError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;
    let p_body_logsink_create = logsink_create;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/logsink", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_logsink_create);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DatabasesCreateLogsink201Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DatabasesCreateLogsink201Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesCreateLogsinkError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To create a read-only replica for a PostgreSQL or MySQL database cluster, send a POST request to `/v2/databases/$DATABASE_ID/replicas` specifying the name it should be given, the size of the node to be used, and the region where it will be located.  **Note**: Read-only replicas are not supported for Caching or Valkey clusters.  The response will be a JSON object with a key called `replica`. The value of this will be an object that contains the standard attributes associated with a database replica. The initial value of the read-only replica's `status` attribute will be `forking`. When the replica is ready to receive traffic, this will transition to `active`.
pub async fn databases_create_replica(configuration: &configuration::Configuration, database_cluster_uuid: &str, database_replica: Option<models::DatabaseReplica>) -> Result<models::DatabasesCreateReplica201Response, Error<DatabasesCreateReplicaError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;
    let p_body_database_replica = database_replica;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/replicas", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_database_replica);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DatabasesCreateReplica201Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DatabasesCreateReplica201Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesCreateReplicaError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To delete a specific database, send a DELETE request to `/v2/databases/$DATABASE_ID/dbs/$DB_NAME`.  A status of 204 will be given. This indicates that the request was processed successfully, but that no response body is needed.  Note: Database management is not supported for Caching or Valkey clusters. 
pub async fn databases_delete(configuration: &configuration::Configuration, database_cluster_uuid: &str, database_name: &str) -> Result<(), Error<DatabasesDeleteError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;
    let p_path_database_name = database_name;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/dbs/{database_name}", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid), database_name=crate::apis::urlencode(p_path_database_name));
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
        let entity: Option<DatabasesDeleteError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To delete a specific connection pool for a PostgreSQL database cluster, send a DELETE request to `/v2/databases/$DATABASE_ID/pools/$POOL_NAME`.  A status of 204 will be given. This indicates that the request was processed successfully, but that no response body is needed. 
pub async fn databases_delete_connection_pool(configuration: &configuration::Configuration, database_cluster_uuid: &str, pool_name: &str) -> Result<(), Error<DatabasesDeleteConnectionPoolError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;
    let p_path_pool_name = pool_name;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/pools/{pool_name}", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid), pool_name=crate::apis::urlencode(p_path_pool_name));
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
        let entity: Option<DatabasesDeleteConnectionPoolError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To delete a specific schema by subject name for a Kafka cluster, send a DELETE request to `/v2/databases/$DATABASE_ID/schema-registry/$SUBJECT_NAME`. 
pub async fn databases_delete_kafka_schema(configuration: &configuration::Configuration, database_cluster_uuid: &str, subject_name: &str) -> Result<(), Error<DatabasesDeleteKafkaSchemaError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;
    let p_path_subject_name = subject_name;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/schema-registry/{subject_name}", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid), subject_name=crate::apis::urlencode(p_path_subject_name));
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
        let entity: Option<DatabasesDeleteKafkaSchemaError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To delete a single topic within a Kafka cluster, send a DELETE request to `/v2/databases/$DATABASE_ID/topics/$TOPIC_NAME`.  A status of 204 will be given. This indicates that the request was processed successfully, but that no response body is needed. 
pub async fn databases_delete_kafka_topic(configuration: &configuration::Configuration, database_cluster_uuid: &str, topic_name: &str) -> Result<(), Error<DatabasesDeleteKafkaTopicError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;
    let p_path_topic_name = topic_name;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/topics/{topic_name}", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid), topic_name=crate::apis::urlencode(p_path_topic_name));
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
        let entity: Option<DatabasesDeleteKafkaTopicError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To delete a logsink for a database cluster, send a DELETE request to `/v2/databases/$DATABASE_ID/logsink/$LOGSINK_ID`. 
pub async fn databases_delete_logsink(configuration: &configuration::Configuration, database_cluster_uuid: &str, logsink_id: &str) -> Result<(), Error<DatabasesDeleteLogsinkError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;
    let p_path_logsink_id = logsink_id;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/logsink/{logsink_id}", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid), logsink_id=crate::apis::urlencode(p_path_logsink_id));
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
        let entity: Option<DatabasesDeleteLogsinkError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To stop an online migration, send a DELETE request to `/v2/databases/$DATABASE_ID/online-migration/$MIGRATION_ID`.  A status of 204 will be given. This indicates that the request was processed successfully, but that no response body is needed. 
pub async fn databases_delete_online_migration(configuration: &configuration::Configuration, database_cluster_uuid: &str, migration_id: &str) -> Result<(), Error<DatabasesDeleteOnlineMigrationError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;
    let p_path_migration_id = migration_id;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/online-migration/{migration_id}", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid), migration_id=crate::apis::urlencode(p_path_migration_id));
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
        let entity: Option<DatabasesDeleteOnlineMigrationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To delete a single index within OpenSearch cluster, send a DELETE request to `/v2/databases/$DATABASE_ID/indexes/$INDEX_NAME`.  A status of 204 will be given. This indicates that the request was processed successfully, but that no response body is needed. 
pub async fn databases_delete_opensearch_index(configuration: &configuration::Configuration, database_cluster_uuid: &str, index_name: &str) -> Result<(), Error<DatabasesDeleteOpensearchIndexError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;
    let p_path_index_name = index_name;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/indexes/{index_name}", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid), index_name=crate::apis::urlencode(p_path_index_name));
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
        let entity: Option<DatabasesDeleteOpensearchIndexError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To remove a specific database user, send a DELETE request to `/v2/databases/$DATABASE_ID/users/$USERNAME`.  A status of 204 will be given. This indicates that the request was processed successfully, but that no response body is needed.  Note: User management is not supported for Caching or Valkey clusters. 
pub async fn databases_delete_user(configuration: &configuration::Configuration, database_cluster_uuid: &str, username: &str) -> Result<(), Error<DatabasesDeleteUserError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;
    let p_path_username = username;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/users/{username}", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid), username=crate::apis::urlencode(p_path_username));
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
        let entity: Option<DatabasesDeleteUserError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To destroy a specific database, send a DELETE request to `/v2/databases/$DATABASE_ID`. A status of 204 will be given. This indicates that the request was processed successfully, but that no response body is needed.
pub async fn databases_destroy_cluster(configuration: &configuration::Configuration, database_cluster_uuid: &str) -> Result<(), Error<DatabasesDestroyClusterError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid));
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
        let entity: Option<DatabasesDestroyClusterError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To destroy a specific read-only replica, send a DELETE request to `/v2/databases/$DATABASE_ID/replicas/$REPLICA_NAME`.  **Note**: Read-only replicas are not supported for Caching or Valkey clusters.  A status of 204 will be given. This indicates that the request was processed successfully, but that no response body is needed.
pub async fn databases_destroy_replica(configuration: &configuration::Configuration, database_cluster_uuid: &str, replica_name: &str) -> Result<(), Error<DatabasesDestroyReplicaError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;
    let p_path_replica_name = replica_name;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/replicas/{replica_name}", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid), replica_name=crate::apis::urlencode(p_path_replica_name));
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
        let entity: Option<DatabasesDestroyReplicaError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To show information about an existing database cluster, send a GET request to `/v2/databases/$DATABASE_ID/dbs/$DB_NAME`.  Note: Database management is not supported for Caching or Valkey clusters.  The response will be a JSON object with a `db` key. This will be set to an object containing the standard database attributes. 
pub async fn databases_get(configuration: &configuration::Configuration, database_cluster_uuid: &str, database_name: &str) -> Result<models::DatabasesAdd201Response, Error<DatabasesGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;
    let p_path_database_name = database_name;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/dbs/{database_name}", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid), database_name=crate::apis::urlencode(p_path_database_name));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DatabasesAdd201Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DatabasesAdd201Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve the autoscale configuration for an existing database cluster, send a GET request to `/v2/databases/$DATABASE_ID/autoscale`. The response will be a JSON object with autoscaling configuration details.
pub async fn databases_get_autoscale(configuration: &configuration::Configuration, database_cluster_uuid: &str) -> Result<models::DatabasesGetAutoscale200Response, Error<DatabasesGetAutoscaleError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/autoscale", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DatabasesGetAutoscale200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DatabasesGetAutoscale200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesGetAutoscaleError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve the public certificate used to secure the connection to the database cluster send a GET request to `/v2/databases/$DATABASE_ID/ca`.  The response will be a JSON object with a `ca` key. This will be set to an object containing the base64 encoding of the public key certificate. 
pub async fn databases_get_ca(configuration: &configuration::Configuration, database_cluster_uuid: &str) -> Result<models::DatabasesGetCa200Response, Error<DatabasesGetCaError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/ca", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DatabasesGetCa200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DatabasesGetCa200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesGetCaError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To show information about an existing database cluster, send a GET request to `/v2/databases/$DATABASE_ID`.  The response will be a JSON object with a database key. This will be set to an object containing the standard database cluster attributes.  The embedded `connection` and `private_connection` objects will contain the information needed to access the database cluster. For multi-node clusters, the `standby_connection` and `standby_private_connection` objects contain the information needed to connect to the cluster's standby node(s).  The embedded maintenance_window object will contain information about any scheduled maintenance for the database cluster.
pub async fn databases_get_cluster(configuration: &configuration::Configuration, database_cluster_uuid: &str) -> Result<models::DatabasesCreateCluster201Response, Error<DatabasesGetClusterError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DatabasesCreateCluster201Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DatabasesCreateCluster201Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesGetClusterError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To show the credentials for all database clusters' metrics endpoints, send a GET request to `/v2/databases/metrics/credentials`. The result will be a JSON object with a `credentials` key.
pub async fn databases_get_cluster_metrics_credentials(configuration: &configuration::Configuration, ) -> Result<models::DatabasesGetClusterMetricsCredentials200Response, Error<DatabasesGetClusterMetricsCredentialsError>> {

    let uri_str = format!("{}/v2/databases/metrics/credentials", configuration.base_path);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DatabasesGetClusterMetricsCredentials200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DatabasesGetClusterMetricsCredentials200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesGetClusterMetricsCredentialsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Shows configuration parameters for an existing database cluster by sending a GET request to `/v2/databases/$DATABASE_ID/config`. The response is a JSON object with a `config` key, which is set to an object containing any database configuration parameters. 
pub async fn databases_get_config(configuration: &configuration::Configuration, database_cluster_uuid: &str) -> Result<models::DatabasesGetConfig200Response, Error<DatabasesGetConfigError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/config", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DatabasesGetConfig200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DatabasesGetConfig200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesGetConfigError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To show information about an existing connection pool for a PostgreSQL database cluster, send a GET request to `/v2/databases/$DATABASE_ID/pools/$POOL_NAME`. The response will be a JSON object with a `pool` key.
pub async fn databases_get_connection_pool(configuration: &configuration::Configuration, database_cluster_uuid: &str, pool_name: &str) -> Result<models::DatabasesAddConnectionPool201Response, Error<DatabasesGetConnectionPoolError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;
    let p_path_pool_name = pool_name;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/pools/{pool_name}", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid), pool_name=crate::apis::urlencode(p_path_pool_name));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DatabasesAddConnectionPool201Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DatabasesAddConnectionPool201Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesGetConnectionPoolError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve the configured eviction policy for an existing Caching or Valkey cluster, send a GET request to `/v2/databases/$DATABASE_ID/eviction_policy`. The response will be a JSON object with an `eviction_policy` key. This will be set to a string representing the eviction policy.
pub async fn databases_get_eviction_policy(configuration: &configuration::Configuration, database_cluster_uuid: &str) -> Result<models::DatabasesGetEvictionPolicy200Response, Error<DatabasesGetEvictionPolicyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/eviction_policy", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DatabasesGetEvictionPolicy200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DatabasesGetEvictionPolicy200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesGetEvictionPolicyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To get a specific schema by subject name for a Kafka cluster, send a GET request to `/v2/databases/$DATABASE_ID/schema-registry/$SUBJECT_NAME`. 
pub async fn databases_get_kafka_schema(configuration: &configuration::Configuration, database_cluster_uuid: &str, subject_name: &str) -> Result<models::KafkaSchemaVersionVerbose, Error<DatabasesGetKafkaSchemaError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;
    let p_path_subject_name = subject_name;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/schema-registry/{subject_name}", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid), subject_name=crate::apis::urlencode(p_path_subject_name));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::KafkaSchemaVersionVerbose`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::KafkaSchemaVersionVerbose`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesGetKafkaSchemaError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve the Schema Registry configuration for a Kafka cluster, send a GET request to `/v2/databases/$DATABASE_ID/schema-registry/config`. The response is a JSON object with a `compatibility_level` key, which is set to an object containing any database configuration parameters. 
pub async fn databases_get_kafka_schema_config(configuration: &configuration::Configuration, database_cluster_uuid: &str) -> Result<models::DatabasesGetKafkaSchemaConfig200Response, Error<DatabasesGetKafkaSchemaConfigError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/schema-registry/config", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DatabasesGetKafkaSchemaConfig200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DatabasesGetKafkaSchemaConfig200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesGetKafkaSchemaConfigError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve the Schema Registry configuration for a Subject of a Kafka cluster, send a GET request to `/v2/databases/$DATABASE_ID/schema-registry/config/$SUBJECT_NAME`. The response is a JSON object with a `compatibility_level` key, which is set to an object containing any database configuration parameters. 
pub async fn databases_get_kafka_schema_subject_config(configuration: &configuration::Configuration, database_cluster_uuid: &str, subject_name: &str) -> Result<models::DatabasesGetKafkaSchemaSubjectConfig200Response, Error<DatabasesGetKafkaSchemaSubjectConfigError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;
    let p_path_subject_name = subject_name;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/schema-registry/config/{subject_name}", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid), subject_name=crate::apis::urlencode(p_path_subject_name));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DatabasesGetKafkaSchemaSubjectConfig200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DatabasesGetKafkaSchemaSubjectConfig200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesGetKafkaSchemaSubjectConfigError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To get a specific schema by subject name for a Kafka cluster, send a GET request to `/v2/databases/$DATABASE_ID/schema-registry/$SUBJECT_NAME/versions/$VERSION`. 
pub async fn databases_get_kafka_schema_version(configuration: &configuration::Configuration, database_cluster_uuid: &str, subject_name: &str, version: &str) -> Result<models::KafkaSchemaVersionVerbose, Error<DatabasesGetKafkaSchemaVersionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;
    let p_path_subject_name = subject_name;
    let p_path_version = version;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/schema-registry/{subject_name}/versions/{version}", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid), subject_name=crate::apis::urlencode(p_path_subject_name), version=crate::apis::urlencode(p_path_version));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::KafkaSchemaVersionVerbose`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::KafkaSchemaVersionVerbose`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesGetKafkaSchemaVersionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve a given topic by name from the set of a Kafka cluster's topics, send a GET request to `/v2/databases/$DATABASE_ID/topics/$TOPIC_NAME`.  The result will be a JSON object with a `topic` key. 
pub async fn databases_get_kafka_topic(configuration: &configuration::Configuration, database_cluster_uuid: &str, topic_name: &str) -> Result<models::DatabasesCreateKafkaTopic201Response, Error<DatabasesGetKafkaTopicError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;
    let p_path_topic_name = topic_name;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/topics/{topic_name}", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid), topic_name=crate::apis::urlencode(p_path_topic_name));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DatabasesCreateKafkaTopic201Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DatabasesCreateKafkaTopic201Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesGetKafkaTopicError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To get a logsink for a database cluster, send a GET request to `/v2/databases/$DATABASE_ID/logsink/$LOGSINK_ID`. 
pub async fn databases_get_logsink(configuration: &configuration::Configuration, database_cluster_uuid: &str, logsink_id: &str) -> Result<models::LogsinkSchema, Error<DatabasesGetLogsinkError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;
    let p_path_logsink_id = logsink_id;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/logsink/{logsink_id}", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid), logsink_id=crate::apis::urlencode(p_path_logsink_id));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::LogsinkSchema`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::LogsinkSchema`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesGetLogsinkError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve the status of the most recent online migration, send a GET request to `/v2/databases/$DATABASE_ID/online-migration`. 
pub async fn databases_get_migration_status(configuration: &configuration::Configuration, database_cluster_uuid: &str) -> Result<models::OnlineMigration, Error<DatabasesGetMigrationStatusError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/online-migration", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::OnlineMigration`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::OnlineMigration`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesGetMigrationStatusError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To show information about an existing database replica, send a GET request to `/v2/databases/$DATABASE_ID/replicas/$REPLICA_NAME`.  **Note**: Read-only replicas are not supported for Caching or Valkey clusters.  The response will be a JSON object with a `replica key`. This will be set to an object containing the standard database replica attributes.
pub async fn databases_get_replica(configuration: &configuration::Configuration, database_cluster_uuid: &str, replica_name: &str) -> Result<models::DatabasesCreateReplica201Response, Error<DatabasesGetReplicaError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;
    let p_path_replica_name = replica_name;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/replicas/{replica_name}", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid), replica_name=crate::apis::urlencode(p_path_replica_name));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DatabasesCreateReplica201Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DatabasesCreateReplica201Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesGetReplicaError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve the configured SQL modes for an existing MySQL cluster, send a GET request to `/v2/databases/$DATABASE_ID/sql_mode`. The response will be a JSON object with a `sql_mode` key. This will be set to a string representing the configured SQL modes.
pub async fn databases_get_sql_mode(configuration: &configuration::Configuration, database_cluster_uuid: &str) -> Result<models::SqlMode, Error<DatabasesGetSqlModeError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/sql_mode", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::SqlMode`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::SqlMode`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesGetSqlModeError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To show information about an existing database user, send a GET request to `/v2/databases/$DATABASE_ID/users/$USERNAME`.  Note: User management is not supported for Caching or Valkey clusters.  The response will be a JSON object with a `user` key. This will be set to an object containing the standard database user attributes. The user's password will not show up unless the `database:view_credentials` scope is present.  For MySQL clusters, additional options will be contained in the `mysql_settings` object.  For Kafka clusters, additional options will be contained in the `settings` object.  For MongoDB clusters, additional information will be contained in the mongo_user_settings object 
pub async fn databases_get_user(configuration: &configuration::Configuration, database_cluster_uuid: &str, username: &str) -> Result<models::DatabasesAddUser201Response, Error<DatabasesGetUserError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;
    let p_path_username = username;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/users/{username}", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid), username=crate::apis::urlencode(p_path_username));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DatabasesAddUser201Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DatabasesAddUser201Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesGetUserError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To start the installation of updates for a database cluster, send a PUT request to `/v2/databases/$DATABASE_ID/install_update`. A successful request will receive a 204 No Content status code with no body in response.
pub async fn databases_install_update(configuration: &configuration::Configuration, database_cluster_uuid: &str) -> Result<(), Error<DatabasesInstallUpdateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/install_update", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

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
        let entity: Option<DatabasesInstallUpdateError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To list all of the databases in a clusters, send a GET request to `/v2/databases/$DATABASE_ID/dbs`.  The result will be a JSON object with a `dbs` key. This will be set to an array of database objects, each of which will contain the standard database attributes.  Note: Database management is not supported for Caching or Valkey clusters. 
pub async fn databases_list(configuration: &configuration::Configuration, database_cluster_uuid: &str) -> Result<models::DatabasesList200Response, Error<DatabasesListError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/dbs", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DatabasesList200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DatabasesList200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesListError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To list all of the available backups of a PostgreSQL or MySQL database cluster, send a GET request to `/v2/databases/$DATABASE_ID/backups`. **Note**: Backups are not supported for Caching or Valkey clusters. The result will be a JSON object with a `backups key`. This will be set to an array of backup objects, each of which will contain the size of the backup and the timestamp at which it was created.
pub async fn databases_list_backups(configuration: &configuration::Configuration, database_cluster_uuid: &str) -> Result<models::DatabasesListBackups200Response, Error<DatabasesListBackupsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/backups", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DatabasesListBackups200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DatabasesListBackups200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesListBackupsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To list all of the database clusters available on your account, send a GET request to `/v2/databases`. To limit the results to database clusters with a specific tag, include the `tag_name` query parameter set to the name of the tag. For example, `/v2/databases?tag_name=$TAG_NAME`.  The result will be a JSON object with a `databases` key. This will be set to an array of database objects, each of which will contain the standard database attributes.  The embedded `connection` and `private_connection` objects will contain the information needed to access the database cluster. For multi-node clusters, the `standby_connection` and `standby_private_connection` objects will contain the information needed to connect to the cluster's standby node(s).  The embedded `maintenance_window` object will contain information about any scheduled maintenance for the database cluster.
pub async fn databases_list_clusters(configuration: &configuration::Configuration, tag_name: Option<&str>) -> Result<models::DatabasesListClusters200Response, Error<DatabasesListClustersError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_tag_name = tag_name;

    let uri_str = format!("{}/v2/databases", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_tag_name {
        req_builder = req_builder.query(&[("tag_name", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DatabasesListClusters200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DatabasesListClusters200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesListClustersError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To list all of the connection pools available to a PostgreSQL database cluster, send a GET request to `/v2/databases/$DATABASE_ID/pools`. The result will be a JSON object with a `pools` key. This will be set to an array of connection pool objects.
pub async fn databases_list_connection_pools(configuration: &configuration::Configuration, database_cluster_uuid: &str) -> Result<models::ConnectionPools, Error<DatabasesListConnectionPoolsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/pools", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ConnectionPools`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ConnectionPools`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesListConnectionPoolsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To list all of the cluster events, send a GET request to `/v2/databases/$DATABASE_ID/events`.  The result will be a JSON object with a `events` key. 
pub async fn databases_list_events_logs(configuration: &configuration::Configuration, database_cluster_uuid: &str) -> Result<models::DatabasesListEventsLogs200Response, Error<DatabasesListEventsLogsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/events", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DatabasesListEventsLogs200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DatabasesListEventsLogs200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesListEventsLogsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To list all of a database cluster's firewall rules (known as \"trusted sources\" in the control panel), send a GET request to `/v2/databases/$DATABASE_ID/firewall`. The result will be a JSON object with a `rules` key.
pub async fn databases_list_firewall_rules(configuration: &configuration::Configuration, database_cluster_uuid: &str) -> Result<models::DatabasesListFirewallRules200Response, Error<DatabasesListFirewallRulesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/firewall", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DatabasesListFirewallRules200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DatabasesListFirewallRules200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesListFirewallRulesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To list all schemas for a Kafka cluster, send a GET request to `/v2/databases/$DATABASE_ID/schema-registry`. 
pub async fn databases_list_kafka_schemas(configuration: &configuration::Configuration, database_cluster_uuid: &str) -> Result<models::DatabasesListKafkaSchemas200Response, Error<DatabasesListKafkaSchemasError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/schema-registry", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DatabasesListKafkaSchemas200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DatabasesListKafkaSchemas200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesListKafkaSchemasError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To list all of a Kafka cluster's topics, send a GET request to `/v2/databases/$DATABASE_ID/topics`.  The result will be a JSON object with a `topics` key. 
pub async fn databases_list_kafka_topics(configuration: &configuration::Configuration, database_cluster_uuid: &str) -> Result<models::DatabasesListKafkaTopics200Response, Error<DatabasesListKafkaTopicsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/topics", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DatabasesListKafkaTopics200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DatabasesListKafkaTopics200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesListKafkaTopicsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To list logsinks for a database cluster, send a GET request to `/v2/databases/$DATABASE_ID/logsink`. 
pub async fn databases_list_logsink(configuration: &configuration::Configuration, database_cluster_uuid: &str) -> Result<models::DatabasesListLogsink200Response, Error<DatabasesListLogsinkError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/logsink", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DatabasesListLogsink200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DatabasesListLogsink200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesListLogsinkError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To list all of a OpenSearch cluster's indexes, send a GET request to `/v2/databases/$DATABASE_ID/indexes`.  The result will be a JSON object with a `indexes` key. 
pub async fn databases_list_opeasearch_indexes(configuration: &configuration::Configuration, database_cluster_uuid: &str) -> Result<models::DatabasesListOpeasearchIndexes200Response, Error<DatabasesListOpeasearchIndexesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/indexes", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DatabasesListOpeasearchIndexes200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DatabasesListOpeasearchIndexes200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesListOpeasearchIndexesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To list all of the options available for the offered database engines, send a GET request to `/v2/databases/options`. The result will be a JSON object with an `options` key.
pub async fn databases_list_options(configuration: &configuration::Configuration, ) -> Result<models::Options, Error<DatabasesListOptionsError>> {

    let uri_str = format!("{}/v2/databases/options", configuration.base_path);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Options`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Options`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesListOptionsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To list all of the read-only replicas associated with a database cluster, send a GET request to `/v2/databases/$DATABASE_ID/replicas`.  **Note**: Read-only replicas are not supported for Caching or Valkey clusters.  The result will be a JSON object with a `replicas` key. This will be set to an array of database replica objects, each of which will contain the standard database replica attributes.
pub async fn databases_list_replicas(configuration: &configuration::Configuration, database_cluster_uuid: &str) -> Result<models::DatabasesListReplicas200Response, Error<DatabasesListReplicasError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/replicas", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DatabasesListReplicas200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DatabasesListReplicas200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesListReplicasError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To list all of the users for your database cluster, send a GET request to `/v2/databases/$DATABASE_ID/users`.  Note: User management is not supported for Caching or Valkey clusters.  The result will be a JSON object with a `users` key. This will be set to an array of database user objects, each of which will contain the standard database user attributes. User passwords will not show without the `database:view_credentials` scope.  For MySQL clusters, additional options will be contained in the mysql_settings object.  For MongoDB clusters, additional information will be contained in the mongo_user_settings object 
pub async fn databases_list_users(configuration: &configuration::Configuration, database_cluster_uuid: &str) -> Result<models::DatabasesListUsers200Response, Error<DatabasesListUsersError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/users", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DatabasesListUsers200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DatabasesListUsers200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesListUsersError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To update the configuration for an existing database cluster, send a PATCH request to `/v2/databases/$DATABASE_ID/config`. 
pub async fn databases_patch_config(configuration: &configuration::Configuration, database_cluster_uuid: &str, database_config: models::DatabaseConfig) -> Result<(), Error<DatabasesPatchConfigError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;
    let p_body_database_config = database_config;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/config", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid));
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_database_config);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesPatchConfigError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To promote a specific read-only replica, send a PUT request to `/v2/databases/$DATABASE_ID/replicas/$REPLICA_NAME/promote`.  **Note**: Read-only replicas are not supported for Caching or Valkey clusters.  A status of 204 will be given. This indicates that the request was processed successfully, but that no response body is needed.
pub async fn databases_promote_replica(configuration: &configuration::Configuration, database_cluster_uuid: &str, replica_name: &str) -> Result<(), Error<DatabasesPromoteReplicaError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;
    let p_path_replica_name = replica_name;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/replicas/{replica_name}/promote", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid), replica_name=crate::apis::urlencode(p_path_replica_name));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

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
        let entity: Option<DatabasesPromoteReplicaError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To reset the password for a database user, send a POST request to `/v2/databases/$DATABASE_ID/users/$USERNAME/reset_auth`.  For `mysql` databases, the authentication method can be specifying by including a key in the JSON body called `mysql_settings` with the `auth_plugin` value specified.  The response will be a JSON object with a `user` key. This will be set to an object containing the standard database user attributes. 
pub async fn databases_reset_auth(configuration: &configuration::Configuration, database_cluster_uuid: &str, username: &str, databases_reset_auth_request: models::DatabasesResetAuthRequest) -> Result<models::DatabasesAddUser201Response, Error<DatabasesResetAuthError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;
    let p_path_username = username;
    let p_body_databases_reset_auth_request = databases_reset_auth_request;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/users/{username}/reset_auth", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid), username=crate::apis::urlencode(p_path_username));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_databases_reset_auth_request);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DatabasesAddUser201Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DatabasesAddUser201Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesResetAuthError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To configure autoscale settings for an existing database cluster, send a PUT request to `/v2/databases/$DATABASE_ID/autoscale`, specifying the autoscale configuration. A successful request will receive a 204 No Content status code with no body in response.
pub async fn databases_update_autoscale(configuration: &configuration::Configuration, database_cluster_uuid: &str, database_autoscale_params: models::DatabaseAutoscaleParams) -> Result<(), Error<DatabasesUpdateAutoscaleError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;
    let p_body_database_autoscale_params = database_autoscale_params;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/autoscale", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_database_autoscale_params);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesUpdateAutoscaleError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To update the credentials for all database clusters' metrics endpoints, send a PUT request to `/v2/databases/metrics/credentials`. A successful request will receive a 204 No Content status code  with no body in response.
pub async fn databases_update_cluster_metrics_credentials(configuration: &configuration::Configuration, database_metrics_credentials: Option<models::DatabaseMetricsCredentials>) -> Result<(), Error<DatabasesUpdateClusterMetricsCredentialsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_database_metrics_credentials = database_metrics_credentials;

    let uri_str = format!("{}/v2/databases/metrics/credentials", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_database_metrics_credentials);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesUpdateClusterMetricsCredentialsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To resize a database cluster, send a PUT request to `/v2/databases/$DATABASE_ID/resize`. The body of the request must specify both the size and num_nodes attributes. A successful request will receive a 202 Accepted status code with no body in response. Querying the database cluster will show that its status attribute will now be set to resizing. This will transition back to online when the resize operation has completed.
pub async fn databases_update_cluster_size(configuration: &configuration::Configuration, database_cluster_uuid: &str, database_cluster_resize: models::DatabaseClusterResize) -> Result<(), Error<DatabasesUpdateClusterSizeError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;
    let p_body_database_cluster_resize = database_cluster_resize;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/resize", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_database_cluster_resize);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesUpdateClusterSizeError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To update a connection pool for a PostgreSQL database cluster, send a PUT request to  `/v2/databases/$DATABASE_ID/pools/$POOL_NAME`.
pub async fn databases_update_connection_pool(configuration: &configuration::Configuration, database_cluster_uuid: &str, pool_name: &str, connection_pool_update: models::ConnectionPoolUpdate) -> Result<(), Error<DatabasesUpdateConnectionPoolError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;
    let p_path_pool_name = pool_name;
    let p_body_connection_pool_update = connection_pool_update;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/pools/{pool_name}", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid), pool_name=crate::apis::urlencode(p_path_pool_name));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_connection_pool_update);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesUpdateConnectionPoolError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To configure an eviction policy for an existing Caching or Valkey cluster, send a PUT request to `/v2/databases/$DATABASE_ID/eviction_policy` specifying the desired policy.
pub async fn databases_update_eviction_policy(configuration: &configuration::Configuration, database_cluster_uuid: &str, databases_update_eviction_policy_request: models::DatabasesUpdateEvictionPolicyRequest) -> Result<(), Error<DatabasesUpdateEvictionPolicyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;
    let p_body_databases_update_eviction_policy_request = databases_update_eviction_policy_request;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/eviction_policy", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_databases_update_eviction_policy_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesUpdateEvictionPolicyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To update a database cluster's firewall rules (known as \"trusted sources\" in the control panel), send a PUT request to `/v2/databases/$DATABASE_ID/firewall` specifying which resources should be able to open connections to the database. You may limit connections to specific Droplets, Kubernetes clusters, or IP addresses. When a tag is provided, any Droplet or Kubernetes node with that tag applied to it will have access. The firewall is limited to 100 rules (or trusted sources). When possible, we recommend [placing your databases into a VPC network](https://docs.digitalocean.com/products/networking/vpc/) to limit access to them instead of using a firewall. A successful
pub async fn databases_update_firewall_rules(configuration: &configuration::Configuration, database_cluster_uuid: &str, databases_update_firewall_rules_request: models::DatabasesUpdateFirewallRulesRequest) -> Result<(), Error<DatabasesUpdateFirewallRulesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;
    let p_body_databases_update_firewall_rules_request = databases_update_firewall_rules_request;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/firewall", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_databases_update_firewall_rules_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesUpdateFirewallRulesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To update the Schema Registry configuration for a Kafka cluster, send a PUT request to `/v2/databases/$DATABASE_ID/schema-registry/config`. The response is a JSON object with a `compatibility_level` key, which is set to an object containing any database configuration parameters. 
pub async fn databases_update_kafka_schema_config(configuration: &configuration::Configuration, database_cluster_uuid: &str, databases_get_kafka_schema_config200_response: Option<models::DatabasesGetKafkaSchemaConfig200Response>) -> Result<models::DatabasesGetKafkaSchemaConfig200Response, Error<DatabasesUpdateKafkaSchemaConfigError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;
    let p_body_databases_get_kafka_schema_config200_response = databases_get_kafka_schema_config200_response;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/schema-registry/config", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_databases_get_kafka_schema_config200_response);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DatabasesGetKafkaSchemaConfig200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DatabasesGetKafkaSchemaConfig200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesUpdateKafkaSchemaConfigError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To update the Schema Registry configuration for a Subject of a Kafka cluster, send a PUT request to `/v2/databases/$DATABASE_ID/schema-registry/config/$SUBJECT_NAME`. The response is a JSON object with a `compatibility_level` key, which is set to an object containing any database configuration parameters. 
pub async fn databases_update_kafka_schema_subject_config(configuration: &configuration::Configuration, database_cluster_uuid: &str, subject_name: &str, databases_get_kafka_schema_config200_response: Option<models::DatabasesGetKafkaSchemaConfig200Response>) -> Result<models::DatabasesGetKafkaSchemaSubjectConfig200Response, Error<DatabasesUpdateKafkaSchemaSubjectConfigError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;
    let p_path_subject_name = subject_name;
    let p_body_databases_get_kafka_schema_config200_response = databases_get_kafka_schema_config200_response;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/schema-registry/config/{subject_name}", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid), subject_name=crate::apis::urlencode(p_path_subject_name));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_databases_get_kafka_schema_config200_response);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DatabasesGetKafkaSchemaSubjectConfig200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DatabasesGetKafkaSchemaSubjectConfig200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesUpdateKafkaSchemaSubjectConfigError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To update a topic attached to a Kafka cluster, send a PUT request to `/v2/databases/$DATABASE_ID/topics/$TOPIC_NAME`.  The result will be a JSON object with a `topic` key. 
pub async fn databases_update_kafka_topic(configuration: &configuration::Configuration, database_cluster_uuid: &str, topic_name: &str, kafka_topic_update: Option<models::KafkaTopicUpdate>) -> Result<models::DatabasesCreateKafkaTopic201Response, Error<DatabasesUpdateKafkaTopicError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;
    let p_path_topic_name = topic_name;
    let p_body_kafka_topic_update = kafka_topic_update;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/topics/{topic_name}", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid), topic_name=crate::apis::urlencode(p_path_topic_name));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_kafka_topic_update);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DatabasesCreateKafkaTopic201Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DatabasesCreateKafkaTopic201Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesUpdateKafkaTopicError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To update a logsink for a database cluster, send a PUT request to `/v2/databases/$DATABASE_ID/logsink/$LOGSINK_ID`. 
pub async fn databases_update_logsink(configuration: &configuration::Configuration, database_cluster_uuid: &str, logsink_id: &str, logsink_update: models::LogsinkUpdate) -> Result<(), Error<DatabasesUpdateLogsinkError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;
    let p_path_logsink_id = logsink_id;
    let p_body_logsink_update = logsink_update;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/logsink/{logsink_id}", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid), logsink_id=crate::apis::urlencode(p_path_logsink_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_logsink_update);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesUpdateLogsinkError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To configure the window when automatic maintenance should be performed for a database cluster, send a PUT request to `/v2/databases/$DATABASE_ID/maintenance`. A successful request will receive a 204 No Content status code with no body in response.
pub async fn databases_update_maintenance_window(configuration: &configuration::Configuration, database_cluster_uuid: &str, database_maintenance_window: Option<models::DatabaseMaintenanceWindow>) -> Result<(), Error<DatabasesUpdateMaintenanceWindowError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;
    let p_body_database_maintenance_window = database_maintenance_window;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/maintenance", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_database_maintenance_window);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesUpdateMaintenanceWindowError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To upgrade the major version of a database, send a PUT request to `/v2/databases/$DATABASE_ID/upgrade`, specifying the target version. A successful request will receive a 204 No Content status code with no body in response.
pub async fn databases_update_major_version(configuration: &configuration::Configuration, database_cluster_uuid: &str, version2: models::Version2) -> Result<(), Error<DatabasesUpdateMajorVersionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;
    let p_body_version2 = version2;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/upgrade", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_version2);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesUpdateMajorVersionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To start an online migration, send a PUT request to `/v2/databases/$DATABASE_ID/online-migration` endpoint. Migrating a cluster establishes a connection with an existing cluster and replicates its contents to the target cluster. Online migration is only available for MySQL, PostgreSQL, Caching, and Valkey clusters. If the existing database is continuously being written to,  the migration process will continue for up to two weeks unless it is manually stopped. Online migration is only available for [MySQL](https://docs.digitalocean.com/products/databases/mysql/how-to/migrate/#:~:text=To%20migrate%20a%20MySQL%20database,then%20select%20Set%20Up%20Migration),  [PostgreSQL](https://docs.digitalocean.com/products/databases/postgresql/how-to/migrate/),  [Caching](https://docs.digitalocean.com/products/databases/redis/how-to/migrate/), and [Valkey](https://docs.digitalocean.com/products/databases/valkey/how-to/migrate/) clusters. 
pub async fn databases_update_online_migration(configuration: &configuration::Configuration, database_cluster_uuid: &str, source_database: models::SourceDatabase) -> Result<models::OnlineMigration, Error<DatabasesUpdateOnlineMigrationError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;
    let p_body_source_database = source_database;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/online-migration", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_source_database);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::OnlineMigration`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::OnlineMigration`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesUpdateOnlineMigrationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To migrate a database cluster to a new region, send a `PUT` request to `/v2/databases/$DATABASE_ID/migrate`. The body of the request must specify a `region` attribute.  A successful request will receive a 202 Accepted status code with no body in response. Querying the database cluster will show that its `status` attribute will now be set to `migrating`. This will transition back to `online` when the migration has completed. 
pub async fn databases_update_region(configuration: &configuration::Configuration, database_cluster_uuid: &str, databases_update_region_request: models::DatabasesUpdateRegionRequest) -> Result<(), Error<DatabasesUpdateRegionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;
    let p_body_databases_update_region_request = databases_update_region_request;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/migrate", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_databases_update_region_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesUpdateRegionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To configure the SQL modes for an existing MySQL cluster, send a PUT request to `/v2/databases/$DATABASE_ID/sql_mode` specifying the desired modes. See the official MySQL 8 documentation for a [full list of supported SQL modes](https://dev.mysql.com/doc/refman/8.0/en/sql-mode.html#sql-mode-full). A successful request will receive a 204 No Content status code with no body in response.
pub async fn databases_update_sql_mode(configuration: &configuration::Configuration, database_cluster_uuid: &str, sql_mode: models::SqlMode) -> Result<(), Error<DatabasesUpdateSqlModeError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;
    let p_body_sql_mode = sql_mode;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/sql_mode", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_sql_mode);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesUpdateSqlModeError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To update an existing database user, send a PUT request to `/v2/databases/$DATABASE_ID/users/$USERNAME` with the desired settings.  **Note**: only `settings` can be updated via this type of request. If you wish to change the name of a user, you must recreate a new user.  The response will be a JSON object with a key called `user`. The value of this will be an object that contains the name of the update database user, along with the `settings` object that has been updated. 
pub async fn databases_update_user(configuration: &configuration::Configuration, database_cluster_uuid: &str, username: &str, databases_update_user_request: models::DatabasesUpdateUserRequest) -> Result<models::DatabasesAddUser201Response, Error<DatabasesUpdateUserError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_database_cluster_uuid = database_cluster_uuid;
    let p_path_username = username;
    let p_body_databases_update_user_request = databases_update_user_request;

    let uri_str = format!("{}/v2/databases/{database_cluster_uuid}/users/{username}", configuration.base_path, database_cluster_uuid=crate::apis::urlencode(p_path_database_cluster_uuid), username=crate::apis::urlencode(p_path_username));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_databases_update_user_request);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DatabasesAddUser201Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DatabasesAddUser201Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DatabasesUpdateUserError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

