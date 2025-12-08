# ApiUpdateAgentFunctionInputPublic

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**agent_uuid** | Option<**String**> | Agent id | [optional]
**description** | Option<**String**> | Funciton description | [optional]
**faas_name** | Option<**String**> | The name of the function in the DigitalOcean functions platform | [optional]
**faas_namespace** | Option<**String**> | The namespace of the function in the DigitalOcean functions platform | [optional]
**function_name** | Option<**String**> | Function name | [optional]
**function_uuid** | Option<**String**> | Function id | [optional]
**input_schema** | Option<[**serde_json::Value**](.md)> | Describe the input schema for the function so the agent may call it | [optional]
**output_schema** | Option<[**serde_json::Value**](.md)> | Describe the output schema for the function so the agent handle its response | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


