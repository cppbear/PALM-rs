// Answer 0

fn test_fmt_overflow() {
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

    #[test]
    fn test_error_fmt_overflow() {
        let error = Error::Overflow;
        let output = format!("{}", error);
        assert_eq!(output, "Overflow when summing weights");
    }
}

