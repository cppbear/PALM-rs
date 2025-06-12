// Answer 0

#[test]
fn test_fmt_parse_error() {
    use std::fmt;

    struct ParseError {
        message: String,
    }

    impl fmt::Display for ParseError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Parse Error: {}", self.message)
        }
    }

    enum Error {
        Parse(ParseError),
        Translate(String),
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Error::Parse(ref x) => x.fmt(f),
                Error::Translate(ref x) => write!(f, "Translate Error: {}", x),
                _ => unreachable!(),
            }
        }
    }

    let parse_error = ParseError {
        message: String::from("Unexpected token"),
    };
    let error = Error::Parse(parse_error);

    let mut output = String::new();
    let result = write!(&mut output, "{}", error);

    assert!(result.is_ok());
    assert_eq!(output, "Parse Error: Unexpected token");
}

#[test]
fn test_fmt_translate_error() {
    use std::fmt;

    struct ParseError {
        message: String,
    }

    impl fmt::Display for ParseError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Parse Error: {}", self.message)
        }
    }

    enum Error {
        Parse(ParseError),
        Translate(String),
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Error::Parse(ref x) => x.fmt(f),
                Error::Translate(ref x) => write!(f, "Translate Error: {}", x),
                _ => unreachable!(),
            }
        }
    }

    let translate_error = String::from("Invalid regex");
    let error = Error::Translate(translate_error);

    let mut output = String::new();
    let result = write!(&mut output, "{}", error);

    assert!(result.is_ok());
    assert_eq!(output, "Translate Error: Invalid regex");
}

