# UserSettingsOpensearchAclInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**index** | Option<**String**> | A regex for matching the indexes that this ACL should apply to. | [optional]
**permission** | Option<**String**> | Permission set applied to the ACL. 'read' allows user to read from the index. 'write' allows for user to write to the index. 'readwrite' allows for both 'read' and 'write' permission. 'deny'(default) restricts user from performing any operation over an index. 'admin' allows for 'readwrite' as well as any operations to administer the index. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


