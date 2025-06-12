// Answer 0

#[derive(Debug)]
enum Error {
    Syntax(&'static str),
    CompiledTooBig(u32),
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
fn test_syntax_error_description() {
    let syntax_error = Error::Syntax("invalid regex");
    assert_eq!(syntax_error.description(), "invalid regex");
}

#[test]
fn test_compiled_too_big_description() {
    let compiled_too_big_error = Error::CompiledTooBig(1024);
    assert_eq!(compiled_too_big_error.description(), "compiled program too big");
}

#[should_panic]
#[test]
fn test_non_exhaustive_case() {
    let _ = Error::__Nonexhaustive.description();
}

