# DropletMultiCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**region** | Option<**String**> | The slug identifier for the region that you wish to deploy the Droplet in. If the specific datacenter is not not important, a slug prefix (e.g. `nyc`) can be used to deploy the Droplet in any of the that region's locations (`nyc1`, `nyc2`, or `nyc3`). If the region is omitted from the create request completely, the Droplet may deploy in any region. | [optional]
**size** | **String** | The slug identifier for the size that you wish to select for this Droplet. | 
**image** | [**models::DropletCreateImage**](droplet_create_image.md) |  | 
**ssh_keys** | Option<[**Vec<models::DropletCreateSshKeysInner>**](droplet_create_ssh_keys_inner.md)> | An array containing the IDs or fingerprints of the SSH keys that you wish to embed in the Droplet's root account upon creation. You must add the keys to your team before they can be embedded on a Droplet.<br>Requires `ssh_key:read` scope. | [optional][default to []]
**backups** | Option<**bool**> | A boolean indicating whether automated backups should be enabled for the Droplet. | [optional][default to false]
**backup_policy** | Option<[**models::DropletBackupPolicy**](droplet_backup_policy.md)> | An object specifying the backup policy for the Droplet. If omitted and `backups` is `true`, the backup plan will default to daily. | [optional]
**ipv6** | Option<**bool**> | A boolean indicating whether to enable IPv6 on the Droplet. | [optional][default to false]
**monitoring** | Option<**bool**> | A boolean indicating whether to install the DigitalOcean agent for monitoring. | [optional][default to false]
**tags** | Option<**Vec<String>**> | A flat array of tag names as strings to apply to the Droplet after it is created. Tag names can either be existing or new tags.<br>Requires `tag:create` scope. | [optional][default to []]
**user_data** | Option<**String**> | A string containing 'user data' which may be used to configure the Droplet on first boot, often a 'cloud-config' file or Bash script. It must be plain text and may not exceed 64 KiB in size. | [optional]
**private_networking** | Option<**bool**> | This parameter has been deprecated. Use `vpc_uuid` instead to specify a VPC network for the Droplet. If no `vpc_uuid` is provided, the Droplet will be placed in your account's default VPC for the region. | [optional][default to false]
**volumes** | Option<**Vec<String>**> | An array of IDs for block storage volumes that will be attached to the Droplet once created. The volumes must not already be attached to an existing Droplet.<br>Requires `block_storage:read` scpoe. | [optional][default to []]
**vpc_uuid** | Option<**String**> | A string specifying the UUID of the VPC to which the Droplet will be assigned. If excluded, the Droplet will be assigned to your account's default VPC for the region.<br>Requires `vpc:read` scope. | [optional]
**with_droplet_agent** | Option<**bool**> | A boolean indicating whether to install the DigitalOcean agent used for providing access to the Droplet web console in the control panel. By default, the agent is installed on new Droplets but installation errors (i.e. OS not supported) are ignored. To prevent it from being installed, set to `false`. To make installation errors fatal, explicitly set it to `true`. | [optional]
**names** | **Vec<String>** | An array of human human-readable strings you wish to use when displaying the Droplet name. Each name, if set to a domain name managed in the DigitalOcean DNS management system, will configure a PTR record for the Droplet. Each name set during creation will also determine the hostname for the Droplet in its internal configuration. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


