// Answer 0

#[derive(Debug)]
enum Error {
    Parse(ParseError),
    Translate(TranslateError),
}

#[derive(Debug)]
struct ParseError;

impl ParseError {
    fn description(&self) -> &str {
        "Parse error occurred"
    }
}

#[derive(Debug)]
struct TranslateError;

impl TranslateError {
    fn description(&self) -> &str {
        "Translate error occurred"
    }
}

impl Error {
    fn description(&self) -> &str {
        match *self {
            Error::Parse(ref x) => x.description(),
            Error::Translate(ref x) => x.description(),
            _ => unreachable!(),
        }
    }
}

#[test]
fn test_parse_error_description() {
    let error = Error::Parse(ParseError);
    assert_eq!(error.description(), "Parse error occurred");
}

#[test]
fn test_translate_error_description() {
    let error = Error::Translate(TranslateError);
    assert_eq!(error.description(), "Translate error occurred");
}

