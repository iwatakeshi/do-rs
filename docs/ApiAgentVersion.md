# ApiAgentVersion

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**agent_uuid** | Option<**String**> | Uuid of the agent this version belongs to | [optional]
**attached_child_agents** | Option<[**Vec<models::ApiAgentChildRelationshipVerion>**](apiAgentChildRelationshipVerion.md)> | List of child agent relationships | [optional]
**attached_functions** | Option<[**Vec<models::ApiAgentFunctionVersion>**](apiAgentFunctionVersion.md)> | List of function versions | [optional]
**attached_guardrails** | Option<[**Vec<models::ApiAgentGuardrailVersion>**](apiAgentGuardrailVersion.md)> | List of guardrail version | [optional]
**attached_knowledgebases** | Option<[**Vec<models::ApiAgentKnowledgeBaseVersion>**](apiAgentKnowledgeBaseVersion.md)> | List of knowledge base agent versions | [optional]
**can_rollback** | Option<**bool**> | Whether the version is able to be rolled back to | [optional]
**created_at** | Option<**String**> | Creation date | [optional]
**created_by_email** | Option<**String**> | User who created this version | [optional]
**currently_applied** | Option<**bool**> | Whether this is the currently applied configuration | [optional]
**description** | Option<**String**> | Description of the agent | [optional]
**id** | Option<**String**> | Unique identifier | [optional]
**instruction** | Option<**String**> | Instruction for the agent | [optional]
**k** | Option<**i64**> | K value for the agent's configuration | [optional]
**max_tokens** | Option<**i64**> | Max tokens setting for the agent | [optional]
**model_name** | Option<**String**> | Name of model associated to the agent version | [optional]
**name** | Option<**String**> | Name of the agent | [optional]
**provide_citations** | Option<**bool**> | Whether the agent should provide in-response citations | [optional]
**retrieval_method** | Option<[**models::ApiRetrievalMethod**](apiRetrievalMethod.md)> |  | [optional]
**tags** | Option<**Vec<String>**> | Tags associated with the agent | [optional]
**temperature** | Option<**f32**> | Temperature setting for the agent | [optional]
**top_p** | Option<**f32**> | Top_p setting for the agent | [optional]
**trigger_action** | Option<**String**> | Action triggering the configuration update | [optional]
**version_hash** | Option<**String**> | Version hash | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


