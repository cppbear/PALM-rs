// Answer 0

#[test]
fn test_format_nonfinite_infinity() {
    let value: f64 = f64::INFINITY;
    let result = value.format_nonfinite();
}

#[test]
fn test_format_nonfinite_max() {
    let value: f64 = f64::MAX;
    let result = value.format_nonfinite();
}

#[test]
fn test_format_nonfinite_min_positive() {
    let value: f64 = f64::MIN_POSITIVE;
    let result = value.format_nonfinite();
}

