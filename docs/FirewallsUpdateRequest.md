# FirewallsUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | A unique ID that can be used to identify and reference a firewall. | [optional][readonly]
**status** | Option<**String**> | A status string indicating the current state of the firewall. This can be \"waiting\", \"succeeded\", or \"failed\". | [optional][readonly]
**created_at** | Option<**String**> | A time value given in ISO8601 combined date and time format that represents when the firewall was created. | [optional][readonly]
**pending_changes** | Option<[**Vec<models::FirewallAllOfPendingChanges>**](firewall_allOf_pending_changes.md)> | An array of objects each containing the fields \"droplet_id\", \"removing\", and \"status\". It is provided to detail exactly which Droplets are having their security policies updated. When empty, all changes have been successfully applied. | [optional][readonly]
**name** | **String** | A human-readable name for a firewall. The name must begin with an alphanumeric character. Subsequent characters must either be alphanumeric characters, a period (.), or a dash (-). | 
**droplet_ids** | Option<**Vec<i32>**> | An array containing the IDs of the Droplets assigned to the firewall. <br><br>Requires `droplet:read` scope. | [optional]
**tags** | Option<**Vec<String>**> | An array containing the names of the Tags assigned to the firewall. <br><br>Requires `tag:read` scope. | [optional]
**inbound_rules** | Option<[**Vec<models::FirewallRulesInboundRulesInner>**](firewall_rules_inbound_rules_inner.md)> |  | [optional]
**outbound_rules** | Option<[**Vec<models::FirewallRulesOutboundRulesInner>**](firewall_rules_outbound_rules_inner.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


