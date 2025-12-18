# ApiEvaluationTraceSpan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> | When the span was created | [optional]
**input** | Option<[**serde_json::Value**](.md)> | Input data for the span (flexible structure - can be messages array, string, etc.) | [optional]
**name** | Option<**String**> | Name/identifier for the span | [optional]
**output** | Option<[**serde_json::Value**](.md)> | Output data from the span (flexible structure - can be message, string, etc.) | [optional]
**retriever_chunks** | Option<[**Vec<models::ApiPromptChunk>**](apiPromptChunk.md)> | Any retriever span chunks that were included as part of the span. | [optional]
**span_level_metric_results** | Option<[**Vec<models::ApiEvaluationMetricResult>**](apiEvaluationMetricResult.md)> | The span-level metric results. | [optional]
**r#type** | Option<[**models::ApiTraceSpanType**](apiTraceSpanType.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


