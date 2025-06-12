// Answer 0

#[test]
fn test_eq_f32_positive_finite() {
    let number = Number::from_f64(1.23).unwrap();
    let value = Value::Number(number);
    let other = 1.23_f32;
    eq_f32(&value, other);
}

#[test]
fn test_eq_f32_negative_finite() {
    let number = Number::from_f64(-4.56).unwrap();
    let value = Value::Number(number);
    let other = -4.56_f32;
    eq_f32(&value, other);
}

#[test]
fn test_eq_f32_zero() {
    let number = Number::from_f64(0.0).unwrap();
    let value = Value::Number(number);
    let other = 0.0_f32;
    eq_f32(&value, other);
}

#[test]
fn test_eq_f32_infinite() {
    let number = Number::from_f64(f64::INFINITY).unwrap();
    let value = Value::Number(number);
    let other = f32::INFINITY;
    eq_f32(&value, other);
}

#[test]
fn test_eq_f32_nan() {
    let number = Number::from_f64(f64::NAN).unwrap();
    let value = Value::Number(number);
    let other = f32::NAN;
    eq_f32(&value, other);
}

#[test]
fn test_eq_f32_out_of_range() {
    let number = Number::from_f64(1e300).unwrap(); 
    let value = Value::Number(number);
    let other = 1e300_f32; 
    eq_f32(&value, other);
}

#[test]
fn test_eq_f32_negative_infinite() {
    let number = Number::from_f64(f64::NEG_INFINITY).unwrap();
    let value = Value::Number(number);
    let other = f32::NEG_INFINITY;
    eq_f32(&value, other);
}

