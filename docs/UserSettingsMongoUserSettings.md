# UserSettingsMongoUserSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**databases** | Option<**Vec<String>**> | A list of databases to which the user should have access. When the database is set to `admin`, the user will have access to all databases based on the user's role i.e. a user with the role `readOnly` assigned to the `admin` database will have read access to all databases. | [optional]
**role** | Option<**String**> | The role to assign to the user with each role mapping to a MongoDB built-in role.  `readOnly` maps to a [read](https://www.mongodb.com/docs/manual/reference/built-in-roles/#mongodb-authrole-read) role. `readWrite` maps to a [readWrite](https://www.mongodb.com/docs/manual/reference/built-in-roles/#mongodb-authrole-readWrite) role. `dbAdmin` maps to a [dbAdmin](https://www.mongodb.com/docs/manual/reference/built-in-roles/#mongodb-authrole-dbAdmin) role. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


