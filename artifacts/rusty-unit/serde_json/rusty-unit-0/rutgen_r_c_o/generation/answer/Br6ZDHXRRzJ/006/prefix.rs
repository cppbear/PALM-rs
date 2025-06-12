// Answer 0

#[test]
fn test_unexpected_value_null() {
    let value = Value::Null;
    let _ = value.unexpected();
}

