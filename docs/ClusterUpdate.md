# ClusterUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | A human-readable name for a Kubernetes cluster. | 
**tags** | Option<**Vec<String>**> | An array of tags applied to the Kubernetes cluster. All clusters are automatically tagged `k8s` and `k8s:$K8S_CLUSTER_ID`. | [optional]
**maintenance_policy** | Option<[**models::MaintenancePolicy**](maintenance_policy.md)> |  | [optional]
**auto_upgrade** | Option<**bool**> | A boolean value indicating whether the cluster will be automatically upgraded to new patch releases during its maintenance window. | [optional][default to false]
**surge_upgrade** | Option<**bool**> | A boolean value indicating whether surge upgrade is enabled/disabled for the cluster. Surge upgrade makes cluster upgrades fast and reliable by bringing up new nodes before destroying the outdated nodes. | [optional][default to false]
**ha** | Option<**bool**> | A boolean value indicating whether the control plane is run in a highly available configuration in the cluster. Highly available control planes incur less downtime. The property cannot be disabled. | [optional][default to false]
**control_plane_firewall** | Option<[**models::ControlPlaneFirewall**](control_plane_firewall.md)> |  | [optional]
**cluster_autoscaler_configuration** | Option<[**models::ClusterAutoscalerConfiguration**](cluster_autoscaler_configuration.md)> |  | [optional]
**routing_agent** | Option<[**models::RoutingAgent**](routing_agent.md)> |  | [optional]
**amd_gpu_device_plugin** | Option<[**models::AmdGpuDevicePlugin**](amd_gpu_device_plugin.md)> |  | [optional]
**amd_gpu_device_metrics_exporter_plugin** | Option<[**models::AmdGpuDeviceMetricsExporterPlugin**](amd_gpu_device_metrics_exporter_plugin.md)> |  | [optional]
**nvidia_gpu_device_plugin** | Option<[**models::NvidiaGpuDevicePlugin**](nvidia_gpu_device_plugin.md)> |  | [optional]
**rdma_shared_dev_plugin** | Option<[**models::RdmaSharedDevPlugin**](rdma_shared_dev_plugin.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


