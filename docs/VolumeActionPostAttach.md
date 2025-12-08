# VolumeActionPostAttach

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | The volume action to initiate. | 
**region** | Option<[**models::RegionSlug**](region_slug.md)> |  | [optional]
**droplet_id** | **i32** | The unique identifier for the Droplet the volume will be attached or detached from. | 
**tags** | Option<**Vec<String>**> | A flat array of tag names as strings to be applied to the resource. Tag names may be for either existing or new tags. <br><br>Requires `tag:create` scope. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


