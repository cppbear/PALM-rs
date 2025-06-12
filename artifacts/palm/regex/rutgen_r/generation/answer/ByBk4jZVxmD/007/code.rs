// Answer 0

fn fmt_test() {
    use std::fmt::{self, Debug, Formatter};
    use std::iter::repeat;

    #[derive(Debug)]
    enum Error {
        Syntax(String),
        CompiledTooBig(usize),
        __Nonexhaustive,
    }

    impl Error {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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

    #[test]
    fn test_syntax_error() {
        let err = Error::Syntax("unexpected token".to_string());
        let mut output = Vec::new();
        {
            let mut formatter = Formatter::for_output(&mut output);
            err.fmt(&mut formatter).unwrap();
        }
        let expected_output = format!(
            "Syntax(\n{}\n{}\n{}\n{}\n)",
            repeat('~').take(79).collect::<String>(),
            "unexpected token",
            repeat('~').take(79).collect::<String>(),
        );
        assert_eq!(String::from_utf8_lossy(&output), expected_output);
    }

    #[test]
    fn test_compiled_too_big() {
        let err = Error::CompiledTooBig(100);
        let mut output = Vec::new();
        {
            let mut formatter = Formatter::for_output(&mut output);
            err.fmt(&mut formatter).unwrap();
        }
        assert_eq!(String::from_utf8_lossy(&output), "CompiledTooBig(100)");
    }

    #[test]
    fn test_non_exhaustive_error() {
        let err = Error::__Nonexhaustive;
        let mut output = Vec::new();
        {
            let mut formatter = Formatter::for_output(&mut output);
            err.fmt(&mut formatter).unwrap();
        }
        assert_eq!(String::from_utf8_lossy(&output), "__Nonexhaustive");
    }
}

