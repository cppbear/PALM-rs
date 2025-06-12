// Answer 0

#[derive(Debug)]
enum Error {
    InvalidInput,
    InvalidWeight,
    InsufficientNonZero,
    Overflow,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match *self {
            Error::InvalidInput => "Weights sequence is empty/too long/unordered",
            Error::InvalidWeight => "A weight is negative, too large or not a valid number",
            Error::InsufficientNonZero => "Not enough weights > zero",
            Error::Overflow => "Overflow when summing weights",
        })
    }
}

#[test]
fn test_error_invalid_input() {
    let error = Error::InvalidInput;
    let expected = "Weights sequence is empty/too long/unordered";
    assert_eq!(format!("{}", error), expected);
}

#[test]
fn test_error_invalid_weight() {
    let error = Error::InvalidWeight;
    let expected = "A weight is negative, too large or not a valid number";
    assert_eq!(format!("{}", error), expected);
}

#[test]
fn test_error_insufficient_non_zero() {
    let error = Error::InsufficientNonZero;
    let expected = "Not enough weights > zero";
    assert_eq!(format!("{}", error), expected);
}

#[test]
fn test_error_overflow() {
    let error = Error::Overflow;
    let expected = "Overflow when summing weights";
    assert_eq!(format!("{}", error), expected);
}

