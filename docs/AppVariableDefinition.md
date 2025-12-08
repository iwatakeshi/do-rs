# AppVariableDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **String** | The variable name | 
**scope** | Option<**String**> | - RUN_TIME: Made available only at run-time - BUILD_TIME: Made available only at build-time - RUN_AND_BUILD_TIME: Made available at both build and run-time | [optional][default to RunAndBuildTime]
**r#type** | Option<**String**> | - GENERAL: A plain-text environment variable - SECRET: A secret encrypted environment variable | [optional][default to General]
**value** | Option<**String**> | The value. If the type is `SECRET`, the value will be encrypted on first submission. On following submissions, the encrypted value should be used. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


