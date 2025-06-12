// Answer 0

#[derive(Debug)]
enum Error {
    Parse(Box<dyn std::error::Error>),
    Translate(Box<dyn std::error::Error>),
    Other,
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
#[should_panic(expected = "unreachable!()")]
fn test_error_description_unreachable() {
    let error = Error::Other;
    let _result = error.description();
}

