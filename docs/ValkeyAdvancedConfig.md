# ValkeyAdvancedConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**valkey_maxmemory_policy** | Option<[**models::EvictionPolicyModel**](eviction_policy_model.md)> |  | [optional]
**valkey_pubsub_client_output_buffer_limit** | Option<**i32**> | Set output buffer limit for pub / sub clients in MB. The value is the hard limit, the soft limit is 1/4 of the hard limit. When setting the limit, be mindful of the available memory in the selected service plan. | [optional]
**valkey_number_of_databases** | Option<**i32**> | Set number of valkey databases. Changing this will cause a restart of valkey service. | [optional]
**valkey_io_threads** | Option<**i32**> | Valkey IO thread count | [optional]
**valkey_lfu_log_factor** | Option<**i32**> | Counter logarithm factor for volatile-lfu and allkeys-lfu maxmemory-policies | [optional][default to 10]
**valkey_lfu_decay_time** | Option<**i32**> | LFU maxmemory-policy counter decay time in minutes | [optional][default to 1]
**valkey_ssl** | Option<**bool**> | Require SSL to access Valkey | [optional][default to true]
**valkey_timeout** | Option<**i32**> | Valkey idle connection timeout in seconds | [optional][default to 300]
**valkey_notify_keyspace_events** | Option<**String**> | Set notify-keyspace-events option. Requires at least `K` or `E` and accepts any combination of the following options. Setting the parameter to `\"\"` disables notifications. - `K` &mdash; Keyspace events - `E` &mdash; Keyevent events - `g` &mdash; Generic commands (e.g. `DEL`, `EXPIRE`, `RENAME`, ...) - `$` &mdash; String commands - `l` &mdash; List commands - `s` &mdash; Set commands - `h` &mdash; Hash commands - `z` &mdash; Sorted set commands - `t` &mdash; Stream commands - `d` &mdash; Module key type events - `x` &mdash; Expired events - `e` &mdash; Evicted events - `m` &mdash; Key miss events - `n` &mdash; New key events - `A` &mdash; Alias for `\"g$lshztxed\"` | [optional][default to ]
**valkey_persistence** | Option<**String**> | When persistence is 'rdb', Valkey does RDB dumps each 10 minutes if any key is changed. Also RDB dumps are done according to backup schedule for backup purposes. When persistence is 'off', no RDB dumps and backups are done, so data can be lost at any moment if service is restarted for any reason, or if service is powered off. Also service can't be forked. | [optional]
**valkey_acl_channels_default** | Option<**String**> | Determines default pub/sub channels' ACL for new users if ACL is not supplied. When this option is not defined, all_channels is assumed to keep backward compatibility. This option doesn't affect Valkey configuration acl-pubsub-default. | [optional]
**frequent_snapshots** | Option<**bool**> | Frequent RDB snapshots When enabled, Valkey will create frequent local RDB snapshots. When disabled, Valkey will only take RDB snapshots when a backup is created, based on the backup schedule. This setting is ignored when valkey_persistence is set to off.  | [optional][default to true]
**valkey_active_expire_effort** | Option<**i32**> | Active expire effort Valkey reclaims expired keys both when accessed and in the background. The background process scans for expired keys to free memory. Increasing the active-expire-effort setting (default 1, max 10) uses more CPU to reclaim expired keys faster, reducing memory usage but potentially increasing latency.  | [optional][default to 1]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


