// Answer 0

#[test]
fn test_index_into_non_array_value() {
    let value = Value::Null; // Constraint: v matches Value::Array(vec) is false
    let index: usize = 0; // You can choose any index here as it will not be used
    let index_ref = &index;

    let result = index_ref.index_into(&value); // Call the method

    assert_eq!(result, None); // Expected return value/type: None
}

#[test]
fn test_index_into_non_array_with_boolean() {
    let value = Value::Bool(true); // Constraint: v matches Value::Array(vec) is false
    let index: usize = 0; // You can choose any index here as it will not be used
    let index_ref = &index;

    let result = index_ref.index_into(&value); // Call the method

    assert_eq!(result, None); // Expected return value/type: None
}

#[test]
fn test_index_into_non_array_with_string() {
    let value = Value::String(String::from("Hello")); // Constraint: v matches Value::Array(vec) is false
    let index: usize = 0; // You can choose any index here as it will not be used
    let index_ref = &index;

    let result = index_ref.index_into(&value); // Call the method

    assert_eq!(result, None); // Expected return value/type: None
}

#[test]
fn test_index_into_non_array_with_object() {
    let value = Value::Object(Map::new()); // Constraint: v matches Value::Array(vec) is false
    let index: usize = 0; // You can choose any index here as it will not be used
    let index_ref = &index;

    let result = index_ref.index_into(&value); // Call the method

    assert_eq!(result, None); // Expected return value/type: None
}

