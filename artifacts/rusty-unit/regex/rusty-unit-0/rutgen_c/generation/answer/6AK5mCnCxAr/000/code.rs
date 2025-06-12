// Answer 0

#[derive(Clone, Debug, Eq, PartialEq)]
struct ParseError {
    message: String,
}

impl error::Error for ParseError {
    fn description(&self) -> &str {
        &self.message
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct TranslateError {
    message: String,
}

impl error::Error for TranslateError {
    fn description(&self) -> &str {
        &self.message
    }
}

#[derive(Debug)]
enum Error {
    Parse(ParseError),
    Translate(TranslateError),
    PropertyNotFound,
    PropertyValueNotFound,
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Parse(ref x) => x.description(),
            Error::Translate(ref x) => x.description(),
            _ => unreachable!(),
        }
    }
}

#[test]
fn test_description_parse_error() {
    let parse_error = Error::Parse(ParseError { message: String::from("Parse error occurred") });
    assert_eq!(parse_error.description(), "Parse error occurred");
}

#[test]
fn test_description_translate_error() {
    let translate_error = Error::Translate(TranslateError { message: String::from("Translate error occurred") });
    assert_eq!(translate_error.description(), "Translate error occurred");
}

#[should_panic]
#[test]
fn test_description_unreachable() {
    let property_error = Error::PropertyNotFound;
    property_error.description();
}

