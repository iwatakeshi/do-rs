# ApiAgentTemplate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> | The agent template's creation date | [optional]
**description** | Option<**String**> | Deprecated - Use summary instead | [optional]
**guardrails** | Option<[**Vec<models::ApiAgentTemplateGuardrail>**](apiAgentTemplateGuardrail.md)> | List of guardrails associated with the agent template | [optional]
**instruction** | Option<**String**> | Instructions for the agent template | [optional]
**k** | Option<**i64**> | The 'k' value for the agent template | [optional]
**knowledge_bases** | Option<[**Vec<models::ApiKnowledgeBase>**](apiKnowledgeBase.md)> | List of knowledge bases associated with the agent template | [optional]
**long_description** | Option<**String**> | The long description of the agent template | [optional]
**max_tokens** | Option<**i64**> | The max_tokens setting for the agent template | [optional]
**model** | Option<[**models::ApiModel**](apiModel.md)> |  | [optional]
**name** | Option<**String**> | Name of the agent template | [optional]
**short_description** | Option<**String**> | The short description of the agent template | [optional]
**summary** | Option<**String**> | The summary of the agent template | [optional]
**tags** | Option<**Vec<String>**> | List of tags associated with the agent template | [optional]
**temperature** | Option<**f32**> | The temperature setting for the agent template | [optional]
**template_type** | Option<[**models::ApiAgentTemplateType**](apiAgentTemplateType.md)> |  | [optional]
**top_p** | Option<**f32**> | The top_p setting for the agent template | [optional]
**updated_at** | Option<**String**> | The agent template's last updated date | [optional]
**uuid** | Option<**String**> | Unique id | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


