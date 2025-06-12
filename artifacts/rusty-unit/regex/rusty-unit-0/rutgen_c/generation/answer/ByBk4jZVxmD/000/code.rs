// Answer 0

#[test]
fn test_error_syntax() {
    let error = Error::Syntax("Invalid regex".to_string());
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", error);
    assert!(result.is_ok());
    assert!(output.contains("Syntax("));
    assert!(output.contains("Invalid regex"));
}

#[test]
fn test_error_compiled_too_big() {
    let error = Error::CompiledTooBig(1024);
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", error);
    assert!(result.is_ok());
    assert!(output.contains("CompiledTooBig(1024)"));
}

#[test]
fn test_error_non_exhaustive() {
    let error = Error::__Nonexhaustive;
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", error);
    assert!(result.is_ok());
    assert!(output.contains("__Nonexhaustive"));
}

