# ApiModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**agreement** | Option<[**models::ApiAgreement**](apiAgreement.md)> |  | [optional]
**created_at** | Option<**String**> | Creation date / time | [optional]
**inference_name** | Option<**String**> | Internally used name | [optional]
**inference_version** | Option<**String**> | Internally used version | [optional]
**is_foundational** | Option<**bool**> | True if it is a foundational model provided by do | [optional]
**kb_default_chunk_size** | Option<**i64**> | Default chunking size limit to show in UI | [optional]
**kb_max_chunk_size** | Option<**i64**> | Maximum chunk size limit of model | [optional]
**kb_min_chunk_size** | Option<**i64**> | Minimum chunking size token limits if model supports KNOWLEDGEBASE usecase | [optional]
**metadata** | Option<[**serde_json::Value**](.md)> | Additional meta data | [optional]
**name** | Option<**String**> | Name of the model | [optional]
**parent_uuid** | Option<**String**> | Unique id of the model, this model is based on | [optional]
**provider** | Option<[**models::ApiModelProvider**](apiModelProvider.md)> |  | [optional]
**updated_at** | Option<**String**> | Last modified | [optional]
**upload_complete** | Option<**bool**> | Model has been fully uploaded | [optional]
**url** | Option<**String**> | Download url | [optional]
**usecases** | Option<[**Vec<models::ApiModelUsecase>**](apiModelUsecase.md)> | Usecases of the model | [optional]
**uuid** | Option<**String**> | Unique id | [optional]
**version** | Option<[**models::ApiModelVersion**](apiModelVersion.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


