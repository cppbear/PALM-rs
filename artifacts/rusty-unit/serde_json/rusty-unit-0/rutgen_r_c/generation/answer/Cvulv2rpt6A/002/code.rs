// Answer 0

#[test]
fn test_eq_f32_with_number() {
    let number_value = Value::Number(Number { n: N::Float(3.14) });
    assert!(eq_f32(&number_value, 3.14));
}

#[test]
fn test_eq_f32_with_number_closest_f32() {
    let number_value = Value::Number(Number { n: N::Float(1.0) });
    assert!(eq_f32(&number_value, 1.0));
}

#[test]
fn test_eq_f32_with_number_not_equal() {
    let number_value = Value::Number(Number { n: N::Float(2.71) });
    assert!(!eq_f32(&number_value, 3.14));
}

#[test]
fn test_eq_f32_with_non_number() {
    let bool_value = Value::Bool(true);
    assert!(!eq_f32(&bool_value, 3.14));

    let string_value = Value::String(String::from("not a number"));
    assert!(!eq_f32(&string_value, 3.14));

    let null_value = Value::Null;
    assert!(!eq_f32(&null_value, 3.14));
}

