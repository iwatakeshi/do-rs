# UserSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pg_allow_replication** | Option<**bool**> | For Postgres clusters, set to `true` for a user with replication rights. This option is not currently supported for other database engines.  | [optional]
**opensearch_acl** | Option<[**Vec<models::UserSettingsOpensearchAclInner>**](user_settings_opensearch_acl_inner.md)> | ACLs (Access Control Lists) specifying permissions on index within a OpenSearch cluster. | [optional]
**acl** | Option<[**Vec<models::UserSettingsAclInner>**](user_settings_acl_inner.md)> | ACLs (Access Control Lists) specifying permissions on topics within a Kafka cluster. | [optional]
**mongo_user_settings** | Option<[**models::UserSettingsMongoUserSettings**](user_settings_mongo_user_settings.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


