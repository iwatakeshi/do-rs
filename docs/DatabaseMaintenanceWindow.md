# DatabaseMaintenanceWindow

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**day** | **String** | The day of the week on which to apply maintenance updates. | 
**hour** | **String** | The hour in UTC at which maintenance updates will be applied in 24 hour format. | 
**pending** | Option<**bool**> | A boolean value indicating whether any maintenance is scheduled to be performed in the next window. | [optional][readonly]
**description** | Option<**Vec<String>**> | A list of strings, each containing information about a pending maintenance update. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


