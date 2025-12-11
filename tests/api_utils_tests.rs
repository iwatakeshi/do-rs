//! Tests for API utility functions

use digitalocean::apis::{parse_deep_object, urlencode};

#[test]
fn test_urlencode_simple_string() {
    let result = urlencode("hello");
    assert_eq!(result, "hello");
}

#[test]
fn test_urlencode_with_spaces() {
    let result = urlencode("hello world");
    assert_eq!(result, "hello+world");
}

#[test]
fn test_urlencode_with_special_characters() {
    let result = urlencode("hello&world=test");
    assert_eq!(result, "hello%26world%3Dtest");
}

#[test]
fn test_urlencode_with_unicode() {
    let result = urlencode("hello%20wörld");
    assert_eq!(result, "hello%2520w%C3%B6rld");
}

#[test]
fn test_urlencode_empty_string() {
    let result = urlencode("");
    assert_eq!(result, "");
}

#[test]
fn test_urlencode_with_string_reference() {
    let s = String::from("test value");
    let result = urlencode(&s);
    assert_eq!(result, "test+value");
}

#[test]
fn test_parse_deep_object_simple() {
    let json = serde_json::json!({
        "key": "value"
    });

    let result = parse_deep_object("prefix", &json);

    assert_eq!(result.len(), 1);
    assert_eq!(result[0], ("prefix[key]".to_string(), "value".to_string()));
}

#[test]
fn test_parse_deep_object_multiple_keys() {
    let json = serde_json::json!({
        "name": "test",
        "id": "123"
    });

    let result = parse_deep_object("filter", &json);

    assert_eq!(result.len(), 2);
    // Results may be in any order due to JSON object iteration
    let has_name = result
        .iter()
        .any(|(k, v)| k == "filter[name]" && v == "test");
    let has_id = result.iter().any(|(k, v)| k == "filter[id]" && v == "123");
    assert!(has_name);
    assert!(has_id);
}

#[test]
fn test_parse_deep_object_nested() {
    let json = serde_json::json!({
        "outer": {
            "inner": "value"
        }
    });

    let result = parse_deep_object("prefix", &json);

    assert_eq!(result.len(), 1);
    assert_eq!(
        result[0],
        ("prefix[outer][inner]".to_string(), "value".to_string())
    );
}

#[test]
fn test_parse_deep_object_with_nested_objects_array() {
    // Arrays are supported when they contain objects
    let json = serde_json::json!({
        "filters": {
            "tags": [
                {"name": "env", "value": "prod"},
                {"name": "region", "value": "us-east"}
            ]
        }
    });

    let result = parse_deep_object("prefix", &json);

    // Each object in the array contributes 2 fields
    assert_eq!(result.len(), 4);
    let has_env_name = result
        .iter()
        .any(|(k, v)| k == "prefix[filters][tags][0][name]" && v == "env");
    let has_env_value = result
        .iter()
        .any(|(k, v)| k == "prefix[filters][tags][0][value]" && v == "prod");
    assert!(has_env_name);
    assert!(has_env_value);
}

#[test]
fn test_parse_deep_object_with_numbers() {
    let json = serde_json::json!({
        "count": 42,
        "price": 19.99
    });

    let result = parse_deep_object("data", &json);

    assert_eq!(result.len(), 2);
    let has_count = result.iter().any(|(k, v)| k == "data[count]" && v == "42");
    let has_price = result
        .iter()
        .any(|(k, v)| k == "data[price]" && v == "19.99");
    assert!(has_count);
    assert!(has_price);
}

#[test]
fn test_parse_deep_object_with_boolean() {
    let json = serde_json::json!({
        "enabled": true,
        "disabled": false
    });

    let result = parse_deep_object("settings", &json);

    assert_eq!(result.len(), 2);
    let has_enabled = result
        .iter()
        .any(|(k, v)| k == "settings[enabled]" && v == "true");
    let has_disabled = result
        .iter()
        .any(|(k, v)| k == "settings[disabled]" && v == "false");
    assert!(has_enabled);
    assert!(has_disabled);
}

#[test]
fn test_parse_deep_object_complex_nested() {
    let json = serde_json::json!({
        "level1": {
            "level2": {
                "value": "deep"
            }
        }
    });

    let result = parse_deep_object("root", &json);

    assert_eq!(result.len(), 1);
    assert_eq!(
        result[0],
        (
            "root[level1][level2][value]".to_string(),
            "deep".to_string()
        )
    );
}

#[test]
fn test_parse_deep_object_empty_object() {
    let json = serde_json::json!({});

    let result = parse_deep_object("prefix", &json);

    assert!(result.is_empty());
}

#[test]
fn test_parse_deep_object_array_of_objects() {
    let json = serde_json::json!({
        "users": [
            {"name": "Alice"},
            {"name": "Bob"}
        ]
    });

    let result = parse_deep_object("data", &json);

    assert_eq!(result.len(), 2);
    let has_alice = result
        .iter()
        .any(|(k, v)| k == "data[users][0][name]" && v == "Alice");
    let has_bob = result
        .iter()
        .any(|(k, v)| k == "data[users][1][name]" && v == "Bob");
    assert!(has_alice);
    assert!(has_bob);
}

#[test]
fn test_parse_deep_object_mixed_types() {
    let json = serde_json::json!({
        "string": "text",
        "number": 123,
        "boolean": true,
        "nested": {
            "key": "value"
        }
    });

    let result = parse_deep_object("obj", &json);

    assert_eq!(result.len(), 4);
    let has_string = result
        .iter()
        .any(|(k, v)| k == "obj[string]" && v == "text");
    let has_number = result.iter().any(|(k, v)| k == "obj[number]" && v == "123");
    let has_boolean = result
        .iter()
        .any(|(k, v)| k == "obj[boolean]" && v == "true");
    let has_nested = result
        .iter()
        .any(|(k, v)| k == "obj[nested][key]" && v == "value");
    assert!(has_string);
    assert!(has_number);
    assert!(has_boolean);
    assert!(has_nested);
}
