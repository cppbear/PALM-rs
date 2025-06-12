// Answer 0

#[test]
fn test_as_str_with_string_value() {
    use serde_json::Value;

    // Create a Value enum for a string
    let string_value = Value::String("test string".to_string());
    
    // Assert that as_str returns Some("test string")
    assert_eq!(string_value.as_str(), Some("test string"));
}

#[test]
fn test_as_str_with_non_string_value() {
    use serde_json::Value;
    
    // Create a Value enum for a number
    let number_value = Value::Number(serde_json::Number::from(42));
    
    // Assert that as_str returns None for non-string values
    assert_eq!(number_value.as_str(), None);
}

#[test]
fn test_as_str_with_null_value() {
    use serde_json::Value;

    // Create a Value enum for a null
    let null_value = Value::Null;

    // Assert that as_str returns None for null values
    assert_eq!(null_value.as_str(), None);
}

#[test]
fn test_as_str_with_boolean_value() {
    use serde_json::Value;

    // Create a Value enum for a boolean
    let boolean_value = Value::Bool(true);

    // Assert that as_str returns None for boolean values
    assert_eq!(boolean_value.as_str(), None);
}

