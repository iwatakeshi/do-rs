# DatabasesListBackups200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**backups** | [**Vec<models::Backup>**](backup.md) |  | 
**scheduled_backup_time** | Option<[**models::DatabasesListBackups200ResponseScheduledBackupTime**](databases_list_backups_200_response_scheduled_backup_time.md)> |  | [optional]
**backup_progress** | Option<**String**> | If a backup is currently in progress, this attribute shows the percentage of completion. If no backup is in progress, this attribute will be hidden. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


