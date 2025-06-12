// Answer 0

#[test]
fn test_is_number_with_integer_value() {
    use serde_json::Value;
    
    let v = Value::Number(Number { n: 42 });
    assert!(v.is_number());
}

#[test]
fn test_is_number_with_floating_point_value() {
    use serde_json::Value;
    
    let v = Value::Number(Number { n: 3.14 });
    assert!(v.is_number());
}

#[test]
fn test_is_number_with_negative_integer() {
    use serde_json::Value;
    
    let v = Value::Number(Number { n: -1 });
    assert!(v.is_number());
}

#[test]
fn test_is_number_with_large_integer() {
    use serde_json::Value;
    
    let v = Value::Number(Number { n: 1_000_000 });
    assert!(v.is_number());
}

#[test]
fn test_is_number_with_zero() {
    use serde_json::Value;
    
    let v = Value::Number(Number { n: 0 });
    assert!(v.is_number());
}

