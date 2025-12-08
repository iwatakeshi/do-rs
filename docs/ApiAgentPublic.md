# ApiAgentPublic

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**chatbot** | Option<[**models::ApiChatbot**](apiChatbot.md)> |  | [optional]
**chatbot_identifiers** | Option<[**Vec<models::ApiAgentChatbotIdentifier>**](apiAgentChatbotIdentifier.md)> | Chatbot identifiers | [optional]
**created_at** | Option<**String**> | Creation date / time | [optional]
**deployment** | Option<[**models::ApiDeployment**](apiDeployment.md)> |  | [optional]
**description** | Option<**String**> | Description of agent | [optional]
**if_case** | Option<**String**> | Instructions to the agent on how to use the route | [optional]
**instruction** | Option<**String**> | Agent instruction. Instructions help your agent to perform its job effectively. See [Write Effective Agent Instructions](https://docs.digitalocean.com/products/genai-platform/concepts/best-practices/#agent-instructions) for best practices. | [optional]
**k** | Option<**i64**> | How many results should be considered from an attached knowledge base | [optional]
**max_tokens** | Option<**i64**> | Specifies the maximum number of tokens the model can process in a single input or output, set as a number between 1 and 512. This determines the length of each response. | [optional]
**model** | Option<[**models::ApiModel**](apiModel.md)> |  | [optional]
**name** | Option<**String**> | Agent name | [optional]
**project_id** | Option<**String**> | The DigitalOcean project ID associated with the agent | [optional]
**provide_citations** | Option<**bool**> | Whether the agent should provide in-response citations | [optional]
**region** | Option<**String**> | Region code | [optional]
**retrieval_method** | Option<[**models::ApiRetrievalMethod**](apiRetrievalMethod.md)> |  | [optional]
**route_created_at** | Option<**String**> | Creation of route date / time | [optional]
**route_created_by** | Option<**String**> | Id of user that created the route | [optional]
**route_name** | Option<**String**> | Route name | [optional]
**route_uuid** | Option<**String**> | Route uuid | [optional]
**tags** | Option<**Vec<String>**> | A set of abitrary tags to organize your agent | [optional]
**temperature** | Option<**f32**> | Controls the model’s creativity, specified as a number between 0 and 1. Lower values produce more predictable and conservative responses, while higher values encourage creativity and variation. | [optional]
**template** | Option<[**models::ApiAgentTemplate**](apiAgentTemplate.md)> |  | [optional]
**top_p** | Option<**f32**> | Defines the cumulative probability threshold for word selection, specified as a number between 0 and 1. Higher values allow for more diverse outputs, while lower values ensure focused and coherent responses. | [optional]
**updated_at** | Option<**String**> | Last modified | [optional]
**url** | Option<**String**> | Access your agent under this url | [optional]
**user_id** | Option<**String**> | Id of user that created the agent | [optional]
**uuid** | Option<**String**> | Unique agent id | [optional]
**version_hash** | Option<**String**> | The latest version of the agent | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


