# KafkaTopicUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**replication_factor** | Option<**i32**> | The number of nodes to replicate data across the cluster. | [optional]
**partition_count** | Option<**i32**> | The number of partitions available for the topic. On update, this value can only be increased. | [optional]
**config** | Option<[**models::KafkaTopicConfig**](kafka_topic_config.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


