# VolumeFull

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The unique identifier for the block storage volume. | [optional][readonly]
**droplet_ids** | Option<**Vec<i32>**> | An array containing the IDs of the Droplets the volume is attached to. Note that at this time, a volume can only be attached to a single Droplet. | [optional][readonly]
**name** | Option<**String**> | A human-readable name for the block storage volume. Must be lowercase and be composed only of numbers, letters and \"-\", up to a limit of 64 characters. The name must begin with a letter. | [optional]
**description** | Option<**String**> | An optional free-form text field to describe a block storage volume. | [optional]
**size_gigabytes** | Option<**i32**> | The size of the block storage volume in GiB (1024^3). This field does not apply  when creating a volume from a snapshot. | [optional]
**created_at** | Option<**String**> | A time value given in ISO8601 combined date and time format that represents when the block storage volume was created. | [optional][readonly]
**tags** | Option<**Vec<String>**> | A flat array of tag names as strings applied to the resource. <br><br>Requires `tag:read` scope. | [optional]
**region** | Option<[**models::Region**](region.md)> | The region that the block storage volume is located in. When setting a region, the value should be the slug identifier for the region. When you query a block storage volume, the entire region object will be returned. | [optional][readonly]
**filesystem_type** | Option<**String**> | The type of filesystem currently in-use on the volume. | [optional]
**filesystem_label** | Option<**String**> | The label currently applied to the filesystem. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


