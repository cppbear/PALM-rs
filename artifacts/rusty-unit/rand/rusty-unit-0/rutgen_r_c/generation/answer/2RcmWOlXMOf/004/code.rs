// Answer 0

#[test]
fn test_fmt_invalid_input() {
    use core::fmt::Write;

    let error_case = Error::InvalidInput;
    let mut output = String::new();
    let result = error_case.fmt(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "Weights sequence is empty/too long/unordered");
}

#[test]
fn test_fmt_invalid_weight() {
    use core::fmt::Write;

    let error_case = Error::InvalidWeight;
    let mut output = String::new();
    let result = error_case.fmt(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "A weight is negative, too large or not a valid number");
}

#[test]
fn test_fmt_insufficient_non_zero() {
    use core::fmt::Write;

    let error_case = Error::InsufficientNonZero;
    let mut output = String::new();
    let result = error_case.fmt(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "Not enough weights > zero");
}

#[test]
fn test_fmt_overflow() {
    use core::fmt::Write;

    let error_case = Error::Overflow;
    let mut output = String::new();
    let result = error_case.fmt(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "Overflow when summing weights");
}

