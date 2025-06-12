// Answer 0

#[test]
fn test_json_display_compact() {
    use std::fmt;
    use serde_json::json;

    let json = json!({ "city": "London", "street": "10 Downing Street" });

    // Compact format:
    let compact = format!("{}", json);
    assert_eq!(compact, "{\"city\":\"London\",\"street\":\"10 Downing Street\"}");
}

#[test]
fn test_json_display_compact_empty() {
    use std::fmt;
    use serde_json::json;

    let json = json!({});

    // Compact format for empty JSON:
    let compact = format!("{}", json);
    assert_eq!(compact, "{}");
}

#[test]
fn test_json_display_compact_nested() {
    use std::fmt;
    use serde_json::json;

    let json = json!({
        "city": "London",
        "address": {
            "street": "10 Downing Street",
            "postcode": "SW1A 2AA"
        }
    });

    // Compact format for nested JSON:
    let compact = format!("{}", json);
    assert_eq!(compact, "{\"city\":\"London\",\"address\":{\"street\":\"10 Downing Street\",\"postcode\":\"SW1A 2AA\"}}");
} 

#[test]
fn test_json_display_compact_array() {
    use std::fmt;
    use serde_json::json;

    let json = json!([1, 2, 3, 4, 5]);

    // Compact format for array:
    let compact = format!("{}", json);
    assert_eq!(compact, "[1,2,3,4,5]");
}

#[test]
fn test_json_display_compact_mixed_types() {
    use std::fmt;
    use serde_json::json;

    let json = json!({
        "name": "Alice",
        "age": 30,
        "hobbies": ["reading", "gaming"],
        "active": true
    });

    // Compact format for mixed types:
    let compact = format!("{}", json);
    assert_eq!(compact, "{\"name\":\"Alice\",\"age\":30,\"hobbies\":[\"reading\",\"gaming\"],\"active\":true}");
}

