// Answer 0

#[derive(Debug)]
enum Error {
    Syntax(String),
    CompiledTooBig(usize),
    __Nonexhaustive,
}

use std::fmt::{self, Write};

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Syntax(ref err) => {
                let hr: String = "~".repeat(79);
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
fn test_error_syntax() {
    let error = Error::Syntax("Invalid regex".to_string());
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert!(output.contains("Syntax("));
    assert!(output.contains("Invalid regex"));
}

#[test]
fn test_error_compiled_too_big() {
    let error = Error::CompiledTooBig(100);
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert!(output.contains("CompiledTooBig"));
    assert!(output.contains(&100.to_string()));
}

#[test]
fn test_error_nonexhaustive() {
    let error = Error::__Nonexhaustive;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert!(output.contains("__Nonexhaustive"));
}

