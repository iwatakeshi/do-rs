# MysqlAdvancedConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**backup_hour** | Option<**i32**> | The hour of day (in UTC) when backup for the service starts. New backup only starts if previous backup has already completed. | [optional]
**backup_minute** | Option<**i32**> | The minute of the backup hour when backup for the service starts. New backup  only starts if previous backup has already completed. | [optional]
**sql_mode** | Option<**String**> | Global SQL mode. If empty, uses MySQL server defaults. Must only include uppercase alphabetic characters, underscores, and commas. | [optional]
**connect_timeout** | Option<**i32**> | The number of seconds that the mysqld server waits for a connect packet before responding with bad handshake. | [optional]
**default_time_zone** | Option<**String**> | Default server time zone, in the form of an offset from UTC (from -12:00 to +12:00), a time zone name (EST), or 'SYSTEM' to use the MySQL server default. | [optional]
**group_concat_max_len** | Option<**i32**> | The maximum permitted result length, in bytes, for the GROUP_CONCAT() function. | [optional]
**information_schema_stats_expiry** | Option<**i32**> | The time, in seconds, before cached statistics expire. | [optional]
**innodb_ft_min_token_size** | Option<**i32**> | The minimum length of words that an InnoDB FULLTEXT index stores. | [optional]
**innodb_ft_server_stopword_table** | Option<**String**> | The InnoDB FULLTEXT index stopword list for all InnoDB tables. | [optional]
**innodb_lock_wait_timeout** | Option<**i32**> | The time, in seconds, that an InnoDB transaction waits for a row lock. before giving up. | [optional]
**innodb_log_buffer_size** | Option<**i32**> | The size of the buffer, in bytes, that InnoDB uses to write to the log files. on disk. | [optional]
**innodb_online_alter_log_max_size** | Option<**i32**> | The upper limit, in bytes, of the size of the temporary log files used during online DDL operations for InnoDB tables. | [optional]
**innodb_print_all_deadlocks** | Option<**bool**> | When enabled, records information about all deadlocks in InnoDB user transactions  in the error log. Disabled by default. | [optional]
**innodb_rollback_on_timeout** | Option<**bool**> | When enabled, transaction timeouts cause InnoDB to abort and roll back the entire transaction. | [optional]
**interactive_timeout** | Option<**i32**> | The time, in seconds, the server waits for activity on an interactive. connection before closing it. | [optional]
**internal_tmp_mem_storage_engine** | Option<**String**> | The storage engine for in-memory internal temporary tables. | [optional]
**net_read_timeout** | Option<**i32**> | The time, in seconds, to wait for more data from an existing connection. aborting the read. | [optional]
**net_write_timeout** | Option<**i32**> | The number of seconds to wait for a block to be written to a connection before aborting the write. | [optional]
**sql_require_primary_key** | Option<**bool**> | Require primary key to be defined for new tables or old tables modified with ALTER TABLE and fail if missing. It is recommended to always have primary keys because various functionality may break if any large table is missing them. | [optional]
**wait_timeout** | Option<**i32**> | The number of seconds the server waits for activity on a noninteractive connection before closing it. | [optional]
**max_allowed_packet** | Option<**i32**> | The size of the largest message, in bytes, that can be received by the server. Default is 67108864 (64M). | [optional]
**max_heap_table_size** | Option<**i32**> | The maximum size, in bytes, of internal in-memory tables. Also set tmp_table_size. Default is 16777216 (16M) | [optional]
**sort_buffer_size** | Option<**i32**> | The sort buffer size, in bytes, for ORDER BY optimization. Default is 262144. (256K). | [optional]
**tmp_table_size** | Option<**i32**> | The maximum size, in bytes, of internal in-memory tables. Also set max_heap_table_size. Default is 16777216 (16M). | [optional]
**slow_query_log** | Option<**bool**> | When enabled, captures slow queries. When disabled, also truncates the mysql.slow_log table. Default is false. | [optional]
**long_query_time** | Option<**f64**> | The time, in seconds, for a query to take to execute before  being captured by slow_query_logs. Default is 10 seconds. | [optional]
**binlog_retention_period** | Option<**f64**> | The minimum amount of time, in seconds, to keep binlog entries before deletion.  This may be extended for services that require binlog entries for longer than the default, for example if using the MySQL Debezium Kafka connector. | [optional]
**innodb_change_buffer_max_size** | Option<**i32**> | Specifies the maximum size of the InnoDB change buffer as a percentage of the buffer pool. | [optional]
**innodb_flush_neighbors** | Option<**i32**> | Specifies whether flushing a page from the InnoDB buffer pool also flushes other dirty pages in the same extent.   - 0 &mdash; disables this functionality, dirty pages in the same extent are not flushed.   - 1 &mdash; flushes contiguous dirty pages in the same extent.   - 2 &mdash; flushes dirty pages in the same extent. | [optional]
**innodb_read_io_threads** | Option<**i32**> | The number of I/O threads for read operations in InnoDB. Changing this parameter will lead to a restart of the MySQL service. | [optional]
**innodb_write_io_threads** | Option<**i32**> | The number of I/O threads for write operations in InnoDB. Changing this parameter will lead to a restart of the MySQL service. | [optional]
**innodb_thread_concurrency** | Option<**i32**> | Defines the maximum number of threads permitted inside of InnoDB. A value of 0 (the default) is interpreted as infinite concurrency (no limit). This variable is intended for performance  tuning on high concurrency systems. | [optional]
**net_buffer_length** | Option<**i32**> | Start sizes of connection buffer and result buffer, must be multiple of 1024. Changing this parameter will lead to a restart of the MySQL service. | [optional]
**log_output** | Option<**String**> | Defines the destination for logs. Can be `INSIGHTS`, `TABLE`, or both (`INSIGHTS,TABLE`), or `NONE` to disable logs. To specify both destinations, use `INSIGHTS,TABLE` (order matters). Default is NONE. | [optional][default to None]
**mysql_incremental_backup** | Option<[**models::MysqlIncrementalBackup**](mysql_incremental_backup.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


