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
pub struct KafkaAdvancedConfig {
    /// Specify the final compression type for a given topic. This configuration accepts the standard compression codecs ('gzip', 'snappy', 'lz4', 'zstd'). It additionally accepts 'uncompressed' which is equivalent to no compression; and 'producer' which means retain the original compression codec set by the producer.
    #[serde(rename = "compression_type", skip_serializing_if = "Option::is_none")]
    pub compression_type: Option<CompressionType>,
    /// The amount of time, in milliseconds, the group coordinator will wait for more consumers to join a new group before performing the first rebalance. A longer delay means potentially fewer rebalances, but increases the time until processing begins. The default value for this is 3 seconds. During development and testing it might be desirable to set this to 0 in order to not delay test execution time.
    #[serde(rename = "group_initial_rebalance_delay_ms", skip_serializing_if = "Option::is_none")]
    pub group_initial_rebalance_delay_ms: Option<i32>,
    /// The minimum allowed session timeout for registered consumers. Longer timeouts give consumers more time to process messages in between heartbeats at the cost of a longer time to detect failures.
    #[serde(rename = "group_min_session_timeout_ms", skip_serializing_if = "Option::is_none")]
    pub group_min_session_timeout_ms: Option<i32>,
    /// The maximum allowed session timeout for registered consumers. Longer timeouts give consumers more time to process messages in between heartbeats at the cost of a longer time to detect failures.
    #[serde(rename = "group_max_session_timeout_ms", skip_serializing_if = "Option::is_none")]
    pub group_max_session_timeout_ms: Option<i32>,
    /// Idle connections timeout: the server socket processor threads close the connections that idle for longer than this.
    #[serde(rename = "connections_max_idle_ms", skip_serializing_if = "Option::is_none")]
    pub connections_max_idle_ms: Option<i32>,
    /// The maximum number of incremental fetch sessions that the broker will maintain.
    #[serde(rename = "max_incremental_fetch_session_cache_slots", skip_serializing_if = "Option::is_none")]
    pub max_incremental_fetch_session_cache_slots: Option<i32>,
    /// The maximum size of message that the server can receive.
    #[serde(rename = "message_max_bytes", skip_serializing_if = "Option::is_none")]
    pub message_max_bytes: Option<i32>,
    /// Log retention window in minutes for offsets topic
    #[serde(rename = "offsets_retention_minutes", skip_serializing_if = "Option::is_none")]
    pub offsets_retention_minutes: Option<i32>,
    /// How long are delete records retained?
    #[serde(rename = "log_cleaner_delete_retention_ms", skip_serializing_if = "Option::is_none")]
    pub log_cleaner_delete_retention_ms: Option<i32>,
    /// Controls log compactor frequency. Larger value means more frequent compactions but also more space wasted for logs. Consider setting log_cleaner_max_compaction_lag_ms to enforce compactions sooner, instead of setting a very high value for this option.
    #[serde(rename = "log_cleaner_min_cleanable_ratio", skip_serializing_if = "Option::is_none")]
    pub log_cleaner_min_cleanable_ratio: Option<f64>,
    /// The maximum amount of time message will remain uncompacted. Only applicable for logs that are being compacted
    #[serde(rename = "log_cleaner_max_compaction_lag_ms", skip_serializing_if = "Option::is_none")]
    pub log_cleaner_max_compaction_lag_ms: Option<i32>,
    /// The minimum time a message will remain uncompacted in the log. Only applicable for logs that are being compacted.
    #[serde(rename = "log_cleaner_min_compaction_lag_ms", skip_serializing_if = "Option::is_none")]
    pub log_cleaner_min_compaction_lag_ms: Option<i32>,
    /// The default cleanup policy for segments beyond the retention window
    #[serde(rename = "log_cleanup_policy", skip_serializing_if = "Option::is_none")]
    pub log_cleanup_policy: Option<LogCleanupPolicy>,
    /// The number of messages accumulated on a log partition before messages are flushed to disk
    #[serde(rename = "log_flush_interval_messages", skip_serializing_if = "Option::is_none")]
    pub log_flush_interval_messages: Option<i32>,
    /// The maximum time in ms that a message in any topic is kept in memory before flushed to disk. If not set, the value in log.flush.scheduler.interval.ms is used
    #[serde(rename = "log_flush_interval_ms", skip_serializing_if = "Option::is_none")]
    pub log_flush_interval_ms: Option<i32>,
    /// The interval with which Kafka adds an entry to the offset index
    #[serde(rename = "log_index_interval_bytes", skip_serializing_if = "Option::is_none")]
    pub log_index_interval_bytes: Option<i32>,
    /// The maximum size in bytes of the offset index
    #[serde(rename = "log_index_size_max_bytes", skip_serializing_if = "Option::is_none")]
    pub log_index_size_max_bytes: Option<i32>,
    /// This configuration controls whether down-conversion of message formats is enabled to satisfy consume requests.
    #[serde(rename = "log_message_downconversion_enable", skip_serializing_if = "Option::is_none")]
    pub log_message_downconversion_enable: Option<bool>,
    /// Define whether the timestamp in the message is message create time or log append time.
    #[serde(rename = "log_message_timestamp_type", skip_serializing_if = "Option::is_none")]
    pub log_message_timestamp_type: Option<LogMessageTimestampType>,
    /// The maximum difference allowed between the timestamp when a broker receives a message and the timestamp specified in the message
    #[serde(rename = "log_message_timestamp_difference_max_ms", skip_serializing_if = "Option::is_none")]
    pub log_message_timestamp_difference_max_ms: Option<i32>,
    /// Controls whether to preallocate a file when creating a new segment
    #[serde(rename = "log_preallocate", skip_serializing_if = "Option::is_none")]
    pub log_preallocate: Option<bool>,
    /// The maximum size of the log before deleting messages
    #[serde(rename = "log_retention_bytes", skip_serializing_if = "Option::is_none")]
    pub log_retention_bytes: Option<i32>,
    /// The number of hours to keep a log file before deleting it
    #[serde(rename = "log_retention_hours", skip_serializing_if = "Option::is_none")]
    pub log_retention_hours: Option<i32>,
    /// The number of milliseconds to keep a log file before deleting it (in milliseconds), If not set, the value in log.retention.minutes is used. If set to -1, no time limit is applied.
    #[serde(rename = "log_retention_ms", skip_serializing_if = "Option::is_none")]
    pub log_retention_ms: Option<i32>,
    /// The maximum jitter to subtract from logRollTimeMillis (in milliseconds). If not set, the value in log.roll.jitter.hours is used
    #[serde(rename = "log_roll_jitter_ms", skip_serializing_if = "Option::is_none")]
    pub log_roll_jitter_ms: Option<i32>,
    /// The maximum time before a new log segment is rolled out (in milliseconds).
    #[serde(rename = "log_roll_ms", skip_serializing_if = "Option::is_none")]
    pub log_roll_ms: Option<i32>,
    /// The maximum size of a single log file
    #[serde(rename = "log_segment_bytes", skip_serializing_if = "Option::is_none")]
    pub log_segment_bytes: Option<i32>,
    /// The amount of time to wait before deleting a file from the filesystem
    #[serde(rename = "log_segment_delete_delay_ms", skip_serializing_if = "Option::is_none")]
    pub log_segment_delete_delay_ms: Option<i32>,
    /// Enable auto creation of topics
    #[serde(rename = "auto_create_topics_enable", skip_serializing_if = "Option::is_none")]
    pub auto_create_topics_enable: Option<bool>,
    /// When a producer sets acks to 'all' (or '-1'), min_insync_replicas specifies the minimum number of replicas that must acknowledge a write for the write to be considered successful.
    #[serde(rename = "min_insync_replicas", skip_serializing_if = "Option::is_none")]
    pub min_insync_replicas: Option<i32>,
    /// Number of partitions for autocreated topics
    #[serde(rename = "num_partitions", skip_serializing_if = "Option::is_none")]
    pub num_partitions: Option<i32>,
    /// Replication factor for autocreated topics
    #[serde(rename = "default_replication_factor", skip_serializing_if = "Option::is_none")]
    pub default_replication_factor: Option<i32>,
    /// The number of bytes of messages to attempt to fetch for each partition (defaults to 1048576). This is not an absolute maximum, if the first record batch in the first non-empty partition of the fetch is larger than this value, the record batch will still be returned to ensure that progress can be made.
    #[serde(rename = "replica_fetch_max_bytes", skip_serializing_if = "Option::is_none")]
    pub replica_fetch_max_bytes: Option<i32>,
    /// Maximum bytes expected for the entire fetch response (defaults to 10485760). Records are fetched in batches, and if the first record batch in the first non-empty partition of the fetch is larger than this value, the record batch will still be returned to ensure that progress can be made. As such, this is not an absolute maximum.
    #[serde(rename = "replica_fetch_response_max_bytes", skip_serializing_if = "Option::is_none")]
    pub replica_fetch_response_max_bytes: Option<i32>,
    /// The maximum number of connections allowed from each ip address (defaults to 2147483647).
    #[serde(rename = "max_connections_per_ip", skip_serializing_if = "Option::is_none")]
    pub max_connections_per_ip: Option<i32>,
    /// The purge interval (in number of requests) of the producer request purgatory (defaults to 1000).
    #[serde(rename = "producer_purgatory_purge_interval_requests", skip_serializing_if = "Option::is_none")]
    pub producer_purgatory_purge_interval_requests: Option<i32>,
    /// The maximum number of bytes in a socket request (defaults to 104857600).
    #[serde(rename = "socket_request_max_bytes", skip_serializing_if = "Option::is_none")]
    pub socket_request_max_bytes: Option<i32>,
    /// The transaction topic segment bytes should be kept relatively small in order to facilitate faster log compaction and cache loads (defaults to 104857600 (100 mebibytes)).
    #[serde(rename = "transaction_state_log_segment_bytes", skip_serializing_if = "Option::is_none")]
    pub transaction_state_log_segment_bytes: Option<i32>,
    /// The interval at which to remove transactions that have expired due to transactional.id.expiration.ms passing (defaults to 3600000 (1 hour)).
    #[serde(rename = "transaction_remove_expired_transaction_cleanup_interval_ms", skip_serializing_if = "Option::is_none")]
    pub transaction_remove_expired_transaction_cleanup_interval_ms: Option<i32>,
    /// Enable creation of schema registry for the Kafka cluster. Schema_registry only works with General Purpose - Dedicated CPU plans.
    #[serde(rename = "schema_registry", skip_serializing_if = "Option::is_none")]
    pub schema_registry: Option<bool>,
}

impl KafkaAdvancedConfig {
    pub fn new() -> KafkaAdvancedConfig {
        KafkaAdvancedConfig {
            compression_type: None,
            group_initial_rebalance_delay_ms: None,
            group_min_session_timeout_ms: None,
            group_max_session_timeout_ms: None,
            connections_max_idle_ms: None,
            max_incremental_fetch_session_cache_slots: None,
            message_max_bytes: None,
            offsets_retention_minutes: None,
            log_cleaner_delete_retention_ms: None,
            log_cleaner_min_cleanable_ratio: None,
            log_cleaner_max_compaction_lag_ms: None,
            log_cleaner_min_compaction_lag_ms: None,
            log_cleanup_policy: None,
            log_flush_interval_messages: None,
            log_flush_interval_ms: None,
            log_index_interval_bytes: None,
            log_index_size_max_bytes: None,
            log_message_downconversion_enable: None,
            log_message_timestamp_type: None,
            log_message_timestamp_difference_max_ms: None,
            log_preallocate: None,
            log_retention_bytes: None,
            log_retention_hours: None,
            log_retention_ms: None,
            log_roll_jitter_ms: None,
            log_roll_ms: None,
            log_segment_bytes: None,
            log_segment_delete_delay_ms: None,
            auto_create_topics_enable: None,
            min_insync_replicas: None,
            num_partitions: None,
            default_replication_factor: None,
            replica_fetch_max_bytes: None,
            replica_fetch_response_max_bytes: None,
            max_connections_per_ip: None,
            producer_purgatory_purge_interval_requests: None,
            socket_request_max_bytes: None,
            transaction_state_log_segment_bytes: None,
            transaction_remove_expired_transaction_cleanup_interval_ms: None,
            schema_registry: None,
        }
    }
}
/// Specify the final compression type for a given topic. This configuration accepts the standard compression codecs ('gzip', 'snappy', 'lz4', 'zstd'). It additionally accepts 'uncompressed' which is equivalent to no compression; and 'producer' which means retain the original compression codec set by the producer.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CompressionType {
    #[serde(rename = "gzip")]
    Gzip,
    #[serde(rename = "snappy")]
    Snappy,
    #[serde(rename = "lz4")]
    Lz4,
    #[serde(rename = "zstd")]
    Zstd,
    #[serde(rename = "uncompressed")]
    Uncompressed,
    #[serde(rename = "producer")]
    Producer,
}

impl Default for CompressionType {
    fn default() -> CompressionType {
        Self::Gzip
    }
}
/// The default cleanup policy for segments beyond the retention window
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LogCleanupPolicy {
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "compact")]
    Compact,
    #[serde(rename = "compact,delete")]
    CompactCommaDelete,
}

impl Default for LogCleanupPolicy {
    fn default() -> LogCleanupPolicy {
        Self::Delete
    }
}
/// Define whether the timestamp in the message is message create time or log append time.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LogMessageTimestampType {
    #[serde(rename = "CreateTime")]
    CreateTime,
    #[serde(rename = "LogAppendTime")]
    LogAppendTime,
}

impl Default for LogMessageTimestampType {
    fn default() -> LogMessageTimestampType {
        Self::CreateTime
    }
}

