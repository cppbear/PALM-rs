// Answer 0

#[test]
fn test_fmt_syntax_error() {
    use std::fmt;

    struct SyntaxError;
    
    impl fmt::Display for SyntaxError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Syntax error occurred.")
        }
    }

    enum Error {
        Syntax(SyntaxError),
        CompiledTooBig(usize),
        __Nonexhaustive,
    }

    let error = Error::Syntax(SyntaxError);
    let mut output = String::new();
    let result = {
        let fmt_output = &mut fmt::Formatter::new(&mut output);
        match error {
            Error::Syntax(ref err) => err.fmt(fmt_output),
            Error::CompiledTooBig(limit) => write!(fmt_output, "Compiled regex exceeds size limit of {} bytes.", limit),
            Error::__Nonexhaustive => unreachable!(),
        }
    };

    assert!(result.is_ok());
    assert_eq!(output, "Syntax error occurred.");
}

