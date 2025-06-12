// Answer 0

#[test]
fn test_format_finite_small_float() {
    let mut buffer = Buffer::new();
    buffer.format_finite(0.123456);
}

#[test]
fn test_format_finite_large_float() {
    let mut buffer = Buffer::new();
    buffer.format_finite(1.2345678901234567E+308);
}

#[test]
fn test_format_finite_negative_float() {
    let mut buffer = Buffer::new();
    buffer.format_finite(-3.14159265);
}

#[test]
fn test_format_finite_zero() {
    let mut buffer = Buffer::new();
    buffer.format_finite(0.0);
}

#[test]
fn test_format_finite_max_negative_float() {
    let mut buffer = Buffer::new();
    buffer.format_finite(-1.7976931348623157E+308);
}

#[test]
fn test_format_finite_max_positive_float() {
    let mut buffer = Buffer::new();
    buffer.format_finite(1.7976931348623157E+308);
}

