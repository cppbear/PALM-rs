// Answer 0

#[test]
fn test_format_finite_small_positive() {
    let mut buffer = Buffer::new();
    let value: f32 = 1.0; // finite positive value
    buffer.format(value);
}

#[test]
fn test_format_finite_small_negative() {
    let mut buffer = Buffer::new();
    let value: f32 = -1.0; // finite negative value
    buffer.format(value);
}

#[test]
fn test_format_finite_large_positive() {
    let mut buffer = Buffer::new();
    let value: f32 = 3.4028235E+38; // maximum finite positive value
    buffer.format(value);
}

#[test]
fn test_format_finite_large_negative() {
    let mut buffer = Buffer::new();
    let value: f32 = -3.4028235E+38; // maximum finite negative value
    buffer.format(value);
}

#[test]
fn test_format_finite_zero() {
    let mut buffer = Buffer::new();
    let value: f32 = 0.0; // finite zero
    buffer.format(value);
}

#[test]
fn test_format_finite_subnormal() {
    let mut buffer = Buffer::new();
    let value: f32 = 1.175494E-38; // positive subnormal floating point number
    buffer.format(value);
}

#[test]
fn test_format_finite_negative_subnormal() {
    let mut buffer = Buffer::new();
    let value: f32 = -1.175494E-38; // negative subnormal floating point number
    buffer.format(value);
}

