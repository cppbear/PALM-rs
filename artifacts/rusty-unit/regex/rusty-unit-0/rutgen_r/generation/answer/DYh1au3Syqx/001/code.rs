// Answer 0

struct Error {
    kind: ErrorKind,
}

enum ErrorKind {
    Syntax(String),
    CompiledTooBig(usize),
    __Nonexhaustive,
}

impl Error {
    fn description(&self) -> &str {
        match *self.kind {
            ErrorKind::Syntax(ref err) => err,
            ErrorKind::CompiledTooBig(_) => "compiled program too big",
            ErrorKind::__Nonexhaustive => unreachable!(),
        }
    }
}

#[test]
#[should_panic]
fn test_description_nonexhaustive() {
    let error = Error { kind: ErrorKind::__Nonexhaustive };
    error.description();
}

#[test]
fn test_description_syntax() {
    let error = Error { kind: ErrorKind::Syntax(String::from("syntax error")) };
    assert_eq!(error.description(), "syntax error");
}

#[test]
fn test_description_compiled_too_big() {
    let error = Error { kind: ErrorKind::CompiledTooBig(1024) };
    assert_eq!(error.description(), "compiled program too big");
}

