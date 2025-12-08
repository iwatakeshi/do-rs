# KafkaAdvancedConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**compression_type** | Option<**String**> | Specify the final compression type for a given topic. This configuration accepts the standard compression codecs ('gzip', 'snappy', 'lz4', 'zstd'). It additionally accepts 'uncompressed' which is equivalent to no compression; and 'producer' which means retain the original compression codec set by the producer. | [optional]
**group_initial_rebalance_delay_ms** | Option<**i32**> | The amount of time, in milliseconds, the group coordinator will wait for more consumers to join a new group before performing the first rebalance. A longer delay means potentially fewer rebalances, but increases the time until processing begins. The default value for this is 3 seconds. During development and testing it might be desirable to set this to 0 in order to not delay test execution time. | [optional]
**group_min_session_timeout_ms** | Option<**i32**> | The minimum allowed session timeout for registered consumers. Longer timeouts give consumers more time to process messages in between heartbeats at the cost of a longer time to detect failures. | [optional]
**group_max_session_timeout_ms** | Option<**i32**> | The maximum allowed session timeout for registered consumers. Longer timeouts give consumers more time to process messages in between heartbeats at the cost of a longer time to detect failures. | [optional]
**connections_max_idle_ms** | Option<**i32**> | Idle connections timeout: the server socket processor threads close the connections that idle for longer than this. | [optional]
**max_incremental_fetch_session_cache_slots** | Option<**i32**> | The maximum number of incremental fetch sessions that the broker will maintain. | [optional]
**message_max_bytes** | Option<**i32**> | The maximum size of message that the server can receive. | [optional]
**offsets_retention_minutes** | Option<**i32**> | Log retention window in minutes for offsets topic | [optional]
**log_cleaner_delete_retention_ms** | Option<**i32**> | How long are delete records retained? | [optional]
**log_cleaner_min_cleanable_ratio** | Option<**f64**> | Controls log compactor frequency. Larger value means more frequent compactions but also more space wasted for logs. Consider setting log_cleaner_max_compaction_lag_ms to enforce compactions sooner, instead of setting a very high value for this option. | [optional]
**log_cleaner_max_compaction_lag_ms** | Option<**i32**> | The maximum amount of time message will remain uncompacted. Only applicable for logs that are being compacted | [optional]
**log_cleaner_min_compaction_lag_ms** | Option<**i32**> | The minimum time a message will remain uncompacted in the log. Only applicable for logs that are being compacted. | [optional]
**log_cleanup_policy** | Option<**String**> | The default cleanup policy for segments beyond the retention window | [optional]
**log_flush_interval_messages** | Option<**i32**> | The number of messages accumulated on a log partition before messages are flushed to disk | [optional]
**log_flush_interval_ms** | Option<**i32**> | The maximum time in ms that a message in any topic is kept in memory before flushed to disk. If not set, the value in log.flush.scheduler.interval.ms is used | [optional]
**log_index_interval_bytes** | Option<**i32**> | The interval with which Kafka adds an entry to the offset index | [optional]
**log_index_size_max_bytes** | Option<**i32**> | The maximum size in bytes of the offset index | [optional]
**log_message_downconversion_enable** | Option<**bool**> | This configuration controls whether down-conversion of message formats is enabled to satisfy consume requests. | [optional]
**log_message_timestamp_type** | Option<**String**> | Define whether the timestamp in the message is message create time or log append time. | [optional]
**log_message_timestamp_difference_max_ms** | Option<**i32**> | The maximum difference allowed between the timestamp when a broker receives a message and the timestamp specified in the message | [optional]
**log_preallocate** | Option<**bool**> | Controls whether to preallocate a file when creating a new segment | [optional]
**log_retention_bytes** | Option<**i32**> | The maximum size of the log before deleting messages | [optional]
**log_retention_hours** | Option<**i32**> | The number of hours to keep a log file before deleting it | [optional]
**log_retention_ms** | Option<**i32**> | The number of milliseconds to keep a log file before deleting it (in milliseconds), If not set, the value in log.retention.minutes is used. If set to -1, no time limit is applied. | [optional]
**log_roll_jitter_ms** | Option<**i32**> | The maximum jitter to subtract from logRollTimeMillis (in milliseconds). If not set, the value in log.roll.jitter.hours is used | [optional]
**log_roll_ms** | Option<**i32**> | The maximum time before a new log segment is rolled out (in milliseconds). | [optional]
**log_segment_bytes** | Option<**i32**> | The maximum size of a single log file | [optional]
**log_segment_delete_delay_ms** | Option<**i32**> | The amount of time to wait before deleting a file from the filesystem | [optional]
**auto_create_topics_enable** | Option<**bool**> | Enable auto creation of topics | [optional][default to false]
**min_insync_replicas** | Option<**i32**> | When a producer sets acks to 'all' (or '-1'), min_insync_replicas specifies the minimum number of replicas that must acknowledge a write for the write to be considered successful. | [optional]
**num_partitions** | Option<**i32**> | Number of partitions for autocreated topics | [optional]
**default_replication_factor** | Option<**i32**> | Replication factor for autocreated topics | [optional]
**replica_fetch_max_bytes** | Option<**i32**> | The number of bytes of messages to attempt to fetch for each partition (defaults to 1048576). This is not an absolute maximum, if the first record batch in the first non-empty partition of the fetch is larger than this value, the record batch will still be returned to ensure that progress can be made. | [optional]
**replica_fetch_response_max_bytes** | Option<**i32**> | Maximum bytes expected for the entire fetch response (defaults to 10485760). Records are fetched in batches, and if the first record batch in the first non-empty partition of the fetch is larger than this value, the record batch will still be returned to ensure that progress can be made. As such, this is not an absolute maximum. | [optional]
**max_connections_per_ip** | Option<**i32**> | The maximum number of connections allowed from each ip address (defaults to 2147483647). | [optional]
**producer_purgatory_purge_interval_requests** | Option<**i32**> | The purge interval (in number of requests) of the producer request purgatory (defaults to 1000). | [optional]
**socket_request_max_bytes** | Option<**i32**> | The maximum number of bytes in a socket request (defaults to 104857600). | [optional]
**transaction_state_log_segment_bytes** | Option<**i32**> | The transaction topic segment bytes should be kept relatively small in order to facilitate faster log compaction and cache loads (defaults to 104857600 (100 mebibytes)). | [optional]
**transaction_remove_expired_transaction_cleanup_interval_ms** | Option<**i32**> | The interval at which to remove transactions that have expired due to transactional.id.expiration.ms passing (defaults to 3600000 (1 hour)). | [optional]
**schema_registry** | Option<**bool**> | Enable creation of schema registry for the Kafka cluster. Schema_registry only works with General Purpose - Dedicated CPU plans. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


