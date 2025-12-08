# FirewallRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uuid** | Option<**String**> | A unique ID for the firewall rule itself. | [optional]
**cluster_uuid** | Option<**String**> | A unique ID for the database cluster to which the rule is applied. | [optional][readonly]
**r#type** | **String** | The type of resource that the firewall rule allows to access the database cluster. | 
**value** | **String** | The ID of the specific resource, the name of a tag applied to a group of resources, or the IP address that the firewall rule allows to access the database cluster. | 
**created_at** | Option<**String**> | A time value given in ISO8601 combined date and time format that represents when the firewall rule was created. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


