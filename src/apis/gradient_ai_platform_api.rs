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

/// struct for typed errors of method [`genai_attach_agent`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiAttachAgentError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_attach_agent_function`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiAttachAgentFunctionError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_attach_knowledge_base`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiAttachKnowledgeBaseError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_attach_knowledge_bases`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiAttachKnowledgeBasesError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_cancel_indexing_job`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiCancelIndexingJobError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_create_agent`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiCreateAgentError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_create_agent_api_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiCreateAgentApiKeyError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_create_anthropic_api_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiCreateAnthropicApiKeyError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_create_data_source_file_upload_presigned_urls`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiCreateDataSourceFileUploadPresignedUrlsError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_create_evaluation_dataset`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiCreateEvaluationDatasetError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_create_evaluation_dataset_file_upload_presigned_urls`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiCreateEvaluationDatasetFileUploadPresignedUrlsError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_create_evaluation_test_case`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiCreateEvaluationTestCaseError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_create_indexing_job`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiCreateIndexingJobError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_create_knowledge_base`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiCreateKnowledgeBaseError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_create_knowledge_base_data_source`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiCreateKnowledgeBaseDataSourceError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_create_model_api_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiCreateModelApiKeyError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_create_oauth2_dropbox_tokens`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiCreateOauth2DropboxTokensError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_create_openai_api_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiCreateOpenaiApiKeyError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_create_scheduled_indexing`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiCreateScheduledIndexingError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_create_workspace`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiCreateWorkspaceError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_delete_agent`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiDeleteAgentError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_delete_agent_api_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiDeleteAgentApiKeyError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_delete_anthropic_api_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiDeleteAnthropicApiKeyError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_delete_knowledge_base`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiDeleteKnowledgeBaseError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_delete_knowledge_base_data_source`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiDeleteKnowledgeBaseDataSourceError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_delete_model_api_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiDeleteModelApiKeyError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_delete_openai_api_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiDeleteOpenaiApiKeyError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_delete_scheduled_indexing`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiDeleteScheduledIndexingError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_delete_workspace`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiDeleteWorkspaceError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_detach_agent`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiDetachAgentError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_detach_agent_function`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiDetachAgentFunctionError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_detach_knowledge_base`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiDetachKnowledgeBaseError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_get_agent`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiGetAgentError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_get_agent_children`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiGetAgentChildrenError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_get_agent_usage`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiGetAgentUsageError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_get_anthropic_api_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiGetAnthropicApiKeyError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_get_evaluation_run`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiGetEvaluationRunError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_get_evaluation_run_prompt_results`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiGetEvaluationRunPromptResultsError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_get_evaluation_run_results`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiGetEvaluationRunResultsError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_get_evaluation_test_case`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiGetEvaluationTestCaseError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_get_indexing_job`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiGetIndexingJobError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_get_indexing_job_details_signed_url`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiGetIndexingJobDetailsSignedUrlError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_get_knowledge_base`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiGetKnowledgeBaseError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_get_oauth2_url`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiGetOauth2UrlError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_get_openai_api_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiGetOpenaiApiKeyError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_get_scheduled_indexing`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiGetScheduledIndexingError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_get_workspace`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiGetWorkspaceError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_list_agent_api_keys`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiListAgentApiKeysError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_list_agent_versions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiListAgentVersionsError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_list_agents`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiListAgentsError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_list_agents_by_anthropic_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiListAgentsByAnthropicKeyError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_list_agents_by_openai_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiListAgentsByOpenaiKeyError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_list_agents_by_workspace`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiListAgentsByWorkspaceError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_list_anthropic_api_keys`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiListAnthropicApiKeysError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_list_datacenter_regions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiListDatacenterRegionsError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_list_evaluation_metrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiListEvaluationMetricsError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_list_evaluation_runs_by_test_case`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiListEvaluationRunsByTestCaseError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_list_evaluation_test_cases`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiListEvaluationTestCasesError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_list_evaluation_test_cases_by_workspace`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiListEvaluationTestCasesByWorkspaceError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_list_indexing_job_data_sources`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiListIndexingJobDataSourcesError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_list_indexing_jobs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiListIndexingJobsError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_list_indexing_jobs_by_knowledge_base`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiListIndexingJobsByKnowledgeBaseError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_list_knowledge_base_data_sources`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiListKnowledgeBaseDataSourcesError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_list_knowledge_bases`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiListKnowledgeBasesError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_list_model_api_keys`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiListModelApiKeysError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_list_models`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiListModelsError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_list_openai_api_keys`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiListOpenaiApiKeysError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_list_workspaces`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiListWorkspacesError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_regenerate_agent_api_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiRegenerateAgentApiKeyError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_regenerate_model_api_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiRegenerateModelApiKeyError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_rollback_to_agent_version`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiRollbackToAgentVersionError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_run_evaluation_test_case`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiRunEvaluationTestCaseError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_update_agent`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiUpdateAgentError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_update_agent_api_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiUpdateAgentApiKeyError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_update_agent_deployment_visibility`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiUpdateAgentDeploymentVisibilityError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_update_agent_function`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiUpdateAgentFunctionError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_update_agents_workspace`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiUpdateAgentsWorkspaceError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_update_anthropic_api_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiUpdateAnthropicApiKeyError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_update_attached_agent`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiUpdateAttachedAgentError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_update_evaluation_test_case`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiUpdateEvaluationTestCaseError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_update_knowledge_base`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiUpdateKnowledgeBaseError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_update_model_api_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiUpdateModelApiKeyError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_update_openai_api_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiUpdateOpenaiApiKeyError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`genai_update_workspace`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenaiUpdateWorkspaceError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status500(models::Error),
    DefaultResponse(models::Error),
    UnknownValue(serde_json::Value),
}

/// To add an agent route to an agent, send a POST request to `/v2/gen-ai/agents/{parent_agent_uuid}/child_agents/{child_agent_uuid}`.
pub async fn genai_attach_agent(
    configuration: &configuration::Configuration,
    parent_agent_uuid: &str,
    child_agent_uuid: &str,
    api_link_agent_input_public: Option<models::ApiLinkAgentInputPublic>,
) -> Result<models::ApiLinkAgentOutput, Error<GenaiAttachAgentError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_parent_agent_uuid = parent_agent_uuid;
    let p_path_child_agent_uuid = child_agent_uuid;
    let p_body_api_link_agent_input_public = api_link_agent_input_public;

    let uri_str = format!(
        "{}/v2/gen-ai/agents/{parent_agent_uuid}/child_agents/{child_agent_uuid}",
        configuration.base_path,
        parent_agent_uuid = crate::apis::urlencode(p_path_parent_agent_uuid),
        child_agent_uuid = crate::apis::urlencode(p_path_child_agent_uuid)
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
    req_builder = req_builder.json(&p_body_api_link_agent_input_public);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiLinkAgentOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiLinkAgentOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiAttachAgentError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To create a function route for an agent, send a POST request to `/v2/gen-ai/agents/{agent_uuid}/functions`.
pub async fn genai_attach_agent_function(
    configuration: &configuration::Configuration,
    agent_uuid: &str,
    api_link_agent_function_input_public: Option<models::ApiLinkAgentFunctionInputPublic>,
) -> Result<models::ApiLinkAgentFunctionOutput, Error<GenaiAttachAgentFunctionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_agent_uuid = agent_uuid;
    let p_body_api_link_agent_function_input_public = api_link_agent_function_input_public;

    let uri_str = format!(
        "{}/v2/gen-ai/agents/{agent_uuid}/functions",
        configuration.base_path,
        agent_uuid = crate::apis::urlencode(p_path_agent_uuid)
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
    req_builder = req_builder.json(&p_body_api_link_agent_function_input_public);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiLinkAgentFunctionOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiLinkAgentFunctionOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiAttachAgentFunctionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To attach a knowledge base to an agent, send a POST request to `/v2/gen-ai/agents/{agent_uuid}/knowledge_bases/{knowledge_base_uuid}`
pub async fn genai_attach_knowledge_base(
    configuration: &configuration::Configuration,
    agent_uuid: &str,
    knowledge_base_uuid: &str,
) -> Result<models::ApiLinkKnowledgeBaseOutput, Error<GenaiAttachKnowledgeBaseError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_agent_uuid = agent_uuid;
    let p_path_knowledge_base_uuid = knowledge_base_uuid;

    let uri_str = format!(
        "{}/v2/gen-ai/agents/{agent_uuid}/knowledge_bases/{knowledge_base_uuid}",
        configuration.base_path,
        agent_uuid = crate::apis::urlencode(p_path_agent_uuid),
        knowledge_base_uuid = crate::apis::urlencode(p_path_knowledge_base_uuid)
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiLinkKnowledgeBaseOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiLinkKnowledgeBaseOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiAttachKnowledgeBaseError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To attach knowledge bases to an agent, send a POST request to `/v2/gen-ai/agents/{agent_uuid}/knowledge_bases`
pub async fn genai_attach_knowledge_bases(
    configuration: &configuration::Configuration,
    agent_uuid: &str,
) -> Result<models::ApiLinkKnowledgeBaseOutput, Error<GenaiAttachKnowledgeBasesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_agent_uuid = agent_uuid;

    let uri_str = format!(
        "{}/v2/gen-ai/agents/{agent_uuid}/knowledge_bases",
        configuration.base_path,
        agent_uuid = crate::apis::urlencode(p_path_agent_uuid)
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiLinkKnowledgeBaseOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiLinkKnowledgeBaseOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiAttachKnowledgeBasesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To cancel an indexing job for a knowledge base, send a PUT request to `/v2/gen-ai/indexing_jobs/{uuid}/cancel`.
pub async fn genai_cancel_indexing_job(
    configuration: &configuration::Configuration,
    uuid: &str,
    api_cancel_knowledge_base_indexing_job_input_public: Option<
        models::ApiCancelKnowledgeBaseIndexingJobInputPublic,
    >,
) -> Result<models::ApiCancelKnowledgeBaseIndexingJobOutput, Error<GenaiCancelIndexingJobError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_uuid = uuid;
    let p_body_api_cancel_knowledge_base_indexing_job_input_public =
        api_cancel_knowledge_base_indexing_job_input_public;

    let uri_str = format!(
        "{}/v2/gen-ai/indexing_jobs/{uuid}/cancel",
        configuration.base_path,
        uuid = crate::apis::urlencode(p_path_uuid)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_api_cancel_knowledge_base_indexing_job_input_public);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiCancelKnowledgeBaseIndexingJobOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiCancelKnowledgeBaseIndexingJobOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiCancelIndexingJobError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To create a new agent, send a POST request to `/v2/gen-ai/agents`. The response body contains a JSON object with the newly created agent object.
pub async fn genai_create_agent(
    configuration: &configuration::Configuration,
    api_create_agent_input_public: Option<models::ApiCreateAgentInputPublic>,
) -> Result<models::ApiCreateAgentOutput, Error<GenaiCreateAgentError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_api_create_agent_input_public = api_create_agent_input_public;

    let uri_str = format!("{}/v2/gen-ai/agents", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_api_create_agent_input_public);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiCreateAgentOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiCreateAgentOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiCreateAgentError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To create an agent API key, send a POST request to `/v2/gen-ai/agents/{agent_uuid}/api_keys`.
pub async fn genai_create_agent_api_key(
    configuration: &configuration::Configuration,
    agent_uuid: &str,
    api_create_agent_api_key_input_public: Option<models::ApiCreateAgentApiKeyInputPublic>,
) -> Result<models::ApiCreateAgentApiKeyOutput, Error<GenaiCreateAgentApiKeyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_agent_uuid = agent_uuid;
    let p_body_api_create_agent_api_key_input_public = api_create_agent_api_key_input_public;

    let uri_str = format!(
        "{}/v2/gen-ai/agents/{agent_uuid}/api_keys",
        configuration.base_path,
        agent_uuid = crate::apis::urlencode(p_path_agent_uuid)
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
    req_builder = req_builder.json(&p_body_api_create_agent_api_key_input_public);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiCreateAgentApiKeyOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiCreateAgentApiKeyOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiCreateAgentApiKeyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To create an Anthropic API key, send a POST request to `/v2/gen-ai/anthropic/keys`.
pub async fn genai_create_anthropic_api_key(
    configuration: &configuration::Configuration,
    api_create_anthropic_api_key_input_public: Option<models::ApiCreateAnthropicApiKeyInputPublic>,
) -> Result<models::ApiCreateAnthropicApiKeyOutput, Error<GenaiCreateAnthropicApiKeyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_api_create_anthropic_api_key_input_public =
        api_create_anthropic_api_key_input_public;

    let uri_str = format!("{}/v2/gen-ai/anthropic/keys", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_api_create_anthropic_api_key_input_public);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiCreateAnthropicApiKeyOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiCreateAnthropicApiKeyOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiCreateAnthropicApiKeyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To create presigned URLs for knowledge base data source file upload, send a POST request to `/v2/gen-ai/knowledge_bases/data_sources/file_upload_presigned_urls`.
pub async fn genai_create_data_source_file_upload_presigned_urls(
    configuration: &configuration::Configuration,
    api_create_data_source_file_upload_presigned_urls_input_public: Option<
        models::ApiCreateDataSourceFileUploadPresignedUrlsInputPublic,
    >,
) -> Result<
    models::ApiCreateDataSourceFileUploadPresignedUrlsOutput,
    Error<GenaiCreateDataSourceFileUploadPresignedUrlsError>,
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_api_create_data_source_file_upload_presigned_urls_input_public =
        api_create_data_source_file_upload_presigned_urls_input_public;

    let uri_str = format!(
        "{}/v2/gen-ai/knowledge_bases/data_sources/file_upload_presigned_urls",
        configuration.base_path
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
    req_builder =
        req_builder.json(&p_body_api_create_data_source_file_upload_presigned_urls_input_public);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiCreateDataSourceFileUploadPresignedUrlsOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiCreateDataSourceFileUploadPresignedUrlsOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiCreateDataSourceFileUploadPresignedUrlsError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To create an evaluation dataset, send a POST request to `/v2/gen-ai/evaluation_datasets`.
pub async fn genai_create_evaluation_dataset(
    configuration: &configuration::Configuration,
    api_create_evaluation_dataset_input_public: Option<
        models::ApiCreateEvaluationDatasetInputPublic,
    >,
) -> Result<models::ApiCreateEvaluationDatasetOutput, Error<GenaiCreateEvaluationDatasetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_api_create_evaluation_dataset_input_public =
        api_create_evaluation_dataset_input_public;

    let uri_str = format!("{}/v2/gen-ai/evaluation_datasets", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_api_create_evaluation_dataset_input_public);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiCreateEvaluationDatasetOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiCreateEvaluationDatasetOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiCreateEvaluationDatasetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To create presigned URLs for evaluation dataset file upload, send a POST request to `/v2/gen-ai/evaluation_datasets/file_upload_presigned_urls`.
pub async fn genai_create_evaluation_dataset_file_upload_presigned_urls(
    configuration: &configuration::Configuration,
    api_create_data_source_file_upload_presigned_urls_input_public: Option<
        models::ApiCreateDataSourceFileUploadPresignedUrlsInputPublic,
    >,
) -> Result<
    models::ApiCreateDataSourceFileUploadPresignedUrlsOutput,
    Error<GenaiCreateEvaluationDatasetFileUploadPresignedUrlsError>,
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_api_create_data_source_file_upload_presigned_urls_input_public =
        api_create_data_source_file_upload_presigned_urls_input_public;

    let uri_str = format!(
        "{}/v2/gen-ai/evaluation_datasets/file_upload_presigned_urls",
        configuration.base_path
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
    req_builder =
        req_builder.json(&p_body_api_create_data_source_file_upload_presigned_urls_input_public);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiCreateDataSourceFileUploadPresignedUrlsOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiCreateDataSourceFileUploadPresignedUrlsOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiCreateEvaluationDatasetFileUploadPresignedUrlsError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To create an evaluation test-case send a POST request to `/v2/gen-ai/evaluation_test_cases`.
pub async fn genai_create_evaluation_test_case(
    configuration: &configuration::Configuration,
    api_create_evaluation_test_case_input_public: Option<
        models::ApiCreateEvaluationTestCaseInputPublic,
    >,
) -> Result<models::ApiCreateEvaluationTestCaseOutput, Error<GenaiCreateEvaluationTestCaseError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_api_create_evaluation_test_case_input_public =
        api_create_evaluation_test_case_input_public;

    let uri_str = format!(
        "{}/v2/gen-ai/evaluation_test_cases",
        configuration.base_path
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
    req_builder = req_builder.json(&p_body_api_create_evaluation_test_case_input_public);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiCreateEvaluationTestCaseOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiCreateEvaluationTestCaseOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiCreateEvaluationTestCaseError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To start an indexing job for a knowledge base, send a POST request to `/v2/gen-ai/indexing_jobs`.
pub async fn genai_create_indexing_job(
    configuration: &configuration::Configuration,
    api_start_knowledge_base_indexing_job_input_public: Option<
        models::ApiStartKnowledgeBaseIndexingJobInputPublic,
    >,
) -> Result<models::ApiStartKnowledgeBaseIndexingJobOutput, Error<GenaiCreateIndexingJobError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_api_start_knowledge_base_indexing_job_input_public =
        api_start_knowledge_base_indexing_job_input_public;

    let uri_str = format!("{}/v2/gen-ai/indexing_jobs", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_api_start_knowledge_base_indexing_job_input_public);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiStartKnowledgeBaseIndexingJobOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiStartKnowledgeBaseIndexingJobOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiCreateIndexingJobError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To create a knowledge base, send a POST request to `/v2/gen-ai/knowledge_bases`.
pub async fn genai_create_knowledge_base(
    configuration: &configuration::Configuration,
    api_create_knowledge_base_input_public: Option<models::ApiCreateKnowledgeBaseInputPublic>,
) -> Result<models::ApiCreateKnowledgeBaseOutput, Error<GenaiCreateKnowledgeBaseError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_api_create_knowledge_base_input_public = api_create_knowledge_base_input_public;

    let uri_str = format!("{}/v2/gen-ai/knowledge_bases", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_api_create_knowledge_base_input_public);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiCreateKnowledgeBaseOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiCreateKnowledgeBaseOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiCreateKnowledgeBaseError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To add a data source to a knowledge base, send a POST request to `/v2/gen-ai/knowledge_bases/{knowledge_base_uuid}/data_sources`.
pub async fn genai_create_knowledge_base_data_source(
    configuration: &configuration::Configuration,
    knowledge_base_uuid: &str,
    api_create_knowledge_base_data_source_input_public: Option<
        models::ApiCreateKnowledgeBaseDataSourceInputPublic,
    >,
) -> Result<
    models::ApiCreateKnowledgeBaseDataSourceOutput,
    Error<GenaiCreateKnowledgeBaseDataSourceError>,
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_knowledge_base_uuid = knowledge_base_uuid;
    let p_body_api_create_knowledge_base_data_source_input_public =
        api_create_knowledge_base_data_source_input_public;

    let uri_str = format!(
        "{}/v2/gen-ai/knowledge_bases/{knowledge_base_uuid}/data_sources",
        configuration.base_path,
        knowledge_base_uuid = crate::apis::urlencode(p_path_knowledge_base_uuid)
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
    req_builder = req_builder.json(&p_body_api_create_knowledge_base_data_source_input_public);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiCreateKnowledgeBaseDataSourceOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiCreateKnowledgeBaseDataSourceOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiCreateKnowledgeBaseDataSourceError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To create a model API key, send a POST request to `/v2/gen-ai/models/api_keys`.
pub async fn genai_create_model_api_key(
    configuration: &configuration::Configuration,
    api_create_model_api_key_input_public: Option<models::ApiCreateModelApiKeyInputPublic>,
) -> Result<models::ApiCreateModelApiKeyOutput, Error<GenaiCreateModelApiKeyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_api_create_model_api_key_input_public = api_create_model_api_key_input_public;

    let uri_str = format!("{}/v2/gen-ai/models/api_keys", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_api_create_model_api_key_input_public);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiCreateModelApiKeyOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiCreateModelApiKeyOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiCreateModelApiKeyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To obtain the refresh token, needed for creation of data sources, send a GET request to `/v2/gen-ai/oauth2/dropbox/tokens`. Pass the code you obtrained from the oauth flow in the field 'code'
pub async fn genai_create_oauth2_dropbox_tokens(
    configuration: &configuration::Configuration,
    api_dropbox_oauth2_get_tokens_input: Option<models::ApiDropboxOauth2GetTokensInput>,
) -> Result<models::ApiDropboxOauth2GetTokensOutput, Error<GenaiCreateOauth2DropboxTokensError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_api_dropbox_oauth2_get_tokens_input = api_dropbox_oauth2_get_tokens_input;

    let uri_str = format!(
        "{}/v2/gen-ai/oauth2/dropbox/tokens",
        configuration.base_path
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
    req_builder = req_builder.json(&p_body_api_dropbox_oauth2_get_tokens_input);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiDropboxOauth2GetTokensOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiDropboxOauth2GetTokensOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiCreateOauth2DropboxTokensError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To create an OpenAI API key, send a POST request to `/v2/gen-ai/openai/keys`.
pub async fn genai_create_openai_api_key(
    configuration: &configuration::Configuration,
    api_create_open_aiapi_key_input_public: Option<models::ApiCreateOpenAiapiKeyInputPublic>,
) -> Result<models::ApiCreateOpenAiapiKeyOutput, Error<GenaiCreateOpenaiApiKeyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_api_create_open_aiapi_key_input_public = api_create_open_aiapi_key_input_public;

    let uri_str = format!("{}/v2/gen-ai/openai/keys", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_api_create_open_aiapi_key_input_public);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiCreateOpenAiapiKeyOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiCreateOpenAiapiKeyOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiCreateOpenaiApiKeyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To create scheduled indexing for a knowledge base, send a POST request to `/v2/gen-ai/scheduled-indexing`.
pub async fn genai_create_scheduled_indexing(
    configuration: &configuration::Configuration,
    api_create_scheduled_indexing_input_public: Option<
        models::ApiCreateScheduledIndexingInputPublic,
    >,
) -> Result<models::ApiCreateScheduledIndexingOutput, Error<GenaiCreateScheduledIndexingError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_api_create_scheduled_indexing_input_public =
        api_create_scheduled_indexing_input_public;

    let uri_str = format!("{}/v2/gen-ai/scheduled-indexing", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_api_create_scheduled_indexing_input_public);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiCreateScheduledIndexingOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiCreateScheduledIndexingOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiCreateScheduledIndexingError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To create a new workspace, send a POST request to `/v2/gen-ai/workspaces`. The response body contains a JSON object with the newly created workspace object.
pub async fn genai_create_workspace(
    configuration: &configuration::Configuration,
    api_create_workspace_input_public: Option<models::ApiCreateWorkspaceInputPublic>,
) -> Result<models::ApiCreateWorkspaceOutput, Error<GenaiCreateWorkspaceError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_api_create_workspace_input_public = api_create_workspace_input_public;

    let uri_str = format!("{}/v2/gen-ai/workspaces", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_api_create_workspace_input_public);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiCreateWorkspaceOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiCreateWorkspaceOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiCreateWorkspaceError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To delete an agent, send a DELETE request to `/v2/gen-ai/agents/{uuid}`.
pub async fn genai_delete_agent(
    configuration: &configuration::Configuration,
    uuid: &str,
) -> Result<models::ApiDeleteAgentOutput, Error<GenaiDeleteAgentError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_uuid = uuid;

    let uri_str = format!(
        "{}/v2/gen-ai/agents/{uuid}",
        configuration.base_path,
        uuid = crate::apis::urlencode(p_path_uuid)
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiDeleteAgentOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiDeleteAgentOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiDeleteAgentError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To delete an API key for an agent, send a DELETE request to `/v2/gen-ai/agents/{agent_uuid}/api_keys/{api_key_uuid}`.
pub async fn genai_delete_agent_api_key(
    configuration: &configuration::Configuration,
    agent_uuid: &str,
    api_key_uuid: &str,
) -> Result<models::ApiDeleteAgentApiKeyOutput, Error<GenaiDeleteAgentApiKeyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_agent_uuid = agent_uuid;
    let p_path_api_key_uuid = api_key_uuid;

    let uri_str = format!(
        "{}/v2/gen-ai/agents/{agent_uuid}/api_keys/{api_key_uuid}",
        configuration.base_path,
        agent_uuid = crate::apis::urlencode(p_path_agent_uuid),
        api_key_uuid = crate::apis::urlencode(p_path_api_key_uuid)
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiDeleteAgentApiKeyOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiDeleteAgentApiKeyOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiDeleteAgentApiKeyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To delete an Anthropic API key, send a DELETE request to `/v2/gen-ai/anthropic/keys/{api_key_uuid}`.
pub async fn genai_delete_anthropic_api_key(
    configuration: &configuration::Configuration,
    api_key_uuid: &str,
) -> Result<models::ApiDeleteAnthropicApiKeyOutput, Error<GenaiDeleteAnthropicApiKeyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_api_key_uuid = api_key_uuid;

    let uri_str = format!(
        "{}/v2/gen-ai/anthropic/keys/{api_key_uuid}",
        configuration.base_path,
        api_key_uuid = crate::apis::urlencode(p_path_api_key_uuid)
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiDeleteAnthropicApiKeyOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiDeleteAnthropicApiKeyOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiDeleteAnthropicApiKeyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To delete a knowledge base, send a DELETE request to `/v2/gen-ai/knowledge_bases/{uuid}`.
pub async fn genai_delete_knowledge_base(
    configuration: &configuration::Configuration,
    uuid: &str,
) -> Result<models::ApiDeleteKnowledgeBaseOutput, Error<GenaiDeleteKnowledgeBaseError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_uuid = uuid;

    let uri_str = format!(
        "{}/v2/gen-ai/knowledge_bases/{uuid}",
        configuration.base_path,
        uuid = crate::apis::urlencode(p_path_uuid)
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiDeleteKnowledgeBaseOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiDeleteKnowledgeBaseOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiDeleteKnowledgeBaseError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To delete a data source from a knowledge base, send a DELETE request to `/v2/gen-ai/knowledge_bases/{knowledge_base_uuid}/data_sources/{data_source_uuid}`.
pub async fn genai_delete_knowledge_base_data_source(
    configuration: &configuration::Configuration,
    knowledge_base_uuid: &str,
    data_source_uuid: &str,
) -> Result<
    models::ApiDeleteKnowledgeBaseDataSourceOutput,
    Error<GenaiDeleteKnowledgeBaseDataSourceError>,
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_knowledge_base_uuid = knowledge_base_uuid;
    let p_path_data_source_uuid = data_source_uuid;

    let uri_str = format!(
        "{}/v2/gen-ai/knowledge_bases/{knowledge_base_uuid}/data_sources/{data_source_uuid}",
        configuration.base_path,
        knowledge_base_uuid = crate::apis::urlencode(p_path_knowledge_base_uuid),
        data_source_uuid = crate::apis::urlencode(p_path_data_source_uuid)
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiDeleteKnowledgeBaseDataSourceOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiDeleteKnowledgeBaseDataSourceOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiDeleteKnowledgeBaseDataSourceError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To delete an API key for a model, send a DELETE request to `/v2/gen-ai/models/api_keys/{api_key_uuid}`.
pub async fn genai_delete_model_api_key(
    configuration: &configuration::Configuration,
    api_key_uuid: &str,
) -> Result<models::ApiDeleteModelApiKeyOutput, Error<GenaiDeleteModelApiKeyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_api_key_uuid = api_key_uuid;

    let uri_str = format!(
        "{}/v2/gen-ai/models/api_keys/{api_key_uuid}",
        configuration.base_path,
        api_key_uuid = crate::apis::urlencode(p_path_api_key_uuid)
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiDeleteModelApiKeyOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiDeleteModelApiKeyOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiDeleteModelApiKeyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To delete an OpenAI API key, send a DELETE request to `/v2/gen-ai/openai/keys/{api_key_uuid}`.
pub async fn genai_delete_openai_api_key(
    configuration: &configuration::Configuration,
    api_key_uuid: &str,
) -> Result<models::ApiDeleteOpenAiapiKeyOutput, Error<GenaiDeleteOpenaiApiKeyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_api_key_uuid = api_key_uuid;

    let uri_str = format!(
        "{}/v2/gen-ai/openai/keys/{api_key_uuid}",
        configuration.base_path,
        api_key_uuid = crate::apis::urlencode(p_path_api_key_uuid)
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiDeleteOpenAiapiKeyOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiDeleteOpenAiapiKeyOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiDeleteOpenaiApiKeyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Delete Scheduled Indexing for knowledge base, send a DELETE request to `/v2/gen-ai/scheduled-indexing/{uuid}`.
pub async fn genai_delete_scheduled_indexing(
    configuration: &configuration::Configuration,
    uuid: &str,
) -> Result<models::ApiDeleteScheduledIndexingOutput, Error<GenaiDeleteScheduledIndexingError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_uuid = uuid;

    let uri_str = format!(
        "{}/v2/gen-ai/scheduled-indexing/{uuid}",
        configuration.base_path,
        uuid = crate::apis::urlencode(p_path_uuid)
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiDeleteScheduledIndexingOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiDeleteScheduledIndexingOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiDeleteScheduledIndexingError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To delete a workspace, send a DELETE request to `/v2/gen-ai/workspace/{workspace_uuid}`.
pub async fn genai_delete_workspace(
    configuration: &configuration::Configuration,
    workspace_uuid: &str,
) -> Result<models::ApiDeleteWorkspaceOutput, Error<GenaiDeleteWorkspaceError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_workspace_uuid = workspace_uuid;

    let uri_str = format!(
        "{}/v2/gen-ai/workspaces/{workspace_uuid}",
        configuration.base_path,
        workspace_uuid = crate::apis::urlencode(p_path_workspace_uuid)
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiDeleteWorkspaceOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiDeleteWorkspaceOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiDeleteWorkspaceError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To delete an agent route from a parent agent, send a DELETE request to `/v2/gen-ai/agents/{parent_agent_uuid}/child_agents/{child_agent_uuid}`.
pub async fn genai_detach_agent(
    configuration: &configuration::Configuration,
    parent_agent_uuid: &str,
    child_agent_uuid: &str,
) -> Result<models::ApiUnlinkAgentOutput, Error<GenaiDetachAgentError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_parent_agent_uuid = parent_agent_uuid;
    let p_path_child_agent_uuid = child_agent_uuid;

    let uri_str = format!(
        "{}/v2/gen-ai/agents/{parent_agent_uuid}/child_agents/{child_agent_uuid}",
        configuration.base_path,
        parent_agent_uuid = crate::apis::urlencode(p_path_parent_agent_uuid),
        child_agent_uuid = crate::apis::urlencode(p_path_child_agent_uuid)
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiUnlinkAgentOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiUnlinkAgentOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiDetachAgentError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To delete a function route from an agent, send a DELETE request to `/v2/gen-ai/agents/{agent_uuid}/functions/{function_uuid}`.
pub async fn genai_detach_agent_function(
    configuration: &configuration::Configuration,
    agent_uuid: &str,
    function_uuid: &str,
) -> Result<models::ApiUnlinkAgentFunctionOutput, Error<GenaiDetachAgentFunctionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_agent_uuid = agent_uuid;
    let p_path_function_uuid = function_uuid;

    let uri_str = format!(
        "{}/v2/gen-ai/agents/{agent_uuid}/functions/{function_uuid}",
        configuration.base_path,
        agent_uuid = crate::apis::urlencode(p_path_agent_uuid),
        function_uuid = crate::apis::urlencode(p_path_function_uuid)
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiUnlinkAgentFunctionOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiUnlinkAgentFunctionOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiDetachAgentFunctionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To detach a knowledge base from an agent, send a DELETE request to `/v2/gen-ai/agents/{agent_uuid}/knowledge_bases/{knowledge_base_uuid}`.
pub async fn genai_detach_knowledge_base(
    configuration: &configuration::Configuration,
    agent_uuid: &str,
    knowledge_base_uuid: &str,
) -> Result<models::ApiUnlinkKnowledgeBaseOutput, Error<GenaiDetachKnowledgeBaseError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_agent_uuid = agent_uuid;
    let p_path_knowledge_base_uuid = knowledge_base_uuid;

    let uri_str = format!(
        "{}/v2/gen-ai/agents/{agent_uuid}/knowledge_bases/{knowledge_base_uuid}",
        configuration.base_path,
        agent_uuid = crate::apis::urlencode(p_path_agent_uuid),
        knowledge_base_uuid = crate::apis::urlencode(p_path_knowledge_base_uuid)
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiUnlinkKnowledgeBaseOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiUnlinkKnowledgeBaseOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiDetachKnowledgeBaseError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To retrieve details of an agent, GET request to `/v2/gen-ai/agents/{uuid}`. The response body is a JSON object containing the agent.
pub async fn genai_get_agent(
    configuration: &configuration::Configuration,
    uuid: &str,
) -> Result<models::ApiGetAgentOutput, Error<GenaiGetAgentError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_uuid = uuid;

    let uri_str = format!(
        "{}/v2/gen-ai/agents/{uuid}",
        configuration.base_path,
        uuid = crate::apis::urlencode(p_path_uuid)
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiGetAgentOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiGetAgentOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiGetAgentError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To view agent routes for an agent, send a GET requtest to `/v2/gen-ai/agents/{uuid}/child_agents`.
pub async fn genai_get_agent_children(
    configuration: &configuration::Configuration,
    uuid: &str,
) -> Result<models::ApiGetChildrenOutput, Error<GenaiGetAgentChildrenError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_uuid = uuid;

    let uri_str = format!(
        "{}/v2/gen-ai/agents/{uuid}/child_agents",
        configuration.base_path,
        uuid = crate::apis::urlencode(p_path_uuid)
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiGetChildrenOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiGetChildrenOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiGetAgentChildrenError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To get agent usage, send a GET request to `/v2/gen-ai/agents/{uuid}/usage`. Returns usage metrics for the specified agent within the provided time range.
pub async fn genai_get_agent_usage(
    configuration: &configuration::Configuration,
    uuid: &str,
    start: Option<&str>,
    stop: Option<&str>,
) -> Result<models::ApiGetAgentUsageOutput, Error<GenaiGetAgentUsageError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_uuid = uuid;
    let p_query_start = start;
    let p_query_stop = stop;

    let uri_str = format!(
        "{}/v2/gen-ai/agents/{uuid}/usage",
        configuration.base_path,
        uuid = crate::apis::urlencode(p_path_uuid)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_start {
        req_builder = req_builder.query(&[("start", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_stop {
        req_builder = req_builder.query(&[("stop", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiGetAgentUsageOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiGetAgentUsageOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiGetAgentUsageError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To retrieve details of an Anthropic API key, send a GET request to `/v2/gen-ai/anthropic/keys/{api_key_uuid}`.
pub async fn genai_get_anthropic_api_key(
    configuration: &configuration::Configuration,
    api_key_uuid: &str,
) -> Result<models::ApiGetAnthropicApiKeyOutput, Error<GenaiGetAnthropicApiKeyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_api_key_uuid = api_key_uuid;

    let uri_str = format!(
        "{}/v2/gen-ai/anthropic/keys/{api_key_uuid}",
        configuration.base_path,
        api_key_uuid = crate::apis::urlencode(p_path_api_key_uuid)
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiGetAnthropicApiKeyOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiGetAnthropicApiKeyOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiGetAnthropicApiKeyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To retrive information about an existing evaluation run, send a GET request to `/v2/gen-ai/evaluation_runs/{evaluation_run_uuid}`.
pub async fn genai_get_evaluation_run(
    configuration: &configuration::Configuration,
    evaluation_run_uuid: &str,
) -> Result<models::ApiGetEvaluationRunOutput, Error<GenaiGetEvaluationRunError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_evaluation_run_uuid = evaluation_run_uuid;

    let uri_str = format!(
        "{}/v2/gen-ai/evaluation_runs/{evaluation_run_uuid}",
        configuration.base_path,
        evaluation_run_uuid = crate::apis::urlencode(p_path_evaluation_run_uuid)
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiGetEvaluationRunOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiGetEvaluationRunOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiGetEvaluationRunError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To retrieve results of an evaluation run, send a GET request to `/v2/gen-ai/evaluation_runs/{evaluation_run_uuid}/results/{prompt_id}`.
pub async fn genai_get_evaluation_run_prompt_results(
    configuration: &configuration::Configuration,
    evaluation_run_uuid: &str,
    prompt_id: i32,
) -> Result<
    models::ApiGetEvaluationRunPromptResultsOutput,
    Error<GenaiGetEvaluationRunPromptResultsError>,
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_evaluation_run_uuid = evaluation_run_uuid;
    let p_path_prompt_id = prompt_id;

    let uri_str = format!(
        "{}/v2/gen-ai/evaluation_runs/{evaluation_run_uuid}/results/{prompt_id}",
        configuration.base_path,
        evaluation_run_uuid = crate::apis::urlencode(p_path_evaluation_run_uuid),
        prompt_id = p_path_prompt_id
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiGetEvaluationRunPromptResultsOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiGetEvaluationRunPromptResultsOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiGetEvaluationRunPromptResultsError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To retrieve results of an evaluation run, send a GET request to `/v2/gen-ai/evaluation_runs/{evaluation_run_uuid}/results`.
pub async fn genai_get_evaluation_run_results(
    configuration: &configuration::Configuration,
    evaluation_run_uuid: &str,
    page: Option<i32>,
    per_page: Option<i32>,
) -> Result<models::ApiGetEvaluationRunResultsOutput, Error<GenaiGetEvaluationRunResultsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_evaluation_run_uuid = evaluation_run_uuid;
    let p_query_page = page;
    let p_query_per_page = per_page;

    let uri_str = format!(
        "{}/v2/gen-ai/evaluation_runs/{evaluation_run_uuid}/results",
        configuration.base_path,
        evaluation_run_uuid = crate::apis::urlencode(p_path_evaluation_run_uuid)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_per_page {
        req_builder = req_builder.query(&[("per_page", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiGetEvaluationRunResultsOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiGetEvaluationRunResultsOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiGetEvaluationRunResultsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To retrive information about an existing evaluation test case, send a GET request to `/v2/gen-ai/evaluation_test_case/{test_case_uuid}`.
pub async fn genai_get_evaluation_test_case(
    configuration: &configuration::Configuration,
    test_case_uuid: &str,
    evaluation_test_case_version: Option<i32>,
) -> Result<models::ApiGetEvaluationTestCaseOutput, Error<GenaiGetEvaluationTestCaseError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_test_case_uuid = test_case_uuid;
    let p_query_evaluation_test_case_version = evaluation_test_case_version;

    let uri_str = format!(
        "{}/v2/gen-ai/evaluation_test_cases/{test_case_uuid}",
        configuration.base_path,
        test_case_uuid = crate::apis::urlencode(p_path_test_case_uuid)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_evaluation_test_case_version {
        req_builder =
            req_builder.query(&[("evaluation_test_case_version", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiGetEvaluationTestCaseOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiGetEvaluationTestCaseOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiGetEvaluationTestCaseError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To get status of an indexing Job for a knowledge base, send a GET request to `/v2/gen-ai/indexing_jobs/{uuid}`.
pub async fn genai_get_indexing_job(
    configuration: &configuration::Configuration,
    uuid: &str,
) -> Result<models::ApiGetKnowledgeBaseIndexingJobOutput, Error<GenaiGetIndexingJobError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_uuid = uuid;

    let uri_str = format!(
        "{}/v2/gen-ai/indexing_jobs/{uuid}",
        configuration.base_path,
        uuid = crate::apis::urlencode(p_path_uuid)
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiGetKnowledgeBaseIndexingJobOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiGetKnowledgeBaseIndexingJobOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiGetIndexingJobError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To get a signed URL for indexing job details, send a GET request to `/v2/gen-ai/indexing_jobs/{uuid}/details_signed_url`.
pub async fn genai_get_indexing_job_details_signed_url(
    configuration: &configuration::Configuration,
    indexing_job_uuid: &str,
) -> Result<
    models::ApiGetIndexingJobDetailsSignedUrlOutput,
    Error<GenaiGetIndexingJobDetailsSignedUrlError>,
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_indexing_job_uuid = indexing_job_uuid;

    let uri_str = format!(
        "{}/v2/gen-ai/indexing_jobs/{indexing_job_uuid}/details_signed_url",
        configuration.base_path,
        indexing_job_uuid = crate::apis::urlencode(p_path_indexing_job_uuid)
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiGetIndexingJobDetailsSignedUrlOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiGetIndexingJobDetailsSignedUrlOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiGetIndexingJobDetailsSignedUrlError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To retrive information about an existing knowledge base, send a GET request to `/v2/gen-ai/knowledge_bases/{uuid}`.
pub async fn genai_get_knowledge_base(
    configuration: &configuration::Configuration,
    uuid: &str,
) -> Result<models::ApiGetKnowledgeBaseOutput, Error<GenaiGetKnowledgeBaseError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_uuid = uuid;

    let uri_str = format!(
        "{}/v2/gen-ai/knowledge_bases/{uuid}",
        configuration.base_path,
        uuid = crate::apis::urlencode(p_path_uuid)
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiGetKnowledgeBaseOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiGetKnowledgeBaseOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiGetKnowledgeBaseError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To generate an Oauth2-URL for use with your localhost, send a GET request to `/v2/gen-ai/oauth2/url`. Pass 'http://localhost:3000 as redirect_url
pub async fn genai_get_oauth2_url(
    configuration: &configuration::Configuration,
    r#type: Option<&str>,
    redirect_url: Option<&str>,
) -> Result<models::ApiGenerateOauth2UrlOutput, Error<GenaiGetOauth2UrlError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_type = r#type;
    let p_query_redirect_url = redirect_url;

    let uri_str = format!("{}/v2/gen-ai/oauth2/url", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_type {
        req_builder = req_builder.query(&[("type", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_redirect_url {
        req_builder = req_builder.query(&[("redirect_url", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiGenerateOauth2UrlOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiGenerateOauth2UrlOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiGetOauth2UrlError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To retrieve details of an OpenAI API key, send a GET request to `/v2/gen-ai/openai/keys/{api_key_uuid}`.
pub async fn genai_get_openai_api_key(
    configuration: &configuration::Configuration,
    api_key_uuid: &str,
) -> Result<models::ApiGetOpenAiapiKeyOutput, Error<GenaiGetOpenaiApiKeyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_api_key_uuid = api_key_uuid;

    let uri_str = format!(
        "{}/v2/gen-ai/openai/keys/{api_key_uuid}",
        configuration.base_path,
        api_key_uuid = crate::apis::urlencode(p_path_api_key_uuid)
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiGetOpenAiapiKeyOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiGetOpenAiapiKeyOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiGetOpenaiApiKeyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Get Scheduled Indexing for knowledge base using knoweldge base uuid, send a GET request to `/v2/gen-ai/scheduled-indexing/knowledge-base/{knowledge_base_uuid}`.
pub async fn genai_get_scheduled_indexing(
    configuration: &configuration::Configuration,
    knowledge_base_uuid: &str,
) -> Result<models::ApiGetScheduledIndexingOutput, Error<GenaiGetScheduledIndexingError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_knowledge_base_uuid = knowledge_base_uuid;

    let uri_str = format!(
        "{}/v2/gen-ai/scheduled-indexing/knowledge-base/{knowledge_base_uuid}",
        configuration.base_path,
        knowledge_base_uuid = crate::apis::urlencode(p_path_knowledge_base_uuid)
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiGetScheduledIndexingOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiGetScheduledIndexingOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiGetScheduledIndexingError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To retrieve details of a workspace, GET request to `/v2/gen-ai/workspaces/{workspace_uuid}`. The response body is a JSON object containing the workspace.
pub async fn genai_get_workspace(
    configuration: &configuration::Configuration,
    workspace_uuid: &str,
) -> Result<models::ApiGetWorkspaceOutput, Error<GenaiGetWorkspaceError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_workspace_uuid = workspace_uuid;

    let uri_str = format!(
        "{}/v2/gen-ai/workspaces/{workspace_uuid}",
        configuration.base_path,
        workspace_uuid = crate::apis::urlencode(p_path_workspace_uuid)
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiGetWorkspaceOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiGetWorkspaceOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiGetWorkspaceError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To list all agent API keys, send a GET request to `/v2/gen-ai/agents/{agent_uuid}/api_keys`.
pub async fn genai_list_agent_api_keys(
    configuration: &configuration::Configuration,
    agent_uuid: &str,
    page: Option<i32>,
    per_page: Option<i32>,
) -> Result<models::ApiListAgentApiKeysOutput, Error<GenaiListAgentApiKeysError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_agent_uuid = agent_uuid;
    let p_query_page = page;
    let p_query_per_page = per_page;

    let uri_str = format!(
        "{}/v2/gen-ai/agents/{agent_uuid}/api_keys",
        configuration.base_path,
        agent_uuid = crate::apis::urlencode(p_path_agent_uuid)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_per_page {
        req_builder = req_builder.query(&[("per_page", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiListAgentApiKeysOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiListAgentApiKeysOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiListAgentApiKeysError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To list all agent versions, send a GET request to `/v2/gen-ai/agents/{uuid}/versions`.
pub async fn genai_list_agent_versions(
    configuration: &configuration::Configuration,
    uuid: &str,
    page: Option<i32>,
    per_page: Option<i32>,
) -> Result<models::ApiListAgentVersionsOutput, Error<GenaiListAgentVersionsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_uuid = uuid;
    let p_query_page = page;
    let p_query_per_page = per_page;

    let uri_str = format!(
        "{}/v2/gen-ai/agents/{uuid}/versions",
        configuration.base_path,
        uuid = crate::apis::urlencode(p_path_uuid)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_per_page {
        req_builder = req_builder.query(&[("per_page", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiListAgentVersionsOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiListAgentVersionsOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiListAgentVersionsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To list all agents, send a GET request to `/v2/gen-ai/agents`.
pub async fn genai_list_agents(
    configuration: &configuration::Configuration,
    only_deployed: Option<bool>,
    page: Option<i32>,
    per_page: Option<i32>,
) -> Result<models::ApiListAgentsOutputPublic, Error<GenaiListAgentsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_only_deployed = only_deployed;
    let p_query_page = page;
    let p_query_per_page = per_page;

    let uri_str = format!("{}/v2/gen-ai/agents", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_only_deployed {
        req_builder = req_builder.query(&[("only_deployed", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_per_page {
        req_builder = req_builder.query(&[("per_page", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiListAgentsOutputPublic`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiListAgentsOutputPublic`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiListAgentsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// List Agents by Anthropic Key.
pub async fn genai_list_agents_by_anthropic_key(
    configuration: &configuration::Configuration,
    uuid: &str,
    page: Option<i32>,
    per_page: Option<i32>,
) -> Result<models::ApiListAgentsByAnthropicKeyOutput, Error<GenaiListAgentsByAnthropicKeyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_uuid = uuid;
    let p_query_page = page;
    let p_query_per_page = per_page;

    let uri_str = format!(
        "{}/v2/gen-ai/anthropic/keys/{uuid}/agents",
        configuration.base_path,
        uuid = crate::apis::urlencode(p_path_uuid)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_per_page {
        req_builder = req_builder.query(&[("per_page", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiListAgentsByAnthropicKeyOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiListAgentsByAnthropicKeyOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiListAgentsByAnthropicKeyError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// List Agents by OpenAI Key.
pub async fn genai_list_agents_by_openai_key(
    configuration: &configuration::Configuration,
    uuid: &str,
    page: Option<i32>,
    per_page: Option<i32>,
) -> Result<models::ApiListAgentsByOpenAiKeyOutput, Error<GenaiListAgentsByOpenaiKeyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_uuid = uuid;
    let p_query_page = page;
    let p_query_per_page = per_page;

    let uri_str = format!(
        "{}/v2/gen-ai/openai/keys/{uuid}/agents",
        configuration.base_path,
        uuid = crate::apis::urlencode(p_path_uuid)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_per_page {
        req_builder = req_builder.query(&[("per_page", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiListAgentsByOpenAiKeyOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiListAgentsByOpenAiKeyOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiListAgentsByOpenaiKeyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To list all agents by a Workspace, send a GET request to `/v2/gen-ai/workspaces/{workspace_uuid}/agents`.
pub async fn genai_list_agents_by_workspace(
    configuration: &configuration::Configuration,
    workspace_uuid: &str,
    only_deployed: Option<bool>,
    page: Option<i32>,
    per_page: Option<i32>,
) -> Result<models::ApiListAgentsByWorkspaceOutput, Error<GenaiListAgentsByWorkspaceError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_workspace_uuid = workspace_uuid;
    let p_query_only_deployed = only_deployed;
    let p_query_page = page;
    let p_query_per_page = per_page;

    let uri_str = format!(
        "{}/v2/gen-ai/workspaces/{workspace_uuid}/agents",
        configuration.base_path,
        workspace_uuid = crate::apis::urlencode(p_path_workspace_uuid)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_only_deployed {
        req_builder = req_builder.query(&[("only_deployed", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_per_page {
        req_builder = req_builder.query(&[("per_page", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiListAgentsByWorkspaceOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiListAgentsByWorkspaceOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiListAgentsByWorkspaceError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To list all Anthropic API keys, send a GET request to `/v2/gen-ai/anthropic/keys`.
pub async fn genai_list_anthropic_api_keys(
    configuration: &configuration::Configuration,
    page: Option<i32>,
    per_page: Option<i32>,
) -> Result<models::ApiListAnthropicApiKeysOutput, Error<GenaiListAnthropicApiKeysError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_page = page;
    let p_query_per_page = per_page;

    let uri_str = format!("{}/v2/gen-ai/anthropic/keys", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_per_page {
        req_builder = req_builder.query(&[("per_page", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiListAnthropicApiKeysOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiListAnthropicApiKeysOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiListAnthropicApiKeysError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To list all datacenter regions, send a GET request to `/v2/gen-ai/regions`.
pub async fn genai_list_datacenter_regions(
    configuration: &configuration::Configuration,
    serves_inference: Option<bool>,
    serves_batch: Option<bool>,
) -> Result<models::ApiListRegionsOutput, Error<GenaiListDatacenterRegionsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_serves_inference = serves_inference;
    let p_query_serves_batch = serves_batch;

    let uri_str = format!("{}/v2/gen-ai/regions", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_serves_inference {
        req_builder = req_builder.query(&[("serves_inference", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_serves_batch {
        req_builder = req_builder.query(&[("serves_batch", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiListRegionsOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiListRegionsOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiListDatacenterRegionsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To list all evaluation metrics, send a GET request to `/v2/gen-ai/evaluation_metrics`.
pub async fn genai_list_evaluation_metrics(
    configuration: &configuration::Configuration,
) -> Result<models::ApiListEvaluationMetricsOutput, Error<GenaiListEvaluationMetricsError>> {
    let uri_str = format!("{}/v2/gen-ai/evaluation_metrics", configuration.base_path);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiListEvaluationMetricsOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiListEvaluationMetricsOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiListEvaluationMetricsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To list all evaluation runs by test case, send a GET request to `/v2/gen-ai/evaluation_test_cases/{evaluation_test_case_uuid}/evaluation_runs`.
pub async fn genai_list_evaluation_runs_by_test_case(
    configuration: &configuration::Configuration,
    evaluation_test_case_uuid: &str,
    evaluation_test_case_version: Option<i32>,
) -> Result<
    models::ApiListEvaluationRunsByTestCaseOutput,
    Error<GenaiListEvaluationRunsByTestCaseError>,
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_evaluation_test_case_uuid = evaluation_test_case_uuid;
    let p_query_evaluation_test_case_version = evaluation_test_case_version;

    let uri_str = format!(
        "{}/v2/gen-ai/evaluation_test_cases/{evaluation_test_case_uuid}/evaluation_runs",
        configuration.base_path,
        evaluation_test_case_uuid = crate::apis::urlencode(p_path_evaluation_test_case_uuid)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_evaluation_test_case_version {
        req_builder =
            req_builder.query(&[("evaluation_test_case_version", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiListEvaluationRunsByTestCaseOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiListEvaluationRunsByTestCaseOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiListEvaluationRunsByTestCaseError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To list all evaluation test cases, send a GET request to `/v2/gen-ai/evaluation_test_cases`.
pub async fn genai_list_evaluation_test_cases(
    configuration: &configuration::Configuration,
) -> Result<models::ApiListEvaluationTestCasesOutput, Error<GenaiListEvaluationTestCasesError>> {
    let uri_str = format!(
        "{}/v2/gen-ai/evaluation_test_cases",
        configuration.base_path
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiListEvaluationTestCasesOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiListEvaluationTestCasesOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiListEvaluationTestCasesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To list all evaluation test cases by a workspace, send a GET request to `/v2/gen-ai/workspaces/{workspace_uuid}/evaluation_test_cases`.
pub async fn genai_list_evaluation_test_cases_by_workspace(
    configuration: &configuration::Configuration,
    workspace_uuid: &str,
) -> Result<
    models::ApiListEvaluationTestCasesByWorkspaceOutput,
    Error<GenaiListEvaluationTestCasesByWorkspaceError>,
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_workspace_uuid = workspace_uuid;

    let uri_str = format!(
        "{}/v2/gen-ai/workspaces/{workspace_uuid}/evaluation_test_cases",
        configuration.base_path,
        workspace_uuid = crate::apis::urlencode(p_path_workspace_uuid)
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiListEvaluationTestCasesByWorkspaceOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiListEvaluationTestCasesByWorkspaceOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiListEvaluationTestCasesByWorkspaceError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To list all datasources for an indexing job, send a GET request to `/v2/gen-ai/indexing_jobs/{indexing_job_uuid}/data_sources`.
pub async fn genai_list_indexing_job_data_sources(
    configuration: &configuration::Configuration,
    indexing_job_uuid: &str,
) -> Result<models::ApiListIndexingJobDataSourcesOutput, Error<GenaiListIndexingJobDataSourcesError>>
{
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_indexing_job_uuid = indexing_job_uuid;

    let uri_str = format!(
        "{}/v2/gen-ai/indexing_jobs/{indexing_job_uuid}/data_sources",
        configuration.base_path,
        indexing_job_uuid = crate::apis::urlencode(p_path_indexing_job_uuid)
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiListIndexingJobDataSourcesOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiListIndexingJobDataSourcesOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiListIndexingJobDataSourcesError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To list all indexing jobs for a knowledge base, send a GET request to `/v2/gen-ai/indexing_jobs`.
pub async fn genai_list_indexing_jobs(
    configuration: &configuration::Configuration,
    page: Option<i32>,
    per_page: Option<i32>,
) -> Result<models::ApiListKnowledgeBaseIndexingJobsOutput, Error<GenaiListIndexingJobsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_page = page;
    let p_query_per_page = per_page;

    let uri_str = format!("{}/v2/gen-ai/indexing_jobs", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_per_page {
        req_builder = req_builder.query(&[("per_page", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiListKnowledgeBaseIndexingJobsOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiListKnowledgeBaseIndexingJobsOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiListIndexingJobsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To list latest 15 indexing jobs for a knowledge base, send a GET request to `/v2/gen-ai/knowledge_bases/{knowledge_base_uuid}/indexing_jobs`.
pub async fn genai_list_indexing_jobs_by_knowledge_base(
    configuration: &configuration::Configuration,
    knowledge_base_uuid: &str,
) -> Result<
    models::ApiListKnowledgeBaseIndexingJobsOutput,
    Error<GenaiListIndexingJobsByKnowledgeBaseError>,
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_knowledge_base_uuid = knowledge_base_uuid;

    let uri_str = format!(
        "{}/v2/gen-ai/knowledge_bases/{knowledge_base_uuid}/indexing_jobs",
        configuration.base_path,
        knowledge_base_uuid = crate::apis::urlencode(p_path_knowledge_base_uuid)
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiListKnowledgeBaseIndexingJobsOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiListKnowledgeBaseIndexingJobsOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiListIndexingJobsByKnowledgeBaseError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To list all data sources for a knowledge base, send a GET request to `/v2/gen-ai/knowledge_bases/{knowledge_base_uuid}/data_sources`.
pub async fn genai_list_knowledge_base_data_sources(
    configuration: &configuration::Configuration,
    knowledge_base_uuid: &str,
    page: Option<i32>,
    per_page: Option<i32>,
) -> Result<
    models::ApiListKnowledgeBaseDataSourcesOutput,
    Error<GenaiListKnowledgeBaseDataSourcesError>,
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_knowledge_base_uuid = knowledge_base_uuid;
    let p_query_page = page;
    let p_query_per_page = per_page;

    let uri_str = format!(
        "{}/v2/gen-ai/knowledge_bases/{knowledge_base_uuid}/data_sources",
        configuration.base_path,
        knowledge_base_uuid = crate::apis::urlencode(p_path_knowledge_base_uuid)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_per_page {
        req_builder = req_builder.query(&[("per_page", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiListKnowledgeBaseDataSourcesOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiListKnowledgeBaseDataSourcesOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiListKnowledgeBaseDataSourcesError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To list all knowledge bases, send a GET request to `/v2/gen-ai/knowledge_bases`.
pub async fn genai_list_knowledge_bases(
    configuration: &configuration::Configuration,
    page: Option<i32>,
    per_page: Option<i32>,
) -> Result<models::ApiListKnowledgeBasesOutput, Error<GenaiListKnowledgeBasesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_page = page;
    let p_query_per_page = per_page;

    let uri_str = format!("{}/v2/gen-ai/knowledge_bases", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_per_page {
        req_builder = req_builder.query(&[("per_page", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiListKnowledgeBasesOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiListKnowledgeBasesOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiListKnowledgeBasesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To list all model API keys, send a GET request to `/v2/gen-ai/models/api_keys`.
pub async fn genai_list_model_api_keys(
    configuration: &configuration::Configuration,
    page: Option<i32>,
    per_page: Option<i32>,
) -> Result<models::ApiListModelApiKeysOutput, Error<GenaiListModelApiKeysError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_page = page;
    let p_query_per_page = per_page;

    let uri_str = format!("{}/v2/gen-ai/models/api_keys", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_per_page {
        req_builder = req_builder.query(&[("per_page", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiListModelApiKeysOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiListModelApiKeysOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiListModelApiKeysError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To list all models, send a GET request to `/v2/gen-ai/models`.
pub async fn genai_list_models(
    configuration: &configuration::Configuration,
    usecases: Option<Vec<String>>,
    public_only: Option<bool>,
    page: Option<i32>,
    per_page: Option<i32>,
) -> Result<models::ApiListModelsOutputPublic, Error<GenaiListModelsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_usecases = usecases;
    let p_query_public_only = public_only;
    let p_query_page = page;
    let p_query_per_page = per_page;

    let uri_str = format!("{}/v2/gen-ai/models", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_usecases {
        req_builder = match "multi" {
            "multi" => req_builder.query(
                &param_value
                    .into_iter()
                    .map(|p| ("usecases".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "usecases",
                &param_value
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref param_value) = p_query_public_only {
        req_builder = req_builder.query(&[("public_only", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_per_page {
        req_builder = req_builder.query(&[("per_page", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiListModelsOutputPublic`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiListModelsOutputPublic`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiListModelsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To list all OpenAI API keys, send a GET request to `/v2/gen-ai/openai/keys`.
pub async fn genai_list_openai_api_keys(
    configuration: &configuration::Configuration,
    page: Option<i32>,
    per_page: Option<i32>,
) -> Result<models::ApiListOpenAiapiKeysOutput, Error<GenaiListOpenaiApiKeysError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_page = page;
    let p_query_per_page = per_page;

    let uri_str = format!("{}/v2/gen-ai/openai/keys", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_per_page {
        req_builder = req_builder.query(&[("per_page", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiListOpenAiapiKeysOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiListOpenAiapiKeysOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiListOpenaiApiKeysError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To list all workspaces, send a GET request to `/v2/gen-ai/workspaces`.
pub async fn genai_list_workspaces(
    configuration: &configuration::Configuration,
) -> Result<models::ApiListWorkspacesOutput, Error<GenaiListWorkspacesError>> {
    let uri_str = format!("{}/v2/gen-ai/workspaces", configuration.base_path);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiListWorkspacesOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiListWorkspacesOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiListWorkspacesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To regenerate an agent API key, send a PUT request to `/v2/gen-ai/agents/{agent_uuid}/api_keys/{api_key_uuid}/regenerate`.
pub async fn genai_regenerate_agent_api_key(
    configuration: &configuration::Configuration,
    agent_uuid: &str,
    api_key_uuid: &str,
) -> Result<models::ApiRegenerateAgentApiKeyOutput, Error<GenaiRegenerateAgentApiKeyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_agent_uuid = agent_uuid;
    let p_path_api_key_uuid = api_key_uuid;

    let uri_str = format!(
        "{}/v2/gen-ai/agents/{agent_uuid}/api_keys/{api_key_uuid}/regenerate",
        configuration.base_path,
        agent_uuid = crate::apis::urlencode(p_path_agent_uuid),
        api_key_uuid = crate::apis::urlencode(p_path_api_key_uuid)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiRegenerateAgentApiKeyOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiRegenerateAgentApiKeyOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiRegenerateAgentApiKeyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To regenerate a model API key, send a PUT request to `/v2/gen-ai/models/api_keys/{api_key_uuid}/regenerate`.
pub async fn genai_regenerate_model_api_key(
    configuration: &configuration::Configuration,
    api_key_uuid: &str,
) -> Result<models::ApiRegenerateModelApiKeyOutput, Error<GenaiRegenerateModelApiKeyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_api_key_uuid = api_key_uuid;

    let uri_str = format!(
        "{}/v2/gen-ai/models/api_keys/{api_key_uuid}/regenerate",
        configuration.base_path,
        api_key_uuid = crate::apis::urlencode(p_path_api_key_uuid)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiRegenerateModelApiKeyOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiRegenerateModelApiKeyOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiRegenerateModelApiKeyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To update to a specific agent version, send a PUT request to `/v2/gen-ai/agents/{uuid}/versions`.
pub async fn genai_rollback_to_agent_version(
    configuration: &configuration::Configuration,
    uuid: &str,
    api_rollback_to_agent_version_input_public: Option<
        models::ApiRollbackToAgentVersionInputPublic,
    >,
) -> Result<models::ApiRollbackToAgentVersionOutput, Error<GenaiRollbackToAgentVersionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_uuid = uuid;
    let p_body_api_rollback_to_agent_version_input_public =
        api_rollback_to_agent_version_input_public;

    let uri_str = format!(
        "{}/v2/gen-ai/agents/{uuid}/versions",
        configuration.base_path,
        uuid = crate::apis::urlencode(p_path_uuid)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_api_rollback_to_agent_version_input_public);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiRollbackToAgentVersionOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiRollbackToAgentVersionOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiRollbackToAgentVersionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To run an evaluation test case, send a POST request to `/v2/gen-ai/evaluation_runs`.
pub async fn genai_run_evaluation_test_case(
    configuration: &configuration::Configuration,
    api_run_evaluation_test_case_input_public: Option<models::ApiRunEvaluationTestCaseInputPublic>,
) -> Result<models::ApiRunEvaluationTestCaseOutput, Error<GenaiRunEvaluationTestCaseError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_api_run_evaluation_test_case_input_public =
        api_run_evaluation_test_case_input_public;

    let uri_str = format!("{}/v2/gen-ai/evaluation_runs", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_api_run_evaluation_test_case_input_public);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiRunEvaluationTestCaseOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiRunEvaluationTestCaseOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiRunEvaluationTestCaseError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To update an agent, send a PUT request to `/v2/gen-ai/agents/{uuid}`. The response body is a JSON object containing the agent.
pub async fn genai_update_agent(
    configuration: &configuration::Configuration,
    uuid: &str,
    api_update_agent_input_public: Option<models::ApiUpdateAgentInputPublic>,
) -> Result<models::ApiUpdateAgentOutput, Error<GenaiUpdateAgentError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_uuid = uuid;
    let p_body_api_update_agent_input_public = api_update_agent_input_public;

    let uri_str = format!(
        "{}/v2/gen-ai/agents/{uuid}",
        configuration.base_path,
        uuid = crate::apis::urlencode(p_path_uuid)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_api_update_agent_input_public);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiUpdateAgentOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiUpdateAgentOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiUpdateAgentError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To update an agent API key, send a PUT request to `/v2/gen-ai/agents/{agent_uuid}/api_keys/{api_key_uuid}`.
pub async fn genai_update_agent_api_key(
    configuration: &configuration::Configuration,
    agent_uuid: &str,
    api_key_uuid: &str,
    api_update_agent_api_key_input_public: Option<models::ApiUpdateAgentApiKeyInputPublic>,
) -> Result<models::ApiUpdateAgentApiKeyOutput, Error<GenaiUpdateAgentApiKeyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_agent_uuid = agent_uuid;
    let p_path_api_key_uuid = api_key_uuid;
    let p_body_api_update_agent_api_key_input_public = api_update_agent_api_key_input_public;

    let uri_str = format!(
        "{}/v2/gen-ai/agents/{agent_uuid}/api_keys/{api_key_uuid}",
        configuration.base_path,
        agent_uuid = crate::apis::urlencode(p_path_agent_uuid),
        api_key_uuid = crate::apis::urlencode(p_path_api_key_uuid)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_api_update_agent_api_key_input_public);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiUpdateAgentApiKeyOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiUpdateAgentApiKeyOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiUpdateAgentApiKeyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Check whether an agent is public or private. To update the agent status, send a PUT request to `/v2/gen-ai/agents/{uuid}/deployment_visibility`.
pub async fn genai_update_agent_deployment_visibility(
    configuration: &configuration::Configuration,
    uuid: &str,
    api_update_agent_deployment_visibility_input_public: Option<
        models::ApiUpdateAgentDeploymentVisibilityInputPublic,
    >,
) -> Result<
    models::ApiUpdateAgentDeploymentVisbilityOutput,
    Error<GenaiUpdateAgentDeploymentVisibilityError>,
> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_uuid = uuid;
    let p_body_api_update_agent_deployment_visibility_input_public =
        api_update_agent_deployment_visibility_input_public;

    let uri_str = format!(
        "{}/v2/gen-ai/agents/{uuid}/deployment_visibility",
        configuration.base_path,
        uuid = crate::apis::urlencode(p_path_uuid)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_api_update_agent_deployment_visibility_input_public);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiUpdateAgentDeploymentVisbilityOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiUpdateAgentDeploymentVisbilityOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiUpdateAgentDeploymentVisibilityError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To update the function route, send a PUT request to `/v2/gen-ai/agents/{agent_uuid}/functions/{function_uuid}`.
pub async fn genai_update_agent_function(
    configuration: &configuration::Configuration,
    agent_uuid: &str,
    function_uuid: &str,
    api_update_agent_function_input_public: Option<models::ApiUpdateAgentFunctionInputPublic>,
) -> Result<models::ApiUpdateAgentFunctionOutput, Error<GenaiUpdateAgentFunctionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_agent_uuid = agent_uuid;
    let p_path_function_uuid = function_uuid;
    let p_body_api_update_agent_function_input_public = api_update_agent_function_input_public;

    let uri_str = format!(
        "{}/v2/gen-ai/agents/{agent_uuid}/functions/{function_uuid}",
        configuration.base_path,
        agent_uuid = crate::apis::urlencode(p_path_agent_uuid),
        function_uuid = crate::apis::urlencode(p_path_function_uuid)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_api_update_agent_function_input_public);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiUpdateAgentFunctionOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiUpdateAgentFunctionOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiUpdateAgentFunctionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To move all listed agents a given workspace, send a PUT request to `/v2/gen-ai/workspaces/{workspace_uuid}/agents`.
pub async fn genai_update_agents_workspace(
    configuration: &configuration::Configuration,
    workspace_uuid: &str,
    api_move_agents_to_workspace_input_public: Option<models::ApiMoveAgentsToWorkspaceInputPublic>,
) -> Result<models::ApiMoveAgentsToWorkspaceOutput, Error<GenaiUpdateAgentsWorkspaceError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_workspace_uuid = workspace_uuid;
    let p_body_api_move_agents_to_workspace_input_public =
        api_move_agents_to_workspace_input_public;

    let uri_str = format!(
        "{}/v2/gen-ai/workspaces/{workspace_uuid}/agents",
        configuration.base_path,
        workspace_uuid = crate::apis::urlencode(p_path_workspace_uuid)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_api_move_agents_to_workspace_input_public);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiMoveAgentsToWorkspaceOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiMoveAgentsToWorkspaceOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiUpdateAgentsWorkspaceError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To update an Anthropic API key, send a PUT request to `/v2/gen-ai/anthropic/keys/{api_key_uuid}`.
pub async fn genai_update_anthropic_api_key(
    configuration: &configuration::Configuration,
    api_key_uuid: &str,
    api_update_anthropic_api_key_input_public: Option<models::ApiUpdateAnthropicApiKeyInputPublic>,
) -> Result<models::ApiUpdateAnthropicApiKeyOutput, Error<GenaiUpdateAnthropicApiKeyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_api_key_uuid = api_key_uuid;
    let p_body_api_update_anthropic_api_key_input_public =
        api_update_anthropic_api_key_input_public;

    let uri_str = format!(
        "{}/v2/gen-ai/anthropic/keys/{api_key_uuid}",
        configuration.base_path,
        api_key_uuid = crate::apis::urlencode(p_path_api_key_uuid)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_api_update_anthropic_api_key_input_public);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiUpdateAnthropicApiKeyOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiUpdateAnthropicApiKeyOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiUpdateAnthropicApiKeyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To update an agent route for an agent, send a PUT request to `/v2/gen-ai/agents/{parent_agent_uuid}/child_agents/{child_agent_uuid}`.
pub async fn genai_update_attached_agent(
    configuration: &configuration::Configuration,
    parent_agent_uuid: &str,
    child_agent_uuid: &str,
    api_update_linked_agent_input_public: Option<models::ApiUpdateLinkedAgentInputPublic>,
) -> Result<models::ApiUpdateLinkedAgentOutput, Error<GenaiUpdateAttachedAgentError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_parent_agent_uuid = parent_agent_uuid;
    let p_path_child_agent_uuid = child_agent_uuid;
    let p_body_api_update_linked_agent_input_public = api_update_linked_agent_input_public;

    let uri_str = format!(
        "{}/v2/gen-ai/agents/{parent_agent_uuid}/child_agents/{child_agent_uuid}",
        configuration.base_path,
        parent_agent_uuid = crate::apis::urlencode(p_path_parent_agent_uuid),
        child_agent_uuid = crate::apis::urlencode(p_path_child_agent_uuid)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_api_update_linked_agent_input_public);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiUpdateLinkedAgentOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiUpdateLinkedAgentOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiUpdateAttachedAgentError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To update an evaluation test-case send a PUT request to `/v2/gen-ai/evaluation_test_cases/{test_case_uuid}`.
pub async fn genai_update_evaluation_test_case(
    configuration: &configuration::Configuration,
    test_case_uuid: &str,
    api_update_evaluation_test_case_input_public: Option<
        models::ApiUpdateEvaluationTestCaseInputPublic,
    >,
) -> Result<models::ApiUpdateEvaluationTestCaseOutput, Error<GenaiUpdateEvaluationTestCaseError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_test_case_uuid = test_case_uuid;
    let p_body_api_update_evaluation_test_case_input_public =
        api_update_evaluation_test_case_input_public;

    let uri_str = format!(
        "{}/v2/gen-ai/evaluation_test_cases/{test_case_uuid}",
        configuration.base_path,
        test_case_uuid = crate::apis::urlencode(p_path_test_case_uuid)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_api_update_evaluation_test_case_input_public);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiUpdateEvaluationTestCaseOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiUpdateEvaluationTestCaseOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiUpdateEvaluationTestCaseError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To update a knowledge base, send a PUT request to `/v2/gen-ai/knowledge_bases/{uuid}`.
pub async fn genai_update_knowledge_base(
    configuration: &configuration::Configuration,
    uuid: &str,
    api_update_knowledge_base_input_public: Option<models::ApiUpdateKnowledgeBaseInputPublic>,
) -> Result<models::ApiUpdateKnowledgeBaseOutput, Error<GenaiUpdateKnowledgeBaseError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_uuid = uuid;
    let p_body_api_update_knowledge_base_input_public = api_update_knowledge_base_input_public;

    let uri_str = format!(
        "{}/v2/gen-ai/knowledge_bases/{uuid}",
        configuration.base_path,
        uuid = crate::apis::urlencode(p_path_uuid)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_api_update_knowledge_base_input_public);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiUpdateKnowledgeBaseOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiUpdateKnowledgeBaseOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiUpdateKnowledgeBaseError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To update a model API key, send a PUT request to `/v2/gen-ai/models/api_keys/{api_key_uuid}`.
pub async fn genai_update_model_api_key(
    configuration: &configuration::Configuration,
    api_key_uuid: &str,
    api_update_model_api_key_input_public: Option<models::ApiUpdateModelApiKeyInputPublic>,
) -> Result<models::ApiUpdateModelApiKeyOutput, Error<GenaiUpdateModelApiKeyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_api_key_uuid = api_key_uuid;
    let p_body_api_update_model_api_key_input_public = api_update_model_api_key_input_public;

    let uri_str = format!(
        "{}/v2/gen-ai/models/api_keys/{api_key_uuid}",
        configuration.base_path,
        api_key_uuid = crate::apis::urlencode(p_path_api_key_uuid)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_api_update_model_api_key_input_public);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiUpdateModelApiKeyOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiUpdateModelApiKeyOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiUpdateModelApiKeyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To update an OpenAI API key, send a PUT request to `/v2/gen-ai/openai/keys/{api_key_uuid}`.
pub async fn genai_update_openai_api_key(
    configuration: &configuration::Configuration,
    api_key_uuid: &str,
    api_update_open_aiapi_key_input_public: Option<models::ApiUpdateOpenAiapiKeyInputPublic>,
) -> Result<models::ApiUpdateOpenAiapiKeyOutput, Error<GenaiUpdateOpenaiApiKeyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_api_key_uuid = api_key_uuid;
    let p_body_api_update_open_aiapi_key_input_public = api_update_open_aiapi_key_input_public;

    let uri_str = format!(
        "{}/v2/gen-ai/openai/keys/{api_key_uuid}",
        configuration.base_path,
        api_key_uuid = crate::apis::urlencode(p_path_api_key_uuid)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_api_update_open_aiapi_key_input_public);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiUpdateOpenAiapiKeyOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiUpdateOpenAiapiKeyOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiUpdateOpenaiApiKeyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// To update a workspace, send a PUT request to `/v2/gen-ai/workspaces/{workspace_uuid}`. The response body is a JSON object containing the workspace.
pub async fn genai_update_workspace(
    configuration: &configuration::Configuration,
    workspace_uuid: &str,
    api_update_workspace_input_public: Option<models::ApiUpdateWorkspaceInputPublic>,
) -> Result<models::ApiUpdateWorkspaceOutput, Error<GenaiUpdateWorkspaceError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_workspace_uuid = workspace_uuid;
    let p_body_api_update_workspace_input_public = api_update_workspace_input_public;

    let uri_str = format!(
        "{}/v2/gen-ai/workspaces/{workspace_uuid}",
        configuration.base_path,
        workspace_uuid = crate::apis::urlencode(p_path_workspace_uuid)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_api_update_workspace_input_public);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiUpdateWorkspaceOutput`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiUpdateWorkspaceOutput`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GenaiUpdateWorkspaceError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
