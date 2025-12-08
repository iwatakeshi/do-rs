# ApiScheduledIndexingInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> | Created at timestamp | [optional]
**days** | Option<**Vec<i32>**> | Days for execution (day is represented same as in a cron expression, e.g. Monday begins with 1 ) | [optional]
**deleted_at** | Option<**String**> | Deleted at timestamp (if soft deleted) | [optional]
**is_active** | Option<**bool**> | Whether the schedule is currently active | [optional]
**knowledge_base_uuid** | Option<**String**> | Knowledge base uuid associated with this schedule | [optional]
**last_ran_at** | Option<**String**> | Last time the schedule was executed | [optional]
**next_run_at** | Option<**String**> | Next scheduled run | [optional]
**time** | Option<**String**> | Scheduled time of execution (HH:MM:SS format) | [optional]
**updated_at** | Option<**String**> | Updated at timestamp | [optional]
**uuid** | Option<**String**> | Unique identifier for the scheduled indexing entry | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


