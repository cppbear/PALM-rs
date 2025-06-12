// Answer 0

#[test]
fn test_unexpected_array_with_varied_types() {
    let value = Value::Array(vec![
        Value::Bool(true),
        Value::Bool(false),
        Value::Number(Number { n: N::from(0) }),
    ]);
    let result = value.unexpected();
}

#[test]
fn test_unexpected_empty_array() {
    let value = Value::Array(vec![]);
    let result = value.unexpected();
}

#[test]
fn test_unexpected_single_element_array() {
    let value = Value::Array(vec![
        Value::Number(Number { n: N::from(5) }),
    ]);
    let result = value.unexpected();
}

#[test]
fn test_unexpected_array_with_mixed_types() {
    let value = Value::Array(vec![
        Value::Bool(true),
        Value::String(String::from("test")),
        Value::Null,
        Value::Number(Number { n: N::from(3.14) }),
    ]);
    let result = value.unexpected();
}

#[test]
fn test_unexpected_large_array() {
    let value = Value::Array((0..1000).map(|i| Value::Number(Number { n: N::from(i) })).collect());
    let result = value.unexpected();
}

