# AppsValidateRollback200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**valid** | Option<**bool**> | Indicates whether the app can be rolled back to the specified deployment. | [optional]
**error** | Option<[**models::AppRollbackValidationCondition**](app_rollback_validation_condition.md)> | Contains the failing condition that is causing the rollback to be invalid. | [optional]
**warnings** | Option<[**Vec<models::AppRollbackValidationCondition>**](app_rollback_validation_condition.md)> | Contains a list of warnings that may cause the rollback to run under unideal circumstances. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


