// Answer 0

#[test]
fn test_error_display_syntax() {
    let error = Error::Syntax("Invalid regex syntax".to_string());
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "Invalid regex syntax");
}

#[test]
fn test_error_display_compiled_too_big() {
    let limit = 1000;
    let error = Error::CompiledTooBig(limit);
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "Compiled regex exceeds size limit of 1000 bytes.");
}

#[should_panic]
fn test_error_display_non_exhaustive() {
    let error = Error::__Nonexhaustive;
    let mut output = String::new();
    let _ = write!(&mut output, "{}", error); // This should panic
}

