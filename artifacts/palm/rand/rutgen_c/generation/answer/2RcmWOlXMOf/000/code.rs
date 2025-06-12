// Answer 0

#[test]
fn test_error_display_invalid_input() {
    let error = Error::InvalidInput;
    let mut output = Vec::new();
    let result = fmt(&error, &mut output);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "Weights sequence is empty/too long/unordered");
}

#[test]
fn test_error_display_invalid_weight() {
    let error = Error::InvalidWeight;
    let mut output = Vec::new();
    let result = fmt(&error, &mut output);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "A weight is negative, too large or not a valid number");
}

#[test]
fn test_error_display_insufficient_non_zero() {
    let error = Error::InsufficientNonZero;
    let mut output = Vec::new();
    let result = fmt(&error, &mut output);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "Not enough weights > zero");
}

#[test]
fn test_error_display_overflow() {
    let error = Error::Overflow;
    let mut output = Vec::new();
    let result = fmt(&error, &mut output);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "Overflow when summing weights");
}

