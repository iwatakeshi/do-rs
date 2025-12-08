# AppStaticSiteSpec

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
**index_document** | Option<**String**> | The name of the index document to use when serving this static site. Default: index.html | [optional][default to index.html]
**error_document** | Option<**String**> | The name of the error document to use when serving this static site. Default: 404.html. If no such file exists within the built assets, App Platform will supply one. | [optional][default to 404.html]
**catchall_document** | Option<**String**> | The name of the document to use as the fallback for any requests to documents that are not found when serving this static site. Only 1 of `catchall_document` or `error_document` can be set. | [optional]
**output_dir** | Option<**String**> | An optional path to where the built assets will be located, relative to the build context. If not set, App Platform will automatically scan for these directory names: `_static`, `dist`, `public`, `build`. | [optional]
**cors** | Option<[**models::AppsCorsPolicy**](apps_cors_policy.md)> | (Deprecated - Use Ingress Rules instead). | [optional]
**routes** | Option<[**Vec<models::AppRouteSpec>**](app_route_spec.md)> | (Deprecated - Use Ingress Rules instead). A list of HTTP routes that should be routed to this component. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


