# Image

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | A unique number that can be used to identify and reference a specific image. | [optional][readonly]
**name** | Option<**String**> | The display name that has been given to an image.  This is what is shown in the control panel and is generally a descriptive title for the image in question. | [optional]
**r#type** | Option<**String**> | Describes the kind of image. It may be one of `base`, `snapshot`, `backup`, `custom`, or `admin`. Respectively, this specifies whether an image is a DigitalOcean base OS image, user-generated Droplet snapshot, automatically created Droplet backup, user-provided virtual machine image, or an image used for DigitalOcean managed resources (e.g. DOKS worker nodes). | [optional]
**distribution** | Option<[**models::Distribution**](distribution.md)> |  | [optional]
**slug** | Option<**String**> | A uniquely identifying string that is associated with each of the DigitalOcean-provided public images. These can be used to reference a public image as an alternative to the numeric id. | [optional]
**public** | Option<**bool**> | This is a boolean value that indicates whether the image in question is public or not. An image that is public is available to all accounts. A non-public image is only accessible from your account. | [optional]
**regions** | Option<[**Vec<models::RegionSlug>**](region_slug.md)> | This attribute is an array of the regions that the image is available in. The regions are represented by their identifying slug values. | [optional]
**created_at** | Option<**String**> | A time value given in ISO8601 combined date and time format that represents when the image was created. | [optional]
**min_disk_size** | Option<**i32**> | The minimum disk size in GB required for a Droplet to use this image. | [optional]
**size_gigabytes** | Option<**f32**> | The size of the image in gigabytes. | [optional]
**description** | Option<**String**> | An optional free-form text field to describe an image. | [optional]
**tags** | Option<**Vec<String>**> | A flat array of tag names as strings to be applied to the resource. Tag names may be for either existing or new tags. <br><br>Requires `tag:create` scope. | [optional]
**status** | Option<**String**> | A status string indicating the state of a custom image. This may be `NEW`,  `available`, `pending`, `deleted`, or `retired`. | [optional]
**error_message** | Option<**String**> | A string containing information about errors that may occur when importing  a custom image. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


