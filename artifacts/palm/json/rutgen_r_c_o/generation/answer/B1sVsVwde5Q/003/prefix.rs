// Answer 0

#[test]
fn test_deserialize_struct_with_non_empty_array() {
    let input = Value::Array(vec![
        Value::String("test".to_string()),
        Value::Number(Number { n: 1 }),
        Value::Bool(true),
        Value::Null,
        Value::Array(vec![Value::String("inner".to_string())])
    ]);
    let _ = input.deserialize_struct("Test", &[]);
}

#[test]
fn test_deserialize_struct_with_empty_array() {
    let input = Value::Array(Vec::new());
    let _ = input.deserialize_struct("Test", &[]);
}

#[test]
fn test_deserialize_struct_with_array_of_null() {
    let input = Value::Array(vec![Value::Null]);
    let _ = input.deserialize_struct("Test", &[]);
}

#[test]
fn test_deserialize_struct_with_mixed_array() {
    let input = Value::Array(vec![
        Value::Bool(false),
        Value::Number(Number { n: 0 }),
        Value::String("another test".to_string())
    ]);
    let _ = input.deserialize_struct("Test", &[]);
}

