# AppDatabaseSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cluster_name** | Option<**String**> | The name of the underlying DigitalOcean DBaaS cluster. This is required for production databases. For dev databases, if cluster_name is not set, a new cluster will be provisioned. | [optional]
**db_name** | Option<**String**> | The name of the MySQL or PostgreSQL database to configure. | [optional]
**db_user** | Option<**String**> | The name of the MySQL or PostgreSQL user to configure. | [optional]
**engine** | Option<**String**> | - MYSQL: MySQL - PG: PostgreSQL - REDIS: Caching - MONGODB: MongoDB - KAFKA: Kafka - OPENSEARCH: OpenSearch - VALKEY: ValKey | [optional][default to Unset]
**name** | **String** | The database's name. The name must be unique across all components within the same app and cannot use capital letters. | 
**production** | Option<**bool**> | Whether this is a production or dev database. | [optional]
**version** | Option<**String**> | The version of the database engine | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


