// Answer 0

#[test]
fn test_format_finite_nan() {
    let mut buffer = Buffer::new();
    let nan_value: f64 = std::f64::NAN;
    buffer.format_finite(nan_value);
}

#[test]
fn test_format_finite_infinity() {
    let mut buffer = Buffer::new();
    let inf_value: f64 = std::f64::INFINITY;
    buffer.format_finite(inf_value);
}

#[test]
fn test_format_finite_neg_infinity() {
    let mut buffer = Buffer::new();
    let neg_inf_value: f64 = std::f64::NEG_INFINITY;
    buffer.format_finite(neg_inf_value);
}

