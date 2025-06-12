// Answer 0

#[test]
fn test_format_nonfinite_negative_infinity() {
    let value: f64 = -1.0;
    let result = value.format_nonfinite();
}

#[test]
fn test_format_nonfinite_negative_infinity_large_exponent() {
    let value: f64 = -1.0e308;
    let result = value.format_nonfinite();
}

#[test]
fn test_format_nonfinite_negative_infinity_small_exponent() {
    let value: f64 = -1.0e-308;
    let result = value.format_nonfinite();
}

