# AutoscalePoolDropletTemplate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name(s) to be applied to all Droplets in the autoscale pool. | [optional]
**region** | **String** | The datacenter in which all of the Droplets will be created. | 
**size** | **String** | The Droplet size to be used for all Droplets in the autoscale pool. | 
**image** | **String** | The Droplet image to be used for all Droplets in the autoscale pool. You may specify the slug or the image ID. | 
**ssh_keys** | **Vec<String>** | The SSH keys to be installed on the Droplets in the autoscale pool. You can either specify the key ID or the fingerprint. Requires `ssh_key:read` scope.  | 
**tags** | Option<**Vec<String>**> | The tags to apply to each of the Droplets in the autoscale pool. Requires `tag:read` scope.  | [optional]
**vpc_uuid** | Option<**String**> | The VPC where the Droplets in the autoscale pool will be created. The VPC must be in the region where you want to create the Droplets. Requires `vpc:read` scope.  | [optional]
**with_droplet_agent** | Option<**bool**> | Installs the Droplet agent. This must be set to true to monitor Droplets for resource utilization scaling. | [optional]
**project_id** | Option<**String**> | The project that the Droplets in the autoscale pool will belong to. Requires `project:read` scope.  | [optional]
**ipv6** | Option<**bool**> | Assigns a unique IPv6 address to each of the Droplets in the autoscale pool. | [optional]
**user_data** | Option<**String**> | A string containing user data that cloud-init consumes to configure a Droplet on first boot. User data is often a cloud-config file or Bash script. It must be plain text and may not exceed 64 KiB in size. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


