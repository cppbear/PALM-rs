// Answer 0

#[test]
fn test_unexpected_with_bool_true() {
    let value = Value::Bool(true);
    let _result = value.unexpected();
}

#[test]
fn test_unexpected_with_bool_false() {
    let value = Value::Bool(false);
    let _result = value.unexpected();
}

