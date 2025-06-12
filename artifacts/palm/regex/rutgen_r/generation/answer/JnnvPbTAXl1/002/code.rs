// Answer 0

#[derive(Debug)]
struct ParseError {
    message: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Parse Error: {}", self.message)
    }
}

#[derive(Debug)]
struct TranslateError {
    message: String,
}

impl std::fmt::Display for TranslateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Translate Error: {}", self.message)
    }
}

#[derive(Debug)]
enum Error {
    Parse(ParseError),
    Translate(TranslateError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::Parse(ref x) => x.fmt(f),
            Error::Translate(ref x) => x.fmt(f),
            _ => unreachable!(),
        }
    }
}

#[test]
fn test_fmt_translate_error() {
    let error_message = String::from("Failed to translate regex");
    let translate_error = TranslateError { message: error_message };
    let error = Error::Translate(translate_error);

    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", error);

    assert!(result.is_ok());
    assert_eq!(buffer, "Translate Error: Failed to translate regex");
}

#[test]
fn test_fmt_parse_error() {
    let error_message = String::from("Failed to parse regex");
    let parse_error = ParseError { message: error_message };
    let error = Error::Parse(parse_error);

    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", error);

    assert!(result.is_ok());
    assert_eq!(buffer, "Parse Error: Failed to parse regex");
}

