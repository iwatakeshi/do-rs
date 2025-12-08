# ApiCreateAgentInputPublic

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**anthropic_key_uuid** | Option<**String**> | Optional Anthropic API key ID to use with Anthropic models | [optional]
**description** | Option<**String**> | A text description of the agent, not used in inference | [optional]
**instruction** | Option<**String**> | Agent instruction. Instructions help your agent to perform its job effectively. See [Write Effective Agent Instructions](https://docs.digitalocean.com/products/genai-platform/concepts/best-practices/#agent-instructions) for best practices. | [optional]
**knowledge_base_uuid** | Option<**Vec<String>**> | Ids of the knowledge base(s) to attach to the agent | [optional]
**model_provider_key_uuid** | Option<**String**> |  | [optional]
**model_uuid** | Option<**String**> | Identifier for the foundation model. | [optional]
**name** | Option<**String**> | Agent name | [optional]
**open_ai_key_uuid** | Option<**String**> | Optional OpenAI API key ID to use with OpenAI models | [optional]
**project_id** | Option<**String**> | The id of the DigitalOcean project this agent will belong to | [optional]
**region** | Option<**String**> | The DigitalOcean region to deploy your agent in | [optional]
**tags** | Option<**Vec<String>**> | Agent tag to organize related resources | [optional]
**workspace_uuid** | Option<**String**> | Identifier for the workspace | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


