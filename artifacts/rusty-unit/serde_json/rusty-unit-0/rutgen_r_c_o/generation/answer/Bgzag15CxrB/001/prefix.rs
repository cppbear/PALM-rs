// Answer 0

#[test]
fn test_is_f64_with_positive_float() {
    let number = Number { n: N::Float(1.5) };
    number.is_f64();
}

#[test]
fn test_is_f64_with_negative_float() {
    let number = Number { n: N::Float(-2.34) };
    number.is_f64();
}

#[test]
fn test_is_f64_with_zero_float() {
    let number = Number { n: N::Float(0.0) };
    number.is_f64();
}

#[test]
fn test_is_f64_with_large_float() {
    let number = Number { n: N::Float(1.7976931348623157e+308) };
    number.is_f64();
}

#[test]
fn test_is_f64_with_large_negative_float() {
    let number = Number { n: N::Float(-1.7976931348623157e+308) };
    number.is_f64();
}

#[test]
fn test_is_f64_with_infinity() {
    let number = Number { n: N::Float(std::f64::INFINITY) };
    number.is_f64();
}

#[test]
fn test_is_f64_with_negative_infinity() {
    let number = Number { n: N::Float(std::f64::NEG_INFINITY) };
    number.is_f64();
}

#[test]
fn test_is_f64_with_nan() {
    let number = Number { n: N::Float(std::f64::NAN) };
    number.is_f64();
}

