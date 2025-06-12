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
            Error::Syntax(ref err) => write!(f, "Syntax error: {}", err),
            Error::CompiledTooBig(limit) => {
                write!(f, "Compiled regex exceeds size limit of {} bytes.", limit)
            }
            Error::__Nonexhaustive => unreachable!(),
        }
    }
}

#[test]
fn test_error_compiled_too_big() {
    let limit = 1024;
    let error = Error::CompiledTooBig(limit);
    let expected_output = format!("Compiled regex exceeds size limit of {} bytes.", limit);
    let result = format!("{}", error);
    assert_eq!(result, expected_output);
}

#[test]
fn test_error_compiled_too_big_zero() {
    let limit = 0;
    let error = Error::CompiledTooBig(limit);
    let expected_output = format!("Compiled regex exceeds size limit of {} bytes.", limit);
    let result = format!("{}", error);
    assert_eq!(result, expected_output);
}

#[test]
#[should_panic]
fn test_error_non_exhaustive() {
    let error = Error::__Nonexhaustive;
    let _ = format!("{}", error); // This should panic
}

