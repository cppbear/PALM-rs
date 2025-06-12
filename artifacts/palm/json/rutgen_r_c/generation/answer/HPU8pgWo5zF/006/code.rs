// Answer 0

#[test]
fn test_serialize_value_string() {
    use serde_json::Value;
    use serde_json::Serializer;

    // Test serializing a simple string
    let value = Value::String("Hello, World!".to_string());
    let mut serialized_output = Vec::new();
    let serializer = Serializer::with_formatter(&mut serialized_output, Default::default());

    let result = value.serialize(serializer);
    assert!(result.is_ok());
    assert_eq!(std::str::from_utf8(&serialized_output).unwrap(), "\"Hello, World!\"");

    // Test serializing an empty string
    let empty_value = Value::String("".to_string());
    let mut empty_serialized_output = Vec::new();
    let empty_serializer = Serializer::with_formatter(&mut empty_serialized_output, Default::default());

    let empty_result = empty_value.serialize(empty_serializer);
    assert!(empty_result.is_ok());
    assert_eq!(std::str::from_utf8(&empty_serialized_output).unwrap(), "\"\"");
}

#[test]
fn test_serialize_value_bool() {
    use serde_json::Value;
    use serde_json::Serializer;

    // Test serializing boolean true
    let value_true = Value::Bool(true);
    let mut true_serialized_output = Vec::new();
    let true_serializer = Serializer::with_formatter(&mut true_serialized_output, Default::default());

    let true_result = value_true.serialize(true_serializer);
    assert!(true_result.is_ok());
    assert_eq!(std::str::from_utf8(&true_serialized_output).unwrap(), "true");

    // Test serializing boolean false
    let value_false = Value::Bool(false);
    let mut false_serialized_output = Vec::new();
    let false_serializer = Serializer::with_formatter(&mut false_serialized_output, Default::default());

    let false_result = value_false.serialize(false_serializer);
    assert!(false_result.is_ok());
    assert_eq!(std::str::from_utf8(&false_serialized_output).unwrap(), "false");
} 

#[test]
fn test_serialize_value_null() {
    use serde_json::Value;
    use serde_json::Serializer;

    // Test serializing null value
    let value_null = Value::Null;
    let mut null_serialized_output = Vec::new();
    let null_serializer = Serializer::with_formatter(&mut null_serialized_output, Default::default());

    let null_result = value_null.serialize(null_serializer);
    assert!(null_result.is_ok());
    assert_eq!(std::str::from_utf8(&null_serialized_output).unwrap(), "null");
}

