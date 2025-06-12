// Answer 0

#[test]
fn test_eq_f32_equal() {
    use serde_json::Value;

    let value = Value::from(3.14f32); // Value::Number(n)
    let other = 3.14f32;
    assert_eq!(eq_f32(&value, other), true);
}

#[test]
fn test_eq_f32_not_equal() {
    use serde_json::Value;

    let value = Value::from(3.14f32); // Value::Number(n)
    let other = 2.71f32;
    assert_eq!(eq_f32(&value, other), false);
}

#[test]
fn test_eq_f32_not_number() {
    use serde_json::Value;

    let value = Value::String("not a number".to_string());
    let other = 3.14f32;
    assert_eq!(eq_f32(&value, other), false);
}

#[test]
fn test_eq_f32_number_with_none() {
    use serde_json::Value;

    let value = Value::from(f64::NAN); // Value::Number(n)
    let other = 3.14f32;
    assert_eq!(eq_f32(&value, other), false);
} 

#[test]
fn test_eq_f32_number_with_some() {
    use serde_json::Value;

    let value = Value::from(0.0f32); // Value::Number(n)
    let other = 0.0f32;
    assert_eq!(eq_f32(&value, other), true);
}

