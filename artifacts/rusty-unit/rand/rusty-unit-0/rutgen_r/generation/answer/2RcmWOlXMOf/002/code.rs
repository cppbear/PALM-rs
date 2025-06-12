// Answer 0

fn test_fmt_error_insufficient_non_zero() {
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

    let error = Error::InsufficientNonZero;
    let mut output = String::new();
    let result = error.fmt(&mut fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(output, "Not enough weights > zero");
}

fn test_fmt_error_overflow() {
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

    let error = Error::Overflow;
    let mut output = String::new();
    let result = error.fmt(&mut fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(output, "Overflow when summing weights");
}

fn test_fmt_error_invalid_input() {
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

    let error = Error::InvalidInput;
    let mut output = String::new();
    let result = error.fmt(&mut fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(output, "Weights sequence is empty/too long/unordered");
}

fn test_fmt_error_invalid_weight() {
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

    let error = Error::InvalidWeight;
    let mut output = String::new();
    let result = error.fmt(&mut fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(output, "A weight is negative, too large or not a valid number");
}

