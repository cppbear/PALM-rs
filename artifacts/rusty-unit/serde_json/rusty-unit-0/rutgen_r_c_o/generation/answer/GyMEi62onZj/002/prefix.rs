// Answer 0

#[test]
fn test_value_debug_object() {
    let map = serde_json::Map::from_iter(vec![
        (String::from("key1"), serde_json::Value::String(String::from("value1"))),
        (String::from("key2"), serde_json::Value::Number(serde_json::Number::from(42))),
    ]);

    let value = serde_json::Value::Object(map);
    let mut formatter = fmt::Formatter::new();
    let _ = value.fmt(&mut formatter);
}

#[test]
fn test_value_debug_array() {
    let array = vec![
        serde_json::Value::String(String::from("item1")),
        serde_json::Value::Bool(true),
        serde_json::Value::Number(serde_json::Number::from(3.14)),
    ];

    let value = serde_json::Value::Array(array);
    let mut formatter = fmt::Formatter::new();
    let _ = value.fmt(&mut formatter);
}

#[test]
fn test_value_debug_empty_object() {
    let map = serde_json::Map::new();

    let value = serde_json::Value::Object(map);
    let mut formatter = fmt::Formatter::new();
    let _ = value.fmt(&mut formatter);
}

#[test]
fn test_value_debug_empty_array() {
    let array: Vec<serde_json::Value> = vec![];

    let value = serde_json::Value::Array(array);
    let mut formatter = fmt::Formatter::new();
    let _ = value.fmt(&mut formatter);
}

