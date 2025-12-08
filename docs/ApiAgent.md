# ApiAgent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**anthropic_api_key** | Option<[**models::ApiAnthropicApiKeyInfo**](apiAnthropicAPIKeyInfo.md)> |  | [optional]
**api_key_infos** | Option<[**Vec<models::ApiAgentApiKeyInfo>**](apiAgentAPIKeyInfo.md)> | Api key infos | [optional]
**api_keys** | Option<[**Vec<models::ApiAgentApiKey>**](apiAgentAPIKey.md)> | Api keys | [optional]
**chatbot** | Option<[**models::ApiChatbot**](apiChatbot.md)> |  | [optional]
**chatbot_identifiers** | Option<[**Vec<models::ApiAgentChatbotIdentifier>**](apiAgentChatbotIdentifier.md)> | Chatbot identifiers | [optional]
**child_agents** | Option<[**Vec<models::ApiAgent>**](apiAgent.md)> | Child agents | [optional]
**conversation_logs_enabled** | Option<**bool**> | Whether conversation logs are enabled for the agent | [optional]
**created_at** | Option<**String**> | Creation date / time | [optional]
**deployment** | Option<[**models::ApiDeployment**](apiDeployment.md)> |  | [optional]
**description** | Option<**String**> | Description of agent | [optional]
**functions** | Option<[**Vec<models::ApiAgentFunction>**](apiAgentFunction.md)> |  | [optional]
**guardrails** | Option<[**Vec<models::ApiAgentGuardrail>**](apiAgentGuardrail.md)> | The guardrails the agent is attached to | [optional]
**if_case** | Option<**String**> |  | [optional]
**instruction** | Option<**String**> | Agent instruction. Instructions help your agent to perform its job effectively. See [Write Effective Agent Instructions](https://docs.digitalocean.com/products/genai-platform/concepts/best-practices/#agent-instructions) for best practices. | [optional]
**k** | Option<**i64**> |  | [optional]
**knowledge_bases** | Option<[**Vec<models::ApiKnowledgeBase>**](apiKnowledgeBase.md)> | Knowledge bases | [optional]
**logging_config** | Option<[**models::ApiAgentLoggingConfig**](apiAgentLoggingConfig.md)> |  | [optional]
**max_tokens** | Option<**i64**> |  | [optional]
**model** | Option<[**models::ApiModel**](apiModel.md)> |  | [optional]
**model_provider_key** | Option<[**models::ApiModelProviderKeyInfo**](apiModelProviderKeyInfo.md)> |  | [optional]
**name** | Option<**String**> | Agent name | [optional]
**openai_api_key** | Option<[**models::ApiOpenAiapiKeyInfo**](apiOpenAIAPIKeyInfo.md)> |  | [optional]
**parent_agents** | Option<[**Vec<models::ApiAgent>**](apiAgent.md)> | Parent agents | [optional]
**project_id** | Option<**String**> |  | [optional]
**provide_citations** | Option<**bool**> | Whether the agent should provide in-response citations | [optional]
**region** | Option<**String**> | Region code | [optional]
**retrieval_method** | Option<[**models::ApiRetrievalMethod**](apiRetrievalMethod.md)> |  | [optional]
**route_created_at** | Option<**String**> | Creation of route date / time | [optional]
**route_created_by** | Option<**String**> |  | [optional]
**route_name** | Option<**String**> | Route name | [optional]
**route_uuid** | Option<**String**> |  | [optional]
**tags** | Option<**Vec<String>**> | Agent tag to organize related resources | [optional]
**temperature** | Option<**f32**> |  | [optional]
**template** | Option<[**models::ApiAgentTemplate**](apiAgentTemplate.md)> |  | [optional]
**top_p** | Option<**f32**> |  | [optional]
**updated_at** | Option<**String**> | Last modified | [optional]
**url** | Option<**String**> | Access your agent under this url | [optional]
**user_id** | Option<**String**> | Id of user that created the agent | [optional]
**uuid** | Option<**String**> | Unique agent id | [optional]
**version_hash** | Option<**String**> | The latest version of the agent | [optional]
**vpc_egress_ips** | Option<**Vec<String>**> | VPC Egress IPs | [optional]
**vpc_uuid** | Option<**String**> |  | [optional]
**workspace** | Option<[**models::ApiWorkspace**](apiWorkspace.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


