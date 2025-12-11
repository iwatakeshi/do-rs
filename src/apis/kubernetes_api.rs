/*
 * DigitalOcean API
 *
 * # Introduction  The DigitalOcean API allows you to manage Droplets and resources within the DigitalOcean cloud in a simple, programmatic way using conventional HTTP requests.  All of the functionality that you are familiar with in the DigitalOcean control panel is also available through the API, allowing you to script the complex actions that your situation requires.  The API documentation will start with a general overview about the design and technology that has been implemented, followed by reference information about specific endpoints.  ## Requests  Any tool that is fluent in HTTP can communicate with the API simply by requesting the correct URI. Requests should be made using the HTTPS protocol so that traffic is encrypted. The interface responds to different methods depending on the action required.  |Method|Usage| |--- |--- | |GET|For simple retrieval of information about your account, Droplets, or environment, you should use the GET method.  The information you request will be returned to you as a JSON object. The attributes defined by the JSON object can be used to form additional requests.  Any request using the GET method is read-only and will not affect any of the objects you are querying.| |DELETE|To destroy a resource and remove it from your account and environment, the DELETE method should be used.  This will remove the specified object if it is found.  If it is not found, the operation will return a response indicating that the object was not found. This idempotency means that you do not have to check for a resource's availability prior to issuing a delete command, the final state will be the same regardless of its existence.| |PUT|To update the information about a resource in your account, the PUT method is available. Like the DELETE Method, the PUT method is idempotent.  It sets the state of the target using the provided values, regardless of their current values. Requests using the PUT method do not need to check the current attributes of the object.| |PATCH|Some resources support partial modification. In these cases, the PATCH method is available. Unlike PUT which generally requires a complete representation of a resource, a PATCH request is a set of instructions on how to modify a resource updating only specific attributes.| |POST|To create a new object, your request should specify the POST method. The POST request includes all of the attributes necessary to create a new object.  When you wish to create a new object, send a POST request to the target endpoint.| |HEAD|Finally, to retrieve metadata information, you should use the HEAD method to get the headers.  This returns only the header of what would be returned with an associated GET request. Response headers contain some useful information about your API access and the results that are available for your request. For instance, the headers contain your current rate-limit value and the amount of time available until the limit resets. It also contains metrics about the total number of objects found, pagination information, and the total content length.|   ## HTTP Statuses  Along with the HTTP methods that the API responds to, it will also return standard HTTP statuses, including error codes.  In the event of a problem, the status will contain the error code, while the body of the response will usually contain additional information about the problem that was encountered.  In general, if the status returned is in the 200 range, it indicates that the request was fulfilled successfully and that no error was encountered.  Return codes in the 400 range typically indicate that there was an issue with the request that was sent. Among other things, this could mean that you did not authenticate correctly, that you are requesting an action that you do not have authorization for, that the object you are requesting does not exist, or that your request is malformed.  If you receive a status in the 500 range, this generally indicates a server-side problem. This means that we are having an issue on our end and cannot fulfill your request currently.  400 and 500 level error responses will include a JSON object in their body, including the following attributes:  |Name|Type|Description| |--- |--- |--- | |id|string|A short identifier corresponding to the HTTP status code returned. For example, the ID for a response returning a 404 status code would be \"not_found.\"| |message|string|A message providing additional information about the error, including details to help resolve it when possible.| |request_id|string|Optionally, some endpoints may include a request ID that should be provided when reporting bugs or opening support tickets to help identify the issue.|  ### Example Error Response  ```     HTTP/1.1 403 Forbidden     {       \"id\":       \"forbidden\",       \"message\":  \"You do not have access for the attempted action.\"     } ```  ## Responses  When a request is successful, a response body will typically be sent back in the form of a JSON object. An exception to this is when a DELETE request is processed, which will result in a successful HTTP 204 status and an empty response body.  Inside of this JSON object, the resource root that was the target of the request will be set as the key. This will be the singular form of the word if the request operated on a single object, and the plural form of the word if a collection was processed.  For example, if you send a GET request to `/v2/droplets/$DROPLET_ID` you will get back an object with a key called \"`droplet`\". However, if you send the GET request to the general collection at `/v2/droplets`, you will get back an object with a key called \"`droplets`\".  The value of these keys will generally be a JSON object for a request on a single object and an array of objects for a request on a collection of objects.  ### Response for a Single Object  ```json     {         \"droplet\": {             \"name\": \"example.com\"             . . .         }     } ```  ### Response for an Object Collection  ```json     {         \"droplets\": [             {                 \"name\": \"example.com\"                 . . .             },             {                 \"name\": \"second.com\"                 . . .             }         ]     } ```  ## Meta  In addition to the main resource root, the response may also contain a `meta` object. This object contains information about the response itself.  The `meta` object contains a `total` key that is set to the total number of objects returned by the request. This has implications on the `links` object and pagination.  The `meta` object will only be displayed when it has a value. Currently, the `meta` object will have a value when a request is made on a collection (like `droplets` or `domains`).   ### Sample Meta Object  ```json     {         . . .         \"meta\": {             \"total\": 43         }         . . .     } ```  ## Links & Pagination  The `links` object is returned as part of the response body when pagination is enabled. By default, 20 objects are returned per page. If the response contains 20 objects or fewer, no `links` object will be returned. If the response contains more than 20 objects, the first 20 will be returned along with the `links` object.  You can request a different pagination limit or force pagination by appending `?per_page=` to the request with the number of items you would like per page. For instance, to show only two results per page, you could add `?per_page=2` to the end of your query. The maximum number of results per page is 200.  The `links` object contains a `pages` object. The `pages` object, in turn, contains keys indicating the relationship of additional pages. The values of these are the URLs of the associated pages. The keys will be one of the following:  *   **first**: The URI of the first page of results. *   **prev**: The URI of the previous sequential page of results. *   **next**: The URI of the next sequential page of results. *   **last**: The URI of the last page of results.  The `pages` object will only include the links that make sense. So for the first page of results, no `first` or `prev` links will ever be set. This convention holds true in other situations where a link would not make sense.  ### Sample Links Object  ```json     {         . . .         \"links\": {             \"pages\": {                 \"last\": \"https://api.digitalocean.com/v2/images?page=2\",                 \"next\": \"https://api.digitalocean.com/v2/images?page=2\"             }         }         . . .     } ```  ## Rate Limit  Requests through the API are rate limited per OAuth token. Current rate limits:  *   5,000 requests per hour *   250 requests per minute (5% of the hourly total)  Once you exceed either limit, you will be rate limited until the next cycle starts. Space out any requests that you would otherwise issue in bursts for the best results.  The rate limiting information is contained within the response headers of each request. The relevant headers are:  *   **ratelimit-limit**: The number of requests that can be made per hour. *   **ratelimit-remaining**: The number of requests that remain before you hit your request limit. See the information below for how the request limits expire. *   **ratelimit-reset**: This represents the time when the oldest request will expire. The value is given in [Unix epoch time](http://en.wikipedia.org/wiki/Unix_time). See below for more information about how request limits expire.  More rate limiting information is returned only within burst limit error response headers: *   **retry-after**: The number of seconds to wait before making another request when rate limited.  As long as the `ratelimit-remaining` count is above zero, you will be able to make additional requests.  The way that a request expires and is removed from the current limit count is important to understand. Rather than counting all of the requests for an hour and resetting the `ratelimit-remaining` value at the end of the hour, each request instead has its own timer.  This means that each request contributes toward the `ratelimit-remaining` count for one complete hour after the request is made. When that request's timer runs out, it is no longer counted towards the request limit.  This has implications on the meaning of the `ratelimit-reset` header as well. Because the entire rate limit is not reset at one time, the value of this header is set to the time when the _oldest_ request will expire.  Keep this in mind if you see your `ratelimit-reset` value change, but not move an entire hour into the future.  If the `ratelimit-remaining` reaches zero, subsequent requests will receive a 429 error code until the request reset has been reached.   `ratelimit-remaining` reaching zero can also indicate that the \"burst limit\" of 250  requests per minute limit was met, even if the 5,000 requests per hour limit was not.  In this case, the 429 error response will include a `retry-after` header to indicate how  long to wait (in seconds) until the request may be retried.  You can see the format of the response in the examples.   **Note:** The following endpoints have special rate limit requirements that are independent of the limits defined above.  *   Only 10 `GET` requests to the `/v2/account/keys` endpoint to list SSH keys can be made per 60 seconds. *   Only 5 requests to any and all `v2/cdn/endpoints` can be made per 10 seconds. This includes `v2/cdn/endpoints`,      `v2/cdn/endpoints/$ENDPOINT_ID`, and `v2/cdn/endpoints/$ENDPOINT_ID/cache`. *   Only 50 strings within the `files` json struct in the `v2/cdn/endpoints/$ENDPOINT_ID/cache` [payload](https://docs.digitalocean.com/reference/api/api-reference/#operation/cdn_purge_cache)      can be requested every 20 seconds.  ### Sample Rate Limit Headers  ```     . . .     ratelimit-limit: 1200     ratelimit-remaining: 1193     rateLimit-reset: 1402425459     . . . ```    ### Sample Rate Limit Headers When Burst Limit is Reached:  ```     . . .     ratelimit-limit: 5000     ratelimit-remaining: 0     rateLimit-reset: 1402425459     retry-after: 29     . . . ```  ### Sample Rate Exceeded Response  ```     429 Too Many Requests     {             id: \"too_many_requests\",             message: \"API Rate limit exceeded.\"     } ```  ## Curl Examples  Throughout this document, some example API requests will be given using the `curl` command. This will allow us to demonstrate the various endpoints in a simple, textual format.      These examples assume that you are using a Linux or macOS command line. To run these commands on a Windows machine, you can either use cmd.exe, PowerShell, or WSL:  * For cmd.exe, use the `set VAR=VALUE` [syntax](https://docs.microsoft.com/en-us/windows-server/administration/windows-commands/set_1) to define environment variables, call them with `%VAR%`, then replace all backslashes (`\\`) in the examples with carets (`^`).  * For PowerShell, use the `$Env:VAR = \"VALUE\"` [syntax](https://docs.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_environment_variables?view=powershell-7.2) to define environment variables, call them with `$Env:VAR`, then replace `curl` with `curl.exe` and all backslashes (`\\`) in the examples with backticks (`` ` ``).  * WSL is a compatibility layer that allows you to emulate a Linux terminal on a Windows machine. Install WSL with our [community tutorial](https://www.digitalocean.com/community/tutorials/how-to-install-the-windows-subsystem-for-linux-2-on-microsoft-windows-10),  then follow this API documentation normally.  The names of account-specific references (like Droplet IDs, for instance) will be represented by variables. For instance, a Droplet ID may be represented by a variable called `$DROPLET_ID`. You can set the associated variables in your environment if you wish to use the examples without modification.  The first variable that you should set to get started is your OAuth authorization token. The next section will go over the details of this, but you can set an environmental variable for it now.  Generate a token by going to the [Apps & API](https://cloud.digitalocean.com/settings/applications) section of the DigitalOcean control panel. Use an existing token if you have saved one, or generate a new token with the \"Generate new token\" button. Copy the generated token and use it to set and export the TOKEN variable in your environment as the example shows.  You may also wish to set some other variables now or as you go along. For example, you may wish to set the `DROPLET_ID` variable to one of your Droplet IDs since this will be used frequently in the API.  If you are following along, make sure you use a Droplet ID that you control so that your commands will execute correctly.  If you need access to the headers of a response through `curl`, you can pass the `-i` flag to display the header information along with the body. If you are only interested in the header, you can instead pass the `-I` flag, which will exclude the response body entirely.   ### Set and Export your OAuth Token  ``` export DIGITALOCEAN_TOKEN=your_token_here ```  ### Set and Export a Variable  ``` export DROPLET_ID=1111111 ```  ## Parameters  There are two different ways to pass parameters in a request with the API.  When passing parameters to create or update an object, parameters should be passed as a JSON object containing the appropriate attribute names and values as key-value pairs. When you use this format, you should specify that you are sending a JSON object in the header. This is done by setting the `Content-Type` header to `application/json`. This ensures that your request is interpreted correctly.  When passing parameters to filter a response on GET requests, parameters can be passed using standard query attributes. In this case, the parameters would be embedded into the URI itself by appending a `?` to the end of the URI and then setting each attribute with an equal sign. Attributes can be separated with a `&`. Tools like `curl` can create the appropriate URI when given parameters and values; this can also be done using the `-F` flag and then passing the key and value as an argument. The argument should take the form of a quoted string with the attribute being set to a value with an equal sign.  ### Pass Parameters as a JSON Object  ```     curl -H \"Authorization: Bearer $DIGITALOCEAN_TOKEN\" \\         -H \"Content-Type: application/json\" \\         -d '{\"name\": \"example.com\", \"ip_address\": \"127.0.0.1\"}' \\         -X POST \"https://api.digitalocean.com/v2/domains\" ```  ### Pass Filter Parameters as a Query String  ```      curl -H \"Authorization: Bearer $DIGITALOCEAN_TOKEN\" \\          -X GET \\          \"https://api.digitalocean.com/v2/images?private=true\" ```  ## Cross Origin Resource Sharing  In order to make requests to the API from other domains, the API implements Cross Origin Resource Sharing (CORS) support.  CORS support is generally used to create AJAX requests outside of the domain that the request originated from. This is necessary to implement projects like control panels utilizing the API. This tells the browser that it can send requests to an outside domain.  The procedure that the browser initiates in order to perform these actions (other than GET requests) begins by sending a \"preflight\" request. This sets the `Origin` header and uses the `OPTIONS` method. The server will reply back with the methods it allows and some of the limits it imposes. The client then sends the actual request if it falls within the allowed constraints.  This process is usually done in the background by the browser, but you can use curl to emulate this process using the example provided. The headers that will be set to show the constraints are:  *   **Access-Control-Allow-Origin**: This is the domain that is sent by the client or browser as the origin of the request. It is set through an `Origin` header. *   **Access-Control-Allow-Methods**: This specifies the allowed options for requests from that domain. This will generally be all available methods. *   **Access-Control-Expose-Headers**: This will contain the headers that will be available to requests from the origin domain. *   **Access-Control-Max-Age**: This is the length of time that the access is considered valid. After this expires, a new preflight should be sent. *   **Access-Control-Allow-Credentials**: This will be set to `true`. It basically allows you to send your OAuth token for authentication.  You should not need to be concerned with the details of these headers, because the browser will typically do all of the work for you.
 *
 * The version of the OpenAPI document: 2.0
 * Contact: api-engineering@digitalocean.com
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, ContentType, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{de::Error as _, Deserialize, Serialize};

/// struct for typed errors of method [`kubernetes_add_node_pool`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KubernetesAddNodePoolError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`kubernetes_add_registries`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KubernetesAddRegistriesError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`kubernetes_add_registry`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KubernetesAddRegistryError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`kubernetes_create_cluster`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KubernetesCreateClusterError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`kubernetes_delete_cluster`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KubernetesDeleteClusterError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`kubernetes_delete_node`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KubernetesDeleteNodeError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`kubernetes_delete_node_pool`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KubernetesDeleteNodePoolError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`kubernetes_destroy_associated_resources_dangerous`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KubernetesDestroyAssociatedResourcesDangerousError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`kubernetes_destroy_associated_resources_selective`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KubernetesDestroyAssociatedResourcesSelectiveError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`kubernetes_get_available_upgrades`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KubernetesGetAvailableUpgradesError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`kubernetes_get_cluster`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KubernetesGetClusterError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`kubernetes_get_cluster_lint_results`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KubernetesGetClusterLintResultsError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`kubernetes_get_cluster_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KubernetesGetClusterUserError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`kubernetes_get_credentials`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KubernetesGetCredentialsError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`kubernetes_get_kubeconfig`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KubernetesGetKubeconfigError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`kubernetes_get_node_pool`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KubernetesGetNodePoolError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`kubernetes_get_status_messages`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KubernetesGetStatusMessagesError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`kubernetes_list_associated_resources`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KubernetesListAssociatedResourcesError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`kubernetes_list_clusters`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KubernetesListClustersError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`kubernetes_list_node_pools`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KubernetesListNodePoolsError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`kubernetes_list_options`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KubernetesListOptionsError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`kubernetes_recycle_node_pool`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KubernetesRecycleNodePoolError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`kubernetes_remove_registries`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KubernetesRemoveRegistriesError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`kubernetes_remove_registry`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KubernetesRemoveRegistryError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`kubernetes_run_cluster_lint`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KubernetesRunClusterLintError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`kubernetes_update_cluster`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KubernetesUpdateClusterError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`kubernetes_update_node_pool`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KubernetesUpdateNodePoolError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`kubernetes_upgrade_cluster`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KubernetesUpgradeClusterError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// To add an additional node pool to a Kubernetes clusters, send a POST request to `/v2/kubernetes/clusters/$K8S_CLUSTER_ID/node_pools` with the following attributes.
pub async fn kubernetes_add_node_pool(
    configuration: &configuration::Configuration,
    cluster_id: &str,
    kubernetes_node_pool: models::KubernetesNodePool,
) -> Result<models::KubernetesAddNodePool201Response, Error<KubernetesAddNodePoolError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;
    let p_body_kubernetes_node_pool = kubernetes_node_pool;

    let uri_str = format!(
        "{}/v2/kubernetes/clusters/{cluster_id}/node_pools",
        configuration.base_path,
        cluster_id = crate::apis::urlencode(p_path_cluster_id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_kubernetes_node_pool);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::KubernetesAddNodePool201Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::KubernetesAddNodePool201Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<KubernetesAddNodePoolError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To integrate the container registries with Kubernetes clusters, send a POST request to `/v2/kubernetes/registries`.
pub async fn kubernetes_add_registries(
    configuration: &configuration::Configuration,
    cluster_registries: Option<models::ClusterRegistries>,
) -> Result<(), Error<KubernetesAddRegistriesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_cluster_registries = cluster_registries;

    let uri_str = format!("{}/v2/kubernetes/registries", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_cluster_registries);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<KubernetesAddRegistriesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To integrate the container registry with Kubernetes clusters, send a POST request to `/v2/kubernetes/registry`.
pub async fn kubernetes_add_registry(
    configuration: &configuration::Configuration,
    cluster_registry: Option<models::ClusterRegistry>,
) -> Result<(), Error<KubernetesAddRegistryError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_cluster_registry = cluster_registry;

    let uri_str = format!("{}/v2/kubernetes/registry", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_cluster_registry);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<KubernetesAddRegistryError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To create a new Kubernetes cluster, send a POST request to `/v2/kubernetes/clusters`. The request must contain at least one node pool with at least one worker.  The request may contain a maintenance window policy describing a time period when disruptive maintenance tasks may be carried out. Omitting the policy implies that a window will be chosen automatically. See [here](https://docs.digitalocean.com/products/kubernetes/how-to/upgrade-cluster/) for details.
pub async fn kubernetes_create_cluster(
    configuration: &configuration::Configuration,
    cluster: models::Cluster,
) -> Result<models::KubernetesCreateCluster201Response, Error<KubernetesCreateClusterError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_cluster = cluster;

    let uri_str = format!("{}/v2/kubernetes/clusters", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_cluster);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::KubernetesCreateCluster201Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::KubernetesCreateCluster201Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<KubernetesCreateClusterError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To delete a Kubernetes cluster and all services deployed to it, send a DELETE request to `/v2/kubernetes/clusters/$K8S_CLUSTER_ID`.  A 204 status code with no body will be returned in response to a successful request.
pub async fn kubernetes_delete_cluster(
    configuration: &configuration::Configuration,
    cluster_id: &str,
) -> Result<(), Error<KubernetesDeleteClusterError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;

    let uri_str = format!(
        "{}/v2/kubernetes/clusters/{cluster_id}",
        configuration.base_path,
        cluster_id = crate::apis::urlencode(p_path_cluster_id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<KubernetesDeleteClusterError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To delete a single node in a pool, send a DELETE request to `/v2/kubernetes/clusters/$K8S_CLUSTER_ID/node_pools/$NODE_POOL_ID/nodes/$NODE_ID`.  Appending the `skip_drain=1` query parameter to the request causes node draining to be skipped. Omitting the query parameter or setting its value to `0` carries out draining prior to deletion.  Appending the `replace=1` query parameter to the request causes the node to be replaced by a new one after deletion. Omitting the query parameter or setting its value to `0` deletes without replacement.
pub async fn kubernetes_delete_node(
    configuration: &configuration::Configuration,
    cluster_id: &str,
    node_pool_id: &str,
    node_id: &str,
    skip_drain: Option<i32>,
    replace: Option<i32>,
) -> Result<(), Error<KubernetesDeleteNodeError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;
    let p_path_node_pool_id = node_pool_id;
    let p_path_node_id = node_id;
    let p_query_skip_drain = skip_drain;
    let p_query_replace = replace;

    let uri_str = format!(
        "{}/v2/kubernetes/clusters/{cluster_id}/node_pools/{node_pool_id}/nodes/{node_id}",
        configuration.base_path,
        cluster_id = crate::apis::urlencode(p_path_cluster_id),
        node_pool_id = crate::apis::urlencode(p_path_node_pool_id),
        node_id = crate::apis::urlencode(p_path_node_id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref param_value) = p_query_skip_drain {
        req_builder = req_builder.query(&[("skip_drain", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_replace {
        req_builder = req_builder.query(&[("replace", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<KubernetesDeleteNodeError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To delete a node pool, send a DELETE request to `/v2/kubernetes/clusters/$K8S_CLUSTER_ID/node_pools/$NODE_POOL_ID`.  A 204 status code with no body will be returned in response to a successful request. Nodes in the pool will subsequently be drained and deleted.
pub async fn kubernetes_delete_node_pool(
    configuration: &configuration::Configuration,
    cluster_id: &str,
    node_pool_id: &str,
) -> Result<(), Error<KubernetesDeleteNodePoolError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;
    let p_path_node_pool_id = node_pool_id;

    let uri_str = format!(
        "{}/v2/kubernetes/clusters/{cluster_id}/node_pools/{node_pool_id}",
        configuration.base_path,
        cluster_id = crate::apis::urlencode(p_path_cluster_id),
        node_pool_id = crate::apis::urlencode(p_path_node_pool_id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<KubernetesDeleteNodePoolError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To delete a Kubernetes cluster with all of its associated resources, send a DELETE request to `/v2/kubernetes/clusters/$K8S_CLUSTER_ID/destroy_with_associated_resources/dangerous`. A 204 status code with no body will be returned in response to a successful request.
pub async fn kubernetes_destroy_associated_resources_dangerous(
    configuration: &configuration::Configuration,
    cluster_id: &str,
) -> Result<(), Error<KubernetesDestroyAssociatedResourcesDangerousError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;

    let uri_str = format!(
        "{}/v2/kubernetes/clusters/{cluster_id}/destroy_with_associated_resources/dangerous",
        configuration.base_path,
        cluster_id = crate::apis::urlencode(p_path_cluster_id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<KubernetesDestroyAssociatedResourcesDangerousError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To delete a Kubernetes cluster along with a subset of its associated resources, send a DELETE request to `/v2/kubernetes/clusters/$K8S_CLUSTER_ID/destroy_with_associated_resources/selective`.  The JSON body of the request should include `load_balancers`, `volumes`, or `volume_snapshots` keys each set to an array of IDs for the associated resources to be destroyed.  The IDs can be found by querying the cluster's associated resources endpoint. Any associated resource not included in the request will remain and continue to accrue changes on your account.
pub async fn kubernetes_destroy_associated_resources_selective(
    configuration: &configuration::Configuration,
    cluster_id: &str,
    destroy_associated_kubernetes_resources: models::DestroyAssociatedKubernetesResources,
) -> Result<(), Error<KubernetesDestroyAssociatedResourcesSelectiveError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;
    let p_body_destroy_associated_kubernetes_resources = destroy_associated_kubernetes_resources;

    let uri_str = format!(
        "{}/v2/kubernetes/clusters/{cluster_id}/destroy_with_associated_resources/selective",
        configuration.base_path,
        cluster_id = crate::apis::urlencode(p_path_cluster_id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_destroy_associated_kubernetes_resources);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<KubernetesDestroyAssociatedResourcesSelectiveError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To determine whether a cluster can be upgraded, and the versions to which it can be upgraded, send a GET request to `/v2/kubernetes/clusters/$K8S_CLUSTER_ID/upgrades`.
pub async fn kubernetes_get_available_upgrades(
    configuration: &configuration::Configuration,
    cluster_id: &str,
) -> Result<
    models::KubernetesGetAvailableUpgrades200Response,
    Error<KubernetesGetAvailableUpgradesError>,
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;

    let uri_str = format!(
        "{}/v2/kubernetes/clusters/{cluster_id}/upgrades",
        configuration.base_path,
        cluster_id = crate::apis::urlencode(p_path_cluster_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::KubernetesGetAvailableUpgrades200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::KubernetesGetAvailableUpgrades200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<KubernetesGetAvailableUpgradesError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To show information about an existing Kubernetes cluster, send a GET request to `/v2/kubernetes/clusters/$K8S_CLUSTER_ID`.
pub async fn kubernetes_get_cluster(
    configuration: &configuration::Configuration,
    cluster_id: &str,
) -> Result<models::KubernetesGetCluster200Response, Error<KubernetesGetClusterError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;

    let uri_str = format!(
        "{}/v2/kubernetes/clusters/{cluster_id}",
        configuration.base_path,
        cluster_id = crate::apis::urlencode(p_path_cluster_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::KubernetesGetCluster200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::KubernetesGetCluster200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<KubernetesGetClusterError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To request clusterlint diagnostics for your cluster, send a GET request to `/v2/kubernetes/clusters/$K8S_CLUSTER_ID/clusterlint`. If the `run_id` query parameter is provided, then the diagnostics for the specific run is fetched. By default, the latest results are shown.  To find out how to address clusterlint feedback, please refer to [the clusterlint check documentation](https://github.com/digitalocean/clusterlint/blob/master/checks.md).
pub async fn kubernetes_get_cluster_lint_results(
    configuration: &configuration::Configuration,
    cluster_id: &str,
    run_id: Option<&str>,
) -> Result<models::ClusterlintResults, Error<KubernetesGetClusterLintResultsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;
    let p_query_run_id = run_id;

    let uri_str = format!(
        "{}/v2/kubernetes/clusters/{cluster_id}/clusterlint",
        configuration.base_path,
        cluster_id = crate::apis::urlencode(p_path_cluster_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_run_id {
        req_builder = req_builder.query(&[("run_id", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ClusterlintResults`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ClusterlintResults`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<KubernetesGetClusterLintResultsError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To show information the user associated with a Kubernetes cluster, send a GET request to `/v2/kubernetes/clusters/$K8S_CLUSTER_ID/user`.
pub async fn kubernetes_get_cluster_user(
    configuration: &configuration::Configuration,
    cluster_id: &str,
) -> Result<models::User, Error<KubernetesGetClusterUserError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;

    let uri_str = format!(
        "{}/v2/kubernetes/clusters/{cluster_id}/user",
        configuration.base_path,
        cluster_id = crate::apis::urlencode(p_path_cluster_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::User`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::User`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<KubernetesGetClusterUserError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// This endpoint returns a JSON object . It can be used to programmatically construct Kubernetes clients which cannot parse kubeconfig files.  The resulting JSON object contains token-based authentication for clusters supporting it, and certificate-based authentication otherwise. For a list of supported versions and more information, see \"[How to Connect to a DigitalOcean Kubernetes Cluster](https://docs.digitalocean.com/products/kubernetes/how-to/connect-to-cluster/)\".  To retrieve credentials for accessing a Kubernetes cluster, send a GET request to `/v2/kubernetes/clusters/$K8S_CLUSTER_ID/credentials`.  Clusters supporting token-based authentication may define an expiration by passing a duration in seconds as a query parameter to `/v2/kubernetes/clusters/$K8S_CLUSTER_ID/credentials?expiry_seconds=$DURATION_IN_SECONDS`. If not set or 0, then the token will have a 7 day expiry. The query parameter has no impact in certificate-based authentication.
pub async fn kubernetes_get_credentials(
    configuration: &configuration::Configuration,
    cluster_id: &str,
    expiry_seconds: Option<i32>,
) -> Result<models::Credentials, Error<KubernetesGetCredentialsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;
    let p_query_expiry_seconds = expiry_seconds;

    let uri_str = format!(
        "{}/v2/kubernetes/clusters/{cluster_id}/credentials",
        configuration.base_path,
        cluster_id = crate::apis::urlencode(p_path_cluster_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_expiry_seconds {
        req_builder = req_builder.query(&[("expiry_seconds", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Credentials`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Credentials`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<KubernetesGetCredentialsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// This endpoint returns a kubeconfig file in YAML format. It can be used to connect to and administer the cluster using the Kubernetes command line tool, `kubectl`, or other programs supporting kubeconfig files (e.g., client libraries).  The resulting kubeconfig file uses token-based authentication for clusters supporting it, and certificate-based authentication otherwise. For a list of supported versions and more information, see \"[How to Connect to a DigitalOcean Kubernetes Cluster](https://docs.digitalocean.com/products/kubernetes/how-to/connect-to-cluster/)\".  To retrieve a kubeconfig file for use with a Kubernetes cluster, send a GET request to `/v2/kubernetes/clusters/$K8S_CLUSTER_ID/kubeconfig`.  Clusters supporting token-based authentication may define an expiration by passing a duration in seconds as a query parameter to `/v2/kubernetes/clusters/$K8S_CLUSTER_ID/kubeconfig?expiry_seconds=$DURATION_IN_SECONDS`. If not set or 0, then the token will have a 7 day expiry. The query parameter has no impact in certificate-based authentication.  Kubernetes Roles granted to a user with a token-based kubeconfig are derived from that user's DigitalOcean role. Predefined roles (Owner, Member, Modifier etc.) have an automatic mapping to Kubernetes roles. Custom roles are not automatically mapped to any Kubernetes roles, and require [additional configuration](https://docs.digitalocean.com/products/kubernetes/how-to/set-up-custom-rolebindings/) by a cluster administrator.
pub async fn kubernetes_get_kubeconfig(
    configuration: &configuration::Configuration,
    cluster_id: &str,
    expiry_seconds: Option<i32>,
) -> Result<(), Error<KubernetesGetKubeconfigError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;
    let p_query_expiry_seconds = expiry_seconds;

    let uri_str = format!(
        "{}/v2/kubernetes/clusters/{cluster_id}/kubeconfig",
        configuration.base_path,
        cluster_id = crate::apis::urlencode(p_path_cluster_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_expiry_seconds {
        req_builder = req_builder.query(&[("expiry_seconds", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<KubernetesGetKubeconfigError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To show information about a specific node pool in a Kubernetes cluster, send a GET request to `/v2/kubernetes/clusters/$K8S_CLUSTER_ID/node_pools/$NODE_POOL_ID`.
pub async fn kubernetes_get_node_pool(
    configuration: &configuration::Configuration,
    cluster_id: &str,
    node_pool_id: &str,
) -> Result<models::KubernetesGetNodePool200Response, Error<KubernetesGetNodePoolError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;
    let p_path_node_pool_id = node_pool_id;

    let uri_str = format!(
        "{}/v2/kubernetes/clusters/{cluster_id}/node_pools/{node_pool_id}",
        configuration.base_path,
        cluster_id = crate::apis::urlencode(p_path_cluster_id),
        node_pool_id = crate::apis::urlencode(p_path_node_pool_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::KubernetesGetNodePool200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::KubernetesGetNodePool200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<KubernetesGetNodePoolError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To retrieve status messages for a Kubernetes cluster, send a GET request to `/v2/kubernetes/clusters/$K8S_CLUSTER_ID/status_messages`. Status messages inform users of any issues that come up during the cluster lifecycle.
pub async fn kubernetes_get_status_messages(
    configuration: &configuration::Configuration,
    cluster_id: &str,
    since: Option<String>,
) -> Result<models::KubernetesGetStatusMessages200Response, Error<KubernetesGetStatusMessagesError>>
{
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;
    let p_query_since = since;

    let uri_str = format!(
        "{}/v2/kubernetes/clusters/{cluster_id}/status_messages",
        configuration.base_path,
        cluster_id = crate::apis::urlencode(p_path_cluster_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_since {
        req_builder = req_builder.query(&[("since", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::KubernetesGetStatusMessages200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::KubernetesGetStatusMessages200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<KubernetesGetStatusMessagesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To list the associated billable resources that can be destroyed along with a cluster, send a GET request to the `/v2/kubernetes/clusters/$K8S_CLUSTER_ID/destroy_with_associated_resources` endpoint.
pub async fn kubernetes_list_associated_resources(
    configuration: &configuration::Configuration,
    cluster_id: &str,
) -> Result<models::AssociatedKubernetesResources, Error<KubernetesListAssociatedResourcesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;

    let uri_str = format!(
        "{}/v2/kubernetes/clusters/{cluster_id}/destroy_with_associated_resources",
        configuration.base_path,
        cluster_id = crate::apis::urlencode(p_path_cluster_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AssociatedKubernetesResources`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AssociatedKubernetesResources`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<KubernetesListAssociatedResourcesError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To list all of the Kubernetes clusters on your account, send a GET request to `/v2/kubernetes/clusters`.
pub async fn kubernetes_list_clusters(
    configuration: &configuration::Configuration,
    per_page: Option<i32>,
    page: Option<i32>,
) -> Result<models::KubernetesListClusters200Response, Error<KubernetesListClustersError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_per_page = per_page;
    let p_query_page = page;

    let uri_str = format!("{}/v2/kubernetes/clusters", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_per_page {
        req_builder = req_builder.query(&[("per_page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::KubernetesListClusters200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::KubernetesListClusters200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<KubernetesListClustersError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To list all of the node pools in a Kubernetes clusters, send a GET request to `/v2/kubernetes/clusters/$K8S_CLUSTER_ID/node_pools`.
pub async fn kubernetes_list_node_pools(
    configuration: &configuration::Configuration,
    cluster_id: &str,
) -> Result<models::KubernetesListNodePools200Response, Error<KubernetesListNodePoolsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;

    let uri_str = format!(
        "{}/v2/kubernetes/clusters/{cluster_id}/node_pools",
        configuration.base_path,
        cluster_id = crate::apis::urlencode(p_path_cluster_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::KubernetesListNodePools200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::KubernetesListNodePools200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<KubernetesListNodePoolsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To list the versions of Kubernetes available for use, the regions that support Kubernetes, and the available node sizes, send a GET request to `/v2/kubernetes/options`.
pub async fn kubernetes_list_options(
    configuration: &configuration::Configuration,
) -> Result<models::KubernetesOptions, Error<KubernetesListOptionsError>> {
    let uri_str = format!("{}/v2/kubernetes/options", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::KubernetesOptions`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::KubernetesOptions`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<KubernetesListOptionsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// The endpoint has been deprecated. Please use the DELETE `/v2/kubernetes/clusters/$K8S_CLUSTER_ID/node_pools/$NODE_POOL_ID/nodes/$NODE_ID` method instead.
pub async fn kubernetes_recycle_node_pool(
    configuration: &configuration::Configuration,
    cluster_id: &str,
    node_pool_id: &str,
    kubernetes_recycle_node_pool_request: models::KubernetesRecycleNodePoolRequest,
) -> Result<(), Error<KubernetesRecycleNodePoolError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;
    let p_path_node_pool_id = node_pool_id;
    let p_body_kubernetes_recycle_node_pool_request = kubernetes_recycle_node_pool_request;

    let uri_str = format!(
        "{}/v2/kubernetes/clusters/{cluster_id}/node_pools/{node_pool_id}/recycle",
        configuration.base_path,
        cluster_id = crate::apis::urlencode(p_path_cluster_id),
        node_pool_id = crate::apis::urlencode(p_path_node_pool_id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_kubernetes_recycle_node_pool_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<KubernetesRecycleNodePoolError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To remove the container registries from Kubernetes clusters, send a DELETE request to `/v2/kubernetes/registries`.
pub async fn kubernetes_remove_registries(
    configuration: &configuration::Configuration,
    cluster_registries: Option<models::ClusterRegistries>,
) -> Result<(), Error<KubernetesRemoveRegistriesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_cluster_registries = cluster_registries;

    let uri_str = format!("{}/v2/kubernetes/registries", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_cluster_registries);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<KubernetesRemoveRegistriesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To remove the container registry from Kubernetes clusters, send a DELETE request to `/v2/kubernetes/registry`.
pub async fn kubernetes_remove_registry(
    configuration: &configuration::Configuration,
    cluster_registry: Option<models::ClusterRegistry>,
) -> Result<(), Error<KubernetesRemoveRegistryError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_cluster_registry = cluster_registry;

    let uri_str = format!("{}/v2/kubernetes/registry", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_cluster_registry);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<KubernetesRemoveRegistryError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Clusterlint helps operators conform to Kubernetes best practices around resources, security and reliability to avoid common problems while operating or upgrading the clusters.  To request a clusterlint run on your cluster, send a POST request to `/v2/kubernetes/clusters/$K8S_CLUSTER_ID/clusterlint`. This will run all checks present in the `doks` group by default, if a request body is not specified. Optionally specify the below attributes.  For information about the available checks, please refer to [the clusterlint check documentation](https://github.com/digitalocean/clusterlint/blob/master/checks.md).
pub async fn kubernetes_run_cluster_lint(
    configuration: &configuration::Configuration,
    cluster_id: &str,
    clusterlint_request: Option<models::ClusterlintRequest>,
) -> Result<models::KubernetesRunClusterLint202Response, Error<KubernetesRunClusterLintError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;
    let p_body_clusterlint_request = clusterlint_request;

    let uri_str = format!(
        "{}/v2/kubernetes/clusters/{cluster_id}/clusterlint",
        configuration.base_path,
        cluster_id = crate::apis::urlencode(p_path_cluster_id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_clusterlint_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::KubernetesRunClusterLint202Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::KubernetesRunClusterLint202Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<KubernetesRunClusterLintError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To update a Kubernetes cluster, send a PUT request to `/v2/kubernetes/clusters/$K8S_CLUSTER_ID` and specify one or more of the attributes below.
pub async fn kubernetes_update_cluster(
    configuration: &configuration::Configuration,
    cluster_id: &str,
    cluster_update: models::ClusterUpdate,
) -> Result<models::KubernetesCreateCluster201Response, Error<KubernetesUpdateClusterError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;
    let p_body_cluster_update = cluster_update;

    let uri_str = format!(
        "{}/v2/kubernetes/clusters/{cluster_id}",
        configuration.base_path,
        cluster_id = crate::apis::urlencode(p_path_cluster_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_cluster_update);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::KubernetesCreateCluster201Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::KubernetesCreateCluster201Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<KubernetesUpdateClusterError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To update the name of a node pool, edit the tags applied to it, or adjust its number of nodes, send a PUT request to `/v2/kubernetes/clusters/$K8S_CLUSTER_ID/node_pools/$NODE_POOL_ID` with the following attributes.
pub async fn kubernetes_update_node_pool(
    configuration: &configuration::Configuration,
    cluster_id: &str,
    node_pool_id: &str,
    kubernetes_node_pool_update: models::KubernetesNodePoolUpdate,
) -> Result<models::KubernetesUpdateNodePool202Response, Error<KubernetesUpdateNodePoolError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;
    let p_path_node_pool_id = node_pool_id;
    let p_body_kubernetes_node_pool_update = kubernetes_node_pool_update;

    let uri_str = format!(
        "{}/v2/kubernetes/clusters/{cluster_id}/node_pools/{node_pool_id}",
        configuration.base_path,
        cluster_id = crate::apis::urlencode(p_path_cluster_id),
        node_pool_id = crate::apis::urlencode(p_path_node_pool_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_kubernetes_node_pool_update);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::KubernetesUpdateNodePool202Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::KubernetesUpdateNodePool202Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<KubernetesUpdateNodePoolError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To immediately upgrade a Kubernetes cluster to a newer patch release of Kubernetes, send a POST request to `/v2/kubernetes/clusters/$K8S_CLUSTER_ID/upgrade`. The body of the request must specify a version attribute.  Available upgrade versions for a cluster can be fetched from `/v2/kubernetes/clusters/$K8S_CLUSTER_ID/upgrades`.
pub async fn kubernetes_upgrade_cluster(
    configuration: &configuration::Configuration,
    cluster_id: &str,
    kubernetes_upgrade_cluster_request: models::KubernetesUpgradeClusterRequest,
) -> Result<(), Error<KubernetesUpgradeClusterError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_cluster_id = cluster_id;
    let p_body_kubernetes_upgrade_cluster_request = kubernetes_upgrade_cluster_request;

    let uri_str = format!(
        "{}/v2/kubernetes/clusters/{cluster_id}/upgrade",
        configuration.base_path,
        cluster_id = crate::apis::urlencode(p_path_cluster_id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_kubernetes_upgrade_cluster_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<KubernetesUpgradeClusterError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
