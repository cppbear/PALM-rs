// Answer 0

#[test]
fn test_value_display_pretty_format() {
    use crate::Value;
    use crate::Number;
    use alloc::vec;

    // Test case 1: Empty Object
    let json_empty_object = Value::Object(Map::new());
    let pretty_empty = format!("{:#}", json_empty_object);
    assert_eq!(pretty_empty, "{\n}");

    // Test case 2: Single Key-Value Pair
    let mut map_single = Map::new();
    map_single.insert("key".to_string(), Value::String("value".to_string()));
    let json_single = Value::Object(map_single);
    let pretty_single = format!("{:#}", json_single);
    assert_eq!(pretty_single, "{\n  \"key\": \"value\"\n}");

    // Test case 3: Multiple Key-Value Pairs
    let mut map_multiple = Map::new();
    map_multiple.insert("city".to_string(), Value::String("London".to_string()));
    map_multiple.insert("street".to_string(), Value::String("10 Downing Street".to_string()));
    let json_multiple = Value::Object(map_multiple);
    let pretty_multiple = format!("{:#}", json_multiple);
    assert_eq!(pretty_multiple,
        "{\n  \"city\": \"London\",\n  \"street\": \"10 Downing Street\"\n}");

    // Test case 4: Nested Objects
    let mut map_nested = Map::new();
    let mut map_address = Map::new();
    map_address.insert("city".to_string(), Value::String("London".to_string()));
    map_address.insert("street".to_string(), Value::String("10 Downing Street".to_string()));
    map_nested.insert("address".to_string(), Value::Object(map_address));
    let json_nested = Value::Object(map_nested);
    let pretty_nested = format!("{:#}", json_nested);
    assert_eq!(pretty_nested,
        "{\n  \"address\": {\n    \"city\": \"London\",\n    \"street\": \"10 Downing Street\"\n  }\n}");
}

