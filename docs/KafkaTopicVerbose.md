# KafkaTopicVerbose

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the Kafka topic. | [optional]
**state** | Option<**String**> | The state of the Kafka topic. | [optional]
**replication_factor** | Option<**i32**> | The number of nodes to replicate data across the cluster. | [optional]
**partitions** | Option<[**Vec<models::KafkaTopicPartition>**](kafka_topic_partition.md)> |  | [optional]
**config** | Option<[**models::KafkaTopicConfig**](kafka_topic_config.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


