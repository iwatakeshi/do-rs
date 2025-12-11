use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl<T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl<T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl<T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl<T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, key),
                    value,
                )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                }
                serde_json::Value::String(s) => {
                    params.push((format!("{}[{}]", prefix, key), s.clone()))
                }
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        return params;
    }

    unimplemented!("Only objects are supported with style=deepObject")
}

/// Internal use only
/// A content type supported by this client.
#[allow(dead_code)]
enum ContentType {
    Json,
    Text,
    Unsupported(String),
}

impl From<&str> for ContentType {
    fn from(content_type: &str) -> Self {
        if content_type.starts_with("application") && content_type.contains("json") {
            return Self::Json;
        } else if content_type.starts_with("text/plain") {
            return Self::Text;
        } else {
            return Self::Unsupported(content_type.to_string());
        }
    }
}

pub mod account_api;
pub mod actions_api;
pub mod add_ons_api;
pub mod apps_api;
pub mod billing_api;
pub mod block_storage_actions_api;
pub mod block_storage_api;
pub mod byoip_prefixes_api;
pub mod cdn_endpoints_api;
pub mod certificates_api;
pub mod class1_click_applications_api;
pub mod container_registries_api;
pub mod container_registry_api;
pub mod databases_api;
pub mod domain_records_api;
pub mod domains_api;
pub mod droplet_actions_api;
pub mod droplet_autoscale_pools_api;
pub mod droplets_api;
pub mod firewalls_api;
pub mod floating_ip_actions_api;
pub mod floating_ips_api;
pub mod functions_api;
pub mod gradient_ai_platform_api;
pub mod image_actions_api;
pub mod images_api;
pub mod kubernetes_api;
pub mod load_balancers_api;
pub mod monitoring_api;
pub mod nfs_actions_api;
pub mod nfs_api;
pub mod partner_network_connect_api;
pub mod project_resources_api;
pub mod projects_api;
pub mod regions_api;
pub mod reserved_ip_actions_api;
pub mod reserved_ips_api;
pub mod reserved_ipv6_actions_api;
pub mod reserved_ipv6_api;
pub mod sizes_api;
pub mod snapshots_api;
pub mod spaces_keys_api;
pub mod ssh_keys_api;
pub mod tags_api;
pub mod uptime_api;
pub mod vpc_peerings_api;
pub mod vpcnat_gateways_api;
pub mod vpcs_api;

pub mod configuration;
