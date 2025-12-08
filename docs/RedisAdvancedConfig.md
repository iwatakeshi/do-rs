# RedisAdvancedConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**redis_maxmemory_policy** | Option<**String**> | A string specifying the desired eviction policy for the Caching cluster.  - `noeviction`: Don't evict any data, returns error when memory limit is reached. - `allkeys-lru:` Evict any key, least recently used (LRU) first. - `allkeys-random`: Evict keys in a random order. - `volatile-lru`: Evict keys with expiration only, least recently used (LRU) first. - `volatile-random`: Evict keys with expiration only in a random order. - `volatile-ttl`: Evict keys with expiration only, shortest time-to-live (TTL) first. | [optional]
**redis_pubsub_client_output_buffer_limit** | Option<**i32**> | Set output buffer limit for pub / sub clients in MB. The value is the hard limit, the soft limit is 1/4 of the hard limit. When setting the limit, be mindful of the available memory in the selected service plan. | [optional]
**redis_number_of_databases** | Option<**i32**> | Set number of redis databases. Changing this will cause a restart of redis service. | [optional]
**redis_io_threads** | Option<**i32**> | Caching IO thread count | [optional]
**redis_lfu_log_factor** | Option<**i32**> | Counter logarithm factor for volatile-lfu and allkeys-lfu maxmemory-policies | [optional][default to 10]
**redis_lfu_decay_time** | Option<**i32**> | LFU maxmemory-policy counter decay time in minutes | [optional][default to 1]
**redis_ssl** | Option<**bool**> | Require SSL to access Caching. - When enabled, Caching accepts only SSL connections on port `25061`. - When disabled, port `25060` is opened for non-SSL connections, while port `25061` remains available for SSL connections.  | [optional][default to true]
**redis_timeout** | Option<**i32**> | Caching idle connection timeout in seconds | [optional][default to 300]
**redis_notify_keyspace_events** | Option<**String**> | Set notify-keyspace-events option. Requires at least `K` or `E` and accepts any combination of the following options. Setting the parameter to `\"\"` disables notifications. - `K` &mdash; Keyspace events - `E` &mdash; Keyevent events - `g` &mdash; Generic commands (e.g. `DEL`, `EXPIRE`, `RENAME`, ...) - `$` &mdash; String commands - `l` &mdash; List commands - `s` &mdash; Set commands - `h` &mdash; Hash commands - `z` &mdash; Sorted set commands - `t` &mdash; Stream commands - `d` &mdash; Module key type events - `x` &mdash; Expired events - `e` &mdash; Evicted events - `m` &mdash; Key miss events - `n` &mdash; New key events - `A` &mdash; Alias for `\"g$lshztxed\"` | [optional][default to ]
**redis_persistence** | Option<**String**> | Creates an RDB dump of the database every 10 minutes that can be used  to recover data after a node crash. The database does not create the  dump if no keys have changed since the last dump. When set to `off`,  the database cannot fork services, and data can be lost if a service  is restarted or powered off. DigitalOcean Managed Caching databases  do not support the Append Only File (AOF) persistence method. | [optional]
**redis_acl_channels_default** | Option<**String**> | Determines default pub/sub channels' ACL for new users if ACL is not supplied. When this option is not defined, all_channels is assumed to keep backward compatibility. This option doesn't affect Caching configuration acl-pubsub-default. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


