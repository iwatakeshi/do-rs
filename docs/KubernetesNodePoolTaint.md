# KubernetesNodePoolTaint

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | Option<**String**> | An arbitrary string. The `key` and `value` fields of the `taint` object form a key-value pair. For example, if the value of the `key` field is \"special\" and the value of the `value` field is \"gpu\", the key value pair would be `special=gpu`. | [optional]
**value** | Option<**String**> | An arbitrary string. The `key` and `value` fields of the `taint` object form a key-value pair. For example, if the value of the `key` field is \"special\" and the value of the `value` field is \"gpu\", the key value pair would be `special=gpu`. | [optional]
**effect** | Option<**String**> | How the node reacts to pods that it won't tolerate. Available effect values are `NoSchedule`, `PreferNoSchedule`, and `NoExecute`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


