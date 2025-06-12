// Answer 0

#[test]
fn test_fmt_syntax_error() {
    use std::fmt;

    struct SyntaxError {
        description: String,
    }

    impl fmt::Display for SyntaxError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.description)
        }
    }

    enum Error {
        Syntax(SyntaxError),
        CompiledTooBig(usize),
        __Nonexhaustive,
    }

    let syntax_error = SyntaxError {
        description: String::from("Invalid regex"),
    };

    let error = Error::Syntax(syntax_error);
    let mut buffer = String::new();
    let result = error.fmt(&mut fmt::Formatter::new(&mut buffer));
    
    assert!(result.is_ok());
    assert_eq!(buffer, "Invalid regex");
}

#[test]
fn test_fmt_compiled_too_big_error() {
    use std::fmt;

    enum Error {
        Syntax(SyntaxError),
        CompiledTooBig(usize),
        __Nonexhaustive,
    }

    let limit = 1024;
    let error = Error::CompiledTooBig(limit);
    let mut buffer = String::new();
    let result = error.fmt(&mut fmt::Formatter::new(&mut buffer));

    assert!(result.is_ok());
    assert_eq!(buffer, "Compiled regex exceeds size limit of 1024 bytes.");
}

#[test]
#[should_panic]
fn test_fmt_non_exhaustive_error() {
    use std::fmt;

    enum Error {
        Syntax(SyntaxError),
        CompiledTooBig(usize),
        __Nonexhaustive,
    }

    let error = Error::__Nonexhaustive;

    let mut buffer = String::new();
    error.fmt(&mut fmt::Formatter::new(&mut buffer));
}

