// Answer 0

fn test_fmt_syntax_error() {
    use std::fmt;
    
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

    let error_instance = Error::Syntax("an example syntax error".to_string());
    let mut output = Vec::new();
    let result = write!(&mut output, "{}", error_instance);
    
    assert!(result.is_ok());
    let output_str = String::from_utf8(output).unwrap();
    let expected_output = format!(
        "Syntax(\n{}\n{}\n{}\n)",
        std::iter::repeat('~').take(79).collect::<String>(),
        "an example syntax error",
        std::iter::repeat('~').take(79).collect::<String>(),
    );

    assert_eq!(output_str, expected_output);
}

