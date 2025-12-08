# Domain

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the domain itself. This should follow the standard domain format of domain.TLD. For instance, `example.com` is a valid domain name. | [optional]
**ip_address** | Option<**String**> | This optional attribute may contain an IP address. When provided, an A record will be automatically created pointing to the apex domain. | [optional]
**ttl** | Option<**i32**> | This value is the time to live for the records on this domain, in seconds. This defines the time frame that clients can cache queried information before a refresh should be requested. | [optional][readonly]
**zone_file** | Option<**String**> | This attribute contains the complete contents of the zone file for the selected domain. Individual domain record resources should be used to get more granular control over records. However, this attribute can also be used to get information about the SOA record, which is created automatically and is not accessible as an individual record resource. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


