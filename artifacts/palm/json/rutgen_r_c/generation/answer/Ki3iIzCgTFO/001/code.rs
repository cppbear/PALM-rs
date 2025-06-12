// Answer 0

#[test]
fn test_serialize_vec_end_empty() {
    let serialize_vec = SerializeVec { vec: Vec::new() };
    let result = serialize_vec.end();
    assert_eq!(result, Ok(Value::Array(Vec::new())));
}

#[test]
fn test_serialize_vec_end_single_element() {
    let serialize_vec = SerializeVec { vec: vec![Value::Bool(true)] };
    let result = serialize_vec.end();
    assert_eq!(result, Ok(Value::Array(vec![Value::Bool(true)])));
}

#[test]
fn test_serialize_vec_end_multiple_elements() {
    let serialize_vec = SerializeVec { 
        vec: vec![Value::Number(Number::from(10)), Value::String("test".to_string()), Value::Null] 
    };
    let result = serialize_vec.end();
    assert_eq!(result, Ok(Value::Array(vec![
        Value::Number(Number::from(10)), 
        Value::String("test".to_string()), 
        Value::Null
    ])));
}

