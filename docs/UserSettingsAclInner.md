# UserSettingsAclInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | An identifier for the ACL. Will be computed after the ACL is created/updated. | [optional]
**topic** | **String** | A regex for matching the topic(s) that this ACL should apply to. | 
**permission** | **String** | Permission set applied to the ACL. 'consume' allows for messages to be consumed from the topic. 'produce' allows for messages to be published to the topic. 'produceconsume' allows for both 'consume' and 'produce' permission. 'admin' allows for 'produceconsume' as well as any operations to administer the topic (delete, update). | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


