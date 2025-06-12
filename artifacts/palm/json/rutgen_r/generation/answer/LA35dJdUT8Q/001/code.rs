// Answer 0

#[test]
fn test_as_object_with_non_object_value() {
    use serde_json::{Value, Map};

    // Test case 1: Value is a string
    let string_value = Value::String("not an object".to_string());
    assert_eq!(string_value.as_object(), None);

    // Test case 2: Value is a number
    let number_value = Value::Number(serde_json::Number::from(42));
    assert_eq!(number_value.as_object(), None);

    // Test case 3: Value is a boolean
    let boolean_value = Value::Bool(true);
    assert_eq!(boolean_value.as_object(), None);

    // Test case 4: Value is an array
    let array_value = Value::Array(vec![Value::String("item1".to_string()), Value::String("item2".to_string())]);
    assert_eq!(array_value.as_object(), None);

    // Test case 5: Value is null
    let null_value = Value::Null;
    assert_eq!(null_value.as_object(), None);
}

