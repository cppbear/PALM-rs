// Answer 0

#[derive(Debug)]
enum Error {
    Parse(ParseError),
    Translate(TranslateError),
    Other,
}

#[derive(Debug)]
struct ParseError {
    message: String,
}

impl ParseError {
    fn description(&self) -> &str {
        &self.message
    }
}

#[derive(Debug)]
struct TranslateError {
    message: String,
}

impl TranslateError {
    fn description(&self) -> &str {
        &self.message
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
fn test_description_with_parse_error() {
    let parse_error = ParseError {
        message: String::from("This is a parse error."),
    };
    let error = Error::Parse(parse_error);
    assert_eq!(error.description(), "This is a parse error.");
}

#[test]
fn test_description_with_translate_error() {
    let translate_error = TranslateError {
        message: String::from("This is a translate error."),
    };
    let error = Error::Translate(translate_error);
    assert_eq!(error.description(), "This is a translate error.");
}

#[should_panic]
#[test]
fn test_description_with_other_error() {
    let error = Error::Other;
    error.description(); // This should panic as per the unreachable!() case.
}

