//! Tests for the Configuration struct and related utilities

use digitalocean::apis::configuration::{ApiKey, Configuration};

#[test]
fn test_configuration_default() {
    let config = Configuration::default();

    assert_eq!(config.base_path, "https://api.digitalocean.com");
    assert_eq!(
        config.user_agent,
        Some("OpenAPI-Generator/2.0/rust".to_owned())
    );
    assert!(config.basic_auth.is_none());
    assert!(config.oauth_access_token.is_none());
    assert!(config.bearer_access_token.is_none());
    assert!(config.api_key.is_none());
}

#[test]
fn test_configuration_new() {
    let config = Configuration::new();

    assert_eq!(config.base_path, "https://api.digitalocean.com");
    assert_eq!(
        config.user_agent,
        Some("OpenAPI-Generator/2.0/rust".to_owned())
    );
}

#[test]
fn test_configuration_with_bearer_token() {
    let mut config = Configuration::new();
    config.bearer_access_token = Some("test_token_123".to_owned());

    assert_eq!(
        config.bearer_access_token,
        Some("test_token_123".to_owned())
    );
}

#[test]
fn test_configuration_with_api_key() {
    let mut config = Configuration::new();
    config.api_key = Some(ApiKey {
        prefix: Some("Bearer".to_owned()),
        key: "my_api_key".to_owned(),
    });

    let api_key = config.api_key.unwrap();
    assert_eq!(api_key.prefix, Some("Bearer".to_owned()));
    assert_eq!(api_key.key, "my_api_key");
}

#[test]
fn test_configuration_with_basic_auth() {
    let mut config = Configuration::new();
    config.basic_auth = Some(("username".to_owned(), Some("password".to_owned())));

    let (username, password) = config.basic_auth.unwrap();
    assert_eq!(username, "username");
    assert_eq!(password, Some("password".to_owned()));
}

#[test]
fn test_configuration_with_custom_base_path() {
    let mut config = Configuration::new();
    config.base_path = "https://custom.api.example.com".to_owned();

    assert_eq!(config.base_path, "https://custom.api.example.com");
}

#[test]
fn test_configuration_clone() {
    let mut config = Configuration::new();
    config.bearer_access_token = Some("test_token".to_owned());
    config.base_path = "https://custom.api.com".to_owned();

    let cloned = config.clone();

    assert_eq!(cloned.base_path, "https://custom.api.com");
    assert_eq!(cloned.bearer_access_token, Some("test_token".to_owned()));
}

#[test]
fn test_api_key_clone() {
    let api_key = ApiKey {
        prefix: Some("Token".to_owned()),
        key: "secret_key".to_owned(),
    };

    let cloned = api_key.clone();

    assert_eq!(cloned.prefix, Some("Token".to_owned()));
    assert_eq!(cloned.key, "secret_key");
}
