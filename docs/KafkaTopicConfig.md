# KafkaTopicConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cleanup_policy** | Option<**String**> | The cleanup_policy sets the retention policy to use on log segments. 'delete' will discard old segments when retention time/size limits are reached. 'compact' will enable log compaction, resulting in retention of the latest value for each key. | [optional][default to Delete]
**compression_type** | Option<**String**> | The compression_type specifies the compression type of the topic. | [optional][default to Producer]
**delete_retention_ms** | Option<**i32**> | The delete_retention_ms specifies how long (in ms) to retain delete tombstone markers for topics. | [optional][default to 86400000]
**file_delete_delay_ms** | Option<**i32**> | The file_delete_delay_ms specifies the time (in ms) to wait before deleting a file from the filesystem. | [optional][default to 60000]
**flush_messages** | Option<**i32**> | The flush_messages specifies the number of messages to accumulate on a log partition before messages are flushed to disk. | [optional]
**flush_ms** | Option<**i32**> | The flush_ms specifies the maximum time (in ms) that a message is kept in memory before being flushed to disk. | [optional]
**index_interval_bytes** | Option<**i32**> | The index_interval_bytes specifies the number of bytes between entries being added into te offset index. | [optional][default to 4096]
**max_compaction_lag_ms** | Option<**i32**> | The max_compaction_lag_ms specifies the maximum amount of time (in ms) that a message will remain uncompacted. This is only applicable if the logs are have compaction enabled. | [optional]
**max_message_bytes** | Option<**i32**> | The max_messages_bytes specifies the largest record batch size (in bytes) that can be sent to the server.  This is calculated after compression if compression is enabled. | [optional][default to 1048588]
**message_down_conversion_enable** | Option<**bool**> | The message_down_conversion_enable specifies whether down-conversion of message formats is enabled to satisfy consumer requests. When 'false', the broker will not perform conversion for consumers expecting older message formats. The broker will respond with an `UNSUPPORTED_VERSION` error for consume requests from these older clients. | [optional][default to true]
**message_format_version** | Option<**String**> | The message_format_version specifies the message format version used by the broker to append messages to the logs. The value of this setting is assumed to be 3.0-IV1 if the broker protocol version is 3.0 or higher. By setting a  particular message format version, all existing messages on disk must be smaller or equal to the specified version. | [optional][default to Variant30Iv1]
**message_timestamp_type** | Option<**String**> | The message_timestamp_type specifies whether to use the message create time or log append time as the timestamp on a message. | [optional][default to CreateTime]
**min_cleanable_dirty_ratio** | Option<**f32**> | The min_cleanable_dirty_ratio specifies the frequency of log compaction (if enabled) in relation to duplicates present in the logs. For example, at 0.5, at most 50% of the log could be duplicates before compaction would begin. | [optional][default to 0.5]
**min_compaction_lag_ms** | Option<**i32**> | The min_compaction_lag_ms specifies the minimum time (in ms) that a message will remain uncompacted in the log. Only relevant if log compaction is enabled. | [optional][default to 0]
**min_insync_replicas** | Option<**i32**> | The min_insync_replicas specifies the number of replicas that must ACK a write for the write to be considered successful. | [optional][default to 1]
**preallocate** | Option<**bool**> | The preallocate specifies whether a file should be preallocated on disk when creating a new log segment. | [optional][default to false]
**retention_bytes** | Option<**i32**> | The retention_bytes specifies the maximum size of the log (in bytes) before deleting messages. -1 indicates that there is no limit. | [optional][default to -1]
**retention_ms** | Option<**i32**> | The retention_ms specifies the maximum amount of time (in ms) to keep a message before deleting it. | [optional][default to 604800000]
**segment_bytes** | Option<**i32**> | The segment_bytes specifies the maximum size of a single log file (in bytes). | [optional][default to 209715200]
**segment_jitter_ms** | Option<**i32**> | The segment_jitter_ms specifies the maximum random jitter subtracted from the scheduled segment roll time to avoid thundering herds of segment rolling. | [optional][default to 0]
**segment_ms** | Option<**i32**> | The segment_ms specifies the period of time after which the log will be forced to roll if the segment file isn't full. This ensures that retention can delete or compact old data. | [optional][default to 604800000]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


