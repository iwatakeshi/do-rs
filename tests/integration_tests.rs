//! Integration tests for API client with mock server

use digitalocean::apis::configuration::Configuration;
use digitalocean::apis::{account_api, regions_api};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn setup_mock_server() -> (MockServer, Configuration) {
    let mock_server = MockServer::start().await;
    let mut config = Configuration::new();
    config.base_path = mock_server.uri();
    config.bearer_access_token = Some("test_token".to_owned());
    (mock_server, config)
}

#[tokio::test]
async fn test_account_get_success() {
    let (mock_server, config) = setup_mock_server().await;

    let response_body = serde_json::json!({
        "account": {
            "droplet_limit": 25,
            "floating_ip_limit": 5,
            "email": "test@example.com",
            "uuid": "550e8400-e29b-41d4-a716-446655440000",
            "email_verified": true,
            "status": "active",
            "status_message": "Account is active"
        }
    });

    Mock::given(method("GET"))
        .and(path("/v2/account"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let result = account_api::account_get(&config).await;

    assert!(result.is_ok(), "account_get failed: {:?}", result.err());
    let response = result.unwrap();
    assert!(response.account.is_some());

    let account = response.account.unwrap();
    assert_eq!(account.droplet_limit, 25);
    assert_eq!(account.floating_ip_limit, 5);
    assert_eq!(account.email, "test@example.com");
    assert!(account.email_verified);
}

#[tokio::test]
async fn test_account_get_unauthorized() {
    let (mock_server, config) = setup_mock_server().await;

    let response_body = serde_json::json!({
        "id": "unauthorized",
        "message": "Unable to authenticate you."
    });

    Mock::given(method("GET"))
        .and(path("/v2/account"))
        .respond_with(ResponseTemplate::new(401).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let result = account_api::account_get(&config).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_account_get_rate_limited() {
    let (mock_server, config) = setup_mock_server().await;

    let response_body = serde_json::json!({
        "id": "too_many_requests",
        "message": "API Rate limit exceeded."
    });

    Mock::given(method("GET"))
        .and(path("/v2/account"))
        .respond_with(ResponseTemplate::new(429).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let result = account_api::account_get(&config).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_account_get_server_error() {
    let (mock_server, config) = setup_mock_server().await;

    let response_body = serde_json::json!({
        "id": "server_error",
        "message": "Internal server error."
    });

    Mock::given(method("GET"))
        .and(path("/v2/account"))
        .respond_with(ResponseTemplate::new(500).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let result = account_api::account_get(&config).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_regions_list_success() {
    let (mock_server, config) = setup_mock_server().await;

    let response_body = serde_json::json!({
        "regions": [
            {
                "name": "New York 1",
                "slug": "nyc1",
                "features": ["metadata", "ipv6", "private_networking"],
                "available": true,
                "sizes": ["s-1vcpu-1gb", "s-2vcpu-2gb"]
            },
            {
                "name": "San Francisco 1",
                "slug": "sfo1",
                "features": ["metadata", "backups"],
                "available": true,
                "sizes": ["s-1vcpu-1gb"]
            }
        ],
        "links": {},
        "meta": {
            "total": 2
        }
    });

    Mock::given(method("GET"))
        .and(path("/v2/regions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let result = regions_api::regions_list(&config, None, None).await;

    assert!(result.is_ok(), "regions_list failed: {:?}", result.err());
    let response = result.unwrap();

    let regions = &response.regions;
    assert_eq!(regions.len(), 2);
    assert_eq!(regions[0].name, "New York 1");
    assert_eq!(regions[0].slug, "nyc1");
    assert!(regions[0].available);
    assert_eq!(regions[1].name, "San Francisco 1");
    assert_eq!(regions[1].slug, "sfo1");
}

#[tokio::test]
async fn test_regions_list_with_pagination() {
    let (mock_server, config) = setup_mock_server().await;

    let response_body = serde_json::json!({
        "regions": [
            {
                "name": "London 1",
                "slug": "lon1",
                "features": ["metadata"],
                "available": true,
                "sizes": ["s-1vcpu-1gb"]
            }
        ],
        "links": {
            "pages": {
                "next": "https://api.digitalocean.com/v2/regions?page=2",
                "last": "https://api.digitalocean.com/v2/regions?page=5"
            }
        },
        "meta": {
            "total": 50
        }
    });

    Mock::given(method("GET"))
        .and(path("/v2/regions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let result = regions_api::regions_list(&config, Some(10), Some(1)).await;

    assert!(result.is_ok(), "regions_list failed: {:?}", result.err());
    let response = result.unwrap();
    assert!(!response.regions.is_empty());
    assert_eq!(response.meta.total, Some(50));
}

#[tokio::test]
async fn test_regions_list_empty() {
    let (mock_server, config) = setup_mock_server().await;

    let response_body = serde_json::json!({
        "regions": [],
        "links": {},
        "meta": {
            "total": 0
        }
    });

    Mock::given(method("GET"))
        .and(path("/v2/regions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let result = regions_api::regions_list(&config, None, None).await;

    assert!(result.is_ok(), "regions_list failed: {:?}", result.err());
    let response = result.unwrap();
    assert!(response.regions.is_empty());
}

#[tokio::test]
async fn test_configuration_with_user_agent() {
    let (mock_server, mut config) = setup_mock_server().await;
    config.user_agent = Some("CustomAgent/1.0".to_owned());

    let response_body = serde_json::json!({
        "account": {
            "droplet_limit": 10,
            "floating_ip_limit": 3,
            "email": "test@example.com",
            "uuid": "123e4567-e89b-12d3-a456-426614174000",
            "email_verified": true,
            "status": "active",
            "status_message": ""
        }
    });

    Mock::given(method("GET"))
        .and(path("/v2/account"))
        .and(header("user-agent", "CustomAgent/1.0"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let result = account_api::account_get(&config).await;

    assert!(result.is_ok(), "account_get failed: {:?}", result.err());
}

#[tokio::test]
async fn test_bearer_token_in_authorization_header() {
    let (mock_server, mut config) = setup_mock_server().await;
    config.bearer_access_token = Some("my_secret_token_12345".to_owned());

    let response_body = serde_json::json!({
        "account": {
            "droplet_limit": 10,
            "floating_ip_limit": 3,
            "email": "test@example.com",
            "uuid": "123e4567-e89b-12d3-a456-426614174000",
            "email_verified": true,
            "status": "active",
            "status_message": ""
        }
    });

    Mock::given(method("GET"))
        .and(path("/v2/account"))
        .and(header("authorization", "Bearer my_secret_token_12345"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let result = account_api::account_get(&config).await;

    assert!(result.is_ok(), "account_get failed: {:?}", result.err());
}

#[tokio::test]
async fn test_account_with_warning_status() {
    let (mock_server, config) = setup_mock_server().await;

    let response_body = serde_json::json!({
        "account": {
            "droplet_limit": 5,
            "floating_ip_limit": 2,
            "email": "warning@example.com",
            "uuid": "550e8400-e29b-41d4-a716-446655440000",
            "email_verified": true,
            "status": "warning",
            "status_message": "Your account is under review"
        }
    });

    Mock::given(method("GET"))
        .and(path("/v2/account"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let result = account_api::account_get(&config).await;

    assert!(result.is_ok(), "account_get failed: {:?}", result.err());
    let response = result.unwrap();
    let account = response.account.unwrap();
    assert_eq!(
        account.status,
        digitalocean::models::account::Status::Warning
    );
    assert_eq!(account.status_message, "Your account is under review");
}

#[tokio::test]
async fn test_account_with_locked_status() {
    let (mock_server, config) = setup_mock_server().await;

    let response_body = serde_json::json!({
        "account": {
            "droplet_limit": 0,
            "floating_ip_limit": 0,
            "email": "locked@example.com",
            "uuid": "550e8400-e29b-41d4-a716-446655440000",
            "email_verified": false,
            "status": "locked",
            "status_message": "Account has been locked due to policy violation"
        }
    });

    Mock::given(method("GET"))
        .and(path("/v2/account"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let result = account_api::account_get(&config).await;

    assert!(result.is_ok(), "account_get failed: {:?}", result.err());
    let response = result.unwrap();
    let account = response.account.unwrap();
    assert_eq!(
        account.status,
        digitalocean::models::account::Status::Locked
    );
    assert!(!account.email_verified);
}

#[tokio::test]
async fn test_account_with_team() {
    let (mock_server, config) = setup_mock_server().await;

    let response_body = serde_json::json!({
        "account": {
            "droplet_limit": 100,
            "floating_ip_limit": 20,
            "email": "team@company.com",
            "uuid": "550e8400-e29b-41d4-a716-446655440000",
            "email_verified": true,
            "status": "active",
            "status_message": "",
            "team": {
                "uuid": "team-uuid-12345",
                "name": "Engineering Team"
            }
        }
    });

    Mock::given(method("GET"))
        .and(path("/v2/account"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let result = account_api::account_get(&config).await;

    assert!(result.is_ok(), "account_get failed: {:?}", result.err());
    let response = result.unwrap();
    let account = response.account.unwrap();
    assert!(account.team.is_some());

    let team = account.team.unwrap();
    assert_eq!(team.uuid, Some("team-uuid-12345".to_owned()));
}
