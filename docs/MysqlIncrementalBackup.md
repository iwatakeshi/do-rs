# MysqlIncrementalBackup

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enabled** | Option<**bool**> | Enable periodic incremental backups. When enabled, full_backup_week_schedule must be set. Incremental backups only store changes since the last backup, making them faster and more storage-efficient than full backups. This is particularly useful for large databases where daily full backups would be too time-consuming or expensive. | [optional]
**full_backup_week_schedule** | Option<**String**> | Comma-separated list of days of the week when full backups should be created. Valid values: mon, tue, wed, thu, fri, sat, sun. Default is null. Example : \"mon,fri,sun\".  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


