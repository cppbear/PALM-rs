// Answer 0

#[test]
fn test_error_display_invalid_input() {
    let error = Error::InvalidInput;
    let _ = format!("{}", error);
}

#[test]
fn test_error_display_invalid_weight() {
    let error = Error::InvalidWeight;
    let _ = format!("{}", error);
}

#[test]
fn test_error_display_insufficient_non_zero() {
    let error = Error::InsufficientNonZero;
    let _ = format!("{}", error);
}

#[test]
fn test_error_display_overflow_i8() {
    let error = Error::Overflow;
    let _ = format!("{}", error);
}

#[test]
fn test_error_display_overflow_i16() {
    let error = Error::Overflow;
    let _ = format!("{}", error);
}

#[test]
fn test_error_display_overflow_i32() {
    let error = Error::Overflow;
    let _ = format!("{}", error);
}

#[test]
fn test_error_display_overflow_i64() {
    let error = Error::Overflow;
    let _ = format!("{}", error);
}

#[test]
fn test_error_display_overflow_i128() {
    let error = Error::Overflow;
    let _ = format!("{}", error);
}

#[test]
fn test_error_display_overflow_isize() {
    let error = Error::Overflow;
    let _ = format!("{}", error);
}

#[test]
fn test_error_display_overflow_f32() {
    let error = Error::Overflow;
    let _ = format!("{}", error);
}

#[test]
fn test_error_display_overflow_f64() {
    let error = Error::Overflow;
    let _ = format!("{}", error);
}

