// Answer 0

#[test]
fn test_error_syntax_debug() {
    use std::fmt::Formatter;
    
    struct ErrorWrapper {
        error: Error,
    }
    
    impl ErrorWrapper {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            self.error.fmt(f)
        }
    }

    let syntax_error = Error::Syntax("This is a syntax error".to_string());
    let mut output = Vec::new();
    {
        let mut formatter = Formatter::new(&mut output);
        syntax_error.fmt(&mut formatter).unwrap();
    }
    let result = String::from_utf8(output).unwrap();

    assert!(result.contains("Syntax("));
    assert!(result.contains("This is a syntax error"));
    assert!(result.contains("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"));
}

#[test]
fn test_error_compiled_too_big_debug() {
    use std::fmt::Formatter;

    let limit_error = Error::CompiledTooBig(1024);
    let mut output = Vec::new();
    {
        let mut formatter = Formatter::new(&mut output);
        limit_error.fmt(&mut formatter).unwrap();
    }
    let result = String::from_utf8(output).unwrap();

    assert!(result.contains("CompiledTooBig"));
    assert!(result.contains("1024"));
}

#[test]
fn test_error_nonexhaustive_debug() {
    use std::fmt::Formatter;

    let nonexhaustive_error = Error::__Nonexhaustive;
    let mut output = Vec::new();
    {
        let mut formatter = Formatter::new(&mut output);
        nonexhaustive_error.fmt(&mut formatter).unwrap();
    }
    let result = String::from_utf8(output).unwrap();

    assert!(result.contains("__Nonexhaustive"));
}

