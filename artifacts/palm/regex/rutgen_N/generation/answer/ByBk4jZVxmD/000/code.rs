// Answer 0

#[test]
fn test_syntax_error_fmt() {
    use std::fmt::{self, Write};
    use std::iter::repeat;

    struct SyntaxError {
        message: String,
    }

    enum Error {
        Syntax(SyntaxError),
        CompiledTooBig(usize),
        __Nonexhaustive,
    }

    impl fmt::Display for SyntaxError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.message)
        }
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Error::Syntax(ref err) => {
                    let hr: String = repeat('~').take(79).collect();
                    writeln!(f, "Syntax(")?;
                    writeln!(f, "{}", hr)?;
                    writeln!(f, "{}", err)?;
                    writeln!(f, "{}", hr)?;
                    write!(f, ")")?;
                    Ok(())
                }
                Error::CompiledTooBig(limit) => {
                    f.debug_tuple("CompiledTooBig")
                        .field(&limit)
                        .finish()
                }
                Error::__Nonexhaustive => {
                    f.debug_tuple("__Nonexhaustive").finish()
                }
            }
        }
    }

    let error = Error::Syntax(SyntaxError { message: "Unexpected token".to_string() });
    let mut output = String::new();
    
    error.fmt(&mut output).unwrap();
    
    let expected_output = format!(
        "Syntax(\n{}\nUnexpected token\n{}\n)",
        repeat('~').take(79).collect::<String>(),
        repeat('~').take(79).collect::<String>()
    );

    assert_eq!(output, expected_output);
}

#[test]
fn test_compiled_too_big_error_fmt() {
    use std::fmt;

    enum Error {
        Syntax(SyntaxError),
        CompiledTooBig(usize),
        __Nonexhaustive,
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Error::CompiledTooBig(limit) => {
                    f.debug_tuple("CompiledTooBig")
                        .field(&limit)
                        .finish()
                }
                _ => Ok(()),
            }
        }
    }

    let error = Error::CompiledTooBig(1024);
    let mut output = String::new();

    error.fmt(&mut output).unwrap();

    let expected_output = "CompiledTooBig(1024)".to_string();

    assert_eq!(output, expected_output);
}

#[test]
fn test_nonexhaustive_error_fmt() {
    use std::fmt;

    enum Error {
        Syntax(SyntaxError),
        CompiledTooBig(usize),
        __Nonexhaustive,
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Error::__Nonexhaustive => {
                    f.debug_tuple("__Nonexhaustive").finish()
                }
                _ => Ok(()),
            }
        }
    }

    let error = Error::__Nonexhaustive;
    let mut output = String::new();

    error.fmt(&mut output).unwrap();

    let expected_output = "__Nonexhaustive".to_string();

    assert_eq!(output, expected_output);
}

