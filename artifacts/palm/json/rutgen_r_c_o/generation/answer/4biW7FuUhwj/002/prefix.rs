// Answer 0

#[test]
fn test_is_number_with_integer_zero() {
    let v = Value::Number(Number { n: 0 });
    v.is_number();
}

#[test]
fn test_is_number_with_integer_positive() {
    let v = Value::Number(Number { n: 1 });
    v.is_number();
}

#[test]
fn test_is_number_with_integer_negative() {
    let v = Value::Number(Number { n: -1 });
    v.is_number();
}

#[test]
fn test_is_number_with_large_integer() {
    let v = Value::Number(Number { n: i64::MAX });
    v.is_number();
}

#[test]
fn test_is_number_with_small_integer() {
    let v = Value::Number(Number { n: i64::MIN });
    v.is_number();
}

#[test]
fn test_is_number_with_float_zero() {
    let v = Value::Number(Number { n: 0.0 });
    v.is_number();
}

#[test]
fn test_is_number_with_float_positive() {
    let v = Value::Number(Number { n: 1.1 });
    v.is_number();
}

#[test]
fn test_is_number_with_float_negative() {
    let v = Value::Number(Number { n: -1.1 });
    v.is_number();
}

#[test]
fn test_is_number_with_large_float() {
    let v = Value::Number(Number { n: f64::MAX });
    v.is_number();
}

#[test]
fn test_is_number_with_small_float() {
    let v = Value::Number(Number { n: f64::MIN });
    v.is_number();
}

