# Region

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The display name of the region.  This will be a full name that is used in the control panel and other interfaces. | 
**slug** | **String** | A human-readable string that is used as a unique identifier for each region. | 
**features** | **Vec<String>** | This attribute is set to an array which contains features available in this region | 
**available** | **bool** | This is a boolean value that represents whether new Droplets can be created in this region. | 
**sizes** | **Vec<String>** | This attribute is set to an array which contains the identifying slugs for the sizes available in this region. sizes:read is required to view. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


