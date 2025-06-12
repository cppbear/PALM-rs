// Answer 0

#[test]
fn test_unexpected_float_positive() {
    let number = Number { n: N::Float(1.0) };
    number.unexpected();
}

#[test]
fn test_unexpected_float_negative() {
    let number = Number { n: N::Float(-1.0) };
    number.unexpected();
}

#[test]
fn test_unexpected_float_zero() {
    let number = Number { n: N::Float(0.0) };
    number.unexpected();
}

#[test]
fn test_unexpected_float_large() {
    let number = Number { n: N::Float(f32::MAX as f64) };
    number.unexpected();
}

#[test]
fn test_unexpected_float_small() {
    let number = Number { n: N::Float(f32::MIN as f64) };
    number.unexpected();
}

#[test]
fn test_unexpected_float_subnormal() {
    let number = Number { n: N::Float(std::f64::EPSILON) };
    number.unexpected();
}

