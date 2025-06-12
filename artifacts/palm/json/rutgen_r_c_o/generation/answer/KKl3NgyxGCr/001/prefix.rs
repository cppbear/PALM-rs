// Answer 0

#[test]
fn test_is_i64_nan() {
    let number = Number { n: N::Float(f64::NAN) };
    number.is_i64();
}

#[test]
fn test_is_i64_infinity() {
    let number = Number { n: N::Float(f64::INFINITY) };
    number.is_i64();
}

#[test]
fn test_is_i64_negative_infinity() {
    let number = Number { n: N::Float(f64::NEG_INFINITY) };
    number.is_i64();
}

