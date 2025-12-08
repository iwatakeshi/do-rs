# SchemaRegistryConnection

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uri** | Option<**String**> | This is provided as a convenience and should be able to be constructed by the other attributes. | [optional][readonly]
**host** | Option<**String**> | The FQDN pointing to the schema registry connection uri. | [optional][readonly]
**port** | Option<**i32**> | The port on which the schema registry is listening. | [optional][readonly]
**user** | Option<**String**> | The default user for the schema registry.<br><br>Requires `database:view_credentials` scope. | [optional][readonly]
**password** | Option<**String**> | The randomly generated password for the schema registry.<br><br>Requires `database:view_credentials` scope. | [optional][readonly]
**ssl** | Option<**bool**> | A boolean value indicating if the connection should be made over SSL. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


