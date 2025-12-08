# SupportedDropletBackupPolicy

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the Droplet backup plan. | [optional]
**possible_window_starts** | Option<**Vec<i32>**> | An array of integers representing the hours of the day that a backup can start.  | [optional]
**window_length_hours** | Option<**i32**> | The number of hours that a backup window is open. | [optional]
**retention_period_days** | Option<**i32**> | The number of days that a backup will be kept. | [optional]
**possible_days** | Option<**Vec<String>**> | The day of the week the backup will occur. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


