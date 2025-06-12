// Answer 0

#[test]
fn test_cause_returns_none() {
    use crate::Error;

    let syntax_error = Error::Syntax(String::from("test syntax"));
    let compiled_too_big_error = Error::CompiledTooBig(1024);
    let non_exhaustive_error = Error::__Nonexhaustive;

    assert_eq!(syntax_error.cause(), None);
    assert_eq!(compiled_too_big_error.cause(), None);
    assert_eq!(non_exhaustive_error.cause(), None);
}

