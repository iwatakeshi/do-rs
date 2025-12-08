# ForwardingRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**entry_protocol** | **String** | The protocol used for traffic to the load balancer. The possible values are: `http`, `https`, `http2`, `http3`, `tcp`, or `udp`. If you set the  `entry_protocol` to `udp`, the `target_protocol` must be set to `udp`.  When using UDP, the load balancer requires that you set up a health  check with a port that uses TCP, HTTP, or HTTPS to work properly.  | 
**entry_port** | **i32** | An integer representing the port on which the load balancer instance will listen. | 
**target_protocol** | **String** | The protocol used for traffic from the load balancer to the backend Droplets. The possible values are: `http`, `https`, `http2`, `tcp`, or `udp`. If you set the `target_protocol` to `udp`, the `entry_protocol` must be set to  `udp`. When using UDP, the load balancer requires that you set up a health  check with a port that uses TCP, HTTP, or HTTPS to work properly.  | 
**target_port** | **i32** | An integer representing the port on the backend Droplets to which the load balancer will send traffic. | 
**certificate_id** | Option<**String**> | The ID of the TLS certificate used for SSL termination if enabled. | [optional]
**tls_passthrough** | Option<**bool**> | A boolean value indicating whether SSL encrypted traffic will be passed through to the backend Droplets. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


