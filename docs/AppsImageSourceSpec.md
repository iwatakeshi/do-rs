# AppsImageSourceSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**registry** | Option<**String**> | The registry name. Must be left empty for the `DOCR` registry type. | [optional]
**registry_type** | Option<**String**> | - DOCKER_HUB: The DockerHub container registry type. - DOCR: The DigitalOcean container registry type. - GHCR: The Github container registry type. | [optional]
**registry_credentials** | Option<**String**> | The credentials to be able to pull the image. The value will be encrypted on first submission. On following submissions, the encrypted value should be used. - \"$username:$access_token\" for registries of type `DOCKER_HUB`. - \"$username:$access_token\" for registries of type `GHCR`. | [optional]
**repository** | Option<**String**> | The repository name. | [optional]
**tag** | Option<**String**> | The repository tag. Defaults to `latest` if not provided and no digest is provided. Cannot be specified if digest is provided. | [optional][default to latest]
**digest** | Option<**String**> | The image digest. Cannot be specified if tag is provided. | [optional]
**deploy_on_push** | Option<[**models::AppsImageSourceSpecDeployOnPush**](apps_image_source_spec_deploy_on_push.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


