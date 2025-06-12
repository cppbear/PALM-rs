// Answer 0

#[test]
fn test_fmt_invalid_input() {
    let error = Error::InvalidInput;
    let mut output = String::new();
    let result = error.fmt(&mut output).unwrap();
    assert_eq!(output, "Weights sequence is empty/too long/unordered");
}

#[test]
fn test_fmt_invalid_weight() {
    let error = Error::InvalidWeight;
    let mut output = String::new();
    let result = error.fmt(&mut output).unwrap();
    assert_eq!(output, "A weight is negative, too large or not a valid number");
}

#[test]
fn test_fmt_insufficient_non_zero() {
    let error = Error::InsufficientNonZero;
    let mut output = String::new();
    let result = error.fmt(&mut output).unwrap();
    assert_eq!(output, "Not enough weights > zero");
}

#[test]
fn test_fmt_overflow() {
    let error = Error::Overflow;
    let mut output = String::new();
    let result = error.fmt(&mut output).unwrap();
    assert_eq!(output, "Overflow when summing weights");
}

