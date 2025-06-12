// Answer 0

#[test]
fn test_error_display_invalid_input() {
    let error = Error::InvalidInput;
    let mut buffer = String::new();
    let result = error.fmt(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer, "Weights sequence is empty/too long/unordered");
}

#[test]
fn test_error_display_invalid_weight() {
    let error = Error::InvalidWeight;
    let mut buffer = String::new();
    let result = error.fmt(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer, "A weight is negative, too large or not a valid number");
}

#[test]
fn test_error_display_insufficient_non_zero() {
    let error = Error::InsufficientNonZero;
    let mut buffer = String::new();
    let result = error.fmt(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer, "Not enough weights > zero");
}

#[test]
fn test_error_display_overflow() {
    let error = Error::Overflow;
    let mut buffer = String::new();
    let result = error.fmt(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer, "Overflow when summing weights");
}

