// Answer 0

#[test]
fn test_syntax_error() {
    let err_msg = "Unexpected token";
    let error_instance = Error::Syntax(err_msg.to_string());
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", error_instance);
    
    assert_eq!(result.is_ok(), true);
    assert!(output.contains("Syntax("));
    assert!(output.contains("Unexpected token"));
}

#[test]
fn test_syntax_error_with_long_message() {
    let err_msg = "This is a very long error message that is meant to test the output format of the Error::Syntax variant.";
    let error_instance = Error::Syntax(err_msg.to_string());
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", error_instance);
    
    assert_eq!(result.is_ok(), true);
    assert!(output.contains("Syntax("));
    assert!(output.contains(&err_msg));
}

#[test]
fn test_compiled_too_big_error() {
    let limit = 1024;
    let error_instance = Error::CompiledTooBig(limit);
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", error_instance);
    
    assert_eq!(result.is_ok(), true);
    assert!(output.contains("CompiledTooBig"));
    assert!(output.contains(&limit.to_string()));
}

#[test]
fn test_non_exhaustive_error() {
    let error_instance = Error::__Nonexhaustive;
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", error_instance);
    
    assert_eq!(result.is_ok(), true);
    assert!(output.contains("__Nonexhaustive"));
}

