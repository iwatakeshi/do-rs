# MongoAdvancedConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**default_read_concern** | Option<**String**> | Specifies the default consistency behavior of reads from the database. Data that is returned from the query with may or may not have been acknowledged by all nodes in the replicaset depending on this value.  Learn more [here](https://www.mongodb.com/docs/manual/reference/read-concern/). | [optional][default to Local]
**default_write_concern** | Option<**String**> | Describes the level of acknowledgment requested from MongoDB for write operations clusters. This field can set to either `majority` or a number `0...n` which will describe the number of nodes that must acknowledge the write operation before it is fully accepted. Setting to `0` will request no acknowledgement of the write operation.  Learn more [here](https://www.mongodb.com/docs/manual/reference/write-concern/). | [optional][default to majority]
**transaction_lifetime_limit_seconds** | Option<**i32**> | Specifies the lifetime of multi-document transactions. Transactions that exceed this limit are considered expired and will be  aborted by a periodic cleanup process. The cleanup process runs every `transactionLifetimeLimitSeconds/2 seconds` or at least  once every 60 seconds. *Changing this parameter will lead to a restart of the MongoDB service.* Learn more [here](https://www.mongodb.com/docs/manual/reference/parameters/#mongodb-parameter-param.transactionLifetimeLimitSeconds). | [optional][default to 60]
**slow_op_threshold_ms** | Option<**i32**> | Operations that run for longer than this threshold are considered slow which are then recorded to the diagnostic logs.  Higher log levels (verbosity) will record all operations regardless of this threshold on the primary node.  *Changing this parameter will lead to a restart of the MongoDB service.* Learn more [here](https://www.mongodb.com/docs/manual/reference/configuration-options/#mongodb-setting-operationProfiling.slowOpThresholdMs). | [optional][default to 100]
**verbosity** | Option<**i32**> | The log message verbosity level. The verbosity level determines the amount of Informational and Debug messages MongoDB outputs. 0 includes informational messages while 1...5 increases the level to include debug messages. *Changing this parameter will lead to a restart of the MongoDB service.* Learn more [here](https://www.mongodb.com/docs/manual/reference/configuration-options/#mongodb-setting-systemLog.verbosity). | [optional][default to 0]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


