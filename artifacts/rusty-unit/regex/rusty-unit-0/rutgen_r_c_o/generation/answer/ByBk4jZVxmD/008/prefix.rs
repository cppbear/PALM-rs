// Answer 0

#[test]
fn test_error_syntax_valid() {
    let error = Error::Syntax(String::from("Valid Syntax Error"));
    let mut output = Vec::new();
    let result = error.fmt(&mut std::fmt::Formatter::new(&mut output));
}

#[test]
fn test_error_syntax_empty() {
    let error = Error::Syntax(String::from(""));
    let mut output = Vec::new();
    let result = error.fmt(&mut std::fmt::Formatter::new(&mut output));
}

#[test]
fn test_error_syntax_long_message() {
    let error = Error::Syntax(String::from("A very long error message that tests the formatting capabilities of the Error enum."));
    let mut output = Vec::new();
    let result = error.fmt(&mut std::fmt::Formatter::new(&mut output));
}

