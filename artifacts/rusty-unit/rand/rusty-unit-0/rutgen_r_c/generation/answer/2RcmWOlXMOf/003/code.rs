// Answer 0

#[test]
fn test_error_invalid_input_display() {
    let error = Error::InvalidInput;
    let mut output = String::new();
    let result = error.fmt(&mut fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(output, "Weights sequence is empty/too long/unordered");
}

#[test]
fn test_error_invalid_weight_display() {
    let error = Error::InvalidWeight;
    let mut output = String::new();
    let result = error.fmt(&mut fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(output, "A weight is negative, too large or not a valid number");
}

#[test]
fn test_error_insufficient_non_zero_display() {
    let error = Error::InsufficientNonZero;
    let mut output = String::new();
    let result = error.fmt(&mut fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(output, "Not enough weights > zero");
}

#[test]
fn test_error_overflow_display() {
    let error = Error::Overflow;
    let mut output = String::new();
    let result = error.fmt(&mut fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(output, "Overflow when summing weights");
}

