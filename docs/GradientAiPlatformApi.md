# \GradientAiPlatformApi

All URIs are relative to *https://api.digitalocean.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**genai_attach_agent**](GradientAiPlatformApi.md#genai_attach_agent) | **POST** /v2/gen-ai/agents/{parent_agent_uuid}/child_agents/{child_agent_uuid} | Add Agent Route to an Agent
[**genai_attach_agent_function**](GradientAiPlatformApi.md#genai_attach_agent_function) | **POST** /v2/gen-ai/agents/{agent_uuid}/functions | Add Function Route to an Agent
[**genai_attach_knowledge_base**](GradientAiPlatformApi.md#genai_attach_knowledge_base) | **POST** /v2/gen-ai/agents/{agent_uuid}/knowledge_bases/{knowledge_base_uuid} | Attach Knowledge Base to an Agent
[**genai_attach_knowledge_bases**](GradientAiPlatformApi.md#genai_attach_knowledge_bases) | **POST** /v2/gen-ai/agents/{agent_uuid}/knowledge_bases | Attach Knowledge Bases to an Agent
[**genai_cancel_indexing_job**](GradientAiPlatformApi.md#genai_cancel_indexing_job) | **PUT** /v2/gen-ai/indexing_jobs/{uuid}/cancel | Cancel Indexing Job for a Knowledge Base
[**genai_create_agent**](GradientAiPlatformApi.md#genai_create_agent) | **POST** /v2/gen-ai/agents | Create an Agent
[**genai_create_agent_api_key**](GradientAiPlatformApi.md#genai_create_agent_api_key) | **POST** /v2/gen-ai/agents/{agent_uuid}/api_keys | Create an Agent API Key
[**genai_create_anthropic_api_key**](GradientAiPlatformApi.md#genai_create_anthropic_api_key) | **POST** /v2/gen-ai/anthropic/keys | Create Anthropic API Key
[**genai_create_data_source_file_upload_presigned_urls**](GradientAiPlatformApi.md#genai_create_data_source_file_upload_presigned_urls) | **POST** /v2/gen-ai/knowledge_bases/data_sources/file_upload_presigned_urls | Create Presigned URLs for Data Source File Upload
[**genai_create_evaluation_dataset**](GradientAiPlatformApi.md#genai_create_evaluation_dataset) | **POST** /v2/gen-ai/evaluation_datasets | Create Evaluation Dataset
[**genai_create_evaluation_dataset_file_upload_presigned_urls**](GradientAiPlatformApi.md#genai_create_evaluation_dataset_file_upload_presigned_urls) | **POST** /v2/gen-ai/evaluation_datasets/file_upload_presigned_urls | Create Presigned URLs for Evaluation Dataset File Upload
[**genai_create_evaluation_test_case**](GradientAiPlatformApi.md#genai_create_evaluation_test_case) | **POST** /v2/gen-ai/evaluation_test_cases | Create Evaluation Test Case.
[**genai_create_indexing_job**](GradientAiPlatformApi.md#genai_create_indexing_job) | **POST** /v2/gen-ai/indexing_jobs | Start Indexing Job for a Knowledge Base
[**genai_create_knowledge_base**](GradientAiPlatformApi.md#genai_create_knowledge_base) | **POST** /v2/gen-ai/knowledge_bases | Create a Knowledge Base
[**genai_create_knowledge_base_data_source**](GradientAiPlatformApi.md#genai_create_knowledge_base_data_source) | **POST** /v2/gen-ai/knowledge_bases/{knowledge_base_uuid}/data_sources | Add Data Source to a Knowledge Base
[**genai_create_model_api_key**](GradientAiPlatformApi.md#genai_create_model_api_key) | **POST** /v2/gen-ai/models/api_keys | Create a Model API Key
[**genai_create_oauth2_dropbox_tokens**](GradientAiPlatformApi.md#genai_create_oauth2_dropbox_tokens) | **POST** /v2/gen-ai/oauth2/dropbox/tokens | Get Oauth2 Dropbox Tokens
[**genai_create_openai_api_key**](GradientAiPlatformApi.md#genai_create_openai_api_key) | **POST** /v2/gen-ai/openai/keys | Create OpenAI API Key
[**genai_create_scheduled_indexing**](GradientAiPlatformApi.md#genai_create_scheduled_indexing) | **POST** /v2/gen-ai/scheduled-indexing | Create scheduled indexing for knowledge base
[**genai_create_workspace**](GradientAiPlatformApi.md#genai_create_workspace) | **POST** /v2/gen-ai/workspaces | Create a Workspace
[**genai_delete_agent**](GradientAiPlatformApi.md#genai_delete_agent) | **DELETE** /v2/gen-ai/agents/{uuid} | Delete an Agent
[**genai_delete_agent_api_key**](GradientAiPlatformApi.md#genai_delete_agent_api_key) | **DELETE** /v2/gen-ai/agents/{agent_uuid}/api_keys/{api_key_uuid} | Delete API Key for an Agent 
[**genai_delete_anthropic_api_key**](GradientAiPlatformApi.md#genai_delete_anthropic_api_key) | **DELETE** /v2/gen-ai/anthropic/keys/{api_key_uuid} | Delete Anthropic API Key
[**genai_delete_knowledge_base**](GradientAiPlatformApi.md#genai_delete_knowledge_base) | **DELETE** /v2/gen-ai/knowledge_bases/{uuid} | Delete a Knowledge Base
[**genai_delete_knowledge_base_data_source**](GradientAiPlatformApi.md#genai_delete_knowledge_base_data_source) | **DELETE** /v2/gen-ai/knowledge_bases/{knowledge_base_uuid}/data_sources/{data_source_uuid} | Delete a Data Source from a Knowledge Base
[**genai_delete_model_api_key**](GradientAiPlatformApi.md#genai_delete_model_api_key) | **DELETE** /v2/gen-ai/models/api_keys/{api_key_uuid} | Delete API Key for a Model
[**genai_delete_openai_api_key**](GradientAiPlatformApi.md#genai_delete_openai_api_key) | **DELETE** /v2/gen-ai/openai/keys/{api_key_uuid} | Delete OpenAI API Key
[**genai_delete_scheduled_indexing**](GradientAiPlatformApi.md#genai_delete_scheduled_indexing) | **DELETE** /v2/gen-ai/scheduled-indexing/{uuid} | Delete Scheduled Indexing
[**genai_delete_workspace**](GradientAiPlatformApi.md#genai_delete_workspace) | **DELETE** /v2/gen-ai/workspaces/{workspace_uuid} | Delete a Workspace
[**genai_detach_agent**](GradientAiPlatformApi.md#genai_detach_agent) | **DELETE** /v2/gen-ai/agents/{parent_agent_uuid}/child_agents/{child_agent_uuid} | Delete Agent Route for an Agent
[**genai_detach_agent_function**](GradientAiPlatformApi.md#genai_detach_agent_function) | **DELETE** /v2/gen-ai/agents/{agent_uuid}/functions/{function_uuid} | Delete Function Route for an Agent
[**genai_detach_knowledge_base**](GradientAiPlatformApi.md#genai_detach_knowledge_base) | **DELETE** /v2/gen-ai/agents/{agent_uuid}/knowledge_bases/{knowledge_base_uuid} | Detach Knowledge Base from an Agent
[**genai_get_agent**](GradientAiPlatformApi.md#genai_get_agent) | **GET** /v2/gen-ai/agents/{uuid} | Retrieve an Existing Agent
[**genai_get_agent_children**](GradientAiPlatformApi.md#genai_get_agent_children) | **GET** /v2/gen-ai/agents/{uuid}/child_agents | View Agent Routes
[**genai_get_agent_usage**](GradientAiPlatformApi.md#genai_get_agent_usage) | **GET** /v2/gen-ai/agents/{uuid}/usage | Get Agent Usage
[**genai_get_anthropic_api_key**](GradientAiPlatformApi.md#genai_get_anthropic_api_key) | **GET** /v2/gen-ai/anthropic/keys/{api_key_uuid} | Get Anthropic API Key
[**genai_get_evaluation_run**](GradientAiPlatformApi.md#genai_get_evaluation_run) | **GET** /v2/gen-ai/evaluation_runs/{evaluation_run_uuid} | Retrieve Information About an Existing Evaluation Run
[**genai_get_evaluation_run_prompt_results**](GradientAiPlatformApi.md#genai_get_evaluation_run_prompt_results) | **GET** /v2/gen-ai/evaluation_runs/{evaluation_run_uuid}/results/{prompt_id} | Retrieve Results of an Evaluation Run Prompt
[**genai_get_evaluation_run_results**](GradientAiPlatformApi.md#genai_get_evaluation_run_results) | **GET** /v2/gen-ai/evaluation_runs/{evaluation_run_uuid}/results | Retrieve Results of an Evaluation Run
[**genai_get_evaluation_test_case**](GradientAiPlatformApi.md#genai_get_evaluation_test_case) | **GET** /v2/gen-ai/evaluation_test_cases/{test_case_uuid} | Retrieve Information About an Existing Evaluation Test Case
[**genai_get_indexing_job**](GradientAiPlatformApi.md#genai_get_indexing_job) | **GET** /v2/gen-ai/indexing_jobs/{uuid} | Retrieve Status of Indexing Job for a Knowledge Base
[**genai_get_indexing_job_details_signed_url**](GradientAiPlatformApi.md#genai_get_indexing_job_details_signed_url) | **GET** /v2/gen-ai/indexing_jobs/{indexing_job_uuid}/details_signed_url | Get Signed URL for Indexing Job Details
[**genai_get_knowledge_base**](GradientAiPlatformApi.md#genai_get_knowledge_base) | **GET** /v2/gen-ai/knowledge_bases/{uuid} | Retrieve Information About an Existing Knowledge Base
[**genai_get_oauth2_url**](GradientAiPlatformApi.md#genai_get_oauth2_url) | **GET** /v2/gen-ai/oauth2/url | Get Oauth2 URL
[**genai_get_openai_api_key**](GradientAiPlatformApi.md#genai_get_openai_api_key) | **GET** /v2/gen-ai/openai/keys/{api_key_uuid} | Get OpenAI API Key
[**genai_get_scheduled_indexing**](GradientAiPlatformApi.md#genai_get_scheduled_indexing) | **GET** /v2/gen-ai/scheduled-indexing/knowledge-base/{knowledge_base_uuid} | Get Scheduled Indexing for Knowledge Base
[**genai_get_workspace**](GradientAiPlatformApi.md#genai_get_workspace) | **GET** /v2/gen-ai/workspaces/{workspace_uuid} | Retrieve an Existing Workspace
[**genai_list_agent_api_keys**](GradientAiPlatformApi.md#genai_list_agent_api_keys) | **GET** /v2/gen-ai/agents/{agent_uuid}/api_keys | List Agent API Keys
[**genai_list_agent_versions**](GradientAiPlatformApi.md#genai_list_agent_versions) | **GET** /v2/gen-ai/agents/{uuid}/versions | List Agent Versions
[**genai_list_agents**](GradientAiPlatformApi.md#genai_list_agents) | **GET** /v2/gen-ai/agents | List Agents
[**genai_list_agents_by_anthropic_key**](GradientAiPlatformApi.md#genai_list_agents_by_anthropic_key) | **GET** /v2/gen-ai/anthropic/keys/{uuid}/agents | List agents by Anthropic key
[**genai_list_agents_by_openai_key**](GradientAiPlatformApi.md#genai_list_agents_by_openai_key) | **GET** /v2/gen-ai/openai/keys/{uuid}/agents | List agents by OpenAI key
[**genai_list_agents_by_workspace**](GradientAiPlatformApi.md#genai_list_agents_by_workspace) | **GET** /v2/gen-ai/workspaces/{workspace_uuid}/agents | List agents by Workspace
[**genai_list_anthropic_api_keys**](GradientAiPlatformApi.md#genai_list_anthropic_api_keys) | **GET** /v2/gen-ai/anthropic/keys | List Anthropic API Keys
[**genai_list_datacenter_regions**](GradientAiPlatformApi.md#genai_list_datacenter_regions) | **GET** /v2/gen-ai/regions | List Datacenter Regions
[**genai_list_evaluation_metrics**](GradientAiPlatformApi.md#genai_list_evaluation_metrics) | **GET** /v2/gen-ai/evaluation_metrics | List Evaluation Metrics
[**genai_list_evaluation_runs_by_test_case**](GradientAiPlatformApi.md#genai_list_evaluation_runs_by_test_case) | **GET** /v2/gen-ai/evaluation_test_cases/{evaluation_test_case_uuid}/evaluation_runs | List Evaluation Runs by Test Case
[**genai_list_evaluation_test_cases**](GradientAiPlatformApi.md#genai_list_evaluation_test_cases) | **GET** /v2/gen-ai/evaluation_test_cases | List Evaluation Test Cases
[**genai_list_evaluation_test_cases_by_workspace**](GradientAiPlatformApi.md#genai_list_evaluation_test_cases_by_workspace) | **GET** /v2/gen-ai/workspaces/{workspace_uuid}/evaluation_test_cases | List Evaluation Test Cases by Workspace
[**genai_list_indexing_job_data_sources**](GradientAiPlatformApi.md#genai_list_indexing_job_data_sources) | **GET** /v2/gen-ai/indexing_jobs/{indexing_job_uuid}/data_sources | List Data Sources for Indexing Job for a Knowledge Base
[**genai_list_indexing_jobs**](GradientAiPlatformApi.md#genai_list_indexing_jobs) | **GET** /v2/gen-ai/indexing_jobs | List Indexing Jobs for a Knowledge Base
[**genai_list_indexing_jobs_by_knowledge_base**](GradientAiPlatformApi.md#genai_list_indexing_jobs_by_knowledge_base) | **GET** /v2/gen-ai/knowledge_bases/{knowledge_base_uuid}/indexing_jobs | List Indexing Jobs for a Knowledge Base
[**genai_list_knowledge_base_data_sources**](GradientAiPlatformApi.md#genai_list_knowledge_base_data_sources) | **GET** /v2/gen-ai/knowledge_bases/{knowledge_base_uuid}/data_sources | List Data Sources for a Knowledge Base
[**genai_list_knowledge_bases**](GradientAiPlatformApi.md#genai_list_knowledge_bases) | **GET** /v2/gen-ai/knowledge_bases | List Knowledge Bases
[**genai_list_model_api_keys**](GradientAiPlatformApi.md#genai_list_model_api_keys) | **GET** /v2/gen-ai/models/api_keys | List Model API Keys
[**genai_list_models**](GradientAiPlatformApi.md#genai_list_models) | **GET** /v2/gen-ai/models | List Available Models
[**genai_list_openai_api_keys**](GradientAiPlatformApi.md#genai_list_openai_api_keys) | **GET** /v2/gen-ai/openai/keys | List OpenAI API Keys
[**genai_list_workspaces**](GradientAiPlatformApi.md#genai_list_workspaces) | **GET** /v2/gen-ai/workspaces | List Workspaces
[**genai_regenerate_agent_api_key**](GradientAiPlatformApi.md#genai_regenerate_agent_api_key) | **PUT** /v2/gen-ai/agents/{agent_uuid}/api_keys/{api_key_uuid}/regenerate | Regenerate API Key for an Agent
[**genai_regenerate_model_api_key**](GradientAiPlatformApi.md#genai_regenerate_model_api_key) | **PUT** /v2/gen-ai/models/api_keys/{api_key_uuid}/regenerate | Regenerate API Key for a Model
[**genai_rollback_to_agent_version**](GradientAiPlatformApi.md#genai_rollback_to_agent_version) | **PUT** /v2/gen-ai/agents/{uuid}/versions | Rollback to Agent Version
[**genai_run_evaluation_test_case**](GradientAiPlatformApi.md#genai_run_evaluation_test_case) | **POST** /v2/gen-ai/evaluation_runs | Run an Evaluation Test Case
[**genai_update_agent**](GradientAiPlatformApi.md#genai_update_agent) | **PUT** /v2/gen-ai/agents/{uuid} | Update an Agent
[**genai_update_agent_api_key**](GradientAiPlatformApi.md#genai_update_agent_api_key) | **PUT** /v2/gen-ai/agents/{agent_uuid}/api_keys/{api_key_uuid} | Update API Key for an Agent
[**genai_update_agent_deployment_visibility**](GradientAiPlatformApi.md#genai_update_agent_deployment_visibility) | **PUT** /v2/gen-ai/agents/{uuid}/deployment_visibility | Update Agent Status
[**genai_update_agent_function**](GradientAiPlatformApi.md#genai_update_agent_function) | **PUT** /v2/gen-ai/agents/{agent_uuid}/functions/{function_uuid} | Update Function Route for an Agent
[**genai_update_agents_workspace**](GradientAiPlatformApi.md#genai_update_agents_workspace) | **PUT** /v2/gen-ai/workspaces/{workspace_uuid}/agents | Move Agents to a Workspace
[**genai_update_anthropic_api_key**](GradientAiPlatformApi.md#genai_update_anthropic_api_key) | **PUT** /v2/gen-ai/anthropic/keys/{api_key_uuid} | Update Anthropic API Key
[**genai_update_attached_agent**](GradientAiPlatformApi.md#genai_update_attached_agent) | **PUT** /v2/gen-ai/agents/{parent_agent_uuid}/child_agents/{child_agent_uuid} | Update Agent Route for an Agent
[**genai_update_evaluation_test_case**](GradientAiPlatformApi.md#genai_update_evaluation_test_case) | **PUT** /v2/gen-ai/evaluation_test_cases/{test_case_uuid} | Update an Evaluation Test Case.
[**genai_update_knowledge_base**](GradientAiPlatformApi.md#genai_update_knowledge_base) | **PUT** /v2/gen-ai/knowledge_bases/{uuid} | Update a Knowledge Base
[**genai_update_model_api_key**](GradientAiPlatformApi.md#genai_update_model_api_key) | **PUT** /v2/gen-ai/models/api_keys/{api_key_uuid} | Update API Key for a Model
[**genai_update_openai_api_key**](GradientAiPlatformApi.md#genai_update_openai_api_key) | **PUT** /v2/gen-ai/openai/keys/{api_key_uuid} | Update OpenAI API Key
[**genai_update_workspace**](GradientAiPlatformApi.md#genai_update_workspace) | **PUT** /v2/gen-ai/workspaces/{workspace_uuid} | Update a Workspace



## genai_attach_agent

> models::ApiLinkAgentOutput genai_attach_agent(parent_agent_uuid, child_agent_uuid, api_link_agent_input_public)
Add Agent Route to an Agent

To add an agent route to an agent, send a POST request to `/v2/gen-ai/agents/{parent_agent_uuid}/child_agents/{child_agent_uuid}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parent_agent_uuid** | **String** | A unique identifier for the parent agent. | [required] |
**child_agent_uuid** | **String** | Routed agent id | [required] |
**api_link_agent_input_public** | Option<[**ApiLinkAgentInputPublic**](ApiLinkAgentInputPublic.md)> |  |  |

### Return type

[**models::ApiLinkAgentOutput**](apiLinkAgentOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_attach_agent_function

> models::ApiLinkAgentFunctionOutput genai_attach_agent_function(agent_uuid, api_link_agent_function_input_public)
Add Function Route to an Agent

To create a function route for an agent, send a POST request to `/v2/gen-ai/agents/{agent_uuid}/functions`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_uuid** | **String** | Agent id | [required] |
**api_link_agent_function_input_public** | Option<[**ApiLinkAgentFunctionInputPublic**](ApiLinkAgentFunctionInputPublic.md)> |  |  |

### Return type

[**models::ApiLinkAgentFunctionOutput**](apiLinkAgentFunctionOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_attach_knowledge_base

> models::ApiLinkKnowledgeBaseOutput genai_attach_knowledge_base(agent_uuid, knowledge_base_uuid)
Attach Knowledge Base to an Agent

To attach a knowledge base to an agent, send a POST request to `/v2/gen-ai/agents/{agent_uuid}/knowledge_bases/{knowledge_base_uuid}`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_uuid** | **String** | A unique identifier for an agent. | [required] |
**knowledge_base_uuid** | **String** | A unique identifier for a knowledge base. | [required] |

### Return type

[**models::ApiLinkKnowledgeBaseOutput**](apiLinkKnowledgeBaseOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_attach_knowledge_bases

> models::ApiLinkKnowledgeBaseOutput genai_attach_knowledge_bases(agent_uuid)
Attach Knowledge Bases to an Agent

To attach knowledge bases to an agent, send a POST request to `/v2/gen-ai/agents/{agent_uuid}/knowledge_bases`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_uuid** | **String** | A unique identifier for an agent. | [required] |

### Return type

[**models::ApiLinkKnowledgeBaseOutput**](apiLinkKnowledgeBaseOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_cancel_indexing_job

> models::ApiCancelKnowledgeBaseIndexingJobOutput genai_cancel_indexing_job(uuid, api_cancel_knowledge_base_indexing_job_input_public)
Cancel Indexing Job for a Knowledge Base

To cancel an indexing job for a knowledge base, send a PUT request to `/v2/gen-ai/indexing_jobs/{uuid}/cancel`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** | A unique identifier for an indexing job. | [required] |
**api_cancel_knowledge_base_indexing_job_input_public** | Option<[**ApiCancelKnowledgeBaseIndexingJobInputPublic**](ApiCancelKnowledgeBaseIndexingJobInputPublic.md)> |  |  |

### Return type

[**models::ApiCancelKnowledgeBaseIndexingJobOutput**](apiCancelKnowledgeBaseIndexingJobOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_create_agent

> models::ApiCreateAgentOutput genai_create_agent(api_create_agent_input_public)
Create an Agent

To create a new agent, send a POST request to `/v2/gen-ai/agents`. The response body contains a JSON object with the newly created agent object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_create_agent_input_public** | Option<[**ApiCreateAgentInputPublic**](ApiCreateAgentInputPublic.md)> |  |  |

### Return type

[**models::ApiCreateAgentOutput**](apiCreateAgentOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_create_agent_api_key

> models::ApiCreateAgentApiKeyOutput genai_create_agent_api_key(agent_uuid, api_create_agent_api_key_input_public)
Create an Agent API Key

To create an agent API key, send a POST request to `/v2/gen-ai/agents/{agent_uuid}/api_keys`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_uuid** | **String** | Agent id | [required] |
**api_create_agent_api_key_input_public** | Option<[**ApiCreateAgentApiKeyInputPublic**](ApiCreateAgentApiKeyInputPublic.md)> |  |  |

### Return type

[**models::ApiCreateAgentApiKeyOutput**](apiCreateAgentAPIKeyOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_create_anthropic_api_key

> models::ApiCreateAnthropicApiKeyOutput genai_create_anthropic_api_key(api_create_anthropic_api_key_input_public)
Create Anthropic API Key

To create an Anthropic API key, send a POST request to `/v2/gen-ai/anthropic/keys`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_create_anthropic_api_key_input_public** | Option<[**ApiCreateAnthropicApiKeyInputPublic**](ApiCreateAnthropicApiKeyInputPublic.md)> |  |  |

### Return type

[**models::ApiCreateAnthropicApiKeyOutput**](apiCreateAnthropicAPIKeyOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_create_data_source_file_upload_presigned_urls

> models::ApiCreateDataSourceFileUploadPresignedUrlsOutput genai_create_data_source_file_upload_presigned_urls(api_create_data_source_file_upload_presigned_urls_input_public)
Create Presigned URLs for Data Source File Upload

To create presigned URLs for knowledge base data source file upload, send a POST request to `/v2/gen-ai/knowledge_bases/data_sources/file_upload_presigned_urls`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_create_data_source_file_upload_presigned_urls_input_public** | Option<[**ApiCreateDataSourceFileUploadPresignedUrlsInputPublic**](ApiCreateDataSourceFileUploadPresignedUrlsInputPublic.md)> |  |  |

### Return type

[**models::ApiCreateDataSourceFileUploadPresignedUrlsOutput**](apiCreateDataSourceFileUploadPresignedUrlsOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_create_evaluation_dataset

> models::ApiCreateEvaluationDatasetOutput genai_create_evaluation_dataset(api_create_evaluation_dataset_input_public)
Create Evaluation Dataset

To create an evaluation dataset, send a POST request to `/v2/gen-ai/evaluation_datasets`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_create_evaluation_dataset_input_public** | Option<[**ApiCreateEvaluationDatasetInputPublic**](ApiCreateEvaluationDatasetInputPublic.md)> |  |  |

### Return type

[**models::ApiCreateEvaluationDatasetOutput**](apiCreateEvaluationDatasetOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_create_evaluation_dataset_file_upload_presigned_urls

> models::ApiCreateDataSourceFileUploadPresignedUrlsOutput genai_create_evaluation_dataset_file_upload_presigned_urls(api_create_data_source_file_upload_presigned_urls_input_public)
Create Presigned URLs for Evaluation Dataset File Upload

To create presigned URLs for evaluation dataset file upload, send a POST request to `/v2/gen-ai/evaluation_datasets/file_upload_presigned_urls`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_create_data_source_file_upload_presigned_urls_input_public** | Option<[**ApiCreateDataSourceFileUploadPresignedUrlsInputPublic**](ApiCreateDataSourceFileUploadPresignedUrlsInputPublic.md)> |  |  |

### Return type

[**models::ApiCreateDataSourceFileUploadPresignedUrlsOutput**](apiCreateDataSourceFileUploadPresignedUrlsOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_create_evaluation_test_case

> models::ApiCreateEvaluationTestCaseOutput genai_create_evaluation_test_case(api_create_evaluation_test_case_input_public)
Create Evaluation Test Case.

To create an evaluation test-case send a POST request to `/v2/gen-ai/evaluation_test_cases`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_create_evaluation_test_case_input_public** | Option<[**ApiCreateEvaluationTestCaseInputPublic**](ApiCreateEvaluationTestCaseInputPublic.md)> |  |  |

### Return type

[**models::ApiCreateEvaluationTestCaseOutput**](apiCreateEvaluationTestCaseOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_create_indexing_job

> models::ApiStartKnowledgeBaseIndexingJobOutput genai_create_indexing_job(api_start_knowledge_base_indexing_job_input_public)
Start Indexing Job for a Knowledge Base

To start an indexing job for a knowledge base, send a POST request to `/v2/gen-ai/indexing_jobs`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_start_knowledge_base_indexing_job_input_public** | Option<[**ApiStartKnowledgeBaseIndexingJobInputPublic**](ApiStartKnowledgeBaseIndexingJobInputPublic.md)> |  |  |

### Return type

[**models::ApiStartKnowledgeBaseIndexingJobOutput**](apiStartKnowledgeBaseIndexingJobOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_create_knowledge_base

> models::ApiCreateKnowledgeBaseOutput genai_create_knowledge_base(api_create_knowledge_base_input_public)
Create a Knowledge Base

To create a knowledge base, send a POST request to `/v2/gen-ai/knowledge_bases`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_create_knowledge_base_input_public** | Option<[**ApiCreateKnowledgeBaseInputPublic**](ApiCreateKnowledgeBaseInputPublic.md)> |  |  |

### Return type

[**models::ApiCreateKnowledgeBaseOutput**](apiCreateKnowledgeBaseOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_create_knowledge_base_data_source

> models::ApiCreateKnowledgeBaseDataSourceOutput genai_create_knowledge_base_data_source(knowledge_base_uuid, api_create_knowledge_base_data_source_input_public)
Add Data Source to a Knowledge Base

To add a data source to a knowledge base, send a POST request to `/v2/gen-ai/knowledge_bases/{knowledge_base_uuid}/data_sources`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**knowledge_base_uuid** | **String** | Knowledge base id | [required] |
**api_create_knowledge_base_data_source_input_public** | Option<[**ApiCreateKnowledgeBaseDataSourceInputPublic**](ApiCreateKnowledgeBaseDataSourceInputPublic.md)> |  |  |

### Return type

[**models::ApiCreateKnowledgeBaseDataSourceOutput**](apiCreateKnowledgeBaseDataSourceOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_create_model_api_key

> models::ApiCreateModelApiKeyOutput genai_create_model_api_key(api_create_model_api_key_input_public)
Create a Model API Key

To create a model API key, send a POST request to `/v2/gen-ai/models/api_keys`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_create_model_api_key_input_public** | Option<[**ApiCreateModelApiKeyInputPublic**](ApiCreateModelApiKeyInputPublic.md)> |  |  |

### Return type

[**models::ApiCreateModelApiKeyOutput**](apiCreateModelAPIKeyOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_create_oauth2_dropbox_tokens

> models::ApiDropboxOauth2GetTokensOutput genai_create_oauth2_dropbox_tokens(api_dropbox_oauth2_get_tokens_input)
Get Oauth2 Dropbox Tokens

To obtain the refresh token, needed for creation of data sources, send a GET request to `/v2/gen-ai/oauth2/dropbox/tokens`. Pass the code you obtrained from the oauth flow in the field 'code'

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_dropbox_oauth2_get_tokens_input** | Option<[**ApiDropboxOauth2GetTokensInput**](ApiDropboxOauth2GetTokensInput.md)> |  |  |

### Return type

[**models::ApiDropboxOauth2GetTokensOutput**](apiDropboxOauth2GetTokensOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_create_openai_api_key

> models::ApiCreateOpenAiapiKeyOutput genai_create_openai_api_key(api_create_open_aiapi_key_input_public)
Create OpenAI API Key

To create an OpenAI API key, send a POST request to `/v2/gen-ai/openai/keys`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_create_open_aiapi_key_input_public** | Option<[**ApiCreateOpenAiapiKeyInputPublic**](ApiCreateOpenAiapiKeyInputPublic.md)> |  |  |

### Return type

[**models::ApiCreateOpenAiapiKeyOutput**](apiCreateOpenAIAPIKeyOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_create_scheduled_indexing

> models::ApiCreateScheduledIndexingOutput genai_create_scheduled_indexing(api_create_scheduled_indexing_input_public)
Create scheduled indexing for knowledge base

To create scheduled indexing for a knowledge base, send a POST request to `/v2/gen-ai/scheduled-indexing`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_create_scheduled_indexing_input_public** | Option<[**ApiCreateScheduledIndexingInputPublic**](ApiCreateScheduledIndexingInputPublic.md)> |  |  |

### Return type

[**models::ApiCreateScheduledIndexingOutput**](apiCreateScheduledIndexingOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_create_workspace

> models::ApiCreateWorkspaceOutput genai_create_workspace(api_create_workspace_input_public)
Create a Workspace

To create a new workspace, send a POST request to `/v2/gen-ai/workspaces`. The response body contains a JSON object with the newly created workspace object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_create_workspace_input_public** | Option<[**ApiCreateWorkspaceInputPublic**](ApiCreateWorkspaceInputPublic.md)> |  |  |

### Return type

[**models::ApiCreateWorkspaceOutput**](apiCreateWorkspaceOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_delete_agent

> models::ApiDeleteAgentOutput genai_delete_agent(uuid)
Delete an Agent

To delete an agent, send a DELETE request to `/v2/gen-ai/agents/{uuid}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** | Unique agent id | [required] |

### Return type

[**models::ApiDeleteAgentOutput**](apiDeleteAgentOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_delete_agent_api_key

> models::ApiDeleteAgentApiKeyOutput genai_delete_agent_api_key(agent_uuid, api_key_uuid)
Delete API Key for an Agent 

To delete an API key for an agent, send a DELETE request to `/v2/gen-ai/agents/{agent_uuid}/api_keys/{api_key_uuid}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_uuid** | **String** | A unique identifier for your agent. | [required] |
**api_key_uuid** | **String** | API key for an agent. | [required] |

### Return type

[**models::ApiDeleteAgentApiKeyOutput**](apiDeleteAgentAPIKeyOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_delete_anthropic_api_key

> models::ApiDeleteAnthropicApiKeyOutput genai_delete_anthropic_api_key(api_key_uuid)
Delete Anthropic API Key

To delete an Anthropic API key, send a DELETE request to `/v2/gen-ai/anthropic/keys/{api_key_uuid}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key_uuid** | **String** | API key ID | [required] |

### Return type

[**models::ApiDeleteAnthropicApiKeyOutput**](apiDeleteAnthropicAPIKeyOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_delete_knowledge_base

> models::ApiDeleteKnowledgeBaseOutput genai_delete_knowledge_base(uuid)
Delete a Knowledge Base

To delete a knowledge base, send a DELETE request to `/v2/gen-ai/knowledge_bases/{uuid}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** | Knowledge base id | [required] |

### Return type

[**models::ApiDeleteKnowledgeBaseOutput**](apiDeleteKnowledgeBaseOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_delete_knowledge_base_data_source

> models::ApiDeleteKnowledgeBaseDataSourceOutput genai_delete_knowledge_base_data_source(knowledge_base_uuid, data_source_uuid)
Delete a Data Source from a Knowledge Base

To delete a data source from a knowledge base, send a DELETE request to `/v2/gen-ai/knowledge_bases/{knowledge_base_uuid}/data_sources/{data_source_uuid}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**knowledge_base_uuid** | **String** | Knowledge base id | [required] |
**data_source_uuid** | **String** | Data source id | [required] |

### Return type

[**models::ApiDeleteKnowledgeBaseDataSourceOutput**](apiDeleteKnowledgeBaseDataSourceOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_delete_model_api_key

> models::ApiDeleteModelApiKeyOutput genai_delete_model_api_key(api_key_uuid)
Delete API Key for a Model

To delete an API key for a model, send a DELETE request to `/v2/gen-ai/models/api_keys/{api_key_uuid}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key_uuid** | **String** | API key for an agent. | [required] |

### Return type

[**models::ApiDeleteModelApiKeyOutput**](apiDeleteModelAPIKeyOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_delete_openai_api_key

> models::ApiDeleteOpenAiapiKeyOutput genai_delete_openai_api_key(api_key_uuid)
Delete OpenAI API Key

To delete an OpenAI API key, send a DELETE request to `/v2/gen-ai/openai/keys/{api_key_uuid}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key_uuid** | **String** | API key ID | [required] |

### Return type

[**models::ApiDeleteOpenAiapiKeyOutput**](apiDeleteOpenAIAPIKeyOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_delete_scheduled_indexing

> models::ApiDeleteScheduledIndexingOutput genai_delete_scheduled_indexing(uuid)
Delete Scheduled Indexing

Delete Scheduled Indexing for knowledge base, send a DELETE request to `/v2/gen-ai/scheduled-indexing/{uuid}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** | UUID of the scheduled indexing | [required] |

### Return type

[**models::ApiDeleteScheduledIndexingOutput**](apiDeleteScheduledIndexingOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_delete_workspace

> models::ApiDeleteWorkspaceOutput genai_delete_workspace(workspace_uuid)
Delete a Workspace

To delete a workspace, send a DELETE request to `/v2/gen-ai/workspace/{workspace_uuid}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_uuid** | **String** | Workspace UUID. | [required] |

### Return type

[**models::ApiDeleteWorkspaceOutput**](apiDeleteWorkspaceOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_detach_agent

> models::ApiUnlinkAgentOutput genai_detach_agent(parent_agent_uuid, child_agent_uuid)
Delete Agent Route for an Agent

To delete an agent route from a parent agent, send a DELETE request to `/v2/gen-ai/agents/{parent_agent_uuid}/child_agents/{child_agent_uuid}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parent_agent_uuid** | **String** | Pagent agent id | [required] |
**child_agent_uuid** | **String** | Routed agent id | [required] |

### Return type

[**models::ApiUnlinkAgentOutput**](apiUnlinkAgentOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_detach_agent_function

> models::ApiUnlinkAgentFunctionOutput genai_detach_agent_function(agent_uuid, function_uuid)
Delete Function Route for an Agent

To delete a function route from an agent, send a DELETE request to `/v2/gen-ai/agents/{agent_uuid}/functions/{function_uuid}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_uuid** | **String** | The id of the agent the function route belongs to. | [required] |
**function_uuid** | **String** | The function route to be destroyed. This does not destroy the function itself. | [required] |

### Return type

[**models::ApiUnlinkAgentFunctionOutput**](apiUnlinkAgentFunctionOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_detach_knowledge_base

> models::ApiUnlinkKnowledgeBaseOutput genai_detach_knowledge_base(agent_uuid, knowledge_base_uuid)
Detach Knowledge Base from an Agent

To detach a knowledge base from an agent, send a DELETE request to `/v2/gen-ai/agents/{agent_uuid}/knowledge_bases/{knowledge_base_uuid}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_uuid** | **String** | Agent id | [required] |
**knowledge_base_uuid** | **String** | Knowledge base id | [required] |

### Return type

[**models::ApiUnlinkKnowledgeBaseOutput**](apiUnlinkKnowledgeBaseOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_get_agent

> models::ApiGetAgentOutput genai_get_agent(uuid)
Retrieve an Existing Agent

To retrieve details of an agent, GET request to `/v2/gen-ai/agents/{uuid}`. The response body is a JSON object containing the agent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** | Unique agent id | [required] |

### Return type

[**models::ApiGetAgentOutput**](apiGetAgentOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_get_agent_children

> models::ApiGetChildrenOutput genai_get_agent_children(uuid)
View Agent Routes

To view agent routes for an agent, send a GET requtest to `/v2/gen-ai/agents/{uuid}/child_agents`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** | Agent id | [required] |

### Return type

[**models::ApiGetChildrenOutput**](apiGetChildrenOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_get_agent_usage

> models::ApiGetAgentUsageOutput genai_get_agent_usage(uuid, start, stop)
Get Agent Usage

To get agent usage, send a GET request to `/v2/gen-ai/agents/{uuid}/usage`. Returns usage metrics for the specified agent within the provided time range.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** | Agent id | [required] |
**start** | Option<**String**> | Return all usage data from this date. |  |
**stop** | Option<**String**> | Return all usage data up to this date, if omitted, will return up to the current date. |  |

### Return type

[**models::ApiGetAgentUsageOutput**](apiGetAgentUsageOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_get_anthropic_api_key

> models::ApiGetAnthropicApiKeyOutput genai_get_anthropic_api_key(api_key_uuid)
Get Anthropic API Key

To retrieve details of an Anthropic API key, send a GET request to `/v2/gen-ai/anthropic/keys/{api_key_uuid}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key_uuid** | **String** | API key ID | [required] |

### Return type

[**models::ApiGetAnthropicApiKeyOutput**](apiGetAnthropicAPIKeyOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_get_evaluation_run

> models::ApiGetEvaluationRunOutput genai_get_evaluation_run(evaluation_run_uuid)
Retrieve Information About an Existing Evaluation Run

To retrive information about an existing evaluation run, send a GET request to `/v2/gen-ai/evaluation_runs/{evaluation_run_uuid}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**evaluation_run_uuid** | **String** | Evaluation run UUID. | [required] |

### Return type

[**models::ApiGetEvaluationRunOutput**](apiGetEvaluationRunOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_get_evaluation_run_prompt_results

> models::ApiGetEvaluationRunPromptResultsOutput genai_get_evaluation_run_prompt_results(evaluation_run_uuid, prompt_id)
Retrieve Results of an Evaluation Run Prompt

To retrieve results of an evaluation run, send a GET request to `/v2/gen-ai/evaluation_runs/{evaluation_run_uuid}/results/{prompt_id}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**evaluation_run_uuid** | **String** | Evaluation run UUID. | [required] |
**prompt_id** | **i32** | Prompt ID to get results for. | [required] |

### Return type

[**models::ApiGetEvaluationRunPromptResultsOutput**](apiGetEvaluationRunPromptResultsOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_get_evaluation_run_results

> models::ApiGetEvaluationRunResultsOutput genai_get_evaluation_run_results(evaluation_run_uuid, page, per_page)
Retrieve Results of an Evaluation Run

To retrieve results of an evaluation run, send a GET request to `/v2/gen-ai/evaluation_runs/{evaluation_run_uuid}/results`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**evaluation_run_uuid** | **String** | Evaluation run UUID. | [required] |
**page** | Option<**i32**> | Page number. |  |
**per_page** | Option<**i32**> | Items per page. |  |

### Return type

[**models::ApiGetEvaluationRunResultsOutput**](apiGetEvaluationRunResultsOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_get_evaluation_test_case

> models::ApiGetEvaluationTestCaseOutput genai_get_evaluation_test_case(test_case_uuid, evaluation_test_case_version)
Retrieve Information About an Existing Evaluation Test Case

To retrive information about an existing evaluation test case, send a GET request to `/v2/gen-ai/evaluation_test_case/{test_case_uuid}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**test_case_uuid** | **String** | The test case uuid to retrieve. | [required] |
**evaluation_test_case_version** | Option<**i32**> | Version of the test case. |  |

### Return type

[**models::ApiGetEvaluationTestCaseOutput**](apiGetEvaluationTestCaseOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_get_indexing_job

> models::ApiGetKnowledgeBaseIndexingJobOutput genai_get_indexing_job(uuid)
Retrieve Status of Indexing Job for a Knowledge Base

To get status of an indexing Job for a knowledge base, send a GET request to `/v2/gen-ai/indexing_jobs/{uuid}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** | Indexing job id | [required] |

### Return type

[**models::ApiGetKnowledgeBaseIndexingJobOutput**](apiGetKnowledgeBaseIndexingJobOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_get_indexing_job_details_signed_url

> models::ApiGetIndexingJobDetailsSignedUrlOutput genai_get_indexing_job_details_signed_url(indexing_job_uuid)
Get Signed URL for Indexing Job Details

To get a signed URL for indexing job details, send a GET request to `/v2/gen-ai/indexing_jobs/{uuid}/details_signed_url`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**indexing_job_uuid** | **String** | The uuid of the indexing job | [required] |

### Return type

[**models::ApiGetIndexingJobDetailsSignedUrlOutput**](apiGetIndexingJobDetailsSignedURLOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_get_knowledge_base

> models::ApiGetKnowledgeBaseOutput genai_get_knowledge_base(uuid)
Retrieve Information About an Existing Knowledge Base

To retrive information about an existing knowledge base, send a GET request to `/v2/gen-ai/knowledge_bases/{uuid}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** | Knowledge base id | [required] |

### Return type

[**models::ApiGetKnowledgeBaseOutput**](apiGetKnowledgeBaseOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_get_oauth2_url

> models::ApiGenerateOauth2UrlOutput genai_get_oauth2_url(r#type, redirect_url)
Get Oauth2 URL

To generate an Oauth2-URL for use with your localhost, send a GET request to `/v2/gen-ai/oauth2/url`. Pass 'http://localhost:3000 as redirect_url

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | Option<**String**> | Type \"google\" / \"dropbox\". |  |
**redirect_url** | Option<**String**> | The redirect url. |  |

### Return type

[**models::ApiGenerateOauth2UrlOutput**](apiGenerateOauth2URLOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_get_openai_api_key

> models::ApiGetOpenAiapiKeyOutput genai_get_openai_api_key(api_key_uuid)
Get OpenAI API Key

To retrieve details of an OpenAI API key, send a GET request to `/v2/gen-ai/openai/keys/{api_key_uuid}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key_uuid** | **String** | API key ID | [required] |

### Return type

[**models::ApiGetOpenAiapiKeyOutput**](apiGetOpenAIAPIKeyOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_get_scheduled_indexing

> models::ApiGetScheduledIndexingOutput genai_get_scheduled_indexing(knowledge_base_uuid)
Get Scheduled Indexing for Knowledge Base

Get Scheduled Indexing for knowledge base using knoweldge base uuid, send a GET request to `/v2/gen-ai/scheduled-indexing/knowledge-base/{knowledge_base_uuid}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**knowledge_base_uuid** | **String** | UUID of the scheduled indexing entry | [required] |

### Return type

[**models::ApiGetScheduledIndexingOutput**](apiGetScheduledIndexingOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_get_workspace

> models::ApiGetWorkspaceOutput genai_get_workspace(workspace_uuid)
Retrieve an Existing Workspace

To retrieve details of a workspace, GET request to `/v2/gen-ai/workspaces/{workspace_uuid}`. The response body is a JSON object containing the workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_uuid** | **String** | Workspace UUID. | [required] |

### Return type

[**models::ApiGetWorkspaceOutput**](apiGetWorkspaceOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_list_agent_api_keys

> models::ApiListAgentApiKeysOutput genai_list_agent_api_keys(agent_uuid, page, per_page)
List Agent API Keys

To list all agent API keys, send a GET request to `/v2/gen-ai/agents/{agent_uuid}/api_keys`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_uuid** | **String** | Agent id | [required] |
**page** | Option<**i32**> | Page number. |  |
**per_page** | Option<**i32**> | Items per page. |  |

### Return type

[**models::ApiListAgentApiKeysOutput**](apiListAgentAPIKeysOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_list_agent_versions

> models::ApiListAgentVersionsOutput genai_list_agent_versions(uuid, page, per_page)
List Agent Versions

To list all agent versions, send a GET request to `/v2/gen-ai/agents/{uuid}/versions`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** | Agent uuid | [required] |
**page** | Option<**i32**> | Page number. |  |
**per_page** | Option<**i32**> | Items per page. |  |

### Return type

[**models::ApiListAgentVersionsOutput**](apiListAgentVersionsOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_list_agents

> models::ApiListAgentsOutputPublic genai_list_agents(only_deployed, page, per_page)
List Agents

To list all agents, send a GET request to `/v2/gen-ai/agents`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**only_deployed** | Option<**bool**> | Only list agents that are deployed. |  |
**page** | Option<**i32**> | Page number. |  |
**per_page** | Option<**i32**> | Items per page. |  |

### Return type

[**models::ApiListAgentsOutputPublic**](apiListAgentsOutputPublic.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_list_agents_by_anthropic_key

> models::ApiListAgentsByAnthropicKeyOutput genai_list_agents_by_anthropic_key(uuid, page, per_page)
List agents by Anthropic key

List Agents by Anthropic Key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** | Unique ID of Anthropic key | [required] |
**page** | Option<**i32**> | Page number. |  |
**per_page** | Option<**i32**> | Items per page. |  |

### Return type

[**models::ApiListAgentsByAnthropicKeyOutput**](apiListAgentsByAnthropicKeyOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_list_agents_by_openai_key

> models::ApiListAgentsByOpenAiKeyOutput genai_list_agents_by_openai_key(uuid, page, per_page)
List agents by OpenAI key

List Agents by OpenAI Key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** | Unique ID of OpenAI key | [required] |
**page** | Option<**i32**> | Page number. |  |
**per_page** | Option<**i32**> | Items per page. |  |

### Return type

[**models::ApiListAgentsByOpenAiKeyOutput**](apiListAgentsByOpenAIKeyOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_list_agents_by_workspace

> models::ApiListAgentsByWorkspaceOutput genai_list_agents_by_workspace(workspace_uuid, only_deployed, page, per_page)
List agents by Workspace

To list all agents by a Workspace, send a GET request to `/v2/gen-ai/workspaces/{workspace_uuid}/agents`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_uuid** | **String** | Workspace UUID. | [required] |
**only_deployed** | Option<**bool**> | Only list agents that are deployed. |  |
**page** | Option<**i32**> | Page number. |  |
**per_page** | Option<**i32**> | Items per page. |  |

### Return type

[**models::ApiListAgentsByWorkspaceOutput**](apiListAgentsByWorkspaceOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_list_anthropic_api_keys

> models::ApiListAnthropicApiKeysOutput genai_list_anthropic_api_keys(page, per_page)
List Anthropic API Keys

To list all Anthropic API keys, send a GET request to `/v2/gen-ai/anthropic/keys`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number. |  |
**per_page** | Option<**i32**> | Items per page. |  |

### Return type

[**models::ApiListAnthropicApiKeysOutput**](apiListAnthropicAPIKeysOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_list_datacenter_regions

> models::ApiListRegionsOutput genai_list_datacenter_regions(serves_inference, serves_batch)
List Datacenter Regions

To list all datacenter regions, send a GET request to `/v2/gen-ai/regions`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**serves_inference** | Option<**bool**> | Include datacenters that serve inference. |  |
**serves_batch** | Option<**bool**> | Include datacenters that are capable of running batch jobs. |  |

### Return type

[**models::ApiListRegionsOutput**](apiListRegionsOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_list_evaluation_metrics

> models::ApiListEvaluationMetricsOutput genai_list_evaluation_metrics()
List Evaluation Metrics

To list all evaluation metrics, send a GET request to `/v2/gen-ai/evaluation_metrics`.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApiListEvaluationMetricsOutput**](apiListEvaluationMetricsOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_list_evaluation_runs_by_test_case

> models::ApiListEvaluationRunsByTestCaseOutput genai_list_evaluation_runs_by_test_case(evaluation_test_case_uuid, evaluation_test_case_version)
List Evaluation Runs by Test Case

To list all evaluation runs by test case, send a GET request to `/v2/gen-ai/evaluation_test_cases/{evaluation_test_case_uuid}/evaluation_runs`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**evaluation_test_case_uuid** | **String** | Evaluation run UUID. | [required] |
**evaluation_test_case_version** | Option<**i32**> | Version of the test case. |  |

### Return type

[**models::ApiListEvaluationRunsByTestCaseOutput**](apiListEvaluationRunsByTestCaseOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_list_evaluation_test_cases

> models::ApiListEvaluationTestCasesOutput genai_list_evaluation_test_cases()
List Evaluation Test Cases

To list all evaluation test cases, send a GET request to `/v2/gen-ai/evaluation_test_cases`.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApiListEvaluationTestCasesOutput**](apiListEvaluationTestCasesOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_list_evaluation_test_cases_by_workspace

> models::ApiListEvaluationTestCasesByWorkspaceOutput genai_list_evaluation_test_cases_by_workspace(workspace_uuid)
List Evaluation Test Cases by Workspace

To list all evaluation test cases by a workspace, send a GET request to `/v2/gen-ai/workspaces/{workspace_uuid}/evaluation_test_cases`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_uuid** | **String** | Workspace UUID. | [required] |

### Return type

[**models::ApiListEvaluationTestCasesByWorkspaceOutput**](apiListEvaluationTestCasesByWorkspaceOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_list_indexing_job_data_sources

> models::ApiListIndexingJobDataSourcesOutput genai_list_indexing_job_data_sources(indexing_job_uuid)
List Data Sources for Indexing Job for a Knowledge Base

To list all datasources for an indexing job, send a GET request to `/v2/gen-ai/indexing_jobs/{indexing_job_uuid}/data_sources`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**indexing_job_uuid** | **String** | Uuid of the indexing job | [required] |

### Return type

[**models::ApiListIndexingJobDataSourcesOutput**](apiListIndexingJobDataSourcesOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_list_indexing_jobs

> models::ApiListKnowledgeBaseIndexingJobsOutput genai_list_indexing_jobs(page, per_page)
List Indexing Jobs for a Knowledge Base

To list all indexing jobs for a knowledge base, send a GET request to `/v2/gen-ai/indexing_jobs`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number. |  |
**per_page** | Option<**i32**> | Items per page. |  |

### Return type

[**models::ApiListKnowledgeBaseIndexingJobsOutput**](apiListKnowledgeBaseIndexingJobsOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_list_indexing_jobs_by_knowledge_base

> models::ApiListKnowledgeBaseIndexingJobsOutput genai_list_indexing_jobs_by_knowledge_base(knowledge_base_uuid)
List Indexing Jobs for a Knowledge Base

To list latest 15 indexing jobs for a knowledge base, send a GET request to `/v2/gen-ai/knowledge_bases/{knowledge_base_uuid}/indexing_jobs`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**knowledge_base_uuid** | **String** | Knowledge base uuid in string | [required] |

### Return type

[**models::ApiListKnowledgeBaseIndexingJobsOutput**](apiListKnowledgeBaseIndexingJobsOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_list_knowledge_base_data_sources

> models::ApiListKnowledgeBaseDataSourcesOutput genai_list_knowledge_base_data_sources(knowledge_base_uuid, page, per_page)
List Data Sources for a Knowledge Base

To list all data sources for a knowledge base, send a GET request to `/v2/gen-ai/knowledge_bases/{knowledge_base_uuid}/data_sources`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**knowledge_base_uuid** | **String** | Knowledge base id | [required] |
**page** | Option<**i32**> | Page number. |  |
**per_page** | Option<**i32**> | Items per page. |  |

### Return type

[**models::ApiListKnowledgeBaseDataSourcesOutput**](apiListKnowledgeBaseDataSourcesOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_list_knowledge_bases

> models::ApiListKnowledgeBasesOutput genai_list_knowledge_bases(page, per_page)
List Knowledge Bases

To list all knowledge bases, send a GET request to `/v2/gen-ai/knowledge_bases`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number. |  |
**per_page** | Option<**i32**> | Items per page. |  |

### Return type

[**models::ApiListKnowledgeBasesOutput**](apiListKnowledgeBasesOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_list_model_api_keys

> models::ApiListModelApiKeysOutput genai_list_model_api_keys(page, per_page)
List Model API Keys

To list all model API keys, send a GET request to `/v2/gen-ai/models/api_keys`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number. |  |
**per_page** | Option<**i32**> | Items per page. |  |

### Return type

[**models::ApiListModelApiKeysOutput**](apiListModelAPIKeysOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_list_models

> models::ApiListModelsOutputPublic genai_list_models(usecases, public_only, page, per_page)
List Available Models

To list all models, send a GET request to `/v2/gen-ai/models`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**usecases** | Option<[**Vec<String>**](String.md)> | Include only models defined for the listed usecases.   - MODEL_USECASE_UNKNOWN: The use case of the model is unknown  - MODEL_USECASE_AGENT: The model maybe used in an agent  - MODEL_USECASE_FINETUNED: The model maybe used for fine tuning  - MODEL_USECASE_KNOWLEDGEBASE: The model maybe used for knowledge bases (embedding models)  - MODEL_USECASE_GUARDRAIL: The model maybe used for guardrails  - MODEL_USECASE_REASONING: The model usecase for reasoning  - MODEL_USECASE_SERVERLESS: The model usecase for serverless inference |  |
**public_only** | Option<**bool**> | Only include models that are publicly available. |  |
**page** | Option<**i32**> | Page number. |  |
**per_page** | Option<**i32**> | Items per page. |  |

### Return type

[**models::ApiListModelsOutputPublic**](apiListModelsOutputPublic.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_list_openai_api_keys

> models::ApiListOpenAiapiKeysOutput genai_list_openai_api_keys(page, per_page)
List OpenAI API Keys

To list all OpenAI API keys, send a GET request to `/v2/gen-ai/openai/keys`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number. |  |
**per_page** | Option<**i32**> | Items per page. |  |

### Return type

[**models::ApiListOpenAiapiKeysOutput**](apiListOpenAIAPIKeysOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_list_workspaces

> models::ApiListWorkspacesOutput genai_list_workspaces()
List Workspaces

To list all workspaces, send a GET request to `/v2/gen-ai/workspaces`.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApiListWorkspacesOutput**](apiListWorkspacesOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_regenerate_agent_api_key

> models::ApiRegenerateAgentApiKeyOutput genai_regenerate_agent_api_key(agent_uuid, api_key_uuid)
Regenerate API Key for an Agent

To regenerate an agent API key, send a PUT request to `/v2/gen-ai/agents/{agent_uuid}/api_keys/{api_key_uuid}/regenerate`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_uuid** | **String** | Agent id | [required] |
**api_key_uuid** | **String** | API key ID | [required] |

### Return type

[**models::ApiRegenerateAgentApiKeyOutput**](apiRegenerateAgentAPIKeyOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_regenerate_model_api_key

> models::ApiRegenerateModelApiKeyOutput genai_regenerate_model_api_key(api_key_uuid)
Regenerate API Key for a Model

To regenerate a model API key, send a PUT request to `/v2/gen-ai/models/api_keys/{api_key_uuid}/regenerate`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key_uuid** | **String** | API key ID | [required] |

### Return type

[**models::ApiRegenerateModelApiKeyOutput**](apiRegenerateModelAPIKeyOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_rollback_to_agent_version

> models::ApiRollbackToAgentVersionOutput genai_rollback_to_agent_version(uuid, api_rollback_to_agent_version_input_public)
Rollback to Agent Version

To update to a specific agent version, send a PUT request to `/v2/gen-ai/agents/{uuid}/versions`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** | Agent unique identifier | [required] |
**api_rollback_to_agent_version_input_public** | Option<[**ApiRollbackToAgentVersionInputPublic**](ApiRollbackToAgentVersionInputPublic.md)> |  |  |

### Return type

[**models::ApiRollbackToAgentVersionOutput**](apiRollbackToAgentVersionOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_run_evaluation_test_case

> models::ApiRunEvaluationTestCaseOutput genai_run_evaluation_test_case(api_run_evaluation_test_case_input_public)
Run an Evaluation Test Case

To run an evaluation test case, send a POST request to `/v2/gen-ai/evaluation_runs`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_run_evaluation_test_case_input_public** | Option<[**ApiRunEvaluationTestCaseInputPublic**](ApiRunEvaluationTestCaseInputPublic.md)> |  |  |

### Return type

[**models::ApiRunEvaluationTestCaseOutput**](apiRunEvaluationTestCaseOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_update_agent

> models::ApiUpdateAgentOutput genai_update_agent(uuid, api_update_agent_input_public)
Update an Agent

To update an agent, send a PUT request to `/v2/gen-ai/agents/{uuid}`. The response body is a JSON object containing the agent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** | Unique agent id | [required] |
**api_update_agent_input_public** | Option<[**ApiUpdateAgentInputPublic**](ApiUpdateAgentInputPublic.md)> |  |  |

### Return type

[**models::ApiUpdateAgentOutput**](apiUpdateAgentOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_update_agent_api_key

> models::ApiUpdateAgentApiKeyOutput genai_update_agent_api_key(agent_uuid, api_key_uuid, api_update_agent_api_key_input_public)
Update API Key for an Agent

To update an agent API key, send a PUT request to `/v2/gen-ai/agents/{agent_uuid}/api_keys/{api_key_uuid}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_uuid** | **String** | Agent id | [required] |
**api_key_uuid** | **String** | API key ID | [required] |
**api_update_agent_api_key_input_public** | Option<[**ApiUpdateAgentApiKeyInputPublic**](ApiUpdateAgentApiKeyInputPublic.md)> |  |  |

### Return type

[**models::ApiUpdateAgentApiKeyOutput**](apiUpdateAgentAPIKeyOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_update_agent_deployment_visibility

> models::ApiUpdateAgentDeploymentVisbilityOutput genai_update_agent_deployment_visibility(uuid, api_update_agent_deployment_visibility_input_public)
Update Agent Status

Check whether an agent is public or private. To update the agent status, send a PUT request to `/v2/gen-ai/agents/{uuid}/deployment_visibility`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** | Unique id | [required] |
**api_update_agent_deployment_visibility_input_public** | Option<[**ApiUpdateAgentDeploymentVisibilityInputPublic**](ApiUpdateAgentDeploymentVisibilityInputPublic.md)> |  |  |

### Return type

[**models::ApiUpdateAgentDeploymentVisbilityOutput**](apiUpdateAgentDeploymentVisbilityOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_update_agent_function

> models::ApiUpdateAgentFunctionOutput genai_update_agent_function(agent_uuid, function_uuid, api_update_agent_function_input_public)
Update Function Route for an Agent

To update the function route, send a PUT request to `/v2/gen-ai/agents/{agent_uuid}/functions/{function_uuid}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_uuid** | **String** | Agent id | [required] |
**function_uuid** | **String** | Function id | [required] |
**api_update_agent_function_input_public** | Option<[**ApiUpdateAgentFunctionInputPublic**](ApiUpdateAgentFunctionInputPublic.md)> |  |  |

### Return type

[**models::ApiUpdateAgentFunctionOutput**](apiUpdateAgentFunctionOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_update_agents_workspace

> models::ApiMoveAgentsToWorkspaceOutput genai_update_agents_workspace(workspace_uuid, api_move_agents_to_workspace_input_public)
Move Agents to a Workspace

To move all listed agents a given workspace, send a PUT request to `/v2/gen-ai/workspaces/{workspace_uuid}/agents`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_uuid** | **String** | Workspace uuid to move agents to | [required] |
**api_move_agents_to_workspace_input_public** | Option<[**ApiMoveAgentsToWorkspaceInputPublic**](ApiMoveAgentsToWorkspaceInputPublic.md)> |  |  |

### Return type

[**models::ApiMoveAgentsToWorkspaceOutput**](apiMoveAgentsToWorkspaceOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_update_anthropic_api_key

> models::ApiUpdateAnthropicApiKeyOutput genai_update_anthropic_api_key(api_key_uuid, api_update_anthropic_api_key_input_public)
Update Anthropic API Key

To update an Anthropic API key, send a PUT request to `/v2/gen-ai/anthropic/keys/{api_key_uuid}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key_uuid** | **String** | API key ID | [required] |
**api_update_anthropic_api_key_input_public** | Option<[**ApiUpdateAnthropicApiKeyInputPublic**](ApiUpdateAnthropicApiKeyInputPublic.md)> |  |  |

### Return type

[**models::ApiUpdateAnthropicApiKeyOutput**](apiUpdateAnthropicAPIKeyOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_update_attached_agent

> models::ApiUpdateLinkedAgentOutput genai_update_attached_agent(parent_agent_uuid, child_agent_uuid, api_update_linked_agent_input_public)
Update Agent Route for an Agent

To update an agent route for an agent, send a PUT request to `/v2/gen-ai/agents/{parent_agent_uuid}/child_agents/{child_agent_uuid}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parent_agent_uuid** | **String** | A unique identifier for the parent agent. | [required] |
**child_agent_uuid** | **String** | Routed agent id | [required] |
**api_update_linked_agent_input_public** | Option<[**ApiUpdateLinkedAgentInputPublic**](ApiUpdateLinkedAgentInputPublic.md)> |  |  |

### Return type

[**models::ApiUpdateLinkedAgentOutput**](apiUpdateLinkedAgentOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_update_evaluation_test_case

> models::ApiUpdateEvaluationTestCaseOutput genai_update_evaluation_test_case(test_case_uuid, api_update_evaluation_test_case_input_public)
Update an Evaluation Test Case.

To update an evaluation test-case send a PUT request to `/v2/gen-ai/evaluation_test_cases/{test_case_uuid}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**test_case_uuid** | **String** | Test-case UUID to update | [required] |
**api_update_evaluation_test_case_input_public** | Option<[**ApiUpdateEvaluationTestCaseInputPublic**](ApiUpdateEvaluationTestCaseInputPublic.md)> |  |  |

### Return type

[**models::ApiUpdateEvaluationTestCaseOutput**](apiUpdateEvaluationTestCaseOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_update_knowledge_base

> models::ApiUpdateKnowledgeBaseOutput genai_update_knowledge_base(uuid, api_update_knowledge_base_input_public)
Update a Knowledge Base

To update a knowledge base, send a PUT request to `/v2/gen-ai/knowledge_bases/{uuid}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** | Knowledge base id | [required] |
**api_update_knowledge_base_input_public** | Option<[**ApiUpdateKnowledgeBaseInputPublic**](ApiUpdateKnowledgeBaseInputPublic.md)> |  |  |

### Return type

[**models::ApiUpdateKnowledgeBaseOutput**](apiUpdateKnowledgeBaseOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_update_model_api_key

> models::ApiUpdateModelApiKeyOutput genai_update_model_api_key(api_key_uuid, api_update_model_api_key_input_public)
Update API Key for a Model

To update a model API key, send a PUT request to `/v2/gen-ai/models/api_keys/{api_key_uuid}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key_uuid** | **String** | API key ID | [required] |
**api_update_model_api_key_input_public** | Option<[**ApiUpdateModelApiKeyInputPublic**](ApiUpdateModelApiKeyInputPublic.md)> |  |  |

### Return type

[**models::ApiUpdateModelApiKeyOutput**](apiUpdateModelAPIKeyOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_update_openai_api_key

> models::ApiUpdateOpenAiapiKeyOutput genai_update_openai_api_key(api_key_uuid, api_update_open_aiapi_key_input_public)
Update OpenAI API Key

To update an OpenAI API key, send a PUT request to `/v2/gen-ai/openai/keys/{api_key_uuid}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key_uuid** | **String** | API key ID | [required] |
**api_update_open_aiapi_key_input_public** | Option<[**ApiUpdateOpenAiapiKeyInputPublic**](ApiUpdateOpenAiapiKeyInputPublic.md)> |  |  |

### Return type

[**models::ApiUpdateOpenAiapiKeyOutput**](apiUpdateOpenAIAPIKeyOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## genai_update_workspace

> models::ApiUpdateWorkspaceOutput genai_update_workspace(workspace_uuid, api_update_workspace_input_public)
Update a Workspace

To update a workspace, send a PUT request to `/v2/gen-ai/workspaces/{workspace_uuid}`. The response body is a JSON object containing the workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_uuid** | **String** | Workspace UUID. | [required] |
**api_update_workspace_input_public** | Option<[**ApiUpdateWorkspaceInputPublic**](ApiUpdateWorkspaceInputPublic.md)> |  |  |

### Return type

[**models::ApiUpdateWorkspaceOutput**](apiUpdateWorkspaceOutput.md)

### Authorization

[bearer_auth](../README.md#bearer_auth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

