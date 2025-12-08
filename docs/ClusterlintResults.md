# ClusterlintResults

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**run_id** | Option<**String**> | Id of the clusterlint run that can be used later to fetch the diagnostics. | [optional]
**requested_at** | Option<**String**> | A time value given in ISO8601 combined date and time format that represents when the schedule clusterlint run request was made. | [optional]
**completed_at** | Option<**String**> | A time value given in ISO8601 combined date and time format that represents when the schedule clusterlint run request was completed. | [optional]
**diagnostics** | Option<[**Vec<models::ClusterlintResultsDiagnosticsInner>**](clusterlint_results_diagnostics_inner.md)> | An array of diagnostics reporting potential problems for the given cluster. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


