// Answer 0

#[test]
fn test_error_syntax() {
    use std::fmt::Formatter;

    let error = Error::Syntax("test error".to_string());
    let mut output = Vec::new();
    {
        let mut formatter = Formatter::new(&mut output);
        let _ = error.fmt(&mut formatter);
    }
}

#[test]
fn test_error_syntax_long_message() {
    use std::fmt::Formatter;

    let error = Error::Syntax("this is a very long error message that exceeds simple expectations".to_string());
    let mut output = Vec::new();
    {
        let mut formatter = Formatter::new(&mut output);
        let _ = error.fmt(&mut formatter);
    }
}

#[test]
fn test_error_syntax_empty_message() {
    use std::fmt::Formatter;

    let error = Error::Syntax("".to_string());
    let mut output = Vec::new();
    {
        let mut formatter = Formatter::new(&mut output);
        let _ = error.fmt(&mut formatter);
    }
}

#[test]
fn test_error_syntax_whitespace_message() {
    use std::fmt::Formatter;

    let error = Error::Syntax("   ".to_string());
    let mut output = Vec::new();
    {
        let mut formatter = Formatter::new(&mut output);
        let _ = error.fmt(&mut formatter);
    }
}

