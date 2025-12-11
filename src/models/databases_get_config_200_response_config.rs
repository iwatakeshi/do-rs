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
pub struct DatabasesGetConfig200ResponseConfig {
    /// The hour of day (in UTC) when backup for the service starts. New backup only starts if previous backup has already completed.
    #[serde(rename = "backup_hour", skip_serializing_if = "Option::is_none")]
    pub backup_hour: Option<i32>,
    /// The minute of the backup hour when backup for the service starts. New backup is only started if previous backup has already completed.
    #[serde(rename = "backup_minute", skip_serializing_if = "Option::is_none")]
    pub backup_minute: Option<i32>,
    /// Global SQL mode. If empty, uses MySQL server defaults. Must only include uppercase alphabetic characters, underscores, and commas.
    #[serde(rename = "sql_mode", skip_serializing_if = "Option::is_none")]
    pub sql_mode: Option<String>,
    /// The number of seconds that the mysqld server waits for a connect packet before responding with bad handshake.
    #[serde(rename = "connect_timeout", skip_serializing_if = "Option::is_none")]
    pub connect_timeout: Option<i32>,
    /// Default server time zone, in the form of an offset from UTC (from -12:00 to +12:00), a time zone name (EST), or 'SYSTEM' to use the MySQL server default.
    #[serde(rename = "default_time_zone", skip_serializing_if = "Option::is_none")]
    pub default_time_zone: Option<String>,
    /// The maximum permitted result length, in bytes, for the GROUP_CONCAT() function.
    #[serde(
        rename = "group_concat_max_len",
        skip_serializing_if = "Option::is_none"
    )]
    pub group_concat_max_len: Option<i32>,
    /// The time, in seconds, before cached statistics expire.
    #[serde(
        rename = "information_schema_stats_expiry",
        skip_serializing_if = "Option::is_none"
    )]
    pub information_schema_stats_expiry: Option<i32>,
    /// The minimum length of words that an InnoDB FULLTEXT index stores.
    #[serde(
        rename = "innodb_ft_min_token_size",
        skip_serializing_if = "Option::is_none"
    )]
    pub innodb_ft_min_token_size: Option<i32>,
    /// The InnoDB FULLTEXT index stopword list for all InnoDB tables.
    #[serde(
        rename = "innodb_ft_server_stopword_table",
        skip_serializing_if = "Option::is_none"
    )]
    pub innodb_ft_server_stopword_table: Option<String>,
    /// The time, in seconds, that an InnoDB transaction waits for a row lock. before giving up.
    #[serde(
        rename = "innodb_lock_wait_timeout",
        skip_serializing_if = "Option::is_none"
    )]
    pub innodb_lock_wait_timeout: Option<i32>,
    /// The size of the buffer, in bytes, that InnoDB uses to write to the log files. on disk.
    #[serde(
        rename = "innodb_log_buffer_size",
        skip_serializing_if = "Option::is_none"
    )]
    pub innodb_log_buffer_size: Option<i32>,
    /// The upper limit, in bytes, of the size of the temporary log files used during online DDL operations for InnoDB tables.
    #[serde(
        rename = "innodb_online_alter_log_max_size",
        skip_serializing_if = "Option::is_none"
    )]
    pub innodb_online_alter_log_max_size: Option<i32>,
    /// When enabled, records information about all deadlocks in InnoDB user transactions  in the error log. Disabled by default.
    #[serde(
        rename = "innodb_print_all_deadlocks",
        skip_serializing_if = "Option::is_none"
    )]
    pub innodb_print_all_deadlocks: Option<bool>,
    /// When enabled, transaction timeouts cause InnoDB to abort and roll back the entire transaction.
    #[serde(
        rename = "innodb_rollback_on_timeout",
        skip_serializing_if = "Option::is_none"
    )]
    pub innodb_rollback_on_timeout: Option<bool>,
    /// The time, in seconds, the server waits for activity on an interactive. connection before closing it.
    #[serde(
        rename = "interactive_timeout",
        skip_serializing_if = "Option::is_none"
    )]
    pub interactive_timeout: Option<i32>,
    /// The storage engine for in-memory internal temporary tables.
    #[serde(
        rename = "internal_tmp_mem_storage_engine",
        skip_serializing_if = "Option::is_none"
    )]
    pub internal_tmp_mem_storage_engine: Option<InternalTmpMemStorageEngine>,
    /// The time, in seconds, to wait for more data from an existing connection. aborting the read.
    #[serde(rename = "net_read_timeout", skip_serializing_if = "Option::is_none")]
    pub net_read_timeout: Option<i32>,
    /// The number of seconds to wait for a block to be written to a connection before aborting the write.
    #[serde(rename = "net_write_timeout", skip_serializing_if = "Option::is_none")]
    pub net_write_timeout: Option<i32>,
    /// Require primary key to be defined for new tables or old tables modified with ALTER TABLE and fail if missing. It is recommended to always have primary keys because various functionality may break if any large table is missing them.
    #[serde(
        rename = "sql_require_primary_key",
        skip_serializing_if = "Option::is_none"
    )]
    pub sql_require_primary_key: Option<bool>,
    /// The number of seconds the server waits for activity on a noninteractive connection before closing it.
    #[serde(rename = "wait_timeout", skip_serializing_if = "Option::is_none")]
    pub wait_timeout: Option<i32>,
    /// The size of the largest message, in bytes, that can be received by the server. Default is 67108864 (64M).
    #[serde(rename = "max_allowed_packet", skip_serializing_if = "Option::is_none")]
    pub max_allowed_packet: Option<i32>,
    /// The maximum size, in bytes, of internal in-memory tables. Also set tmp_table_size. Default is 16777216 (16M)
    #[serde(
        rename = "max_heap_table_size",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_heap_table_size: Option<i32>,
    /// The sort buffer size, in bytes, for ORDER BY optimization. Default is 262144. (256K).
    #[serde(rename = "sort_buffer_size", skip_serializing_if = "Option::is_none")]
    pub sort_buffer_size: Option<i32>,
    /// The maximum size, in bytes, of internal in-memory tables. Also set max_heap_table_size. Default is 16777216 (16M).
    #[serde(rename = "tmp_table_size", skip_serializing_if = "Option::is_none")]
    pub tmp_table_size: Option<i32>,
    /// When enabled, captures slow queries. When disabled, also truncates the mysql.slow_log table. Default is false.
    #[serde(rename = "slow_query_log", skip_serializing_if = "Option::is_none")]
    pub slow_query_log: Option<bool>,
    /// The time, in seconds, for a query to take to execute before  being captured by slow_query_logs. Default is 10 seconds.
    #[serde(rename = "long_query_time", skip_serializing_if = "Option::is_none")]
    pub long_query_time: Option<f64>,
    /// The minimum amount of time, in seconds, to keep binlog entries before deletion.  This may be extended for services that require binlog entries for longer than the default, for example if using the MySQL Debezium Kafka connector.
    #[serde(
        rename = "binlog_retention_period",
        skip_serializing_if = "Option::is_none"
    )]
    pub binlog_retention_period: Option<f64>,
    /// Specifies the maximum size of the InnoDB change buffer as a percentage of the buffer pool.
    #[serde(
        rename = "innodb_change_buffer_max_size",
        skip_serializing_if = "Option::is_none"
    )]
    pub innodb_change_buffer_max_size: Option<i32>,
    /// Specifies whether flushing a page from the InnoDB buffer pool also flushes other dirty pages in the same extent.   - 0 &mdash; disables this functionality, dirty pages in the same extent are not flushed.   - 1 &mdash; flushes contiguous dirty pages in the same extent.   - 2 &mdash; flushes dirty pages in the same extent.
    #[serde(
        rename = "innodb_flush_neighbors",
        skip_serializing_if = "Option::is_none"
    )]
    pub innodb_flush_neighbors: Option<InnodbFlushNeighbors>,
    /// The number of I/O threads for read operations in InnoDB. Changing this parameter will lead to a restart of the MySQL service.
    #[serde(
        rename = "innodb_read_io_threads",
        skip_serializing_if = "Option::is_none"
    )]
    pub innodb_read_io_threads: Option<i32>,
    /// The number of I/O threads for write operations in InnoDB. Changing this parameter will lead to a restart of the MySQL service.
    #[serde(
        rename = "innodb_write_io_threads",
        skip_serializing_if = "Option::is_none"
    )]
    pub innodb_write_io_threads: Option<i32>,
    /// Defines the maximum number of threads permitted inside of InnoDB. A value of 0 (the default) is interpreted as infinite concurrency (no limit). This variable is intended for performance  tuning on high concurrency systems.
    #[serde(
        rename = "innodb_thread_concurrency",
        skip_serializing_if = "Option::is_none"
    )]
    pub innodb_thread_concurrency: Option<i32>,
    /// Start sizes of connection buffer and result buffer, must be multiple of 1024. Changing this parameter will lead to a restart of the MySQL service.
    #[serde(rename = "net_buffer_length", skip_serializing_if = "Option::is_none")]
    pub net_buffer_length: Option<i32>,
    /// Defines the destination for logs. Can be `INSIGHTS`, `TABLE`, or both (`INSIGHTS,TABLE`), or `NONE` to disable logs. To specify both destinations, use `INSIGHTS,TABLE` (order matters). Default is NONE.
    #[serde(rename = "log_output", skip_serializing_if = "Option::is_none")]
    pub log_output: Option<LogOutput>,
    #[serde(
        rename = "mysql_incremental_backup",
        skip_serializing_if = "Option::is_none"
    )]
    pub mysql_incremental_backup: Option<Box<models::MysqlIncrementalBackup>>,
    /// Specifies the maximum age (in transactions) that a table's pg_class.relfrozenxid field can attain before a VACUUM operation is forced to prevent transaction ID wraparound within the table. Note that the system will launch autovacuum processes to prevent wraparound even when autovacuum is otherwise disabled. This parameter will cause the server to be restarted.
    #[serde(
        rename = "autovacuum_freeze_max_age",
        skip_serializing_if = "Option::is_none"
    )]
    pub autovacuum_freeze_max_age: Option<i32>,
    /// Specifies the maximum number of autovacuum processes (other than the autovacuum launcher) that may be running at any one time. The default is three. This parameter can only be set at server start.
    #[serde(
        rename = "autovacuum_max_workers",
        skip_serializing_if = "Option::is_none"
    )]
    pub autovacuum_max_workers: Option<i32>,
    /// Specifies the minimum delay, in seconds, between autovacuum runs on any given database. The default is one minute.
    #[serde(rename = "autovacuum_naptime", skip_serializing_if = "Option::is_none")]
    pub autovacuum_naptime: Option<i32>,
    /// Specifies the minimum number of updated or deleted tuples needed to trigger a VACUUM in any one table. The default is 50 tuples.
    #[serde(
        rename = "autovacuum_vacuum_threshold",
        skip_serializing_if = "Option::is_none"
    )]
    pub autovacuum_vacuum_threshold: Option<i32>,
    /// Specifies the minimum number of inserted, updated, or deleted tuples needed to trigger an ANALYZE in any one table. The default is 50 tuples.
    #[serde(
        rename = "autovacuum_analyze_threshold",
        skip_serializing_if = "Option::is_none"
    )]
    pub autovacuum_analyze_threshold: Option<i32>,
    /// Specifies a fraction, in a decimal value, of the table size to add to autovacuum_vacuum_threshold when deciding whether to trigger a VACUUM. The default is 0.2 (20% of table size).
    #[serde(
        rename = "autovacuum_vacuum_scale_factor",
        skip_serializing_if = "Option::is_none"
    )]
    pub autovacuum_vacuum_scale_factor: Option<f64>,
    /// Specifies a fraction, in a decimal value, of the table size to add to autovacuum_analyze_threshold when deciding whether to trigger an ANALYZE. The default is 0.2 (20% of table size).
    #[serde(
        rename = "autovacuum_analyze_scale_factor",
        skip_serializing_if = "Option::is_none"
    )]
    pub autovacuum_analyze_scale_factor: Option<f64>,
    /// Specifies the cost delay value, in milliseconds, that will be used in automatic VACUUM operations. If -1, uses the regular vacuum_cost_delay value, which is 20 milliseconds.
    #[serde(
        rename = "autovacuum_vacuum_cost_delay",
        skip_serializing_if = "Option::is_none"
    )]
    pub autovacuum_vacuum_cost_delay: Option<i32>,
    /// Specifies the cost limit value that will be used in automatic VACUUM operations. If -1 is specified (which is the default), the regular vacuum_cost_limit value will be used.
    #[serde(
        rename = "autovacuum_vacuum_cost_limit",
        skip_serializing_if = "Option::is_none"
    )]
    pub autovacuum_vacuum_cost_limit: Option<i32>,
    /// Specifies the delay, in milliseconds, between activity rounds for the background writer. Default is 200 ms.
    #[serde(rename = "bgwriter_delay", skip_serializing_if = "Option::is_none")]
    pub bgwriter_delay: Option<i32>,
    /// The amount of kilobytes that need to be written by the background writer before attempting to force the OS to issue these writes to underlying storage. Specified in kilobytes, default is 512.  Setting of 0 disables forced writeback.
    #[serde(
        rename = "bgwriter_flush_after",
        skip_serializing_if = "Option::is_none"
    )]
    pub bgwriter_flush_after: Option<i32>,
    /// The maximum number of buffers that the background writer can write. Setting this to zero disables background writing. Default is 100.
    #[serde(
        rename = "bgwriter_lru_maxpages",
        skip_serializing_if = "Option::is_none"
    )]
    pub bgwriter_lru_maxpages: Option<i32>,
    /// The average recent need for new buffers is multiplied by bgwriter_lru_multiplier to arrive at an estimate of the number that will be needed during the next round, (up to bgwriter_lru_maxpages). 1.0 represents a “just in time” policy of writing exactly the number of buffers predicted to be needed. Larger values provide some cushion against spikes in demand, while smaller values intentionally leave writes to be done by server processes. The default is 2.0.
    #[serde(
        rename = "bgwriter_lru_multiplier",
        skip_serializing_if = "Option::is_none"
    )]
    pub bgwriter_lru_multiplier: Option<f64>,
    /// The amount of time, in milliseconds, to wait on a lock before checking to see if there is a deadlock condition.
    #[serde(rename = "deadlock_timeout", skip_serializing_if = "Option::is_none")]
    pub deadlock_timeout: Option<i32>,
    /// Specifies the default TOAST compression method for values of compressible columns (the default is lz4).
    #[serde(
        rename = "default_toast_compression",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_toast_compression: Option<DefaultToastCompression>,
    /// Time out sessions with open transactions after this number of milliseconds
    #[serde(
        rename = "idle_in_transaction_session_timeout",
        skip_serializing_if = "Option::is_none"
    )]
    pub idle_in_transaction_session_timeout: Option<i32>,
    /// Activates, in a boolean, the system-wide use of Just-in-Time Compilation (JIT).
    #[serde(rename = "jit", skip_serializing_if = "Option::is_none")]
    pub jit: Option<bool>,
    /// Causes each action executed by autovacuum to be logged if it ran for at least the specified number of milliseconds. Setting this to zero logs all autovacuum actions. Minus-one (the default) disables logging autovacuum actions.
    #[serde(
        rename = "log_autovacuum_min_duration",
        skip_serializing_if = "Option::is_none"
    )]
    pub log_autovacuum_min_duration: Option<i32>,
    /// Controls the amount of detail written in the server log for each message that is logged.
    #[serde(
        rename = "log_error_verbosity",
        skip_serializing_if = "Option::is_none"
    )]
    pub log_error_verbosity: Option<LogErrorVerbosity>,
    /// Selects one of the available log-formats. These can support popular log analyzers like pgbadger, pganalyze, etc.
    #[serde(rename = "log_line_prefix", skip_serializing_if = "Option::is_none")]
    pub log_line_prefix: Option<LogLinePrefix>,
    /// Log statements that take more than this number of milliseconds to run. If -1, disables.
    #[serde(
        rename = "log_min_duration_statement",
        skip_serializing_if = "Option::is_none"
    )]
    pub log_min_duration_statement: Option<i32>,
    /// PostgreSQL maximum number of files that can be open per process.
    #[serde(
        rename = "max_files_per_process",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_files_per_process: Option<i32>,
    /// PostgreSQL maximum prepared transactions. Once increased, this parameter cannot be lowered from its set value.
    #[serde(
        rename = "max_prepared_transactions",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_prepared_transactions: Option<i32>,
    /// PostgreSQL maximum predicate locks per transaction.
    #[serde(
        rename = "max_pred_locks_per_transaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_pred_locks_per_transaction: Option<i32>,
    /// PostgreSQL maximum locks per transaction. Once increased, this parameter cannot be lowered from its set value.
    #[serde(
        rename = "max_locks_per_transaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_locks_per_transaction: Option<i32>,
    /// Maximum depth of the stack in bytes.
    #[serde(rename = "max_stack_depth", skip_serializing_if = "Option::is_none")]
    pub max_stack_depth: Option<i32>,
    /// Max standby archive delay in milliseconds.
    #[serde(
        rename = "max_standby_archive_delay",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_standby_archive_delay: Option<i32>,
    /// Max standby streaming delay in milliseconds.
    #[serde(
        rename = "max_standby_streaming_delay",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_standby_streaming_delay: Option<i32>,
    /// PostgreSQL maximum replication slots.
    #[serde(
        rename = "max_replication_slots",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_replication_slots: Option<i32>,
    /// PostgreSQL maximum logical replication workers (taken from the pool of max_parallel_workers).
    #[serde(
        rename = "max_logical_replication_workers",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_logical_replication_workers: Option<i32>,
    /// Sets the maximum number of workers that the system can support for parallel queries.
    #[serde(
        rename = "max_parallel_workers",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_parallel_workers: Option<i32>,
    /// Sets the maximum number of workers that can be started by a single Gather or Gather Merge node.
    #[serde(
        rename = "max_parallel_workers_per_gather",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_parallel_workers_per_gather: Option<i32>,
    /// Sets the maximum number of background processes that the system can support. Once increased, this parameter cannot be lowered from its set value.
    #[serde(
        rename = "max_worker_processes",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_worker_processes: Option<i32>,
    /// Controls which role to use for pg_partman's scheduled background tasks. Must consist of alpha-numeric characters, dots, underscores, or dashes. May not start with dash or dot. Maximum of 64 characters.
    #[serde(
        rename = "pg_partman_bgw.role",
        skip_serializing_if = "Option::is_none"
    )]
    pub pg_partman_bgw_role: Option<String>,
    /// Sets the time interval to run pg_partman's scheduled tasks.
    #[serde(
        rename = "pg_partman_bgw.interval",
        skip_serializing_if = "Option::is_none"
    )]
    pub pg_partman_bgw_interval: Option<i32>,
    /// Controls which statements are counted. Specify 'top' to track top-level statements (those issued directly by clients), 'all' to also track nested statements (such as statements invoked within functions), or 'none' to disable statement statistics collection. The default value is top.
    #[serde(
        rename = "pg_stat_statements.track",
        skip_serializing_if = "Option::is_none"
    )]
    pub pg_stat_statements_track: Option<PgStatStatementsTrack>,
    /// PostgreSQL temporary file limit in KiB. If -1, sets to unlimited.
    #[serde(rename = "temp_file_limit", skip_serializing_if = "Option::is_none")]
    pub temp_file_limit: Option<i32>,
    /// PostgreSQL service timezone
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Specifies the number of bytes reserved to track the currently executing command for each active session.
    #[serde(
        rename = "track_activity_query_size",
        skip_serializing_if = "Option::is_none"
    )]
    pub track_activity_query_size: Option<i32>,
    /// Record commit time of transactions.
    #[serde(
        rename = "track_commit_timestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub track_commit_timestamp: Option<TrackCommitTimestamp>,
    /// Enables tracking of function call counts and time used.
    #[serde(rename = "track_functions", skip_serializing_if = "Option::is_none")]
    pub track_functions: Option<TrackFunctions>,
    /// Enables timing of database I/O calls. This parameter is off by default, because it will repeatedly query the operating system for the current time, which may cause significant overhead on some platforms.
    #[serde(rename = "track_io_timing", skip_serializing_if = "Option::is_none")]
    pub track_io_timing: Option<TrackIoTiming>,
    /// PostgreSQL maximum WAL senders. Once increased, this parameter cannot be lowered from its set value.
    #[serde(rename = "max_wal_senders", skip_serializing_if = "Option::is_none")]
    pub max_wal_senders: Option<i32>,
    /// Terminate replication connections that are inactive for longer than this amount of time, in milliseconds. Setting this value to zero disables the timeout. Must be either 0 or between 5000 and 10800000.
    #[serde(rename = "wal_sender_timeout", skip_serializing_if = "Option::is_none")]
    pub wal_sender_timeout: Option<i32>,
    /// WAL flush interval in milliseconds. Note that setting this value to lower than the default 200ms may negatively impact performance
    #[serde(rename = "wal_writer_delay", skip_serializing_if = "Option::is_none")]
    pub wal_writer_delay: Option<i32>,
    /// Percentage of total RAM that the database server uses for shared memory buffers.  Valid range is 20-60 (float), which corresponds to 20% - 60%.  This setting adjusts the shared_buffers configuration value.
    #[serde(
        rename = "shared_buffers_percentage",
        skip_serializing_if = "Option::is_none"
    )]
    pub shared_buffers_percentage: Option<f64>,
    #[serde(rename = "pgbouncer", skip_serializing_if = "Option::is_none")]
    pub pgbouncer: Option<Box<models::PgbouncerAdvancedConfig>>,
    /// The maximum amount of memory, in MB, used by a query operation (such as a sort or hash table) before writing to temporary disk files. Default is 1MB + 0.075% of total RAM (up to 32MB).
    #[serde(rename = "work_mem", skip_serializing_if = "Option::is_none")]
    pub work_mem: Option<i32>,
    #[serde(rename = "timescaledb", skip_serializing_if = "Option::is_none")]
    pub timescaledb: Option<Box<models::TimescaledbAdvancedConfig>>,
    /// Synchronous replication type. Note that the service plan also needs to support synchronous replication.
    #[serde(
        rename = "synchronous_replication",
        skip_serializing_if = "Option::is_none"
    )]
    pub synchronous_replication: Option<SynchronousReplication>,
    /// Enable the pg_stat_monitor extension. <b>Enabling this extension will cause the cluster to be restarted.</b> When this extension is enabled, pg_stat_statements results for utility commands are unreliable.
    #[serde(
        rename = "stat_monitor_enable",
        skip_serializing_if = "Option::is_none"
    )]
    pub stat_monitor_enable: Option<bool>,
    /// Number of seconds of master unavailability before triggering database failover to standby. The default value is 60.
    #[serde(
        rename = "max_failover_replication_time_lag",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_failover_replication_time_lag: Option<i32>,
    /// Sets the PostgreSQL maximum number of concurrent connections to the database server. This is a limited-release parameter. Contact your account team to confirm your eligibility. You cannot decrease this parameter value when set. For services with a read replica, first increase the read replica's value. After the change is applied to the replica, you can increase the primary service's value. Changing this parameter causes a service restart.
    #[serde(rename = "max_connections", skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i32>,
    /// PostgreSQL maximum WAL size (MB) reserved for replication slots. If -1 is specified, replication slots may retain an unlimited amount of WAL files. The default is -1 (upstream default). wal_keep_size minimum WAL size setting takes precedence over this.
    #[serde(
        rename = "max_slot_wal_keep_size",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_slot_wal_keep_size: Option<i32>,
    /// A string specifying the desired eviction policy for the Caching cluster.  - `noeviction`: Don't evict any data, returns error when memory limit is reached. - `allkeys-lru:` Evict any key, least recently used (LRU) first. - `allkeys-random`: Evict keys in a random order. - `volatile-lru`: Evict keys with expiration only, least recently used (LRU) first. - `volatile-random`: Evict keys with expiration only in a random order. - `volatile-ttl`: Evict keys with expiration only, shortest time-to-live (TTL) first.
    #[serde(
        rename = "redis_maxmemory_policy",
        skip_serializing_if = "Option::is_none"
    )]
    pub redis_maxmemory_policy: Option<RedisMaxmemoryPolicy>,
    /// Set output buffer limit for pub / sub clients in MB. The value is the hard limit, the soft limit is 1/4 of the hard limit. When setting the limit, be mindful of the available memory in the selected service plan.
    #[serde(
        rename = "redis_pubsub_client_output_buffer_limit",
        skip_serializing_if = "Option::is_none"
    )]
    pub redis_pubsub_client_output_buffer_limit: Option<i32>,
    /// Set number of redis databases. Changing this will cause a restart of redis service.
    #[serde(
        rename = "redis_number_of_databases",
        skip_serializing_if = "Option::is_none"
    )]
    pub redis_number_of_databases: Option<i32>,
    /// Caching IO thread count
    #[serde(rename = "redis_io_threads", skip_serializing_if = "Option::is_none")]
    pub redis_io_threads: Option<i32>,
    /// Counter logarithm factor for volatile-lfu and allkeys-lfu maxmemory-policies
    #[serde(
        rename = "redis_lfu_log_factor",
        skip_serializing_if = "Option::is_none"
    )]
    pub redis_lfu_log_factor: Option<i32>,
    /// LFU maxmemory-policy counter decay time in minutes
    #[serde(
        rename = "redis_lfu_decay_time",
        skip_serializing_if = "Option::is_none"
    )]
    pub redis_lfu_decay_time: Option<i32>,
    /// Require SSL to access Caching. - When enabled, Caching accepts only SSL connections on port `25061`. - When disabled, port `25060` is opened for non-SSL connections, while port `25061` remains available for SSL connections.
    #[serde(rename = "redis_ssl", skip_serializing_if = "Option::is_none")]
    pub redis_ssl: Option<bool>,
    /// Caching idle connection timeout in seconds
    #[serde(rename = "redis_timeout", skip_serializing_if = "Option::is_none")]
    pub redis_timeout: Option<i32>,
    /// Set notify-keyspace-events option. Requires at least `K` or `E` and accepts any combination of the following options. Setting the parameter to `\"\"` disables notifications. - `K` &mdash; Keyspace events - `E` &mdash; Keyevent events - `g` &mdash; Generic commands (e.g. `DEL`, `EXPIRE`, `RENAME`, ...) - `$` &mdash; String commands - `l` &mdash; List commands - `s` &mdash; Set commands - `h` &mdash; Hash commands - `z` &mdash; Sorted set commands - `t` &mdash; Stream commands - `d` &mdash; Module key type events - `x` &mdash; Expired events - `e` &mdash; Evicted events - `m` &mdash; Key miss events - `n` &mdash; New key events - `A` &mdash; Alias for `\"g$lshztxed\"`
    #[serde(
        rename = "redis_notify_keyspace_events",
        skip_serializing_if = "Option::is_none"
    )]
    pub redis_notify_keyspace_events: Option<String>,
    /// Creates an RDB dump of the database every 10 minutes that can be used  to recover data after a node crash. The database does not create the  dump if no keys have changed since the last dump. When set to `off`,  the database cannot fork services, and data can be lost if a service  is restarted or powered off. DigitalOcean Managed Caching databases  do not support the Append Only File (AOF) persistence method.
    #[serde(rename = "redis_persistence", skip_serializing_if = "Option::is_none")]
    pub redis_persistence: Option<RedisPersistence>,
    /// Determines default pub/sub channels' ACL for new users if ACL is not supplied. When this option is not defined, all_channels is assumed to keep backward compatibility. This option doesn't affect Caching configuration acl-pubsub-default.
    #[serde(
        rename = "redis_acl_channels_default",
        skip_serializing_if = "Option::is_none"
    )]
    pub redis_acl_channels_default: Option<RedisAclChannelsDefault>,
    #[serde(
        rename = "valkey_maxmemory_policy",
        skip_serializing_if = "Option::is_none"
    )]
    pub valkey_maxmemory_policy: Option<models::EvictionPolicyModel>,
    /// Set output buffer limit for pub / sub clients in MB. The value is the hard limit, the soft limit is 1/4 of the hard limit. When setting the limit, be mindful of the available memory in the selected service plan.
    #[serde(
        rename = "valkey_pubsub_client_output_buffer_limit",
        skip_serializing_if = "Option::is_none"
    )]
    pub valkey_pubsub_client_output_buffer_limit: Option<i32>,
    /// Set number of valkey databases. Changing this will cause a restart of valkey service.
    #[serde(
        rename = "valkey_number_of_databases",
        skip_serializing_if = "Option::is_none"
    )]
    pub valkey_number_of_databases: Option<i32>,
    /// Valkey IO thread count
    #[serde(rename = "valkey_io_threads", skip_serializing_if = "Option::is_none")]
    pub valkey_io_threads: Option<i32>,
    /// Counter logarithm factor for volatile-lfu and allkeys-lfu maxmemory-policies
    #[serde(
        rename = "valkey_lfu_log_factor",
        skip_serializing_if = "Option::is_none"
    )]
    pub valkey_lfu_log_factor: Option<i32>,
    /// LFU maxmemory-policy counter decay time in minutes
    #[serde(
        rename = "valkey_lfu_decay_time",
        skip_serializing_if = "Option::is_none"
    )]
    pub valkey_lfu_decay_time: Option<i32>,
    /// Require SSL to access Valkey
    #[serde(rename = "valkey_ssl", skip_serializing_if = "Option::is_none")]
    pub valkey_ssl: Option<bool>,
    /// Valkey idle connection timeout in seconds
    #[serde(rename = "valkey_timeout", skip_serializing_if = "Option::is_none")]
    pub valkey_timeout: Option<i32>,
    /// Set notify-keyspace-events option. Requires at least `K` or `E` and accepts any combination of the following options. Setting the parameter to `\"\"` disables notifications. - `K` &mdash; Keyspace events - `E` &mdash; Keyevent events - `g` &mdash; Generic commands (e.g. `DEL`, `EXPIRE`, `RENAME`, ...) - `$` &mdash; String commands - `l` &mdash; List commands - `s` &mdash; Set commands - `h` &mdash; Hash commands - `z` &mdash; Sorted set commands - `t` &mdash; Stream commands - `d` &mdash; Module key type events - `x` &mdash; Expired events - `e` &mdash; Evicted events - `m` &mdash; Key miss events - `n` &mdash; New key events - `A` &mdash; Alias for `\"g$lshztxed\"`
    #[serde(
        rename = "valkey_notify_keyspace_events",
        skip_serializing_if = "Option::is_none"
    )]
    pub valkey_notify_keyspace_events: Option<String>,
    /// When persistence is 'rdb', Valkey does RDB dumps each 10 minutes if any key is changed. Also RDB dumps are done according to backup schedule for backup purposes. When persistence is 'off', no RDB dumps and backups are done, so data can be lost at any moment if service is restarted for any reason, or if service is powered off. Also service can't be forked.
    #[serde(rename = "valkey_persistence", skip_serializing_if = "Option::is_none")]
    pub valkey_persistence: Option<ValkeyPersistence>,
    /// Determines default pub/sub channels' ACL for new users if ACL is not supplied. When this option is not defined, all_channels is assumed to keep backward compatibility. This option doesn't affect Valkey configuration acl-pubsub-default.
    #[serde(
        rename = "valkey_acl_channels_default",
        skip_serializing_if = "Option::is_none"
    )]
    pub valkey_acl_channels_default: Option<ValkeyAclChannelsDefault>,
    /// Frequent RDB snapshots When enabled, Valkey will create frequent local RDB snapshots. When disabled, Valkey will only take RDB snapshots when a backup is created, based on the backup schedule. This setting is ignored when valkey_persistence is set to off.
    #[serde(rename = "frequent_snapshots", skip_serializing_if = "Option::is_none")]
    pub frequent_snapshots: Option<bool>,
    /// Active expire effort Valkey reclaims expired keys both when accessed and in the background. The background process scans for expired keys to free memory. Increasing the active-expire-effort setting (default 1, max 10) uses more CPU to reclaim expired keys faster, reducing memory usage but potentially increasing latency.
    #[serde(
        rename = "valkey_active_expire_effort",
        skip_serializing_if = "Option::is_none"
    )]
    pub valkey_active_expire_effort: Option<i32>,
    /// Specify the final compression type for a given topic. This configuration accepts the standard compression codecs ('gzip', 'snappy', 'lz4', 'zstd'). It additionally accepts 'uncompressed' which is equivalent to no compression; and 'producer' which means retain the original compression codec set by the producer.
    #[serde(rename = "compression_type", skip_serializing_if = "Option::is_none")]
    pub compression_type: Option<CompressionType>,
    /// The amount of time, in milliseconds, the group coordinator will wait for more consumers to join a new group before performing the first rebalance. A longer delay means potentially fewer rebalances, but increases the time until processing begins. The default value for this is 3 seconds. During development and testing it might be desirable to set this to 0 in order to not delay test execution time.
    #[serde(
        rename = "group_initial_rebalance_delay_ms",
        skip_serializing_if = "Option::is_none"
    )]
    pub group_initial_rebalance_delay_ms: Option<i32>,
    /// The minimum allowed session timeout for registered consumers. Longer timeouts give consumers more time to process messages in between heartbeats at the cost of a longer time to detect failures.
    #[serde(
        rename = "group_min_session_timeout_ms",
        skip_serializing_if = "Option::is_none"
    )]
    pub group_min_session_timeout_ms: Option<i32>,
    /// The maximum allowed session timeout for registered consumers. Longer timeouts give consumers more time to process messages in between heartbeats at the cost of a longer time to detect failures.
    #[serde(
        rename = "group_max_session_timeout_ms",
        skip_serializing_if = "Option::is_none"
    )]
    pub group_max_session_timeout_ms: Option<i32>,
    /// Idle connections timeout: the server socket processor threads close the connections that idle for longer than this.
    #[serde(
        rename = "connections_max_idle_ms",
        skip_serializing_if = "Option::is_none"
    )]
    pub connections_max_idle_ms: Option<i32>,
    /// The maximum number of incremental fetch sessions that the broker will maintain.
    #[serde(
        rename = "max_incremental_fetch_session_cache_slots",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_incremental_fetch_session_cache_slots: Option<i32>,
    /// The maximum size of message that the server can receive.
    #[serde(rename = "message_max_bytes", skip_serializing_if = "Option::is_none")]
    pub message_max_bytes: Option<i32>,
    /// Log retention window in minutes for offsets topic
    #[serde(
        rename = "offsets_retention_minutes",
        skip_serializing_if = "Option::is_none"
    )]
    pub offsets_retention_minutes: Option<i32>,
    /// How long are delete records retained?
    #[serde(
        rename = "log_cleaner_delete_retention_ms",
        skip_serializing_if = "Option::is_none"
    )]
    pub log_cleaner_delete_retention_ms: Option<i32>,
    /// Controls log compactor frequency. Larger value means more frequent compactions but also more space wasted for logs. Consider setting log_cleaner_max_compaction_lag_ms to enforce compactions sooner, instead of setting a very high value for this option.
    #[serde(
        rename = "log_cleaner_min_cleanable_ratio",
        skip_serializing_if = "Option::is_none"
    )]
    pub log_cleaner_min_cleanable_ratio: Option<f64>,
    /// The maximum amount of time message will remain uncompacted. Only applicable for logs that are being compacted
    #[serde(
        rename = "log_cleaner_max_compaction_lag_ms",
        skip_serializing_if = "Option::is_none"
    )]
    pub log_cleaner_max_compaction_lag_ms: Option<i32>,
    /// The minimum time a message will remain uncompacted in the log. Only applicable for logs that are being compacted.
    #[serde(
        rename = "log_cleaner_min_compaction_lag_ms",
        skip_serializing_if = "Option::is_none"
    )]
    pub log_cleaner_min_compaction_lag_ms: Option<i32>,
    /// The default cleanup policy for segments beyond the retention window
    #[serde(rename = "log_cleanup_policy", skip_serializing_if = "Option::is_none")]
    pub log_cleanup_policy: Option<LogCleanupPolicy>,
    /// The number of messages accumulated on a log partition before messages are flushed to disk
    #[serde(
        rename = "log_flush_interval_messages",
        skip_serializing_if = "Option::is_none"
    )]
    pub log_flush_interval_messages: Option<i32>,
    /// The maximum time in ms that a message in any topic is kept in memory before flushed to disk. If not set, the value in log.flush.scheduler.interval.ms is used
    #[serde(
        rename = "log_flush_interval_ms",
        skip_serializing_if = "Option::is_none"
    )]
    pub log_flush_interval_ms: Option<i32>,
    /// The interval with which Kafka adds an entry to the offset index
    #[serde(
        rename = "log_index_interval_bytes",
        skip_serializing_if = "Option::is_none"
    )]
    pub log_index_interval_bytes: Option<i32>,
    /// The maximum size in bytes of the offset index
    #[serde(
        rename = "log_index_size_max_bytes",
        skip_serializing_if = "Option::is_none"
    )]
    pub log_index_size_max_bytes: Option<i32>,
    /// This configuration controls whether down-conversion of message formats is enabled to satisfy consume requests.
    #[serde(
        rename = "log_message_downconversion_enable",
        skip_serializing_if = "Option::is_none"
    )]
    pub log_message_downconversion_enable: Option<bool>,
    /// Define whether the timestamp in the message is message create time or log append time.
    #[serde(
        rename = "log_message_timestamp_type",
        skip_serializing_if = "Option::is_none"
    )]
    pub log_message_timestamp_type: Option<LogMessageTimestampType>,
    /// The maximum difference allowed between the timestamp when a broker receives a message and the timestamp specified in the message
    #[serde(
        rename = "log_message_timestamp_difference_max_ms",
        skip_serializing_if = "Option::is_none"
    )]
    pub log_message_timestamp_difference_max_ms: Option<i32>,
    /// Controls whether to preallocate a file when creating a new segment
    #[serde(rename = "log_preallocate", skip_serializing_if = "Option::is_none")]
    pub log_preallocate: Option<bool>,
    /// The maximum size of the log before deleting messages
    #[serde(
        rename = "log_retention_bytes",
        skip_serializing_if = "Option::is_none"
    )]
    pub log_retention_bytes: Option<i32>,
    /// The number of hours to keep a log file before deleting it
    #[serde(
        rename = "log_retention_hours",
        skip_serializing_if = "Option::is_none"
    )]
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
    #[serde(
        rename = "log_segment_delete_delay_ms",
        skip_serializing_if = "Option::is_none"
    )]
    pub log_segment_delete_delay_ms: Option<i32>,
    /// Enable auto creation of topics
    #[serde(
        rename = "auto_create_topics_enable",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_create_topics_enable: Option<bool>,
    /// When a producer sets acks to 'all' (or '-1'), min_insync_replicas specifies the minimum number of replicas that must acknowledge a write for the write to be considered successful.
    #[serde(
        rename = "min_insync_replicas",
        skip_serializing_if = "Option::is_none"
    )]
    pub min_insync_replicas: Option<i32>,
    /// Number of partitions for autocreated topics
    #[serde(rename = "num_partitions", skip_serializing_if = "Option::is_none")]
    pub num_partitions: Option<i32>,
    /// Replication factor for autocreated topics
    #[serde(
        rename = "default_replication_factor",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_replication_factor: Option<i32>,
    /// The number of bytes of messages to attempt to fetch for each partition (defaults to 1048576). This is not an absolute maximum, if the first record batch in the first non-empty partition of the fetch is larger than this value, the record batch will still be returned to ensure that progress can be made.
    #[serde(
        rename = "replica_fetch_max_bytes",
        skip_serializing_if = "Option::is_none"
    )]
    pub replica_fetch_max_bytes: Option<i32>,
    /// Maximum bytes expected for the entire fetch response (defaults to 10485760). Records are fetched in batches, and if the first record batch in the first non-empty partition of the fetch is larger than this value, the record batch will still be returned to ensure that progress can be made. As such, this is not an absolute maximum.
    #[serde(
        rename = "replica_fetch_response_max_bytes",
        skip_serializing_if = "Option::is_none"
    )]
    pub replica_fetch_response_max_bytes: Option<i32>,
    /// The maximum number of connections allowed from each ip address (defaults to 2147483647).
    #[serde(
        rename = "max_connections_per_ip",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_connections_per_ip: Option<i32>,
    /// The purge interval (in number of requests) of the producer request purgatory (defaults to 1000).
    #[serde(
        rename = "producer_purgatory_purge_interval_requests",
        skip_serializing_if = "Option::is_none"
    )]
    pub producer_purgatory_purge_interval_requests: Option<i32>,
    /// The maximum number of bytes in a socket request (defaults to 104857600).
    #[serde(
        rename = "socket_request_max_bytes",
        skip_serializing_if = "Option::is_none"
    )]
    pub socket_request_max_bytes: Option<i32>,
    /// The transaction topic segment bytes should be kept relatively small in order to facilitate faster log compaction and cache loads (defaults to 104857600 (100 mebibytes)).
    #[serde(
        rename = "transaction_state_log_segment_bytes",
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_state_log_segment_bytes: Option<i32>,
    /// The interval at which to remove transactions that have expired due to transactional.id.expiration.ms passing (defaults to 3600000 (1 hour)).
    #[serde(
        rename = "transaction_remove_expired_transaction_cleanup_interval_ms",
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_remove_expired_transaction_cleanup_interval_ms: Option<i32>,
    /// Enable creation of schema registry for the Kafka cluster. Schema_registry only works with General Purpose - Dedicated CPU plans.
    #[serde(rename = "schema_registry", skip_serializing_if = "Option::is_none")]
    pub schema_registry: Option<bool>,
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
    /// Specifies the default consistency behavior of reads from the database. Data that is returned from the query with may or may not have been acknowledged by all nodes in the replicaset depending on this value.  Learn more [here](https://www.mongodb.com/docs/manual/reference/read-concern/).
    #[serde(
        rename = "default_read_concern",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_read_concern: Option<DefaultReadConcern>,
    /// Describes the level of acknowledgment requested from MongoDB for write operations clusters. This field can set to either `majority` or a number `0...n` which will describe the number of nodes that must acknowledge the write operation before it is fully accepted. Setting to `0` will request no acknowledgement of the write operation.  Learn more [here](https://www.mongodb.com/docs/manual/reference/write-concern/).
    #[serde(
        rename = "default_write_concern",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_write_concern: Option<String>,
    /// Specifies the lifetime of multi-document transactions. Transactions that exceed this limit are considered expired and will be  aborted by a periodic cleanup process. The cleanup process runs every `transactionLifetimeLimitSeconds/2 seconds` or at least  once every 60 seconds. *Changing this parameter will lead to a restart of the MongoDB service.* Learn more [here](https://www.mongodb.com/docs/manual/reference/parameters/#mongodb-parameter-param.transactionLifetimeLimitSeconds).
    #[serde(
        rename = "transaction_lifetime_limit_seconds",
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_lifetime_limit_seconds: Option<i32>,
    /// Operations that run for longer than this threshold are considered slow which are then recorded to the diagnostic logs.  Higher log levels (verbosity) will record all operations regardless of this threshold on the primary node.  *Changing this parameter will lead to a restart of the MongoDB service.* Learn more [here](https://www.mongodb.com/docs/manual/reference/configuration-options/#mongodb-setting-operationProfiling.slowOpThresholdMs).
    #[serde(
        rename = "slow_op_threshold_ms",
        skip_serializing_if = "Option::is_none"
    )]
    pub slow_op_threshold_ms: Option<i32>,
    /// The log message verbosity level. The verbosity level determines the amount of Informational and Debug messages MongoDB outputs. 0 includes informational messages while 1...5 increases the level to include debug messages. *Changing this parameter will lead to a restart of the MongoDB service.* Learn more [here](https://www.mongodb.com/docs/manual/reference/configuration-options/#mongodb-setting-systemLog.verbosity).
    #[serde(rename = "verbosity", skip_serializing_if = "Option::is_none")]
    pub verbosity: Option<i32>,
}

impl DatabasesGetConfig200ResponseConfig {
    pub fn new() -> DatabasesGetConfig200ResponseConfig {
        DatabasesGetConfig200ResponseConfig {
            backup_hour: None,
            backup_minute: None,
            sql_mode: None,
            connect_timeout: None,
            default_time_zone: None,
            group_concat_max_len: None,
            information_schema_stats_expiry: None,
            innodb_ft_min_token_size: None,
            innodb_ft_server_stopword_table: None,
            innodb_lock_wait_timeout: None,
            innodb_log_buffer_size: None,
            innodb_online_alter_log_max_size: None,
            innodb_print_all_deadlocks: None,
            innodb_rollback_on_timeout: None,
            interactive_timeout: None,
            internal_tmp_mem_storage_engine: None,
            net_read_timeout: None,
            net_write_timeout: None,
            sql_require_primary_key: None,
            wait_timeout: None,
            max_allowed_packet: None,
            max_heap_table_size: None,
            sort_buffer_size: None,
            tmp_table_size: None,
            slow_query_log: None,
            long_query_time: None,
            binlog_retention_period: None,
            innodb_change_buffer_max_size: None,
            innodb_flush_neighbors: None,
            innodb_read_io_threads: None,
            innodb_write_io_threads: None,
            innodb_thread_concurrency: None,
            net_buffer_length: None,
            log_output: None,
            mysql_incremental_backup: None,
            autovacuum_freeze_max_age: None,
            autovacuum_max_workers: None,
            autovacuum_naptime: None,
            autovacuum_vacuum_threshold: None,
            autovacuum_analyze_threshold: None,
            autovacuum_vacuum_scale_factor: None,
            autovacuum_analyze_scale_factor: None,
            autovacuum_vacuum_cost_delay: None,
            autovacuum_vacuum_cost_limit: None,
            bgwriter_delay: None,
            bgwriter_flush_after: None,
            bgwriter_lru_maxpages: None,
            bgwriter_lru_multiplier: None,
            deadlock_timeout: None,
            default_toast_compression: None,
            idle_in_transaction_session_timeout: None,
            jit: None,
            log_autovacuum_min_duration: None,
            log_error_verbosity: None,
            log_line_prefix: None,
            log_min_duration_statement: None,
            max_files_per_process: None,
            max_prepared_transactions: None,
            max_pred_locks_per_transaction: None,
            max_locks_per_transaction: None,
            max_stack_depth: None,
            max_standby_archive_delay: None,
            max_standby_streaming_delay: None,
            max_replication_slots: None,
            max_logical_replication_workers: None,
            max_parallel_workers: None,
            max_parallel_workers_per_gather: None,
            max_worker_processes: None,
            pg_partman_bgw_role: None,
            pg_partman_bgw_interval: None,
            pg_stat_statements_track: None,
            temp_file_limit: None,
            timezone: None,
            track_activity_query_size: None,
            track_commit_timestamp: None,
            track_functions: None,
            track_io_timing: None,
            max_wal_senders: None,
            wal_sender_timeout: None,
            wal_writer_delay: None,
            shared_buffers_percentage: None,
            pgbouncer: None,
            work_mem: None,
            timescaledb: None,
            synchronous_replication: None,
            stat_monitor_enable: None,
            max_failover_replication_time_lag: None,
            max_connections: None,
            max_slot_wal_keep_size: None,
            redis_maxmemory_policy: None,
            redis_pubsub_client_output_buffer_limit: None,
            redis_number_of_databases: None,
            redis_io_threads: None,
            redis_lfu_log_factor: None,
            redis_lfu_decay_time: None,
            redis_ssl: None,
            redis_timeout: None,
            redis_notify_keyspace_events: None,
            redis_persistence: None,
            redis_acl_channels_default: None,
            valkey_maxmemory_policy: None,
            valkey_pubsub_client_output_buffer_limit: None,
            valkey_number_of_databases: None,
            valkey_io_threads: None,
            valkey_lfu_log_factor: None,
            valkey_lfu_decay_time: None,
            valkey_ssl: None,
            valkey_timeout: None,
            valkey_notify_keyspace_events: None,
            valkey_persistence: None,
            valkey_acl_channels_default: None,
            frequent_snapshots: None,
            valkey_active_expire_effort: None,
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
            default_read_concern: None,
            default_write_concern: None,
            transaction_lifetime_limit_seconds: None,
            slow_op_threshold_ms: None,
            verbosity: None,
        }
    }
}
/// The storage engine for in-memory internal temporary tables.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InternalTmpMemStorageEngine {
    #[serde(rename = "TempTable")]
    TempTable,
    #[serde(rename = "MEMORY")]
    Memory,
}

impl Default for InternalTmpMemStorageEngine {
    fn default() -> InternalTmpMemStorageEngine {
        Self::TempTable
    }
}
/// Specifies whether flushing a page from the InnoDB buffer pool also flushes other dirty pages in the same extent.   - 0 &mdash; disables this functionality, dirty pages in the same extent are not flushed.   - 1 &mdash; flushes contiguous dirty pages in the same extent.   - 2 &mdash; flushes dirty pages in the same extent.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InnodbFlushNeighbors {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
    #[serde(rename = "2")]
    Variant2,
}

impl Default for InnodbFlushNeighbors {
    fn default() -> InnodbFlushNeighbors {
        Self::Variant0
    }
}
/// Defines the destination for logs. Can be `INSIGHTS`, `TABLE`, or both (`INSIGHTS,TABLE`), or `NONE` to disable logs. To specify both destinations, use `INSIGHTS,TABLE` (order matters). Default is NONE.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LogOutput {
    #[serde(rename = "INSIGHTS")]
    Insights,
    #[serde(rename = "TABLE")]
    Table,
    #[serde(rename = "INSIGHTS,TABLE")]
    InsightsCommaTable,
    #[serde(rename = "NONE")]
    None,
}

impl Default for LogOutput {
    fn default() -> LogOutput {
        Self::Insights
    }
}
/// Specifies the default TOAST compression method for values of compressible columns (the default is lz4).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DefaultToastCompression {
    #[serde(rename = "lz4")]
    Lz4,
    #[serde(rename = "pglz")]
    Pglz,
}

impl Default for DefaultToastCompression {
    fn default() -> DefaultToastCompression {
        Self::Lz4
    }
}
/// Controls the amount of detail written in the server log for each message that is logged.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LogErrorVerbosity {
    #[serde(rename = "TERSE")]
    Terse,
    #[serde(rename = "DEFAULT")]
    Default,
    #[serde(rename = "VERBOSE")]
    Verbose,
}

impl Default for LogErrorVerbosity {
    fn default() -> LogErrorVerbosity {
        Self::Terse
    }
}
/// Selects one of the available log-formats. These can support popular log analyzers like pgbadger, pganalyze, etc.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LogLinePrefix {
    #[serde(rename = "pid=%p,user=%u,db=%d,app=%a,client=%h")]
    PidEqualPercentPCommaUserEqualPercentUCommaDbEqualPercentDCommaAppEqualPercentACommaClientEqualPercentH,
    #[serde(rename = "%m [%p] %q[user=%u,db=%d,app=%a]")]
    PercentMLeftSquareBracketPercentPRightSquareBracketPercentQLeftSquareBracketUserEqualPercentUCommaDbEqualPercentDCommaAppEqualPercentARightSquareBracket,
    #[serde(rename = "%t [%p]: [%l-1] user=%u,db=%d,app=%a,client=%h")]
    PercentTLeftSquareBracketPercentPRightSquareBracketColonLeftSquareBracketPercentL1RightSquareBracketUserEqualPercentUCommaDbEqualPercentDCommaAppEqualPercentACommaClientEqualPercentH,
}

impl Default for LogLinePrefix {
    fn default() -> LogLinePrefix {
        Self::PidEqualPercentPCommaUserEqualPercentUCommaDbEqualPercentDCommaAppEqualPercentACommaClientEqualPercentH
    }
}
/// Controls which statements are counted. Specify 'top' to track top-level statements (those issued directly by clients), 'all' to also track nested statements (such as statements invoked within functions), or 'none' to disable statement statistics collection. The default value is top.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PgStatStatementsTrack {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "top")]
    Top,
    #[serde(rename = "none")]
    None,
}

impl Default for PgStatStatementsTrack {
    fn default() -> PgStatStatementsTrack {
        Self::All
    }
}
/// Record commit time of transactions.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TrackCommitTimestamp {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "on")]
    On,
}

impl Default for TrackCommitTimestamp {
    fn default() -> TrackCommitTimestamp {
        Self::Off
    }
}
/// Enables tracking of function call counts and time used.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TrackFunctions {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "pl")]
    Pl,
    #[serde(rename = "none")]
    None,
}

impl Default for TrackFunctions {
    fn default() -> TrackFunctions {
        Self::All
    }
}
/// Enables timing of database I/O calls. This parameter is off by default, because it will repeatedly query the operating system for the current time, which may cause significant overhead on some platforms.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TrackIoTiming {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "on")]
    On,
}

impl Default for TrackIoTiming {
    fn default() -> TrackIoTiming {
        Self::Off
    }
}
/// Synchronous replication type. Note that the service plan also needs to support synchronous replication.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SynchronousReplication {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "quorum")]
    Quorum,
}

impl Default for SynchronousReplication {
    fn default() -> SynchronousReplication {
        Self::Off
    }
}
/// A string specifying the desired eviction policy for the Caching cluster.  - `noeviction`: Don't evict any data, returns error when memory limit is reached. - `allkeys-lru:` Evict any key, least recently used (LRU) first. - `allkeys-random`: Evict keys in a random order. - `volatile-lru`: Evict keys with expiration only, least recently used (LRU) first. - `volatile-random`: Evict keys with expiration only in a random order. - `volatile-ttl`: Evict keys with expiration only, shortest time-to-live (TTL) first.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RedisMaxmemoryPolicy {
    #[serde(rename = "noeviction")]
    Noeviction,
    #[serde(rename = "allkeys-lru")]
    AllkeysLru,
    #[serde(rename = "allkeys-random")]
    AllkeysRandom,
    #[serde(rename = "volatile-lru")]
    VolatileLru,
    #[serde(rename = "volatile-random")]
    VolatileRandom,
    #[serde(rename = "volatile-ttl")]
    VolatileTtl,
}

impl Default for RedisMaxmemoryPolicy {
    fn default() -> RedisMaxmemoryPolicy {
        Self::Noeviction
    }
}
/// Creates an RDB dump of the database every 10 minutes that can be used  to recover data after a node crash. The database does not create the  dump if no keys have changed since the last dump. When set to `off`,  the database cannot fork services, and data can be lost if a service  is restarted or powered off. DigitalOcean Managed Caching databases  do not support the Append Only File (AOF) persistence method.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RedisPersistence {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "rdb")]
    Rdb,
}

impl Default for RedisPersistence {
    fn default() -> RedisPersistence {
        Self::Off
    }
}
/// Determines default pub/sub channels' ACL for new users if ACL is not supplied. When this option is not defined, all_channels is assumed to keep backward compatibility. This option doesn't affect Caching configuration acl-pubsub-default.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RedisAclChannelsDefault {
    #[serde(rename = "allchannels")]
    Allchannels,
    #[serde(rename = "resetchannels")]
    Resetchannels,
}

impl Default for RedisAclChannelsDefault {
    fn default() -> RedisAclChannelsDefault {
        Self::Allchannels
    }
}
/// When persistence is 'rdb', Valkey does RDB dumps each 10 minutes if any key is changed. Also RDB dumps are done according to backup schedule for backup purposes. When persistence is 'off', no RDB dumps and backups are done, so data can be lost at any moment if service is restarted for any reason, or if service is powered off. Also service can't be forked.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ValkeyPersistence {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "rdb")]
    Rdb,
}

impl Default for ValkeyPersistence {
    fn default() -> ValkeyPersistence {
        Self::Off
    }
}
/// Determines default pub/sub channels' ACL for new users if ACL is not supplied. When this option is not defined, all_channels is assumed to keep backward compatibility. This option doesn't affect Valkey configuration acl-pubsub-default.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ValkeyAclChannelsDefault {
    #[serde(rename = "allchannels")]
    Allchannels,
    #[serde(rename = "resetchannels")]
    Resetchannels,
}

impl Default for ValkeyAclChannelsDefault {
    fn default() -> ValkeyAclChannelsDefault {
        Self::Allchannels
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
/// Specifies the default consistency behavior of reads from the database. Data that is returned from the query with may or may not have been acknowledged by all nodes in the replicaset depending on this value.  Learn more [here](https://www.mongodb.com/docs/manual/reference/read-concern/).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DefaultReadConcern {
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "majority")]
    Majority,
}

impl Default for DefaultReadConcern {
    fn default() -> DefaultReadConcern {
        Self::Local
    }
}
