//! Tests for model serialization and deserialization

use digitalocean::models::{
    Account, AccountGet200Response, AccountTeam, Action, Error, MetaProperties, Region,
    RegionsList200Response,
};

#[test]
fn test_account_serialization() {
    let account = Account::new(
        25,
        3,
        "user@example.com".to_owned(),
        "550e8400-e29b-41d4-a716-446655440000".to_owned(),
        true,
        digitalocean::models::account::Status::Active,
        "Account is active".to_owned(),
    );

    let json = serde_json::to_string(&account).unwrap();
    let deserialized: Account = serde_json::from_str(&json).unwrap();

    assert_eq!(account, deserialized);
    assert_eq!(deserialized.droplet_limit, 25);
    assert_eq!(deserialized.floating_ip_limit, 3);
    assert_eq!(deserialized.email, "user@example.com");
    assert!(deserialized.email_verified);
}

#[test]
fn test_account_deserialization_from_json() {
    let json = r#"{
        "droplet_limit": 10,
        "floating_ip_limit": 5,
        "email": "test@example.com",
        "uuid": "123e4567-e89b-12d3-a456-426614174000",
        "email_verified": true,
        "status": "active",
        "status_message": "All good"
    }"#;

    let account: Account = serde_json::from_str(json).unwrap();

    assert_eq!(account.droplet_limit, 10);
    assert_eq!(account.floating_ip_limit, 5);
    assert_eq!(account.email, "test@example.com");
    assert_eq!(account.uuid, "123e4567-e89b-12d3-a456-426614174000");
    assert!(account.email_verified);
    assert_eq!(
        account.status,
        digitalocean::models::account::Status::Active
    );
    assert_eq!(account.status_message, "All good");
}

#[test]
fn test_account_status_variants() {
    let active_json = r#""active""#;
    let warning_json = r#""warning""#;
    let locked_json = r#""locked""#;

    let active: digitalocean::models::account::Status = serde_json::from_str(active_json).unwrap();
    let warning: digitalocean::models::account::Status =
        serde_json::from_str(warning_json).unwrap();
    let locked: digitalocean::models::account::Status = serde_json::from_str(locked_json).unwrap();

    assert_eq!(active, digitalocean::models::account::Status::Active);
    assert_eq!(warning, digitalocean::models::account::Status::Warning);
    assert_eq!(locked, digitalocean::models::account::Status::Locked);
}

#[test]
fn test_account_status_default() {
    let status = digitalocean::models::account::Status::default();
    assert_eq!(status, digitalocean::models::account::Status::Active);
}

#[test]
fn test_account_get_200_response() {
    let json = r#"{
        "account": {
            "droplet_limit": 25,
            "floating_ip_limit": 5,
            "email": "user@example.com",
            "uuid": "550e8400-e29b-41d4-a716-446655440000",
            "email_verified": true,
            "status": "active",
            "status_message": ""
        }
    }"#;

    let response: AccountGet200Response = serde_json::from_str(json).unwrap();

    assert!(response.account.is_some());
    let account = response.account.unwrap();
    assert_eq!(account.droplet_limit, 25);
    assert_eq!(account.email, "user@example.com");
}

#[test]
fn test_region_serialization() {
    let region = Region::new(
        "New York 1".to_owned(),
        "nyc1".to_owned(),
        vec!["metadata".to_owned(), "ipv6".to_owned()],
        true,
        vec!["s-1vcpu-1gb".to_owned(), "s-2vcpu-2gb".to_owned()],
    );

    let json = serde_json::to_string(&region).unwrap();
    let deserialized: Region = serde_json::from_str(&json).unwrap();

    assert_eq!(region, deserialized);
    assert_eq!(deserialized.name, "New York 1");
    assert_eq!(deserialized.slug, "nyc1");
    assert!(deserialized.available);
    assert_eq!(deserialized.features.len(), 2);
    assert_eq!(deserialized.sizes.len(), 2);
}

#[test]
fn test_region_deserialization_from_json() {
    let json = r#"{
        "name": "San Francisco 1",
        "slug": "sfo1",
        "features": ["private_networking", "backups"],
        "available": false,
        "sizes": ["s-1vcpu-1gb"]
    }"#;

    let region: Region = serde_json::from_str(json).unwrap();

    assert_eq!(region.name, "San Francisco 1");
    assert_eq!(region.slug, "sfo1");
    assert!(!region.available);
    assert_eq!(region.features, vec!["private_networking", "backups"]);
    assert_eq!(region.sizes, vec!["s-1vcpu-1gb"]);
}

#[test]
fn test_regions_list_200_response() {
    let json = r#"{
        "regions": [
            {
                "name": "New York 1",
                "slug": "nyc1",
                "features": ["metadata"],
                "available": true,
                "sizes": ["s-1vcpu-1gb"]
            },
            {
                "name": "London 1",
                "slug": "lon1",
                "features": ["ipv6"],
                "available": true,
                "sizes": ["s-2vcpu-2gb"]
            }
        ],
        "meta": {
            "total": 2
        }
    }"#;

    let response: RegionsList200Response = serde_json::from_str(json).unwrap();

    // regions is a Vec, not Option<Vec>
    assert_eq!(response.regions.len(), 2);
    assert_eq!(response.regions[0].slug, "nyc1");
    assert_eq!(response.regions[1].slug, "lon1");
}

#[test]
fn test_error_model_serialization() {
    let error = Error::new("not_found".to_owned(), "Resource not found".to_owned());

    let json = serde_json::to_string(&error).unwrap();
    let deserialized: Error = serde_json::from_str(&json).unwrap();

    assert_eq!(error, deserialized);
    assert_eq!(deserialized.id, "not_found");
    assert_eq!(deserialized.message, "Resource not found");
}

#[test]
fn test_error_model_deserialization() {
    let json = r#"{
        "id": "forbidden",
        "message": "You do not have access for the attempted action."
    }"#;

    let error: Error = serde_json::from_str(json).unwrap();

    assert_eq!(error.id, "forbidden");
    assert_eq!(
        error.message,
        "You do not have access for the attempted action."
    );
}

#[test]
fn test_action_model_deserialization() {
    let json = r#"{
        "id": 12345,
        "status": "in-progress",
        "type": "create",
        "started_at": "2023-01-01T00:00:00Z",
        "completed_at": null,
        "resource_id": 67890,
        "resource_type": "droplet",
        "region_slug": "nyc1"
    }"#;

    let action: Action = serde_json::from_str(json).unwrap();

    assert_eq!(action.id, Some(12345));
    assert_eq!(
        action.status,
        Some(digitalocean::models::action::Status::InProgress)
    );
    assert_eq!(action.r#type, Some("create".to_owned()));
    assert_eq!(action.started_at, Some("2023-01-01T00:00:00Z".to_owned()));
    // resource_id is Option<Option<i32>> due to double_option
    assert_eq!(action.resource_id, Some(Some(67890)));
    assert_eq!(action.resource_type, Some("droplet".to_owned()));
    assert_eq!(action.region_slug, Some(Some("nyc1".to_owned())));
}

#[test]
fn test_action_status_variants() {
    let in_progress_json = r#""in-progress""#;
    let completed_json = r#""completed""#;
    let errored_json = r#""errored""#;

    let in_progress: digitalocean::models::action::Status =
        serde_json::from_str(in_progress_json).unwrap();
    let completed: digitalocean::models::action::Status =
        serde_json::from_str(completed_json).unwrap();
    let errored: digitalocean::models::action::Status =
        serde_json::from_str(errored_json).unwrap();

    assert_eq!(
        in_progress,
        digitalocean::models::action::Status::InProgress
    );
    assert_eq!(completed, digitalocean::models::action::Status::Completed);
    assert_eq!(errored, digitalocean::models::action::Status::Errored);
}

#[test]
fn test_account_team_serialization() {
    // AccountTeam::new() takes no arguments, fields are optional
    let mut team = AccountTeam::new();
    team.uuid = Some("550e8400-e29b-41d4-a716-446655440000".to_owned());
    team.name = Some("Test Team".to_owned());

    let json = serde_json::to_string(&team).unwrap();
    let deserialized: AccountTeam = serde_json::from_str(&json).unwrap();

    assert_eq!(team, deserialized);
    assert_eq!(
        deserialized.uuid,
        Some("550e8400-e29b-41d4-a716-446655440000".to_owned())
    );
}

#[test]
fn test_account_with_team() {
    let json = r#"{
        "droplet_limit": 10,
        "floating_ip_limit": 5,
        "email": "team@example.com",
        "uuid": "123e4567-e89b-12d3-a456-426614174000",
        "email_verified": true,
        "status": "active",
        "status_message": "",
        "team": {
            "uuid": "team-uuid-12345",
            "name": "My Team"
        }
    }"#;

    let account: Account = serde_json::from_str(json).unwrap();

    assert!(account.team.is_some());
    let team = account.team.unwrap();
    assert_eq!(team.uuid, Some("team-uuid-12345".to_owned()));
}

#[test]
fn test_region_empty_features_and_sizes() {
    let json = r#"{
        "name": "Test Region",
        "slug": "test1",
        "features": [],
        "available": true,
        "sizes": []
    }"#;

    let region: Region = serde_json::from_str(json).unwrap();

    assert_eq!(region.name, "Test Region");
    assert!(region.features.is_empty());
    assert!(region.sizes.is_empty());
}

#[test]
fn test_model_default_values() {
    let account = Account::default();
    assert_eq!(account.droplet_limit, 0);
    assert_eq!(account.floating_ip_limit, 0);
    assert_eq!(account.email, "");
    assert!(!account.email_verified);

    let region = Region::default();
    assert_eq!(region.name, "");
    assert_eq!(region.slug, "");
    assert!(region.features.is_empty());
    assert!(!region.available);
}

#[test]
fn test_meta_properties() {
    // MetaProperties::new() takes no arguments, total is optional
    let mut meta = MetaProperties::new();
    meta.total = Some(42);

    let json = serde_json::to_string(&meta).unwrap();
    let deserialized: MetaProperties = serde_json::from_str(&json).unwrap();

    assert_eq!(meta, deserialized);
    assert_eq!(deserialized.total, Some(42));
}

#[test]
fn test_droplet_create_ssh_keys_inner_integer() {
    use digitalocean::models::DropletCreateSshKeysInner;

    // Test creating from integer ID
    let ssh_key = DropletCreateSshKeysInner::from_id(12345);
    
    // Test serialization
    let json = serde_json::to_string(&ssh_key).unwrap();
    assert_eq!(json, "12345");
    
    // Test deserialization
    let deserialized: DropletCreateSshKeysInner = serde_json::from_str("12345").unwrap();
    assert_eq!(ssh_key, deserialized);
    
    // Test From trait
    let from_int: DropletCreateSshKeysInner = 67890i64.into();
    let json2 = serde_json::to_string(&from_int).unwrap();
    assert_eq!(json2, "67890");
}

#[test]
fn test_droplet_create_ssh_keys_inner_string() {
    use digitalocean::models::DropletCreateSshKeysInner;

    // Test creating from fingerprint string
    let ssh_key = DropletCreateSshKeysInner::from_fingerprint(
        "aa:bb:cc:dd:ee:ff:00:11:22:33:44:55:66:77:88:99".to_string()
    );
    
    // Test serialization
    let json = serde_json::to_string(&ssh_key).unwrap();
    assert_eq!(json, r#""aa:bb:cc:dd:ee:ff:00:11:22:33:44:55:66:77:88:99""#);
    
    // Test deserialization
    let deserialized: DropletCreateSshKeysInner = 
        serde_json::from_str(r#""aa:bb:cc:dd:ee:ff:00:11:22:33:44:55:66:77:88:99""#).unwrap();
    assert_eq!(ssh_key, deserialized);
    
    // Test From<String> trait
    let from_string: DropletCreateSshKeysInner = 
        "11:22:33:44:55:66:77:88:99:aa:bb:cc:dd:ee:ff:00".to_string().into();
    let json2 = serde_json::to_string(&from_string).unwrap();
    assert_eq!(json2, r#""11:22:33:44:55:66:77:88:99:aa:bb:cc:dd:ee:ff:00""#);
    
    // Test From<&str> trait
    let from_str: DropletCreateSshKeysInner = "test-fingerprint".into();
    let json3 = serde_json::to_string(&from_str).unwrap();
    assert_eq!(json3, r#""test-fingerprint""#);
}

#[test]
fn test_droplet_create_ssh_keys_inner_array() {
    use digitalocean::models::DropletCreateSshKeysInner;

    // Test mixed array of integers and strings
    let ssh_keys = vec![
        DropletCreateSshKeysInner::from_id(123),
        DropletCreateSshKeysInner::from_fingerprint("aa:bb:cc:dd:ee:ff".to_string()),
        456i64.into(),
        "fingerprint-string".into(),
    ];
    
    // Test serialization of array
    let json = serde_json::to_string(&ssh_keys).unwrap();
    assert_eq!(json, r#"[123,"aa:bb:cc:dd:ee:ff",456,"fingerprint-string"]"#);
    
    // Test deserialization of array
    let deserialized: Vec<DropletCreateSshKeysInner> = 
        serde_json::from_str(r#"[123,"aa:bb:cc:dd:ee:ff",456,"fingerprint-string"]"#).unwrap();
    assert_eq!(ssh_keys, deserialized);
}

#[test]
fn test_droplet_create_with_ssh_keys() {
    use digitalocean::models::{DropletCreate, DropletCreateImage, DropletCreateSshKeysInner};

    // Create a droplet with SSH keys
    let image = DropletCreateImage::Integer(12345);
    let mut droplet = DropletCreate::new("s-1vcpu-1gb".to_string(), image);
    
    droplet.ssh_keys = Some(vec![
        DropletCreateSshKeysInner::from_id(123),
        DropletCreateSshKeysInner::from_fingerprint("aa:bb:cc:dd:ee:ff".to_string()),
    ]);
    
    // Serialize to JSON
    let json = serde_json::to_string(&droplet).unwrap();
    
    // Verify the SSH keys are serialized correctly (should be an array with mixed types)
    assert!(json.contains(r#""ssh_keys":[123,"aa:bb:cc:dd:ee:ff"]"#));
    
    // Deserialize back
    let deserialized: DropletCreate = serde_json::from_str(&json).unwrap();
    assert_eq!(droplet.ssh_keys, deserialized.ssh_keys);
}
