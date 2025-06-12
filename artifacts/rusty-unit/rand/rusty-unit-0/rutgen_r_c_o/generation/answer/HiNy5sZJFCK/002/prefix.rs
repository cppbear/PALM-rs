// Answer 0

#[test]
fn test_error_empty_range_low_greater_than_high() {
    let error = Error::EmptyRange;
    let mut buf = String::new();
    let mut formatter = fmt::Formatter::new(&mut buf);
    let _ = error.fmt(&mut formatter);
}

#[test]
fn test_error_empty_range_equal_low_and_high() {
    let error = Error::EmptyRange;
    let mut buf = String::new();
    let mut formatter = fmt::Formatter::new(&mut buf);
    let _ = error.fmt(&mut formatter);
}

