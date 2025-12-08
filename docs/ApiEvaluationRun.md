# ApiEvaluationRun

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**agent_deleted** | Option<**bool**> | Whether agent is deleted | [optional]
**agent_name** | Option<**String**> | Agent name | [optional]
**agent_uuid** | Option<**String**> | Agent UUID. | [optional]
**agent_version_hash** | Option<**String**> | Version hash | [optional]
**agent_workspace_uuid** | Option<**String**> | Agent workspace uuid | [optional]
**created_by_user_email** | Option<**String**> |  | [optional]
**created_by_user_id** | Option<**String**> |  | [optional]
**error_description** | Option<**String**> | The error description | [optional]
**evaluation_run_uuid** | Option<**String**> | Evaluation run UUID. | [optional]
**evaluation_test_case_workspace_uuid** | Option<**String**> | Evaluation test case workspace uuid | [optional]
**finished_at** | Option<**String**> | Run end time. | [optional]
**pass_status** | Option<**bool**> | The pass status of the evaluation run based on the star metric. | [optional]
**queued_at** | Option<**String**> | Run queued time. | [optional]
**run_level_metric_results** | Option<[**Vec<models::ApiEvaluationMetricResult>**](apiEvaluationMetricResult.md)> |  | [optional]
**run_name** | Option<**String**> | Run name. | [optional]
**star_metric_result** | Option<[**models::ApiEvaluationMetricResult**](apiEvaluationMetricResult.md)> |  | [optional]
**started_at** | Option<**String**> | Run start time. | [optional]
**status** | Option<[**models::ApiEvaluationRunStatus**](apiEvaluationRunStatus.md)> |  | [optional]
**test_case_description** | Option<**String**> | Test case description. | [optional]
**test_case_name** | Option<**String**> | Test case name. | [optional]
**test_case_uuid** | Option<**String**> | Test-case UUID. | [optional]
**test_case_version** | Option<**i64**> | Test-case-version. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


