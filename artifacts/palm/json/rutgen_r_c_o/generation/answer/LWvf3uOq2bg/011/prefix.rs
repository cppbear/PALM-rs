// Answer 0

#[test]
fn test_number_out_of_range_negative() {
    let error_code = ErrorCode::NumberOutOfRange;
    let mut formatter = fmt::Formatter::new();
    let _ = error_code.fmt(&mut formatter);
}

#[test]
fn test_number_out_of_range_positive() {
    let error_code = ErrorCode::NumberOutOfRange;
    let mut formatter = fmt::Formatter::new();
    let _ = error_code.fmt(&mut formatter);
}

