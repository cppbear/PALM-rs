// Answer 0

#[test]
fn test_description_syntax_error() {
    let syntax_error = Error::Syntax("invalid syntax".to_string());
    let result = syntax_error.description();
    assert_eq!(result, "invalid syntax");
}

#[test]
fn test_description_compiled_too_big() {
    let compiled_error = Error::CompiledTooBig(1024);
    let result = compiled_error.description();
    assert_eq!(result, "compiled program too big");
}

#[should_panic]
fn test_description_non_exhaustive_variant() {
    let _non_exhaustive = Error::__Nonexhaustive;
    let _ = _non_exhaustive.description();
}

