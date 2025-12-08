# ApiUpdateAgentInputPublic

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**agent_log_insights_enabled** | Option<**bool**> |  | [optional]
**allowed_domains** | Option<**Vec<String>**> | Optional list of allowed domains for the chatbot - Must use fully qualified domain name (FQDN) such as https://example.com | [optional]
**anthropic_key_uuid** | Option<**String**> | Optional anthropic key uuid for use with anthropic models | [optional]
**conversation_logs_enabled** | Option<**bool**> | Optional update of conversation logs enabled | [optional]
**description** | Option<**String**> | Agent description | [optional]
**instruction** | Option<**String**> | Agent instruction. Instructions help your agent to perform its job effectively. See [Write Effective Agent Instructions](https://docs.digitalocean.com/products/genai-platform/concepts/best-practices/#agent-instructions) for best practices. | [optional]
**k** | Option<**i64**> | How many results should be considered from an attached knowledge base | [optional]
**max_tokens** | Option<**i64**> | Specifies the maximum number of tokens the model can process in a single input or output, set as a number between 1 and 512. This determines the length of each response. | [optional]
**model_provider_key_uuid** | Option<**String**> | Optional Model Provider uuid for use with provider models | [optional]
**model_uuid** | Option<**String**> | Identifier for the foundation model. | [optional]
**name** | Option<**String**> | Agent name | [optional]
**open_ai_key_uuid** | Option<**String**> | Optional OpenAI key uuid for use with OpenAI models | [optional]
**project_id** | Option<**String**> | The id of the DigitalOcean project this agent will belong to | [optional]
**provide_citations** | Option<**bool**> |  | [optional]
**retrieval_method** | Option<[**models::ApiRetrievalMethod**](apiRetrievalMethod.md)> |  | [optional]
**tags** | Option<**Vec<String>**> | A set of abitrary tags to organize your agent | [optional]
**temperature** | Option<**f32**> | Controls the model’s creativity, specified as a number between 0 and 1. Lower values produce more predictable and conservative responses, while higher values encourage creativity and variation. | [optional]
**top_p** | Option<**f32**> | Defines the cumulative probability threshold for word selection, specified as a number between 0 and 1. Higher values allow for more diverse outputs, while lower values ensure focused and coherent responses. | [optional]
**uuid** | Option<**String**> | Unique agent id | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


