// Answer 0

#[test]
fn test_is_nonfinite_nan() {
    let input: f64 = f64::NAN;
    let result = input.is_nonfinite();
}

#[test]
fn test_is_nonfinite_infinity() {
    let input: f64 = f64::INFINITY;
    let result = input.is_nonfinite();
}

#[test]
fn test_is_nonfinite_neg_infinity() {
    let input: f64 = f64::NEG_INFINITY;
    let result = input.is_nonfinite();
}

