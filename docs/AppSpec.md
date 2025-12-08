# AppSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the app. Must be unique across all apps in the same account. | 
**region** | Option<**String**> | The slug form of the geographical origin of the app. Default: `nearest available` | [optional]
**disable_edge_cache** | Option<**bool**> | If set to `true`, the app will **not** be cached at the edge (CDN). Enable this option if you want to manage CDN configuration yourself—whether by using an external CDN provider or by handling static content and caching within your app. This setting is also recommended for apps that require real-time data or serve dynamic content, such as those using Server-Sent Events (SSE) over GET, or hosting an MCP (Model Context Protocol) Server that utilizes SSE.   **Note:** This feature is not available for static site components.   For more information, see [Disable CDN Cache](https://docs.digitalocean.com/products/app-platform/how-to/cache-content/#disable-cdn-cache). | [optional][default to false]
**disable_email_obfuscation** | Option<**bool**> | If set to `true`, email addresses in the app will not be obfuscated. This is useful for apps that require email addresses to be visible (in the HTML markup). | [optional][default to false]
**enhanced_threat_control_enabled** | Option<**bool**> | If set to `true`, suspicious requests will go through additional security checks to help mitigate layer 7 DDoS attacks. | [optional][default to false]
**domains** | Option<[**Vec<models::AppDomainSpec>**](app_domain_spec.md)> | A set of hostnames where the application will be available. | [optional]
**services** | Option<[**Vec<models::AppServiceSpec>**](app_service_spec.md)> | Workloads which expose publicly-accessible HTTP services. | [optional]
**static_sites** | Option<[**Vec<models::AppStaticSiteSpec>**](app_static_site_spec.md)> | Content which can be rendered to static web assets. | [optional]
**jobs** | Option<[**Vec<models::AppJobSpec>**](app_job_spec.md)> | Pre and post deployment workloads which do not expose publicly-accessible HTTP routes. | [optional]
**workers** | Option<[**Vec<models::AppWorkerSpec>**](app_worker_spec.md)> | Workloads which do not expose publicly-accessible HTTP services. | [optional]
**functions** | Option<[**Vec<models::AppFunctionsSpec>**](app_functions_spec.md)> | Workloads which expose publicly-accessible HTTP services via Functions Components. | [optional]
**databases** | Option<[**Vec<models::AppDatabaseSpec>**](app_database_spec.md)> | Database instances which can provide persistence to workloads within the application. | [optional]
**ingress** | Option<[**models::AppIngressSpec**](app_ingress_spec.md)> |  | [optional]
**egress** | Option<[**models::AppEgressSpec**](app_egress_spec.md)> |  | [optional]
**maintenance** | Option<[**models::AppMaintenanceSpec**](app_maintenance_spec.md)> |  | [optional]
**vpc** | Option<[**models::AppsVpc**](apps_vpc.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


