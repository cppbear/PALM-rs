// Answer 0

#[derive(Clone, Debug, PartialEq)]
struct MockParseError;

impl error::Error for MockParseError {
    fn description(&self) -> &str {
        "Mock Parse Error"
    }
}

#[derive(Clone, Debug, PartialEq)]
struct MockTranslateError;

impl error::Error for MockTranslateError {
    fn description(&self) -> &str {
        "Mock Translate Error"
    }
}

#[derive(Clone, Debug, PartialEq)]
enum MockError {
    Parse(MockParseError),
    Translate(MockTranslateError),
}

impl error::Error for MockError {
    fn description(&self) -> &str {
        match *self {
            MockError::Parse(ref x) => x.description(),
            MockError::Translate(ref x) => x.description(),
        }
    }
}

#[test]
fn test_description_parse_error() {
    let error = MockError::Parse(MockParseError);
    assert_eq!(error.description(), "Mock Parse Error");
}

#[test]
fn test_description_translate_error() {
    let error = MockError::Translate(MockTranslateError);
    assert_eq!(error.description(), "Mock Translate Error");
}

#[test]
#[should_panic]
fn test_description_unreachable() {
    let error = "string"; // deliberately causing a panic as this is not a valid error type
    let _ = error.description(); // this will panic
}

