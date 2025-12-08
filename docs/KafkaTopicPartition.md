# KafkaTopicPartition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**size** | Option<**i32**> | Size of the topic partition in bytes. | [optional]
**id** | Option<**i32**> | An identifier for the partition. | [optional]
**in_sync_replicas** | Option<**i32**> | The number of nodes that are in-sync (have the latest data) for the given partition | [optional]
**earliest_offset** | Option<**i32**> | The earliest consumer offset amongst consumer groups. | [optional]
**consumer_groups** | Option<[**Vec<models::KafkaTopicPartitionConsumerGroupsInner>**](kafka_topic_partition_consumer_groups_inner.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


