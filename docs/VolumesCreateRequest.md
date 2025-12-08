# VolumesCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The unique identifier for the block storage volume. | [optional][readonly]
**droplet_ids** | Option<**Vec<i32>**> | An array containing the IDs of the Droplets the volume is attached to. Note that at this time, a volume can only be attached to a single Droplet. | [optional][readonly]
**name** | **String** | A human-readable name for the block storage volume. Must be lowercase and be composed only of numbers, letters and \"-\", up to a limit of 64 characters. The name must begin with a letter. | 
**description** | Option<**String**> | An optional free-form text field to describe a block storage volume. | [optional]
**size_gigabytes** | **i32** | The size of the block storage volume in GiB (1024^3). This field does not apply  when creating a volume from a snapshot. | 
**created_at** | Option<**String**> | A time value given in ISO8601 combined date and time format that represents when the block storage volume was created. | [optional][readonly]
**tags** | Option<**Vec<String>**> | A flat array of tag names as strings to be applied to the resource. Tag names may be for either existing or new tags. <br><br>Requires `tag:create` scope. | [optional]
**snapshot_id** | Option<**String**> | The unique identifier for the volume snapshot from which to create the volume. | [optional]
**filesystem_type** | Option<**String**> | The name of the filesystem type to be used on the volume. When provided, the volume will automatically be formatted to the specified filesystem type. Currently, the available options are `ext4` and `xfs`. Pre-formatted volumes are automatically mounted when attached to Ubuntu, Debian, Fedora, Fedora Atomic, and CentOS Droplets created on or after April 26, 2018. Attaching pre-formatted volumes to other Droplets is not recommended. | [optional]
**region** | [**models::RegionSlug**](region_slug.md) |  | 
**filesystem_label** | Option<**String**> | The label applied to the filesystem. Labels for ext4 type filesystems may contain 16 characters while labels for xfs type filesystems are limited to 12 characters. May only be used in conjunction with filesystem_type. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


