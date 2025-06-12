// Answer 0

#[test]
fn test_json_display_pretty_format() {
    use std::fmt;
    use serde_json::json;

    let json = json!({ "city": "London", "street": "10 Downing Street" });

    // Pretty format (alternate is true)
    let expected_pretty = "{\n  \"city\": \"London\",\n  \"street\": \"10 Downing Street\"\n}";
    let pretty = format!("{:#}", json);
    assert_eq!(pretty, expected_pretty);
}

#[test]
fn test_json_display_empty_object() {
    use std::fmt;
    use serde_json::json;

    let json = json!({});

    // Pretty format (alternate is true) for empty object
    let expected_pretty = "{\n}\n";
    let pretty = format!("{:#}", json);
    assert_eq!(pretty, expected_pretty);
}

#[test]
fn test_json_display_nested_object() {
    use std::fmt;
    use serde_json::json;

    let json = json!({ 
        "location": {
            "city": "London",
            "street": "10 Downing Street"
        }
    });

    // Pretty format (alternate is true) for nested object
    let expected_pretty = "{\n  \"location\": {\n    \"city\": \"London\",\n    \"street\": \"10 Downing Street\"\n  }\n}";
    let pretty = format!("{:#}", json);
    assert_eq!(pretty, expected_pretty);
}

