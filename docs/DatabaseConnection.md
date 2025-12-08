# DatabaseConnection

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uri** | Option<**String**> | A connection string in the format accepted by the `psql` command. This is provided as a convenience and should be able to be constructed by the other attributes. | [optional][readonly]
**database** | Option<**String**> | The name of the default database. | [optional][readonly]
**host** | Option<**String**> | The FQDN pointing to the database cluster's current primary node. | [optional][readonly]
**port** | Option<**i32**> | The port on which the database cluster is listening. | [optional][readonly]
**user** | Option<**String**> | The default user for the database.<br><br>Requires `database:view_credentials` scope. | [optional][readonly]
**password** | Option<**String**> | The randomly generated password for the default user.<br><br>Requires `database:view_credentials` scope. | [optional][readonly]
**ssl** | Option<**bool**> | A boolean value indicating if the connection should be made over SSL. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


