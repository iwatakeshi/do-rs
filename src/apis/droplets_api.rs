/*
 * DigitalOcean API
 *
 * # Introduction  The DigitalOcean API allows you to manage Droplets and resources within the DigitalOcean cloud in a simple, programmatic way using conventional HTTP requests.  All of the functionality that you are familiar with in the DigitalOcean control panel is also available through the API, allowing you to script the complex actions that your situation requires.  The API documentation will start with a general overview about the design and technology that has been implemented, followed by reference information about specific endpoints.  ## Requests  Any tool that is fluent in HTTP can communicate with the API simply by requesting the correct URI. Requests should be made using the HTTPS protocol so that traffic is encrypted. The interface responds to different methods depending on the action required.  |Method|Usage| |--- |--- | |GET|For simple retrieval of information about your account, Droplets, or environment, you should use the GET method.  The information you request will be returned to you as a JSON object. The attributes defined by the JSON object can be used to form additional requests.  Any request using the GET method is read-only and will not affect any of the objects you are querying.| |DELETE|To destroy a resource and remove it from your account and environment, the DELETE method should be used.  This will remove the specified object if it is found.  If it is not found, the operation will return a response indicating that the object was not found. This idempotency means that you do not have to check for a resource's availability prior to issuing a delete command, the final state will be the same regardless of its existence.| |PUT|To update the information about a resource in your account, the PUT method is available. Like the DELETE Method, the PUT method is idempotent.  It sets the state of the target using the provided values, regardless of their current values. Requests using the PUT method do not need to check the current attributes of the object.| |PATCH|Some resources support partial modification. In these cases, the PATCH method is available. Unlike PUT which generally requires a complete representation of a resource, a PATCH request is a set of instructions on how to modify a resource updating only specific attributes.| |POST|To create a new object, your request should specify the POST method. The POST request includes all of the attributes necessary to create a new object.  When you wish to create a new object, send a POST request to the target endpoint.| |HEAD|Finally, to retrieve metadata information, you should use the HEAD method to get the headers.  This returns only the header of what would be returned with an associated GET request. Response headers contain some useful information about your API access and the results that are available for your request. For instance, the headers contain your current rate-limit value and the amount of time available until the limit resets. It also contains metrics about the total number of objects found, pagination information, and the total content length.|   ## HTTP Statuses  Along with the HTTP methods that the API responds to, it will also return standard HTTP statuses, including error codes.  In the event of a problem, the status will contain the error code, while the body of the response will usually contain additional information about the problem that was encountered.  In general, if the status returned is in the 200 range, it indicates that the request was fulfilled successfully and that no error was encountered.  Return codes in the 400 range typically indicate that there was an issue with the request that was sent. Among other things, this could mean that you did not authenticate correctly, that you are requesting an action that you do not have authorization for, that the object you are requesting does not exist, or that your request is malformed.  If you receive a status in the 500 range, this generally indicates a server-side problem. This means that we are having an issue on our end and cannot fulfill your request currently.  400 and 500 level error responses will include a JSON object in their body, including the following attributes:  |Name|Type|Description| |--- |--- |--- | |id|string|A short identifier corresponding to the HTTP status code returned. For example, the ID for a response returning a 404 status code would be \"not_found.\"| |message|string|A message providing additional information about the error, including details to help resolve it when possible.| |request_id|string|Optionally, some endpoints may include a request ID that should be provided when reporting bugs or opening support tickets to help identify the issue.|  ### Example Error Response  ```     HTTP/1.1 403 Forbidden     {       \"id\":       \"forbidden\",       \"message\":  \"You do not have access for the attempted action.\"     } ```  ## Responses  When a request is successful, a response body will typically be sent back in the form of a JSON object. An exception to this is when a DELETE request is processed, which will result in a successful HTTP 204 status and an empty response body.  Inside of this JSON object, the resource root that was the target of the request will be set as the key. This will be the singular form of the word if the request operated on a single object, and the plural form of the word if a collection was processed.  For example, if you send a GET request to `/v2/droplets/$DROPLET_ID` you will get back an object with a key called \"`droplet`\". However, if you send the GET request to the general collection at `/v2/droplets`, you will get back an object with a key called \"`droplets`\".  The value of these keys will generally be a JSON object for a request on a single object and an array of objects for a request on a collection of objects.  ### Response for a Single Object  ```json     {         \"droplet\": {             \"name\": \"example.com\"             . . .         }     } ```  ### Response for an Object Collection  ```json     {         \"droplets\": [             {                 \"name\": \"example.com\"                 . . .             },             {                 \"name\": \"second.com\"                 . . .             }         ]     } ```  ## Meta  In addition to the main resource root, the response may also contain a `meta` object. This object contains information about the response itself.  The `meta` object contains a `total` key that is set to the total number of objects returned by the request. This has implications on the `links` object and pagination.  The `meta` object will only be displayed when it has a value. Currently, the `meta` object will have a value when a request is made on a collection (like `droplets` or `domains`).   ### Sample Meta Object  ```json     {         . . .         \"meta\": {             \"total\": 43         }         . . .     } ```  ## Links & Pagination  The `links` object is returned as part of the response body when pagination is enabled. By default, 20 objects are returned per page. If the response contains 20 objects or fewer, no `links` object will be returned. If the response contains more than 20 objects, the first 20 will be returned along with the `links` object.  You can request a different pagination limit or force pagination by appending `?per_page=` to the request with the number of items you would like per page. For instance, to show only two results per page, you could add `?per_page=2` to the end of your query. The maximum number of results per page is 200.  The `links` object contains a `pages` object. The `pages` object, in turn, contains keys indicating the relationship of additional pages. The values of these are the URLs of the associated pages. The keys will be one of the following:  *   **first**: The URI of the first page of results. *   **prev**: The URI of the previous sequential page of results. *   **next**: The URI of the next sequential page of results. *   **last**: The URI of the last page of results.  The `pages` object will only include the links that make sense. So for the first page of results, no `first` or `prev` links will ever be set. This convention holds true in other situations where a link would not make sense.  ### Sample Links Object  ```json     {         . . .         \"links\": {             \"pages\": {                 \"last\": \"https://api.digitalocean.com/v2/images?page=2\",                 \"next\": \"https://api.digitalocean.com/v2/images?page=2\"             }         }         . . .     } ```  ## Rate Limit  Requests through the API are rate limited per OAuth token. Current rate limits:  *   5,000 requests per hour *   250 requests per minute (5% of the hourly total)  Once you exceed either limit, you will be rate limited until the next cycle starts. Space out any requests that you would otherwise issue in bursts for the best results.  The rate limiting information is contained within the response headers of each request. The relevant headers are:  *   **ratelimit-limit**: The number of requests that can be made per hour. *   **ratelimit-remaining**: The number of requests that remain before you hit your request limit. See the information below for how the request limits expire. *   **ratelimit-reset**: This represents the time when the oldest request will expire. The value is given in [Unix epoch time](http://en.wikipedia.org/wiki/Unix_time). See below for more information about how request limits expire.  More rate limiting information is returned only within burst limit error response headers: *   **retry-after**: The number of seconds to wait before making another request when rate limited.  As long as the `ratelimit-remaining` count is above zero, you will be able to make additional requests.  The way that a request expires and is removed from the current limit count is important to understand. Rather than counting all of the requests for an hour and resetting the `ratelimit-remaining` value at the end of the hour, each request instead has its own timer.  This means that each request contributes toward the `ratelimit-remaining` count for one complete hour after the request is made. When that request's timer runs out, it is no longer counted towards the request limit.  This has implications on the meaning of the `ratelimit-reset` header as well. Because the entire rate limit is not reset at one time, the value of this header is set to the time when the _oldest_ request will expire.  Keep this in mind if you see your `ratelimit-reset` value change, but not move an entire hour into the future.  If the `ratelimit-remaining` reaches zero, subsequent requests will receive a 429 error code until the request reset has been reached.   `ratelimit-remaining` reaching zero can also indicate that the \"burst limit\" of 250  requests per minute limit was met, even if the 5,000 requests per hour limit was not.  In this case, the 429 error response will include a `retry-after` header to indicate how  long to wait (in seconds) until the request may be retried.  You can see the format of the response in the examples.   **Note:** The following endpoints have special rate limit requirements that are independent of the limits defined above.  *   Only 10 `GET` requests to the `/v2/account/keys` endpoint to list SSH keys can be made per 60 seconds. *   Only 5 requests to any and all `v2/cdn/endpoints` can be made per 10 seconds. This includes `v2/cdn/endpoints`,      `v2/cdn/endpoints/$ENDPOINT_ID`, and `v2/cdn/endpoints/$ENDPOINT_ID/cache`. *   Only 50 strings within the `files` json struct in the `v2/cdn/endpoints/$ENDPOINT_ID/cache` [payload](https://docs.digitalocean.com/reference/api/api-reference/#operation/cdn_purge_cache)      can be requested every 20 seconds.  ### Sample Rate Limit Headers  ```     . . .     ratelimit-limit: 1200     ratelimit-remaining: 1193     rateLimit-reset: 1402425459     . . . ```    ### Sample Rate Limit Headers When Burst Limit is Reached:  ```     . . .     ratelimit-limit: 5000     ratelimit-remaining: 0     rateLimit-reset: 1402425459     retry-after: 29     . . . ```  ### Sample Rate Exceeded Response  ```     429 Too Many Requests     {             id: \"too_many_requests\",             message: \"API Rate limit exceeded.\"     } ```  ## Curl Examples  Throughout this document, some example API requests will be given using the `curl` command. This will allow us to demonstrate the various endpoints in a simple, textual format.      These examples assume that you are using a Linux or macOS command line. To run these commands on a Windows machine, you can either use cmd.exe, PowerShell, or WSL:  * For cmd.exe, use the `set VAR=VALUE` [syntax](https://docs.microsoft.com/en-us/windows-server/administration/windows-commands/set_1) to define environment variables, call them with `%VAR%`, then replace all backslashes (`\\`) in the examples with carets (`^`).  * For PowerShell, use the `$Env:VAR = \"VALUE\"` [syntax](https://docs.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_environment_variables?view=powershell-7.2) to define environment variables, call them with `$Env:VAR`, then replace `curl` with `curl.exe` and all backslashes (`\\`) in the examples with backticks (`` ` ``).  * WSL is a compatibility layer that allows you to emulate a Linux terminal on a Windows machine. Install WSL with our [community tutorial](https://www.digitalocean.com/community/tutorials/how-to-install-the-windows-subsystem-for-linux-2-on-microsoft-windows-10),  then follow this API documentation normally.  The names of account-specific references (like Droplet IDs, for instance) will be represented by variables. For instance, a Droplet ID may be represented by a variable called `$DROPLET_ID`. You can set the associated variables in your environment if you wish to use the examples without modification.  The first variable that you should set to get started is your OAuth authorization token. The next section will go over the details of this, but you can set an environmental variable for it now.  Generate a token by going to the [Apps & API](https://cloud.digitalocean.com/settings/applications) section of the DigitalOcean control panel. Use an existing token if you have saved one, or generate a new token with the \"Generate new token\" button. Copy the generated token and use it to set and export the TOKEN variable in your environment as the example shows.  You may also wish to set some other variables now or as you go along. For example, you may wish to set the `DROPLET_ID` variable to one of your Droplet IDs since this will be used frequently in the API.  If you are following along, make sure you use a Droplet ID that you control so that your commands will execute correctly.  If you need access to the headers of a response through `curl`, you can pass the `-i` flag to display the header information along with the body. If you are only interested in the header, you can instead pass the `-I` flag, which will exclude the response body entirely.   ### Set and Export your OAuth Token  ``` export DIGITALOCEAN_TOKEN=your_token_here ```  ### Set and Export a Variable  ``` export DROPLET_ID=1111111 ```  ## Parameters  There are two different ways to pass parameters in a request with the API.  When passing parameters to create or update an object, parameters should be passed as a JSON object containing the appropriate attribute names and values as key-value pairs. When you use this format, you should specify that you are sending a JSON object in the header. This is done by setting the `Content-Type` header to `application/json`. This ensures that your request is interpreted correctly.  When passing parameters to filter a response on GET requests, parameters can be passed using standard query attributes. In this case, the parameters would be embedded into the URI itself by appending a `?` to the end of the URI and then setting each attribute with an equal sign. Attributes can be separated with a `&`. Tools like `curl` can create the appropriate URI when given parameters and values; this can also be done using the `-F` flag and then passing the key and value as an argument. The argument should take the form of a quoted string with the attribute being set to a value with an equal sign.  ### Pass Parameters as a JSON Object  ```     curl -H \"Authorization: Bearer $DIGITALOCEAN_TOKEN\" \\         -H \"Content-Type: application/json\" \\         -d '{\"name\": \"example.com\", \"ip_address\": \"127.0.0.1\"}' \\         -X POST \"https://api.digitalocean.com/v2/domains\" ```  ### Pass Filter Parameters as a Query String  ```      curl -H \"Authorization: Bearer $DIGITALOCEAN_TOKEN\" \\          -X GET \\          \"https://api.digitalocean.com/v2/images?private=true\" ```  ## Cross Origin Resource Sharing  In order to make requests to the API from other domains, the API implements Cross Origin Resource Sharing (CORS) support.  CORS support is generally used to create AJAX requests outside of the domain that the request originated from. This is necessary to implement projects like control panels utilizing the API. This tells the browser that it can send requests to an outside domain.  The procedure that the browser initiates in order to perform these actions (other than GET requests) begins by sending a \"preflight\" request. This sets the `Origin` header and uses the `OPTIONS` method. The server will reply back with the methods it allows and some of the limits it imposes. The client then sends the actual request if it falls within the allowed constraints.  This process is usually done in the background by the browser, but you can use curl to emulate this process using the example provided. The headers that will be set to show the constraints are:  *   **Access-Control-Allow-Origin**: This is the domain that is sent by the client or browser as the origin of the request. It is set through an `Origin` header. *   **Access-Control-Allow-Methods**: This specifies the allowed options for requests from that domain. This will generally be all available methods. *   **Access-Control-Expose-Headers**: This will contain the headers that will be available to requests from the origin domain. *   **Access-Control-Max-Age**: This is the length of time that the access is considered valid. After this expires, a new preflight should be sent. *   **Access-Control-Allow-Credentials**: This will be set to `true`. It basically allows you to send your OAuth token for authentication.  You should not need to be concerned with the details of these headers, because the browser will typically do all of the work for you. 
 *
 * The version of the OpenAPI document: 2.0
 * Contact: api-engineering@digitalocean.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`droplets_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DropletsCreateError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`droplets_destroy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DropletsDestroyError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`droplets_destroy_by_tag`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DropletsDestroyByTagError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`droplets_destroy_retry_with_associated_resources`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DropletsDestroyRetryWithAssociatedResourcesError {
    Status401(models::Error),
    Status404(models::Error),
    Status409(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`droplets_destroy_with_associated_resources_dangerous`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DropletsDestroyWithAssociatedResourcesDangerousError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`droplets_destroy_with_associated_resources_selective`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DropletsDestroyWithAssociatedResourcesSelectiveError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`droplets_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DropletsGetError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`droplets_get_backup_policy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DropletsGetBackupPolicyError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`droplets_get_destroy_associated_resources_status`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DropletsGetDestroyAssociatedResourcesStatusError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`droplets_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DropletsListError {
    Status401(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`droplets_list_associated_resources`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DropletsListAssociatedResourcesError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`droplets_list_backup_policies`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DropletsListBackupPoliciesError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`droplets_list_backups`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DropletsListBackupsError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`droplets_list_firewalls`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DropletsListFirewallsError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`droplets_list_kernels`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DropletsListKernelsError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`droplets_list_neighbors`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DropletsListNeighborsError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`droplets_list_neighbors_ids`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DropletsListNeighborsIdsError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`droplets_list_snapshots`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DropletsListSnapshotsError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`droplets_list_supported_backup_policies`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DropletsListSupportedBackupPoliciesError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    UnknownValue(serde_json::Value),
}


/// To create a new Droplet, send a POST request to `/v2/droplets` setting the required attributes.  A Droplet will be created using the provided information. The response body will contain a JSON object with a key called `droplet`. The value will be an object containing the standard attributes for your new Droplet. The response code, 202 Accepted, does not indicate the success or failure of the operation, just that the request has been accepted for processing. The `actions` returned as part of the response's `links` object can be used to check the status of the Droplet create event.  ### Create Multiple Droplets  Creating multiple Droplets is very similar to creating a single Droplet. Instead of sending `name` as a string, send `names` as an array of strings. A Droplet will be created for each name you send using the associated information. Up to ten Droplets may be created this way at a time.  Rather than returning a single Droplet, the response body will contain a JSON array with a key called `droplets`. This will be set to an array of JSON objects, each of which will contain the standard Droplet attributes. The response code, 202 Accepted, does not indicate the success or failure of any operation, just that the request has been accepted for processing. The array of `actions` returned as part of the response's `links` object can be used to check the status of each individual Droplet create event. 
pub async fn droplets_create(configuration: &configuration::Configuration, droplets_create_request: Option<models::DropletsCreateRequest>) -> Result<models::DropletsCreate202Response, Error<DropletsCreateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_droplets_create_request = droplets_create_request;

    let uri_str = format!("{}/v2/droplets", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_droplets_create_request);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DropletsCreate202Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DropletsCreate202Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DropletsCreateError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To delete a Droplet, send a DELETE request to `/v2/droplets/$DROPLET_ID`.  A successful request will receive a 204 status code with no body in response. This indicates that the request was processed successfully. 
pub async fn droplets_destroy(configuration: &configuration::Configuration, droplet_id: i32) -> Result<(), Error<DropletsDestroyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_droplet_id = droplet_id;

    let uri_str = format!("{}/v2/droplets/{droplet_id}", configuration.base_path, droplet_id=p_path_droplet_id);
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

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
        let entity: Option<DropletsDestroyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To delete **all** Droplets assigned to a specific tag, include the `tag_name` query parameter set to the name of the tag in your DELETE request. For example, `/v2/droplets?tag_name=$TAG_NAME`.  This endpoint requires `tag:read` scope.  A successful request will receive a 204 status code with no body in response. This indicates that the request was processed successfully. 
pub async fn droplets_destroy_by_tag(configuration: &configuration::Configuration, tag_name: &str) -> Result<(), Error<DropletsDestroyByTagError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_tag_name = tag_name;

    let uri_str = format!("{}/v2/droplets", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    req_builder = req_builder.query(&[("tag_name", &p_query_tag_name.to_string())]);
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
        let entity: Option<DropletsDestroyByTagError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// If the status of a request to destroy a Droplet with its associated resources reported any errors, it can be retried by sending a POST request to the `/v2/droplets/$DROPLET_ID/destroy_with_associated_resources/retry` endpoint.  Only one destroy can be active at a time per Droplet. If a retry is issued while another destroy is in progress for the Droplet a 409 status code will be returned. A successful response will include a 202 response code and no content. 
pub async fn droplets_destroy_retry_with_associated_resources(configuration: &configuration::Configuration, droplet_id: i32) -> Result<(), Error<DropletsDestroyRetryWithAssociatedResourcesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_droplet_id = droplet_id;

    let uri_str = format!("{}/v2/droplets/{droplet_id}/destroy_with_associated_resources/retry", configuration.base_path, droplet_id=p_path_droplet_id);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

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
        let entity: Option<DropletsDestroyRetryWithAssociatedResourcesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To destroy a Droplet along with all of its associated resources, send a DELETE request to the `/v2/droplets/$DROPLET_ID/destroy_with_associated_resources/dangerous` endpoint. The headers of this request must include an `X-Dangerous` key set to `true`. To preview which resources will be destroyed, first query the Droplet's associated resources. This operation _can not_ be reverse and should be used with caution.  A successful response will include a 202 response code and no content. Use the status endpoint to check on the success or failure of the destruction of the individual resources. 
pub async fn droplets_destroy_with_associated_resources_dangerous(configuration: &configuration::Configuration, droplet_id: i32, x_dangerous: bool) -> Result<(), Error<DropletsDestroyWithAssociatedResourcesDangerousError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_droplet_id = droplet_id;
    let p_header_x_dangerous = x_dangerous;

    let uri_str = format!("{}/v2/droplets/{droplet_id}/destroy_with_associated_resources/dangerous", configuration.base_path, droplet_id=p_path_droplet_id);
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.header("X-Dangerous", p_header_x_dangerous.to_string());
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
        let entity: Option<DropletsDestroyWithAssociatedResourcesDangerousError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To destroy a Droplet along with a sub-set of its associated resources, send a DELETE request to the `/v2/droplets/$DROPLET_ID/destroy_with_associated_resources/selective` endpoint. The JSON body of the request should include `reserved_ips`, `snapshots`, `volumes`, or `volume_snapshots` keys each set to an array of IDs for the associated resources to be destroyed. The IDs can be found by querying the Droplet's associated resources. Any associated resource not included in the request will remain and continue to accrue changes on your account.  A successful response will include a 202 response code and no content. Use the status endpoint to check on the success or failure of the destruction of the individual resources. 
pub async fn droplets_destroy_with_associated_resources_selective(configuration: &configuration::Configuration, droplet_id: i32, selective_destroy_associated_resource: Option<models::SelectiveDestroyAssociatedResource>) -> Result<(), Error<DropletsDestroyWithAssociatedResourcesSelectiveError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_droplet_id = droplet_id;
    let p_body_selective_destroy_associated_resource = selective_destroy_associated_resource;

    let uri_str = format!("{}/v2/droplets/{droplet_id}/destroy_with_associated_resources/selective", configuration.base_path, droplet_id=p_path_droplet_id);
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_selective_destroy_associated_resource);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<DropletsDestroyWithAssociatedResourcesSelectiveError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To show information about an individual Droplet, send a GET request to `/v2/droplets/$DROPLET_ID`. 
pub async fn droplets_get(configuration: &configuration::Configuration, droplet_id: i32) -> Result<models::DropletsGet200Response, Error<DropletsGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_droplet_id = droplet_id;

    let uri_str = format!("{}/v2/droplets/{droplet_id}", configuration.base_path, droplet_id=p_path_droplet_id);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DropletsGet200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DropletsGet200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DropletsGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To show information about an individual Droplet's backup policy, send a GET request to `/v2/droplets/$DROPLET_ID/backups/policy`. 
pub async fn droplets_get_backup_policy(configuration: &configuration::Configuration, droplet_id: i32) -> Result<models::DropletsGetBackupPolicy200Response, Error<DropletsGetBackupPolicyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_droplet_id = droplet_id;

    let uri_str = format!("{}/v2/droplets/{droplet_id}/backups/policy", configuration.base_path, droplet_id=p_path_droplet_id);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DropletsGetBackupPolicy200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DropletsGetBackupPolicy200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DropletsGetBackupPolicyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To check on the status of a request to destroy a Droplet with its associated resources, send a GET request to the `/v2/droplets/$DROPLET_ID/destroy_with_associated_resources/status` endpoint. 
pub async fn droplets_get_destroy_associated_resources_status(configuration: &configuration::Configuration, droplet_id: i32) -> Result<models::AssociatedResourceStatus, Error<DropletsGetDestroyAssociatedResourcesStatusError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_droplet_id = droplet_id;

    let uri_str = format!("{}/v2/droplets/{droplet_id}/destroy_with_associated_resources/status", configuration.base_path, droplet_id=p_path_droplet_id);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AssociatedResourceStatus`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AssociatedResourceStatus`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DropletsGetDestroyAssociatedResourcesStatusError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To list all Droplets in your account, send a GET request to `/v2/droplets`.  The response body will be a JSON object with a key of `droplets`. This will be set to an array containing objects each representing a Droplet. These will contain the standard Droplet attributes.  ### Filtering Results by Tag  It's possible to request filtered results by including certain query parameters. To only list Droplets assigned to a specific tag, include the `tag_name` query parameter set to the name of the tag in your GET request. For example, `/v2/droplets?tag_name=$TAG_NAME`.  ### GPU Droplets  By default, only non-GPU Droplets are returned. To list only GPU Droplets, set the `type` query parameter to `gpus`. For example, `/v2/droplets?type=gpus`. 
pub async fn droplets_list(configuration: &configuration::Configuration, per_page: Option<i32>, page: Option<i32>, tag_name: Option<&str>, name: Option<&str>, r#type: Option<&str>) -> Result<models::DropletsList200Response, Error<DropletsListError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_per_page = per_page;
    let p_query_page = page;
    let p_query_tag_name = tag_name;
    let p_query_name = name;
    let p_query_type = r#type;

    let uri_str = format!("{}/v2/droplets", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_per_page {
        req_builder = req_builder.query(&[("per_page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_tag_name {
        req_builder = req_builder.query(&[("tag_name", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_name {
        req_builder = req_builder.query(&[("name", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_type {
        req_builder = req_builder.query(&[("type", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DropletsList200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DropletsList200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DropletsListError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To list the associated billable resources that can be destroyed along with a Droplet, send a GET request to the `/v2/droplets/$DROPLET_ID/destroy_with_associated_resources` endpoint.  This endpoint will only return resources that you are authorized to see. For example, to see associated Reserved IPs, include the `reserved_ip:read` scope.  The response will be a JSON object containing `snapshots`, `volumes`, and `volume_snapshots` keys. Each will be set to an array of objects containing information about the associated resources. 
pub async fn droplets_list_associated_resources(configuration: &configuration::Configuration, droplet_id: i32) -> Result<models::DropletsListAssociatedResources200Response, Error<DropletsListAssociatedResourcesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_droplet_id = droplet_id;

    let uri_str = format!("{}/v2/droplets/{droplet_id}/destroy_with_associated_resources", configuration.base_path, droplet_id=p_path_droplet_id);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DropletsListAssociatedResources200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DropletsListAssociatedResources200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DropletsListAssociatedResourcesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To list information about the backup policies for all Droplets in the account, send a GET request to `/v2/droplets/backups/policies`. 
pub async fn droplets_list_backup_policies(configuration: &configuration::Configuration, per_page: Option<i32>, page: Option<i32>) -> Result<models::DropletsListBackupPolicies200Response, Error<DropletsListBackupPoliciesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_per_page = per_page;
    let p_query_page = page;

    let uri_str = format!("{}/v2/droplets/backups/policies", configuration.base_path);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DropletsListBackupPolicies200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DropletsListBackupPolicies200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DropletsListBackupPoliciesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve any backups associated with a Droplet, send a GET request to `/v2/droplets/$DROPLET_ID/backups`.  You will get back a JSON object that has a `backups` key. This will be set to an array of backup objects, each of which contain the standard Droplet backup attributes. 
pub async fn droplets_list_backups(configuration: &configuration::Configuration, droplet_id: i32, per_page: Option<i32>, page: Option<i32>) -> Result<models::DropletsListBackups200Response, Error<DropletsListBackupsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_droplet_id = droplet_id;
    let p_query_per_page = per_page;
    let p_query_page = page;

    let uri_str = format!("{}/v2/droplets/{droplet_id}/backups", configuration.base_path, droplet_id=p_path_droplet_id);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DropletsListBackups200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DropletsListBackups200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DropletsListBackupsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve a list of all firewalls available to a Droplet, send a GET request to `/v2/droplets/$DROPLET_ID/firewalls`  The response will be a JSON object that has a key called `firewalls`. This will be set to an array of `firewall` objects, each of which contain the standard `firewall` attributes. 
pub async fn droplets_list_firewalls(configuration: &configuration::Configuration, droplet_id: i32, per_page: Option<i32>, page: Option<i32>) -> Result<models::DropletsListFirewalls200Response, Error<DropletsListFirewallsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_droplet_id = droplet_id;
    let p_query_per_page = per_page;
    let p_query_page = page;

    let uri_str = format!("{}/v2/droplets/{droplet_id}/firewalls", configuration.base_path, droplet_id=p_path_droplet_id);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DropletsListFirewalls200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DropletsListFirewalls200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DropletsListFirewallsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve a list of all kernels available to a Droplet, send a GET request to `/v2/droplets/$DROPLET_ID/kernels`  The response will be a JSON object that has a key called `kernels`. This will be set to an array of `kernel` objects, each of which contain the standard `kernel` attributes. 
pub async fn droplets_list_kernels(configuration: &configuration::Configuration, droplet_id: i32, per_page: Option<i32>, page: Option<i32>) -> Result<models::DropletsListKernels200Response, Error<DropletsListKernelsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_droplet_id = droplet_id;
    let p_query_per_page = per_page;
    let p_query_page = page;

    let uri_str = format!("{}/v2/droplets/{droplet_id}/kernels", configuration.base_path, droplet_id=p_path_droplet_id);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DropletsListKernels200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DropletsListKernels200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DropletsListKernelsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve a list of any \"neighbors\" (i.e. Droplets that are co-located on the same physical hardware) for a specific Droplet, send a GET request to `/v2/droplets/$DROPLET_ID/neighbors`.  The results will be returned as a JSON object with a key of `droplets`. This will be set to an array containing objects representing any other Droplets that share the same physical hardware. An empty array indicates that the Droplet is not co-located any other Droplets associated with your account. 
pub async fn droplets_list_neighbors(configuration: &configuration::Configuration, droplet_id: i32) -> Result<models::DropletsListNeighbors200Response, Error<DropletsListNeighborsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_droplet_id = droplet_id;

    let uri_str = format!("{}/v2/droplets/{droplet_id}/neighbors", configuration.base_path, droplet_id=p_path_droplet_id);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DropletsListNeighbors200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DropletsListNeighbors200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DropletsListNeighborsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve a list of all Droplets that are co-located on the same physical hardware, send a GET request to `/v2/reports/droplet_neighbors_ids`.  The results will be returned as a JSON object with a key of `neighbor_ids`. This will be set to an array of arrays. Each array will contain a set of Droplet IDs for Droplets that share a physical server. An empty array indicates that all Droplets associated with your account are located on separate physical hardware. 
pub async fn droplets_list_neighbors_ids(configuration: &configuration::Configuration, ) -> Result<models::NeighborIds, Error<DropletsListNeighborsIdsError>> {

    let uri_str = format!("{}/v2/reports/droplet_neighbors_ids", configuration.base_path);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::NeighborIds`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::NeighborIds`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DropletsListNeighborsIdsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve the snapshots that have been created from a Droplet, send a GET request to `/v2/droplets/$DROPLET_ID/snapshots`.  You will get back a JSON object that has a `snapshots` key. This will be set to an array of snapshot objects, each of which contain the standard Droplet snapshot attributes. 
pub async fn droplets_list_snapshots(configuration: &configuration::Configuration, droplet_id: i32, per_page: Option<i32>, page: Option<i32>) -> Result<models::DropletsListSnapshots200Response, Error<DropletsListSnapshotsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_droplet_id = droplet_id;
    let p_query_per_page = per_page;
    let p_query_page = page;

    let uri_str = format!("{}/v2/droplets/{droplet_id}/snapshots", configuration.base_path, droplet_id=p_path_droplet_id);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DropletsListSnapshots200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DropletsListSnapshots200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DropletsListSnapshotsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// To retrieve a list of all supported Droplet backup policies, send a GET request to `/v2/droplets/backups/supported_policies`. 
pub async fn droplets_list_supported_backup_policies(configuration: &configuration::Configuration, ) -> Result<models::DropletsListSupportedBackupPolicies200Response, Error<DropletsListSupportedBackupPoliciesError>> {

    let uri_str = format!("{}/v2/droplets/backups/supported_policies", configuration.base_path);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DropletsListSupportedBackupPolicies200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DropletsListSupportedBackupPolicies200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DropletsListSupportedBackupPoliciesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

