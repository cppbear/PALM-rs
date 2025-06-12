// Answer 0

#[test]
fn test_eq_f64_with_zero() {
    let value = Value::Number(Number::from(0.0));
    let result = eq_f64(&value, 0.0);
}

#[test]
fn test_eq_f64_with_negative_one() {
    let value = Value::Number(Number::from(-1.0));
    let result = eq_f64(&value, -1.0);
}

#[test]
fn test_eq_f64_with_one() {
    let value = Value::Number(Number::from(1.0));
    let result = eq_f64(&value, 1.0);
}

#[test]
fn test_eq_f64_with_f64_max() {
    let value = Value::Number(Number::from(f64::MAX));
    let result = eq_f64(&value, f64::MAX);
}

#[test]
fn test_eq_f64_with_f64_min() {
    let value = Value::Number(Number::from(f64::MIN));
    let result = eq_f64(&value, f64::MIN);
}

#[test]
fn test_eq_f64_with_nan() {
    let value = Value::Number(Number::from(f64::NAN));
    let result = eq_f64(&value, f64::NAN);
}

#[test]
fn test_eq_f64_with_infinity() {
    let value = Value::Number(Number::from(f64::INFINITY));
    let result = eq_f64(&value, f64::INFINITY);
}

#[test]
fn test_eq_f64_with_negative_infinity() {
    let value = Value::Number(Number::from(-f64::INFINITY));
    let result = eq_f64(&value, -f64::INFINITY);
}

