# Tags

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the tag. Tags may contain letters, numbers, colons, dashes, and underscores. There is a limit of 255 characters per tag.  **Note:** Tag names are case stable, which means the capitalization you use when you first create a tag is canonical.  When working with tags in the API, you must use the tag's canonical capitalization. For example, if you create a tag named \"PROD\", the URL to add that tag to a resource would be `https://api.digitalocean.com/v2/tags/PROD/resources` (not `/v2/tags/prod/resources`).  Tagged resources in the control panel will always display the canonical capitalization. For example, if you create a tag named \"PROD\", you can tag resources in the control panel by entering \"prod\". The tag will still display with its canonical capitalization, \"PROD\".  | [optional]
**resources** | Option<[**models::TagsResources**](tags_resources.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


