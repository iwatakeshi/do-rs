# AppJobSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name. Must be unique across all components within the same app. | 
**git** | Option<[**models::AppsGitSourceSpec**](apps_git_source_spec.md)> |  | [optional]
**github** | Option<[**models::AppsGithubSourceSpec**](apps_github_source_spec.md)> |  | [optional]
**gitlab** | Option<[**models::AppsGitlabSourceSpec**](apps_gitlab_source_spec.md)> |  | [optional]
**bitbucket** | Option<[**models::AppsBitbucketSourceSpec**](apps_bitbucket_source_spec.md)> |  | [optional]
**image** | Option<[**models::AppsImageSourceSpec**](apps_image_source_spec.md)> |  | [optional]
**dockerfile_path** | Option<**String**> | The path to the Dockerfile relative to the root of the repo. If set, it will be used to build this component. Otherwise, App Platform will attempt to build it using buildpacks. | [optional]
**build_command** | Option<**String**> | An optional build command to run while building this component from source. | [optional]
**run_command** | Option<**String**> | An optional run command to override the component's default. | [optional]
**source_dir** | Option<**String**> | An optional path to the working directory to use for the build. For Dockerfile builds, this will be used as the build context. Must be relative to the root of the repo. | [optional]
**envs** | Option<[**Vec<models::AppVariableDefinition>**](app_variable_definition.md)> | A list of environment variables made available to the component. | [optional]
**environment_slug** | Option<**String**> | An environment slug describing the type of this app. For a full list, please refer to [the product documentation](https://docs.digitalocean.com/products/app-platform/). | [optional]
**log_destinations** | Option<[**Vec<models::AppLogDestinationDefinition>**](app_log_destination_definition.md)> | A list of configured log forwarding destinations. | [optional]
**instance_count** | Option<**i64**> | The amount of instances that this component should be scaled to. Default: 1. Must not be set if autoscaling is used. | [optional][default to 1]
**instance_size_slug** | Option<**String**> | The instance size to use for this component. Default: `apps-s-1vcpu-0.5gb` | [optional]
**autoscaling** | Option<[**models::AppComponentInstanceBaseAutoscaling**](app_component_instance_base_autoscaling.md)> |  | [optional]
**kind** | Option<**String**> | - UNSPECIFIED: Default job type, will auto-complete to POST_DEPLOY kind. - PRE_DEPLOY: Indicates a job that runs before an app deployment. - POST_DEPLOY: Indicates a job that runs after an app deployment. - FAILED_DEPLOY: Indicates a job that runs after a component fails to deploy. | [optional][default to Unspecified]
**termination** | Option<[**models::AppJobSpecTermination**](app_job_spec_termination.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


