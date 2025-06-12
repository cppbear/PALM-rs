// Answer 0

#[test]
fn test_fmt_syntax_error() {
    let error = Error::Syntax("Invalid syntax".to_string());
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "Invalid syntax");
}

#[test]
fn test_fmt_compiled_too_big_error() {
    let error = Error::CompiledTooBig(1024);
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "Compiled regex exceeds size limit of 1024 bytes.");
}

#[test]
#[should_panic]
fn test_fmt_non_exhaustive_error() {
    let error = Error::__Nonexhaustive;
    let _ = write!(&mut String::new(), "{}", error);
}

