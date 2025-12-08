# AppsUpdateAppRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**spec** | [**models::AppSpec**](app_spec.md) |  | 
**update_all_source_versions** | Option<**bool**> | Whether or not to update the source versions (for example fetching a new commit or image digest) of all components. By default (when this is false) only newly added sources will be updated to avoid changes like updating the scale of a component from also updating the respective code. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


