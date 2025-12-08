# ApiCreateKnowledgeBaseInputPublic

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**database_id** | Option<**String**> | Identifier of the DigitalOcean OpenSearch database this knowledge base will use, optional. If not provided, we create a new database for the knowledge base in the same region as the knowledge base. | [optional]
**datasources** | Option<[**Vec<models::ApiKbDataSource>**](apiKBDataSource.md)> | The data sources to use for this knowledge base. See [Organize Data Sources](https://docs.digitalocean.com/products/genai-platform/concepts/best-practices/#spaces-buckets) for more information on data sources best practices. | [optional]
**embedding_model_uuid** | Option<**String**> | Identifier for the [embedding model](https://docs.digitalocean.com/products/genai-platform/details/models/#embedding-models). | [optional]
**name** | Option<**String**> | Name of the knowledge base. | [optional]
**project_id** | Option<**String**> | Identifier of the DigitalOcean project this knowledge base will belong to. | [optional]
**region** | Option<**String**> | The datacenter region to deploy the knowledge base in. | [optional]
**tags** | Option<**Vec<String>**> | Tags to organize your knowledge base. | [optional]
**vpc_uuid** | Option<**String**> | The VPC to deploy the knowledge base database in | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


