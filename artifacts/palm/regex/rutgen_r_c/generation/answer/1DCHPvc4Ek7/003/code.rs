// Answer 0

#[test]
fn test_error_syntax_display() {
    let error_syntax = Error::Syntax(String::from("Invalid regex syntax"));
    let mut output = String::new();
    let result = write!(&mut output, "{}", error_syntax);
    assert!(result.is_ok());
    assert_eq!(output, "Invalid regex syntax");
}

#[test]
fn test_error_compiled_too_big_display() {
    let error_compiled_too_big = Error::CompiledTooBig(100);
    let mut output = String::new();
    let result = write!(&mut output, "{}", error_compiled_too_big);
    assert!(result.is_ok());
    assert_eq!(output, "Compiled regex exceeds size limit of 100 bytes.");
}

#[test]
#[should_panic]
fn test_error_nonexhaustive_display() {
    let error_nonexhaustive = Error::__Nonexhaustive;
    let mut output = String::new();
    let _ = write!(&mut output, "{}", error_nonexhaustive);
}

