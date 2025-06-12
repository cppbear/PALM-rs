// Answer 0

#[derive(Debug)]
enum Error {
    Syntax(&'static str),
    CompiledTooBig(usize),
    __Nonexhaustive,
}

impl Error {
    fn description(&self) -> &str {
        match *self {
            Error::Syntax(ref err) => err,
            Error::CompiledTooBig(_) => "compiled program too big",
            Error::__Nonexhaustive => unreachable!(),
        }
    }
}

#[test]
fn test_error_description_syntax() {
    let error_case = Error::Syntax("invalid regex syntax");
    assert_eq!(error_case.description(), "invalid regex syntax");
}

#[test]
fn test_error_description_compiled_too_big() {
    let error_case = Error::CompiledTooBig(1024);
    assert_eq!(error_case.description(), "compiled program too big");
}

#[test]
#[should_panic]
fn test_error_description_non_exhaustive() {
    let error_case = Error::__Nonexhaustive;
    error_case.description(); // This should panic due to unreachable statement
}

