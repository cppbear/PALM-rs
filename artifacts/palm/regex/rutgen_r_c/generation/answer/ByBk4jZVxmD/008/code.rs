// Answer 0

#[test]
fn test_error_syntax_debug() {
    let error = Error::Syntax("This is a syntax error.".to_string());
    let mut buffer = Vec::new();
    {
        let formatter = &mut fmt::Formatter::new(&mut buffer);
        let result = error.fmt(formatter);
        assert_eq!(result, Ok(()));
    }
    let output = String::from_utf8(buffer).unwrap();
    assert!(output.contains("Syntax("));
    assert!(output.contains("This is a syntax error."));
}

#[test]
fn test_error_syntax_debug_long_message() {
    let error = Error::Syntax("An error message that is sufficiently long to test formatting.".to_string());
    let mut buffer = Vec::new();
    {
        let formatter = &mut fmt::Formatter::new(&mut buffer);
        let result = error.fmt(formatter);
        assert_eq!(result, Ok(()));
    }
    let output = String::from_utf8(buffer).unwrap();
    assert!(output.contains("Syntax("));
    assert!(output.contains("An error message that is sufficiently long to test formatting."));
}

#[test]
fn test_error_syntax_debug_empty_message() {
    let error = Error::Syntax("".to_string());
    let mut buffer = Vec::new();
    {
        let formatter = &mut fmt::Formatter::new(&mut buffer);
        let result = error.fmt(formatter);
        assert_eq!(result, Ok(()));
    }
    let output = String::from_utf8(buffer).unwrap();
    assert!(output.contains("Syntax("));
    assert!(output.contains(""));
}

