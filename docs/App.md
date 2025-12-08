# App

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active_deployment** | Option<[**models::AppsDeployment**](apps_deployment.md)> |  | [optional]
**created_at** | Option<**String**> |  | [optional][readonly]
**default_ingress** | Option<**String**> |  | [optional][readonly]
**domains** | Option<[**Vec<models::AppsDomain>**](apps_domain.md)> |  | [optional][readonly]
**id** | Option<**String**> |  | [optional][readonly]
**in_progress_deployment** | Option<[**models::AppsDeployment**](apps_deployment.md)> |  | [optional]
**last_deployment_created_at** | Option<**String**> |  | [optional][readonly]
**live_domain** | Option<**String**> |  | [optional][readonly]
**live_url** | Option<**String**> |  | [optional][readonly]
**live_url_base** | Option<**String**> |  | [optional][readonly]
**owner_uuid** | Option<**String**> |  | [optional][readonly]
**pending_deployment** | Option<[**models::AppsDeployment**](apps_deployment.md)> | The most recent pending deployment. For CreateApp and UpdateApp transactions this is guaranteed to reflect the associated deployment. | [optional]
**project_id** | Option<**String**> | Requires `project:read` scope. | [optional][readonly]
**region** | Option<[**models::AppsRegion**](apps_region.md)> |  | [optional]
**spec** | [**models::AppSpec**](app_spec.md) |  | 
**tier_slug** | Option<**String**> |  | [optional][readonly]
**updated_at** | Option<**String**> |  | [optional][readonly]
**pinned_deployment** | Option<[**models::AppsDeployment**](apps_deployment.md)> | The deployment that the app is pinned to. | [optional]
**dedicated_ips** | Option<[**Vec<models::AppsDedicatedEgressIp>**](apps_dedicated_egress_ip.md)> |  | [optional][readonly]
**vpc** | Option<[**models::AppsVpc**](apps_vpc.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


