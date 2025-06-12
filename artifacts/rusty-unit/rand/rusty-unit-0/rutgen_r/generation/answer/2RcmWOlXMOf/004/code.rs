// Answer 0

#[test]
fn test_error_invalid_input() {
    use std::fmt;

    #[derive(Debug)]
    enum Error {
        InvalidInput,
        InvalidWeight,
        InsufficientNonZero,
        Overflow,
    }

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

    let error = Error::InvalidInput;
    let result = format!("{}", error);
    assert_eq!(result, "Weights sequence is empty/too long/unordered");
}

#[test]
fn test_error_invalid_weight() {
    use std::fmt;

    #[derive(Debug)]
    enum Error {
        InvalidInput,
        InvalidWeight,
        InsufficientNonZero,
        Overflow,
    }

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

    let error = Error::InvalidWeight;
    let result = format!("{}", error);
    assert_eq!(result, "A weight is negative, too large or not a valid number");
}

#[test]
fn test_error_insufficient_non_zero() {
    use std::fmt;

    #[derive(Debug)]
    enum Error {
        InvalidInput,
        InvalidWeight,
        InsufficientNonZero,
        Overflow,
    }

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

    let error = Error::InsufficientNonZero;
    let result = format!("{}", error);
    assert_eq!(result, "Not enough weights > zero");
}

#[test]
fn test_error_overflow() {
    use std::fmt;

    #[derive(Debug)]
    enum Error {
        InvalidInput,
        InvalidWeight,
        InsufficientNonZero,
        Overflow,
    }

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

    let error = Error::Overflow;
    let result = format!("{}", error);
    assert_eq!(result, "Overflow when summing weights");
}

