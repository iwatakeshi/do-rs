# DatabasesAddUserRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of a database user. | 
**role** | Option<**String**> | A string representing the database user's role. The value will be either \"primary\" or \"normal\".  | [optional][readonly]
**password** | Option<**String**> | A randomly generated password for the database user.<br>Requires `database:view_credentials` scope. | [optional][readonly]
**access_cert** | Option<**String**> | Access certificate for TLS client authentication. (Kafka only) | [optional][readonly]
**access_key** | Option<**String**> | Access key for TLS client authentication. (Kafka only) | [optional][readonly]
**mysql_settings** | Option<[**models::MysqlSettings**](mysql_settings.md)> |  | [optional]
**settings** | Option<[**models::UserSettings**](user_settings.md)> |  | [optional]
**readonly** | Option<**bool**> | (To be deprecated: use settings.mongo_user_settings.role instead for access controls to MongoDB databases).  For MongoDB clusters, set to `true` to create a read-only user. This option is not currently supported for other database engines.              | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


