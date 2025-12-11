/*
 * DigitalOcean API
 *
 * # Introduction  The DigitalOcean API allows you to manage Droplets and resources within the DigitalOcean cloud in a simple, programmatic way using conventional HTTP requests.  All of the functionality that you are familiar with in the DigitalOcean control panel is also available through the API, allowing you to script the complex actions that your situation requires.  The API documentation will start with a general overview about the design and technology that has been implemented, followed by reference information about specific endpoints.  ## Requests  Any tool that is fluent in HTTP can communicate with the API simply by requesting the correct URI. Requests should be made using the HTTPS protocol so that traffic is encrypted. The interface responds to different methods depending on the action required.  |Method|Usage| |--- |--- | |GET|For simple retrieval of information about your account, Droplets, or environment, you should use the GET method.  The information you request will be returned to you as a JSON object. The attributes defined by the JSON object can be used to form additional requests.  Any request using the GET method is read-only and will not affect any of the objects you are querying.| |DELETE|To destroy a resource and remove it from your account and environment, the DELETE method should be used.  This will remove the specified object if it is found.  If it is not found, the operation will return a response indicating that the object was not found. This idempotency means that you do not have to check for a resource's availability prior to issuing a delete command, the final state will be the same regardless of its existence.| |PUT|To update the information about a resource in your account, the PUT method is available. Like the DELETE Method, the PUT method is idempotent.  It sets the state of the target using the provided values, regardless of their current values. Requests using the PUT method do not need to check the current attributes of the object.| |PATCH|Some resources support partial modification. In these cases, the PATCH method is available. Unlike PUT which generally requires a complete representation of a resource, a PATCH request is a set of instructions on how to modify a resource updating only specific attributes.| |POST|To create a new object, your request should specify the POST method. The POST request includes all of the attributes necessary to create a new object.  When you wish to create a new object, send a POST request to the target endpoint.| |HEAD|Finally, to retrieve metadata information, you should use the HEAD method to get the headers.  This returns only the header of what would be returned with an associated GET request. Response headers contain some useful information about your API access and the results that are available for your request. For instance, the headers contain your current rate-limit value and the amount of time available until the limit resets. It also contains metrics about the total number of objects found, pagination information, and the total content length.|   ## HTTP Statuses  Along with the HTTP methods that the API responds to, it will also return standard HTTP statuses, including error codes.  In the event of a problem, the status will contain the error code, while the body of the response will usually contain additional information about the problem that was encountered.  In general, if the status returned is in the 200 range, it indicates that the request was fulfilled successfully and that no error was encountered.  Return codes in the 400 range typically indicate that there was an issue with the request that was sent. Among other things, this could mean that you did not authenticate correctly, that you are requesting an action that you do not have authorization for, that the object you are requesting does not exist, or that your request is malformed.  If you receive a status in the 500 range, this generally indicates a server-side problem. This means that we are having an issue on our end and cannot fulfill your request currently.  400 and 500 level error responses will include a JSON object in their body, including the following attributes:  |Name|Type|Description| |--- |--- |--- | |id|string|A short identifier corresponding to the HTTP status code returned. For example, the ID for a response returning a 404 status code would be \"not_found.\"| |message|string|A message providing additional information about the error, including details to help resolve it when possible.| |request_id|string|Optionally, some endpoints may include a request ID that should be provided when reporting bugs or opening support tickets to help identify the issue.|  ### Example Error Response  ```     HTTP/1.1 403 Forbidden     {       \"id\":       \"forbidden\",       \"message\":  \"You do not have access for the attempted action.\"     } ```  ## Responses  When a request is successful, a response body will typically be sent back in the form of a JSON object. An exception to this is when a DELETE request is processed, which will result in a successful HTTP 204 status and an empty response body.  Inside of this JSON object, the resource root that was the target of the request will be set as the key. This will be the singular form of the word if the request operated on a single object, and the plural form of the word if a collection was processed.  For example, if you send a GET request to `/v2/droplets/$DROPLET_ID` you will get back an object with a key called \"`droplet`\". However, if you send the GET request to the general collection at `/v2/droplets`, you will get back an object with a key called \"`droplets`\".  The value of these keys will generally be a JSON object for a request on a single object and an array of objects for a request on a collection of objects.  ### Response for a Single Object  ```json     {         \"droplet\": {             \"name\": \"example.com\"             . . .         }     } ```  ### Response for an Object Collection  ```json     {         \"droplets\": [             {                 \"name\": \"example.com\"                 . . .             },             {                 \"name\": \"second.com\"                 . . .             }         ]     } ```  ## Meta  In addition to the main resource root, the response may also contain a `meta` object. This object contains information about the response itself.  The `meta` object contains a `total` key that is set to the total number of objects returned by the request. This has implications on the `links` object and pagination.  The `meta` object will only be displayed when it has a value. Currently, the `meta` object will have a value when a request is made on a collection (like `droplets` or `domains`).   ### Sample Meta Object  ```json     {         . . .         \"meta\": {             \"total\": 43         }         . . .     } ```  ## Links & Pagination  The `links` object is returned as part of the response body when pagination is enabled. By default, 20 objects are returned per page. If the response contains 20 objects or fewer, no `links` object will be returned. If the response contains more than 20 objects, the first 20 will be returned along with the `links` object.  You can request a different pagination limit or force pagination by appending `?per_page=` to the request with the number of items you would like per page. For instance, to show only two results per page, you could add `?per_page=2` to the end of your query. The maximum number of results per page is 200.  The `links` object contains a `pages` object. The `pages` object, in turn, contains keys indicating the relationship of additional pages. The values of these are the URLs of the associated pages. The keys will be one of the following:  *   **first**: The URI of the first page of results. *   **prev**: The URI of the previous sequential page of results. *   **next**: The URI of the next sequential page of results. *   **last**: The URI of the last page of results.  The `pages` object will only include the links that make sense. So for the first page of results, no `first` or `prev` links will ever be set. This convention holds true in other situations where a link would not make sense.  ### Sample Links Object  ```json     {         . . .         \"links\": {             \"pages\": {                 \"last\": \"https://api.digitalocean.com/v2/images?page=2\",                 \"next\": \"https://api.digitalocean.com/v2/images?page=2\"             }         }         . . .     } ```  ## Rate Limit  Requests through the API are rate limited per OAuth token. Current rate limits:  *   5,000 requests per hour *   250 requests per minute (5% of the hourly total)  Once you exceed either limit, you will be rate limited until the next cycle starts. Space out any requests that you would otherwise issue in bursts for the best results.  The rate limiting information is contained within the response headers of each request. The relevant headers are:  *   **ratelimit-limit**: The number of requests that can be made per hour. *   **ratelimit-remaining**: The number of requests that remain before you hit your request limit. See the information below for how the request limits expire. *   **ratelimit-reset**: This represents the time when the oldest request will expire. The value is given in [Unix epoch time](http://en.wikipedia.org/wiki/Unix_time). See below for more information about how request limits expire.  More rate limiting information is returned only within burst limit error response headers: *   **retry-after**: The number of seconds to wait before making another request when rate limited.  As long as the `ratelimit-remaining` count is above zero, you will be able to make additional requests.  The way that a request expires and is removed from the current limit count is important to understand. Rather than counting all of the requests for an hour and resetting the `ratelimit-remaining` value at the end of the hour, each request instead has its own timer.  This means that each request contributes toward the `ratelimit-remaining` count for one complete hour after the request is made. When that request's timer runs out, it is no longer counted towards the request limit.  This has implications on the meaning of the `ratelimit-reset` header as well. Because the entire rate limit is not reset at one time, the value of this header is set to the time when the _oldest_ request will expire.  Keep this in mind if you see your `ratelimit-reset` value change, but not move an entire hour into the future.  If the `ratelimit-remaining` reaches zero, subsequent requests will receive a 429 error code until the request reset has been reached.   `ratelimit-remaining` reaching zero can also indicate that the \"burst limit\" of 250  requests per minute limit was met, even if the 5,000 requests per hour limit was not.  In this case, the 429 error response will include a `retry-after` header to indicate how  long to wait (in seconds) until the request may be retried.  You can see the format of the response in the examples.   **Note:** The following endpoints have special rate limit requirements that are independent of the limits defined above.  *   Only 10 `GET` requests to the `/v2/account/keys` endpoint to list SSH keys can be made per 60 seconds. *   Only 5 requests to any and all `v2/cdn/endpoints` can be made per 10 seconds. This includes `v2/cdn/endpoints`,      `v2/cdn/endpoints/$ENDPOINT_ID`, and `v2/cdn/endpoints/$ENDPOINT_ID/cache`. *   Only 50 strings within the `files` json struct in the `v2/cdn/endpoints/$ENDPOINT_ID/cache` [payload](https://docs.digitalocean.com/reference/api/api-reference/#operation/cdn_purge_cache)      can be requested every 20 seconds.  ### Sample Rate Limit Headers  ```     . . .     ratelimit-limit: 1200     ratelimit-remaining: 1193     rateLimit-reset: 1402425459     . . . ```    ### Sample Rate Limit Headers When Burst Limit is Reached:  ```     . . .     ratelimit-limit: 5000     ratelimit-remaining: 0     rateLimit-reset: 1402425459     retry-after: 29     . . . ```  ### Sample Rate Exceeded Response  ```     429 Too Many Requests     {             id: \"too_many_requests\",             message: \"API Rate limit exceeded.\"     } ```  ## Curl Examples  Throughout this document, some example API requests will be given using the `curl` command. This will allow us to demonstrate the various endpoints in a simple, textual format.      These examples assume that you are using a Linux or macOS command line. To run these commands on a Windows machine, you can either use cmd.exe, PowerShell, or WSL:  * For cmd.exe, use the `set VAR=VALUE` [syntax](https://docs.microsoft.com/en-us/windows-server/administration/windows-commands/set_1) to define environment variables, call them with `%VAR%`, then replace all backslashes (`\\`) in the examples with carets (`^`).  * For PowerShell, use the `$Env:VAR = \"VALUE\"` [syntax](https://docs.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_environment_variables?view=powershell-7.2) to define environment variables, call them with `$Env:VAR`, then replace `curl` with `curl.exe` and all backslashes (`\\`) in the examples with backticks (`` ` ``).  * WSL is a compatibility layer that allows you to emulate a Linux terminal on a Windows machine. Install WSL with our [community tutorial](https://www.digitalocean.com/community/tutorials/how-to-install-the-windows-subsystem-for-linux-2-on-microsoft-windows-10),  then follow this API documentation normally.  The names of account-specific references (like Droplet IDs, for instance) will be represented by variables. For instance, a Droplet ID may be represented by a variable called `$DROPLET_ID`. You can set the associated variables in your environment if you wish to use the examples without modification.  The first variable that you should set to get started is your OAuth authorization token. The next section will go over the details of this, but you can set an environmental variable for it now.  Generate a token by going to the [Apps & API](https://cloud.digitalocean.com/settings/applications) section of the DigitalOcean control panel. Use an existing token if you have saved one, or generate a new token with the \"Generate new token\" button. Copy the generated token and use it to set and export the TOKEN variable in your environment as the example shows.  You may also wish to set some other variables now or as you go along. For example, you may wish to set the `DROPLET_ID` variable to one of your Droplet IDs since this will be used frequently in the API.  If you are following along, make sure you use a Droplet ID that you control so that your commands will execute correctly.  If you need access to the headers of a response through `curl`, you can pass the `-i` flag to display the header information along with the body. If you are only interested in the header, you can instead pass the `-I` flag, which will exclude the response body entirely.   ### Set and Export your OAuth Token  ``` export DIGITALOCEAN_TOKEN=your_token_here ```  ### Set and Export a Variable  ``` export DROPLET_ID=1111111 ```  ## Parameters  There are two different ways to pass parameters in a request with the API.  When passing parameters to create or update an object, parameters should be passed as a JSON object containing the appropriate attribute names and values as key-value pairs. When you use this format, you should specify that you are sending a JSON object in the header. This is done by setting the `Content-Type` header to `application/json`. This ensures that your request is interpreted correctly.  When passing parameters to filter a response on GET requests, parameters can be passed using standard query attributes. In this case, the parameters would be embedded into the URI itself by appending a `?` to the end of the URI and then setting each attribute with an equal sign. Attributes can be separated with a `&`. Tools like `curl` can create the appropriate URI when given parameters and values; this can also be done using the `-F` flag and then passing the key and value as an argument. The argument should take the form of a quoted string with the attribute being set to a value with an equal sign.  ### Pass Parameters as a JSON Object  ```     curl -H \"Authorization: Bearer $DIGITALOCEAN_TOKEN\" \\         -H \"Content-Type: application/json\" \\         -d '{\"name\": \"example.com\", \"ip_address\": \"127.0.0.1\"}' \\         -X POST \"https://api.digitalocean.com/v2/domains\" ```  ### Pass Filter Parameters as a Query String  ```      curl -H \"Authorization: Bearer $DIGITALOCEAN_TOKEN\" \\          -X GET \\          \"https://api.digitalocean.com/v2/images?private=true\" ```  ## Cross Origin Resource Sharing  In order to make requests to the API from other domains, the API implements Cross Origin Resource Sharing (CORS) support.  CORS support is generally used to create AJAX requests outside of the domain that the request originated from. This is necessary to implement projects like control panels utilizing the API. This tells the browser that it can send requests to an outside domain.  The procedure that the browser initiates in order to perform these actions (other than GET requests) begins by sending a \"preflight\" request. This sets the `Origin` header and uses the `OPTIONS` method. The server will reply back with the methods it allows and some of the limits it imposes. The client then sends the actual request if it falls within the allowed constraints.  This process is usually done in the background by the browser, but you can use curl to emulate this process using the example provided. The headers that will be set to show the constraints are:  *   **Access-Control-Allow-Origin**: This is the domain that is sent by the client or browser as the origin of the request. It is set through an `Origin` header. *   **Access-Control-Allow-Methods**: This specifies the allowed options for requests from that domain. This will generally be all available methods. *   **Access-Control-Expose-Headers**: This will contain the headers that will be available to requests from the origin domain. *   **Access-Control-Max-Age**: This is the length of time that the access is considered valid. After this expires, a new preflight should be sent. *   **Access-Control-Allow-Credentials**: This will be set to `true`. It basically allows you to send your OAuth token for authentication.  You should not need to be concerned with the details of these headers, because the browser will typically do all of the work for you.
 *
 * The version of the OpenAPI document: 2.0
 * Contact: api-engineering@digitalocean.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OpensearchAdvancedConfig {
    /// Maximum content length for HTTP requests to the OpenSearch HTTP API, in bytes.
    #[serde(
        rename = "http_max_content_length_bytes",
        skip_serializing_if = "Option::is_none"
    )]
    pub http_max_content_length_bytes: Option<i32>,
    /// Maximum size of allowed headers, in bytes.
    #[serde(
        rename = "http_max_header_size_bytes",
        skip_serializing_if = "Option::is_none"
    )]
    pub http_max_header_size_bytes: Option<i32>,
    /// Maximum length of an HTTP URL, in bytes.
    #[serde(
        rename = "http_max_initial_line_length_bytes",
        skip_serializing_if = "Option::is_none"
    )]
    pub http_max_initial_line_length_bytes: Option<i32>,
    /// Maximum number of clauses Lucene BooleanQuery can have.  Only increase it if necessary, as it may cause performance issues.
    #[serde(
        rename = "indices_query_bool_max_clause_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub indices_query_bool_max_clause_count: Option<i32>,
    /// Maximum amount of heap memory used for field data cache, expressed as a percentage. Decreasing the value too much will increase overhead of loading field data. Increasing the value too much will decrease amount of heap available for other operations.
    #[serde(
        rename = "indices_fielddata_cache_size_percentage",
        skip_serializing_if = "Option::is_none"
    )]
    pub indices_fielddata_cache_size_percentage: Option<i32>,
    /// Total amount of heap used for indexing buffer before writing segments to disk, expressed as a percentage. Too low value will slow down indexing; too high value will increase indexing performance but causes performance issues for query performance.
    #[serde(
        rename = "indices_memory_index_buffer_size_percentage",
        skip_serializing_if = "Option::is_none"
    )]
    pub indices_memory_index_buffer_size_percentage: Option<i32>,
    /// Minimum amount of heap used for indexing buffer before writing segments to disk, in mb. Works in conjunction with indices_memory_index_buffer_size_percentage, each being enforced.
    #[serde(
        rename = "indices_memory_min_index_buffer_size_mb",
        skip_serializing_if = "Option::is_none"
    )]
    pub indices_memory_min_index_buffer_size_mb: Option<i32>,
    /// Maximum amount of heap used for indexing buffer before writing segments to disk, in mb. Works in conjunction with indices_memory_index_buffer_size_percentage, each being enforced. The default is unbounded.
    #[serde(
        rename = "indices_memory_max_index_buffer_size_mb",
        skip_serializing_if = "Option::is_none"
    )]
    pub indices_memory_max_index_buffer_size_mb: Option<i32>,
    /// Maximum amount of heap used for query cache.  Too low value will decrease query performance and increase performance for other operations; too high value will cause issues with other functionality.
    #[serde(
        rename = "indices_queries_cache_size_percentage",
        skip_serializing_if = "Option::is_none"
    )]
    pub indices_queries_cache_size_percentage: Option<i32>,
    /// Limits total inbound and outbound recovery traffic for each node, expressed in mb per second. Applies to both peer recoveries as well as snapshot recoveries (i.e., restores from a snapshot).
    #[serde(
        rename = "indices_recovery_max_mb_per_sec",
        skip_serializing_if = "Option::is_none"
    )]
    pub indices_recovery_max_mb_per_sec: Option<i32>,
    /// Maximum number of file chunks sent in parallel for each recovery.
    #[serde(
        rename = "indices_recovery_max_concurrent_file_chunks",
        skip_serializing_if = "Option::is_none"
    )]
    pub indices_recovery_max_concurrent_file_chunks: Option<i32>,
    /// Number of workers in the search operation thread pool.  Do note this may have maximum value depending on CPU count - value is automatically lowered if set to higher than maximum value.
    #[serde(
        rename = "thread_pool_search_size",
        skip_serializing_if = "Option::is_none"
    )]
    pub thread_pool_search_size: Option<i32>,
    /// Number of workers in the search throttled operation thread pool. This pool is used for searching frozen indices. Do note this may have maximum value depending on CPU count - value is automatically lowered if set to higher than maximum value.
    #[serde(
        rename = "thread_pool_search_throttled_size",
        skip_serializing_if = "Option::is_none"
    )]
    pub thread_pool_search_throttled_size: Option<i32>,
    /// Number of workers in the get operation thread pool.  Do note this may have maximum value depending on CPU count - value is automatically lowered if set to higher than maximum value.
    #[serde(
        rename = "thread_pool_get_size",
        skip_serializing_if = "Option::is_none"
    )]
    pub thread_pool_get_size: Option<i32>,
    /// Number of workers in the analyze operation thread pool.  Do note this may have maximum value depending on CPU count - value is automatically lowered if set to higher than maximum value.
    #[serde(
        rename = "thread_pool_analyze_size",
        skip_serializing_if = "Option::is_none"
    )]
    pub thread_pool_analyze_size: Option<i32>,
    /// Number of workers in the write operation thread pool.  Do note this may have maximum value depending on CPU count - value is automatically lowered if set to higher than maximum value.
    #[serde(
        rename = "thread_pool_write_size",
        skip_serializing_if = "Option::is_none"
    )]
    pub thread_pool_write_size: Option<i32>,
    /// Number of workers in the force merge operation thread pool. This pool is used for forcing a merge between shards of one or more indices. Do note this may have maximum value depending on CPU count - value is automatically lowered if set to higher than maximum value.
    #[serde(
        rename = "thread_pool_force_merge_size",
        skip_serializing_if = "Option::is_none"
    )]
    pub thread_pool_force_merge_size: Option<i32>,
    /// Size of queue for operations in the search thread pool.
    #[serde(
        rename = "thread_pool_search_queue_size",
        skip_serializing_if = "Option::is_none"
    )]
    pub thread_pool_search_queue_size: Option<i32>,
    /// Size of queue for operations in the search throttled thread pool.
    #[serde(
        rename = "thread_pool_search_throttled_queue_size",
        skip_serializing_if = "Option::is_none"
    )]
    pub thread_pool_search_throttled_queue_size: Option<i32>,
    /// Size of queue for operations in the get thread pool.
    #[serde(
        rename = "thread_pool_get_queue_size",
        skip_serializing_if = "Option::is_none"
    )]
    pub thread_pool_get_queue_size: Option<i32>,
    /// Size of queue for operations in the analyze thread pool.
    #[serde(
        rename = "thread_pool_analyze_queue_size",
        skip_serializing_if = "Option::is_none"
    )]
    pub thread_pool_analyze_queue_size: Option<i32>,
    /// Size of queue for operations in the write thread pool.
    #[serde(
        rename = "thread_pool_write_queue_size",
        skip_serializing_if = "Option::is_none"
    )]
    pub thread_pool_write_queue_size: Option<i32>,
    /// Specifies whether ISM is enabled or not.
    #[serde(rename = "ism_enabled", skip_serializing_if = "Option::is_none")]
    pub ism_enabled: Option<bool>,
    /// Specifies whether audit history is enabled or not. The logs from ISM are automatically indexed to a logs document.
    #[serde(
        rename = "ism_history_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub ism_history_enabled: Option<bool>,
    /// Maximum age before rolling over the audit history index, in hours.
    #[serde(
        rename = "ism_history_max_age_hours",
        skip_serializing_if = "Option::is_none"
    )]
    pub ism_history_max_age_hours: Option<i32>,
    /// Maximum number of documents before rolling over the audit history index.
    #[serde(
        rename = "ism_history_max_docs",
        skip_serializing_if = "Option::is_none"
    )]
    pub ism_history_max_docs: Option<i32>,
    /// The time between rollover checks for the audit history index, in hours.
    #[serde(
        rename = "ism_history_rollover_check_period_hours",
        skip_serializing_if = "Option::is_none"
    )]
    pub ism_history_rollover_check_period_hours: Option<i32>,
    /// Length of time long audit history indices are kept, in days.
    #[serde(
        rename = "ism_history_rollover_retention_period_days",
        skip_serializing_if = "Option::is_none"
    )]
    pub ism_history_rollover_retention_period_days: Option<i32>,
    /// Maximum number of aggregation buckets allowed in a single response.
    #[serde(rename = "search_max_buckets", skip_serializing_if = "Option::is_none")]
    pub search_max_buckets: Option<i32>,
    /// Specifices whether to allow automatic creation of indices.
    #[serde(
        rename = "action_auto_create_index_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub action_auto_create_index_enabled: Option<bool>,
    /// Specifies whether to allow security audit logging.
    #[serde(
        rename = "enable_security_audit",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_security_audit: Option<bool>,
    /// Specifies whether to require explicit index names when deleting indices.
    #[serde(
        rename = "action_destructive_requires_name",
        skip_serializing_if = "Option::is_none"
    )]
    pub action_destructive_requires_name: Option<bool>,
    /// Maximum number of shards allowed per data node.
    #[serde(
        rename = "cluster_max_shards_per_node",
        skip_serializing_if = "Option::is_none"
    )]
    pub cluster_max_shards_per_node: Option<i32>,
    /// Compatibility mode sets OpenSearch to report its version as 7.10 so clients continue to work.
    #[serde(
        rename = "override_main_response_version",
        skip_serializing_if = "Option::is_none"
    )]
    pub override_main_response_version: Option<bool>,
    /// Limits the number of inline script compilations within a period of time. Default is use-context
    #[serde(
        rename = "script_max_compilations_rate",
        skip_serializing_if = "Option::is_none"
    )]
    pub script_max_compilations_rate: Option<String>,
    /// Maximum concurrent incoming/outgoing shard recoveries (normally replicas) are allowed to happen per node .
    #[serde(
        rename = "cluster_routing_allocation_node_concurrent_recoveries",
        skip_serializing_if = "Option::is_none"
    )]
    pub cluster_routing_allocation_node_concurrent_recoveries: Option<i32>,
    /// Allowlist of remote IP addresses for reindexing. Changing this value will cause all OpenSearch instances to restart.
    #[serde(
        rename = "reindex_remote_whitelist",
        skip_serializing_if = "Option::is_none"
    )]
    pub reindex_remote_whitelist: Option<Vec<String>>,
    /// Enable or disable filtering of alerting by backend roles.
    #[serde(
        rename = "plugins_alerting_filter_by_backend_roles_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub plugins_alerting_filter_by_backend_roles_enabled: Option<bool>,
    /// Enable or disable KNN memory circuit breaker.
    #[serde(
        rename = "knn_memory_circuit_breaker_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub knn_memory_circuit_breaker_enabled: Option<bool>,
    /// Maximum amount of memory in percentage that can be used for the KNN index. Defaults to 50% of the JVM heap size.  0 is used to set it to null which can be used to invalidate caches.
    #[serde(
        rename = "knn_memory_circuit_breaker_limit",
        skip_serializing_if = "Option::is_none"
    )]
    pub knn_memory_circuit_breaker_limit: Option<i32>,
    /// DigitalOcean automatically resets the `index.refresh_interval` to the default value (once per second) to  ensure that new documents are quickly available for search queries. If you are setting your own refresh intervals,  you can disable this by setting this field to true.
    #[serde(
        rename = "keep_index_refresh_interval",
        skip_serializing_if = "Option::is_none"
    )]
    pub keep_index_refresh_interval: Option<bool>,
}

impl OpensearchAdvancedConfig {
    pub fn new() -> OpensearchAdvancedConfig {
        OpensearchAdvancedConfig {
            http_max_content_length_bytes: None,
            http_max_header_size_bytes: None,
            http_max_initial_line_length_bytes: None,
            indices_query_bool_max_clause_count: None,
            indices_fielddata_cache_size_percentage: None,
            indices_memory_index_buffer_size_percentage: None,
            indices_memory_min_index_buffer_size_mb: None,
            indices_memory_max_index_buffer_size_mb: None,
            indices_queries_cache_size_percentage: None,
            indices_recovery_max_mb_per_sec: None,
            indices_recovery_max_concurrent_file_chunks: None,
            thread_pool_search_size: None,
            thread_pool_search_throttled_size: None,
            thread_pool_get_size: None,
            thread_pool_analyze_size: None,
            thread_pool_write_size: None,
            thread_pool_force_merge_size: None,
            thread_pool_search_queue_size: None,
            thread_pool_search_throttled_queue_size: None,
            thread_pool_get_queue_size: None,
            thread_pool_analyze_queue_size: None,
            thread_pool_write_queue_size: None,
            ism_enabled: None,
            ism_history_enabled: None,
            ism_history_max_age_hours: None,
            ism_history_max_docs: None,
            ism_history_rollover_check_period_hours: None,
            ism_history_rollover_retention_period_days: None,
            search_max_buckets: None,
            action_auto_create_index_enabled: None,
            enable_security_audit: None,
            action_destructive_requires_name: None,
            cluster_max_shards_per_node: None,
            override_main_response_version: None,
            script_max_compilations_rate: None,
            cluster_routing_allocation_node_concurrent_recoveries: None,
            reindex_remote_whitelist: None,
            plugins_alerting_filter_by_backend_roles_enabled: None,
            knn_memory_circuit_breaker_enabled: None,
            knn_memory_circuit_breaker_limit: None,
            keep_index_refresh_interval: None,
        }
    }
}
