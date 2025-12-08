# AddonsResourceNew

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**app_slug** | **String** | The slug identifier for the application associated with the resource. | 
**plan_slug** | **String** | The slug identifier for the plan associated with the resource. | 
**name** | **String** | The name of the addon resource. | 
**metadata** | [**Vec<models::AddonsResourceMetadata>**](addons_resource_metadata.md) | Metadata associated with the resource, set by the user. Metadata expected varies per app, and can be verified with a GET request to \"/v2/add-ons/apps/{app_slug}/metadata\" | 
**linked_droplet_id** | Option<**i32**> | ID of the droplet to be linked to this resource, if applicable. | [optional]
**fleet_uuid** | Option<**String**> | UUID of the fleet/project to which this resource will belong. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


