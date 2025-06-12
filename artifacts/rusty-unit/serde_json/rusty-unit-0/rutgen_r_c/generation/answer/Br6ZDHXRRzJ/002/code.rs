// Answer 0

#[test]
fn test_unexpected_array() {
    use serde_json::value::Value;
    
    let value = Value::Array(vec![Value::Bool(true), Value::String("test".to_owned())]);
    
    // Check that the unexpected method returns the expected Unexpected::Seq variant
    let result = value.unexpected();
    match result {
        Unexpected::Seq => {},
        _ => panic!("Expected Unexpected::Seq, but got {:?}", result),
    }
}

#[test]
fn test_unexpected_empty_array() {
    use serde_json::value::Value;

    let value = Value::Array(vec![]);

    // Check that the unexpected method returns the expected Unexpected::Seq variant
    let result = value.unexpected();
    match result {
        Unexpected::Seq => {},
        _ => panic!("Expected Unexpected::Seq, but got {:?}", result),
    }
}

