# DropletBackupPolicy

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**plan** | Option<**String**> | The backup plan used for the Droplet. The plan can be either `daily` or `weekly`. | [optional]
**weekday** | Option<**String**> | The day of the week on which the backup will occur. | [optional]
**hour** | Option<**i32**> | The hour of the day that the backup window will start. | [optional]
**window_length_hours** | Option<**i32**> | The length of the backup window starting from `hour`. | [optional][readonly]
**retention_period_days** | Option<**i32**> | The number of days the backup will be retained. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


