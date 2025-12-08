# PgbouncerAdvancedConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**server_reset_query_always** | Option<**bool**> | Run server_reset_query (DISCARD ALL) in all pooling modes. | [optional]
**ignore_startup_parameters** | Option<**Vec<String>**> | List of parameters to ignore when given in startup packet. | [optional]
**min_pool_size** | Option<**i32**> | If current server connections are below this number, adds more. Improves behavior when usual load comes suddenly back after period of total inactivity. The value is effectively capped at the pool size. | [optional]
**server_lifetime** | Option<**i32**> | The pooler closes any unused server connection that has been connected longer than this amount of seconds. | [optional]
**server_idle_timeout** | Option<**i32**> | Drops server connections if they have been idle more than this many seconds.  If 0, timeout is disabled.  | [optional]
**autodb_pool_size** | Option<**i32**> | If non-zero, automatically creates a pool of that size per user when a pool doesn't exist. | [optional]
**autodb_pool_mode** | Option<**String**> | PGBouncer pool mode | [optional]
**autodb_max_db_connections** | Option<**i32**> | Only allows a maximum this many server connections per database (regardless of user). If 0, allows unlimited connections. | [optional]
**autodb_idle_timeout** | Option<**i32**> | If the automatically-created database pools have been unused this many seconds, they are freed. If 0, timeout is disabled. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


