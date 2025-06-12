// Answer 0

#[test]
fn test_description_syntax_error() {
    let error = Error::Syntax("Invalid regex syntax".to_string());
    assert_eq!(error.description(), "Invalid regex syntax");
}

#[test]
fn test_description_compiled_too_big() {
    let error = Error::CompiledTooBig(1024);
    assert_eq!(error.description(), "compiled program too big");
}

#[should_panic]
fn test_description_non_exhaustive() {
    let error = Error::__Nonexhaustive;
    let _ = error.description();
}

