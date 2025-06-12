// Answer 0

#[derive(Debug)]
enum Error {
    InvalidInput,
    InvalidWeight,
    InsufficientNonZero,
    Overflow,
}

use std::fmt;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Error::InvalidInput => "Weights sequence is empty/too long/unordered",
            Error::InvalidWeight => "A weight is negative, too large or not a valid number",
            Error::InsufficientNonZero => "Not enough weights > zero",
            Error::Overflow => "Overflow when summing weights",
        })
    }
}

#[test]
fn test_invalid_input_error() {
    let error = Error::InvalidInput;
    let result = format!("{}", error);
    assert_eq!(result, "Weights sequence is empty/too long/unordered");
}

#[test]
fn test_invalid_weight_error() {
    let error = Error::InvalidWeight;
    let result = format!("{}", error);
    assert_eq!(result, "A weight is negative, too large or not a valid number");
}

#[test]
fn test_insufficient_non_zero_error() {
    let error = Error::InsufficientNonZero;
    let result = format!("{}", error);
    assert_eq!(result, "Not enough weights > zero");
}

#[test]
fn test_overflow_error() {
    let error = Error::Overflow;
    let result = format!("{}", error);
    assert_eq!(result, "Overflow when summing weights");
}

