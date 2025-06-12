// Answer 0

fn eq_bool(value: &Value, other: bool) -> bool {
    value.as_bool() == Some(other)
}

#[test]
fn test_eq_bool_true() {
    let value_true = Value::Bool(true);
    assert_eq!(eq_bool(&value_true, true), true);
}

#[test]
fn test_eq_bool_false() {
    let value_false = Value::Bool(false);
    assert_eq!(eq_bool(&value_false, false), true);
}

#[test]
fn test_eq_bool_none() {
    let value_none = Value::Null; // Assuming `Value::Null` represents a null value
    assert_eq!(eq_bool(&value_none, true), false);
    assert_eq!(eq_bool(&value_none, false), false);
}

#[test]
fn test_eq_bool_mismatched() {
    let value_mismatched = Value::Bool(true);
    assert_eq!(eq_bool(&value_mismatched, false), false);
}

