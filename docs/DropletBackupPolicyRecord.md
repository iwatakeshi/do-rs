# DropletBackupPolicyRecord

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**droplet_id** | Option<**i32**> | The unique identifier for the Droplet. | [optional]
**backup_enabled** | Option<**bool**> | A boolean value indicating whether backups are enabled for the Droplet. | [optional]
**backup_policy** | Option<[**models::DropletBackupPolicy**](droplet_backup_policy.md)> | An object specifying the backup policy for the Droplet. | [optional]
**next_backup_window** | Option<[**models::DropletNextBackupWindow**](droplet_next_backup_window.md)> | An object containing keys with the start and end times of the window during which the backup will occur. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


