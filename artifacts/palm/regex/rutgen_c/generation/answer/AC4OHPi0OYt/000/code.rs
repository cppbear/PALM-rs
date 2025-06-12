// Answer 0

#[test]
fn test_cause_returns_none() {
    let error = super::Error::Syntax(String::from("syntax error"));
    assert_eq!(error.cause(), None);
}

#[test]
fn test_cause_returns_none_for_compiled_too_big() {
    let error = super::Error::CompiledTooBig(256);
    assert_eq!(error.cause(), None);
}

#[test]
fn test_cause_returns_none_for_non_exhaustive() {
    let error = super::Error::__Nonexhaustive;
    assert_eq!(error.cause(), None);
}

