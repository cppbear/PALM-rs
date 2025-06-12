// Answer 0

fn test_error_syntax() {
    use std::fmt::{self, Write};

    #[derive(Debug)]
    enum Error {
        Syntax(String),
        CompiledTooBig(usize),
        __Nonexhaustive,
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Error::Syntax(ref err) => {
                    let hr: String = std::iter::repeat('~').take(79).collect();
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

    let err_msg = "This is a syntax error"; 

    let error = Error::Syntax(err_msg.to_string());
    let mut output = String::new();
    let result = write!(output, "{}", error);
    assert!(result.is_ok());
    assert!(output.contains("Syntax("));
    assert!(output.contains(&std::iter::repeat('~').take(79).collect::<String>()));
    assert!(output.contains(err_msg));
    assert!(output.ends_with(')'));
}

fn test_error_compiled_too_big() {
    use std::fmt::{self, Write};

    #[derive(Debug)]
    enum Error {
        Syntax(String),
        CompiledTooBig(usize),
        __Nonexhaustive,
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Error::Syntax(ref err) => {
                    let hr: String = std::iter::repeat('~').take(79).collect();
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

    let error = Error::CompiledTooBig(1024);
    let mut output = String::new();
    let result = write!(output, "{}", error);
    assert!(result.is_ok());
    assert!(output.contains("CompiledTooBig"));
}

fn test_error_nonexhaustive() {
    use std::fmt::{self, Write};

    #[derive(Debug)]
    enum Error {
        Syntax(String),
        CompiledTooBig(usize),
        __Nonexhaustive,
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Error::Syntax(ref err) => {
                    let hr: String = std::iter::repeat('~').take(79).collect();
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

    let error = Error::__Nonexhaustive;
    let mut output = String::new();
    let result = write!(output, "{}", error);
    assert!(result.is_ok());
    assert!(output.contains("__Nonexhaustive"));
}

