// Answer 0

#[test]
fn test_fmt_invalid_input() {
    let error = Error::InvalidInput;
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", error);
}

#[test]
fn test_fmt_invalid_weight() {
    let error = Error::InvalidWeight;
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", error);
}

#[test]
fn test_fmt_insufficient_non_zero() {
    let error = Error::InsufficientNonZero;
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", error);
}

#[test]
fn test_fmt_overflow() {
    let error = Error::Overflow;
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", error);
}

