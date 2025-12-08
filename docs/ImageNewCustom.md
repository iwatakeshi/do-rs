# ImageNewCustom

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The display name that has been given to an image.  This is what is shown in the control panel and is generally a descriptive title for the image in question. | 
**distribution** | Option<[**models::Distribution**](distribution.md)> |  | [optional]
**description** | Option<**String**> | An optional free-form text field to describe an image. | [optional]
**url** | **String** | A URL from which the custom Linux virtual machine image may be retrieved.  The image it points to must be in the raw, qcow2, vhdx, vdi, or vmdk format.  It may be compressed using gzip or bzip2 and must be smaller than 100 GB after being decompressed. | 
**region** | [**models::RegionSlug**](region_slug.md) |  | 
**tags** | Option<**Vec<String>**> | A flat array of tag names as strings to be applied to the resource. Tag names may be for either existing or new tags. <br><br>Requires `tag:create` scope. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


