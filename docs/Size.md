# Size

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**slug** | **String** | A human-readable string that is used to uniquely identify each size. | 
**memory** | **i32** | The amount of RAM allocated to Droplets created of this size. The value is represented in megabytes. | 
**vcpus** | **i32** | The number of CPUs allocated to Droplets of this size. | 
**disk** | **i32** | The amount of disk space set aside for Droplets of this size. The value is represented in gigabytes. | 
**transfer** | **f32** | The amount of transfer bandwidth that is available for Droplets created in this size. This only counts traffic on the public interface. The value is given in terabytes. | 
**price_monthly** | **f32** | This attribute describes the monthly cost of this Droplet size if the Droplet is kept for an entire month. The value is measured in US dollars. | 
**price_hourly** | **f32** | This describes the price of the Droplet size as measured hourly. The value is measured in US dollars. | 
**regions** | **Vec<String>** | An array containing the region slugs where this size is available for Droplet creates. | 
**available** | **bool** | This is a boolean value that represents whether new Droplets can be created with this size. | [default to true]
**description** | **String** | A string describing the class of Droplets created from this size. For example: Basic, General Purpose, CPU-Optimized, Memory-Optimized, or Storage-Optimized. | 
**disk_info** | Option<[**Vec<models::DiskInfo>**](disk_info.md)> | An array of objects containing information about the disks available to Droplets created with this size. | [optional]
**gpu_info** | Option<[**models::GpuInfo**](gpu_info.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


