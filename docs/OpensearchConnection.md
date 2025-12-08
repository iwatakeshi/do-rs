# OpensearchConnection

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uri** | Option<**String**> | This is provided as a convenience and should be able to be constructed by the other attributes. | [optional][readonly]
**host** | Option<**String**> | The FQDN pointing to the opensearch cluster's current primary node. | [optional][readonly]
**port** | Option<**i32**> | The port on which the opensearch dashboard is listening. | [optional][readonly]
**user** | Option<**String**> | The default user for the opensearch dashboard.<br><br>Requires `database:view_credentials` scope. | [optional][readonly]
**password** | Option<**String**> | The randomly generated password for the default user.<br><br>Requires `database:view_credentials` scope. | [optional][readonly]
**ssl** | Option<**bool**> | A boolean value indicating if the connection should be made over SSL. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


