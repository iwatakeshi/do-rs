# DatabaseClusterRead

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | A unique ID that can be used to identify and reference a database cluster. | [optional][readonly]
**name** | **String** | A unique, human-readable name referring to a database cluster. | 
**engine** | **String** | A slug representing the database engine used for the cluster. The possible values are: \"pg\" for PostgreSQL, \"mysql\" for MySQL, \"redis\" for Caching, \"mongodb\" for MongoDB, \"kafka\" for Kafka, \"opensearch\" for OpenSearch, and \"valkey\" for Valkey. | 
**version** | Option<**String**> | A string representing the version of the database engine in use for the cluster. | [optional]
**semantic_version** | Option<**String**> | A string representing the semantic version of the database engine in use for the cluster. | [optional][readonly]
**num_nodes** | **i32** | The number of nodes in the database cluster. | 
**size** | **String** | The slug identifier representing the size of the nodes in the database cluster. | 
**region** | **String** | The slug identifier for the region where the database cluster is located. | 
**status** | Option<**String**> | A string representing the current status of the database cluster. | [optional][readonly]
**created_at** | Option<**String**> | A time value given in ISO8601 combined date and time format that represents when the database cluster was created. | [optional][readonly]
**private_network_uuid** | Option<**String**> | A string specifying the UUID of the VPC to which the database cluster will be assigned. If excluded, the cluster when creating a new database cluster, it will be assigned to your account's default VPC for the region. <br><br>Requires `vpc:read` scope. | [optional]
**tags** | Option<**Vec<String>**> | An array of tags that have been applied to the database cluster. <br><br>Requires `tag:read` scope. | [optional]
**db_names** | Option<**Vec<String>**> | An array of strings containing the names of databases created in the database cluster. | [optional][readonly]
**ui_connection** | Option<[**models::OpensearchConnection**](opensearch_connection.md)> | The connection details for OpenSearch dashboard.  | [optional][readonly]
**schema_registry_connection** | Option<[**models::SchemaRegistryConnection**](schema_registry_connection.md)> | The connection details for Schema Registry. | [optional][readonly]
**connection** | Option<[**models::DatabaseConnection**](database_connection.md)> |  | [optional][readonly]
**private_connection** | Option<[**models::DatabaseConnection**](database_connection.md)> |  | [optional][readonly]
**standby_connection** | Option<[**models::DatabaseConnection**](database_connection.md)> |  | [optional][readonly]
**standby_private_connection** | Option<[**models::DatabaseConnection**](database_connection.md)> |  | [optional][readonly]
**users** | Option<[**Vec<models::DatabaseUser>**](database_user.md)> |  | [optional][readonly]
**maintenance_window** | Option<[**models::DatabaseMaintenanceWindow**](database_maintenance_window.md)> |  | [optional][readonly]
**project_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The ID of the project that the database cluster is assigned to. If excluded when creating a new database cluster, it will be assigned to your default project.<br><br>Requires `project:read` scope. | [optional]
**rules** | Option<[**Vec<models::FirewallRule>**](firewall_rule.md)> |  | [optional]
**version_end_of_life** | Option<**String**> | A timestamp referring to the date when the particular version will no longer be supported. If null, the version does not have an end of life timeline. | [optional][readonly]
**version_end_of_availability** | Option<**String**> | A timestamp referring to the date when the particular version will no longer be available for creating new clusters. If null, the version does not have an end of availability timeline. | [optional][readonly]
**storage_size_mib** | Option<**i32**> | Additional storage added to the cluster, in MiB. If null, no additional storage is added to the cluster, beyond what is provided as a base amount from the 'size' and any previously added additional storage. | [optional]
**metrics_endpoints** | Option<[**Vec<models::DatabaseServiceEndpoint>**](database_service_endpoint.md)> | Public hostname and port of the cluster's metrics endpoint(s). Includes one record for the cluster's primary node and a second entry for the cluster's standby node(s). | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


