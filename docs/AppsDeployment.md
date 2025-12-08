# AppsDeployment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cause** | Option<**String**> |  | [optional]
**cloned_from** | Option<**String**> |  | [optional]
**created_at** | Option<**String**> |  | [optional]
**id** | Option<**String**> |  | [optional]
**jobs** | Option<[**Vec<models::AppsDeploymentJob>**](apps_deployment_job.md)> |  | [optional]
**functions** | Option<[**Vec<models::AppsDeploymentFunctions>**](apps_deployment_functions.md)> |  | [optional]
**phase** | Option<[**models::AppsDeploymentPhase**](apps_deployment_phase.md)> |  | [optional]
**phase_last_updated_at** | Option<**String**> |  | [optional]
**progress** | Option<[**models::AppsDeploymentProgress**](apps_deployment_progress.md)> |  | [optional]
**services** | Option<[**Vec<models::AppsDeploymentService>**](apps_deployment_service.md)> |  | [optional]
**spec** | Option<[**models::AppSpec**](app_spec.md)> |  | [optional]
**static_sites** | Option<[**Vec<models::AppsDeploymentStaticSite>**](apps_deployment_static_site.md)> |  | [optional]
**tier_slug** | Option<**String**> |  | [optional][readonly]
**updated_at** | Option<**String**> |  | [optional]
**workers** | Option<[**Vec<models::AppsDeploymentWorker>**](apps_deployment_worker.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


