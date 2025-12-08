# DropletsListAssociatedResources200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**reserved_ips** | Option<[**Vec<models::AssociatedResource>**](associated_resource.md)> | Reserved IPs that are associated with this Droplet.<br>Requires `reserved_ip:read` scope. | [optional]
**floating_ips** | Option<[**Vec<models::AssociatedResource>**](associated_resource.md)> | Floating IPs that are associated with this Droplet.<br>Requires `reserved_ip:read` scope. | [optional]
**snapshots** | Option<[**Vec<models::AssociatedResource>**](associated_resource.md)> | Snapshots that are associated with this Droplet.<br>Requires `image:read` scope. | [optional]
**volumes** | Option<[**Vec<models::AssociatedResource>**](associated_resource.md)> | Volumes that are associated with this Droplet.<br>Requires `block_storage:read` scope. | [optional]
**volume_snapshots** | Option<[**Vec<models::AssociatedResource>**](associated_resource.md)> | Volume Snapshots that are associated with this Droplet.<br>Requires `block_storage_snapshot:read` scope. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


