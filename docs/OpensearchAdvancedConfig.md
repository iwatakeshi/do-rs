# OpensearchAdvancedConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**http_max_content_length_bytes** | Option<**i32**> | Maximum content length for HTTP requests to the OpenSearch HTTP API, in bytes. | [optional][default to 100000000]
**http_max_header_size_bytes** | Option<**i32**> | Maximum size of allowed headers, in bytes. | [optional][default to 8192]
**http_max_initial_line_length_bytes** | Option<**i32**> | Maximum length of an HTTP URL, in bytes. | [optional][default to 4096]
**indices_query_bool_max_clause_count** | Option<**i32**> | Maximum number of clauses Lucene BooleanQuery can have.  Only increase it if necessary, as it may cause performance issues. | [optional][default to 1024]
**indices_fielddata_cache_size_percentage** | Option<**i32**> | Maximum amount of heap memory used for field data cache, expressed as a percentage. Decreasing the value too much will increase overhead of loading field data. Increasing the value too much will decrease amount of heap available for other operations. | [optional]
**indices_memory_index_buffer_size_percentage** | Option<**i32**> | Total amount of heap used for indexing buffer before writing segments to disk, expressed as a percentage. Too low value will slow down indexing; too high value will increase indexing performance but causes performance issues for query performance. | [optional][default to 10]
**indices_memory_min_index_buffer_size_mb** | Option<**i32**> | Minimum amount of heap used for indexing buffer before writing segments to disk, in mb. Works in conjunction with indices_memory_index_buffer_size_percentage, each being enforced. | [optional][default to 48]
**indices_memory_max_index_buffer_size_mb** | Option<**i32**> | Maximum amount of heap used for indexing buffer before writing segments to disk, in mb. Works in conjunction with indices_memory_index_buffer_size_percentage, each being enforced. The default is unbounded. | [optional]
**indices_queries_cache_size_percentage** | Option<**i32**> | Maximum amount of heap used for query cache.  Too low value will decrease query performance and increase performance for other operations; too high value will cause issues with other functionality. | [optional][default to 10]
**indices_recovery_max_mb_per_sec** | Option<**i32**> | Limits total inbound and outbound recovery traffic for each node, expressed in mb per second. Applies to both peer recoveries as well as snapshot recoveries (i.e., restores from a snapshot). | [optional][default to 40]
**indices_recovery_max_concurrent_file_chunks** | Option<**i32**> | Maximum number of file chunks sent in parallel for each recovery. | [optional][default to 2]
**thread_pool_search_size** | Option<**i32**> | Number of workers in the search operation thread pool.  Do note this may have maximum value depending on CPU count - value is automatically lowered if set to higher than maximum value. | [optional]
**thread_pool_search_throttled_size** | Option<**i32**> | Number of workers in the search throttled operation thread pool. This pool is used for searching frozen indices. Do note this may have maximum value depending on CPU count - value is automatically lowered if set to higher than maximum value. | [optional]
**thread_pool_get_size** | Option<**i32**> | Number of workers in the get operation thread pool.  Do note this may have maximum value depending on CPU count - value is automatically lowered if set to higher than maximum value. | [optional]
**thread_pool_analyze_size** | Option<**i32**> | Number of workers in the analyze operation thread pool.  Do note this may have maximum value depending on CPU count - value is automatically lowered if set to higher than maximum value. | [optional]
**thread_pool_write_size** | Option<**i32**> | Number of workers in the write operation thread pool.  Do note this may have maximum value depending on CPU count - value is automatically lowered if set to higher than maximum value. | [optional]
**thread_pool_force_merge_size** | Option<**i32**> | Number of workers in the force merge operation thread pool. This pool is used for forcing a merge between shards of one or more indices. Do note this may have maximum value depending on CPU count - value is automatically lowered if set to higher than maximum value. | [optional]
**thread_pool_search_queue_size** | Option<**i32**> | Size of queue for operations in the search thread pool. | [optional]
**thread_pool_search_throttled_queue_size** | Option<**i32**> | Size of queue for operations in the search throttled thread pool. | [optional]
**thread_pool_get_queue_size** | Option<**i32**> | Size of queue for operations in the get thread pool. | [optional]
**thread_pool_analyze_queue_size** | Option<**i32**> | Size of queue for operations in the analyze thread pool. | [optional]
**thread_pool_write_queue_size** | Option<**i32**> | Size of queue for operations in the write thread pool. | [optional]
**ism_enabled** | Option<**bool**> | Specifies whether ISM is enabled or not. | [optional][default to true]
**ism_history_enabled** | Option<**bool**> | Specifies whether audit history is enabled or not. The logs from ISM are automatically indexed to a logs document. | [optional][default to true]
**ism_history_max_age_hours** | Option<**i32**> | Maximum age before rolling over the audit history index, in hours. | [optional][default to 24]
**ism_history_max_docs** | Option<**i32**> | Maximum number of documents before rolling over the audit history index. | [optional][default to 2500000]
**ism_history_rollover_check_period_hours** | Option<**i32**> | The time between rollover checks for the audit history index, in hours. | [optional][default to 8]
**ism_history_rollover_retention_period_days** | Option<**i32**> | Length of time long audit history indices are kept, in days. | [optional][default to 30]
**search_max_buckets** | Option<**i32**> | Maximum number of aggregation buckets allowed in a single response. | [optional][default to 10000]
**action_auto_create_index_enabled** | Option<**bool**> | Specifices whether to allow automatic creation of indices. | [optional][default to true]
**enable_security_audit** | Option<**bool**> | Specifies whether to allow security audit logging. | [optional][default to false]
**action_destructive_requires_name** | Option<**bool**> | Specifies whether to require explicit index names when deleting indices. | [optional]
**cluster_max_shards_per_node** | Option<**i32**> | Maximum number of shards allowed per data node. | [optional]
**override_main_response_version** | Option<**bool**> | Compatibility mode sets OpenSearch to report its version as 7.10 so clients continue to work. | [optional][default to false]
**script_max_compilations_rate** | Option<**String**> | Limits the number of inline script compilations within a period of time. Default is use-context | [optional][default to use-context]
**cluster_routing_allocation_node_concurrent_recoveries** | Option<**i32**> | Maximum concurrent incoming/outgoing shard recoveries (normally replicas) are allowed to happen per node . | [optional][default to 2]
**reindex_remote_whitelist** | Option<**Vec<String>**> | Allowlist of remote IP addresses for reindexing. Changing this value will cause all OpenSearch instances to restart. | [optional]
**plugins_alerting_filter_by_backend_roles_enabled** | Option<**bool**> | Enable or disable filtering of alerting by backend roles. | [optional][default to false]
**knn_memory_circuit_breaker_enabled** | Option<**bool**> | Enable or disable KNN memory circuit breaker. | [optional][default to true]
**knn_memory_circuit_breaker_limit** | Option<**i32**> | Maximum amount of memory in percentage that can be used for the KNN index. Defaults to 50% of the JVM heap size.  0 is used to set it to null which can be used to invalidate caches. | [optional][default to 50]
**keep_index_refresh_interval** | Option<**bool**> | DigitalOcean automatically resets the `index.refresh_interval` to the default value (once per second) to  ensure that new documents are quickly available for search queries. If you are setting your own refresh intervals,  you can disable this by setting this field to true. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


