# LoadBalancerBase

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | A unique ID that can be used to identify and reference a load balancer. | [optional][readonly]
**name** | Option<**String**> | A human-readable name for a load balancer instance. | [optional]
**project_id** | Option<**String**> | The ID of the project that the load balancer is associated with. If no ID is provided at creation, the load balancer associates with the user's default project. If an invalid project ID is provided, the load balancer will not be created. | [optional]
**ip** | Option<**String**> | An attribute containing the public-facing IP address of the load balancer. | [optional][readonly]
**ipv6** | Option<**String**> | An attribute containing the public-facing IPv6 address of the load balancer. | [optional][readonly]
**size_unit** | Option<**i32**> | How many nodes the load balancer contains. Each additional node increases the load balancer's ability to manage more connections. Load balancers can be scaled up or down, and you can change the number of nodes after creation up to once per hour. This field is currently not available in the AMS2, NYC2, or SFO1 regions. Use the `size` field to scale load balancers that reside in these regions. | [optional][default to 1]
**size** | Option<**String**> | This field has been replaced by the `size_unit` field for all regions except in AMS2, NYC2, and SFO1. Each available load balancer size now equates to the load balancer having a set number of nodes. * `lb-small` = 1 node * `lb-medium` = 3 nodes * `lb-large` = 6 nodes  You can resize load balancers after creation up to once per hour. You cannot resize a load balancer within the first hour of its creation. | [optional][default to LbSmall]
**algorithm** | Option<**String**> | This field has been deprecated. You can no longer specify an algorithm for load balancers. | [optional][default to RoundRobin]
**status** | Option<**String**> | A status string indicating the current state of the load balancer. This can be `new`, `active`, or `errored`. | [optional][readonly]
**created_at** | Option<**String**> | A time value given in ISO8601 combined date and time format that represents when the load balancer was created. | [optional][readonly]
**forwarding_rules** | [**Vec<models::ForwardingRule>**](forwarding_rule.md) | An array of objects specifying the forwarding rules for a load balancer. | 
**health_check** | Option<[**models::HealthCheck**](health_check.md)> |  | [optional]
**sticky_sessions** | Option<[**models::StickySessions**](sticky_sessions.md)> |  | [optional]
**redirect_http_to_https** | Option<**bool**> | A boolean value indicating whether HTTP requests to the load balancer on port 80 will be redirected to HTTPS on port 443. | [optional][default to false]
**enable_proxy_protocol** | Option<**bool**> | A boolean value indicating whether PROXY Protocol is in use. | [optional][default to false]
**enable_backend_keepalive** | Option<**bool**> | A boolean value indicating whether HTTP keepalive connections are maintained to target Droplets. | [optional][default to false]
**http_idle_timeout_seconds** | Option<**i32**> | An integer value which configures the idle timeout for HTTP requests to the target droplets. | [optional][default to 60]
**vpc_uuid** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | A string specifying the UUID of the VPC to which the load balancer is assigned. | [optional]
**disable_lets_encrypt_dns_records** | Option<**bool**> | A boolean value indicating whether to disable automatic DNS record creation for Let's Encrypt certificates that are added to the load balancer. | [optional][default to false]
**firewall** | Option<[**models::LbFirewall**](lb_firewall.md)> |  | [optional]
**network** | Option<**String**> | A string indicating whether the load balancer should be external or internal. Internal load balancers have no public IPs and are only accessible to resources on the same VPC network. This property cannot be updated after creating the load balancer. | [optional][default to External]
**network_stack** | Option<**String**> | A string indicating whether the load balancer will support IPv4 or both IPv4 and IPv6 networking. This property cannot be updated after creating the load balancer. | [optional][default to Ipv4]
**r#type** | Option<**String**> | A string indicating whether the load balancer should be a standard regional HTTP load balancer, a regional network load balancer that routes traffic at the TCP/UDP transport layer, or a global load balancer. | [optional][default to Regional]
**domains** | Option<[**Vec<models::Domains>**](domains.md)> | An array of objects specifying the domain configurations for a Global load balancer. | [optional]
**glb_settings** | Option<[**models::GlbSettings**](glb_settings.md)> |  | [optional]
**target_load_balancer_ids** | Option<**Vec<String>**> | An array containing the UUIDs of the Regional load balancers to be used as target backends for a Global load balancer. | [optional]
**tls_cipher_policy** | Option<**String**> | A string indicating the policy for the TLS cipher suites used by the load balancer. The possible values are `DEFAULT` or `STRONG`. The default value is `DEFAULT`. | [optional][default to Default]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


