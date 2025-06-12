// Answer 0

#[test]
fn test_is_u64_negative_float() {
    let number = Number { n: N::Float(-1.0) };
    number.is_u64();
}

#[test]
fn test_is_u64_zero_float() {
    let number = Number { n: N::Float(0.0) };
    number.is_u64();
}

#[test]
fn test_is_u64_positive_float() {
    let number = Number { n: N::Float(1.0) };
    number.is_u64();
}

#[test]
fn test_is_u64_large_positive_float() {
    let number = Number { n: N::Float(1.0e308) };
    number.is_u64();
}

#[test]
fn test_is_u64_large_negative_float() {
    let number = Number { n: N::Float(-1.0e308) };
    number.is_u64();
}

#[test]
fn test_is_u64_non_finite_float() {
    let number = Number { n: N::Float(f64::INFINITY) };
    number.is_u64();
}

#[test]
fn test_is_u64_non_finite_negative_float() {
    let number = Number { n: N::Float(f64::NEG_INFINITY) };
    number.is_u64();
}

#[test]
fn test_is_u64_nan_float() {
    let number = Number { n: N::Float(f64::NAN) };
    number.is_u64();
}

