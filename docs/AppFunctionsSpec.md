# AppFunctionsSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cors** | Option<[**models::AppsCorsPolicy**](apps_cors_policy.md)> | (Deprecated - Use Ingress Rules instead). | [optional]
**routes** | Option<[**Vec<models::AppRouteSpec>**](app_route_spec.md)> | (Deprecated - Use Ingress Rules instead). A list of HTTP routes that should be routed to this component. | [optional]
**name** | **String** | The name. Must be unique across all components within the same app. | 
**source_dir** | Option<**String**> | An optional path to the working directory to use for the build. For Dockerfile builds, this will be used as the build context. Must be relative to the root of the repo. | [optional]
**alerts** | Option<[**Vec<models::AppAlertSpec>**](app_alert_spec.md)> |  | [optional]
**envs** | Option<[**Vec<models::AppVariableDefinition>**](app_variable_definition.md)> | A list of environment variables made available to the component. | [optional]
**git** | Option<[**models::AppsGitSourceSpec**](apps_git_source_spec.md)> |  | [optional]
**github** | Option<[**models::AppsGithubSourceSpec**](apps_github_source_spec.md)> |  | [optional]
**gitlab** | Option<[**models::AppsGitlabSourceSpec**](apps_gitlab_source_spec.md)> |  | [optional]
**bitbucket** | Option<[**models::AppsBitbucketSourceSpec**](apps_bitbucket_source_spec.md)> |  | [optional]
**log_destinations** | Option<[**Vec<models::AppLogDestinationDefinition>**](app_log_destination_definition.md)> | A list of configured log forwarding destinations. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


