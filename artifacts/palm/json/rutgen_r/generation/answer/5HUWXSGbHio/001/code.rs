// Answer 0

#[test]
fn test_fmt_with_null() {
    let value = Value::Null;
    let formatted = format!("{:?}", value);
    assert_eq!(formatted, "null");
}

#[test]
fn test_fmt_with_bool() {
    let value = Value::Bool(true);
    let formatted = format!("{:?}", value);
    assert_eq!(formatted, "boolean");
}

#[test]
fn test_fmt_with_number() {
    let value = Value::Number(serde_json::Number::from(42));
    let formatted = format!("{:?}", value);
    assert_eq!(formatted, "number");
}

#[test]
fn test_fmt_with_string() {
    let value = Value::String(String::from("test"));
    let formatted = format!("{:?}", value);
    assert_eq!(formatted, "string");
}

#[test]
fn test_fmt_with_array() {
    let value = Value::Array(vec![Value::Number(1), Value::Number(2)]);
    let formatted = format!("{:?}", value);
    assert_eq!(formatted, "array");
}

#[test]
fn test_fmt_with_object() {
    let value = Value::Object(serde_json::Map::new());
    let formatted = format!("{:?}", value);
    assert_eq!(formatted, "object");
}

