// Answer 0

#[derive(Debug)]
enum Error {
    EmptyRange,
    NonFinite,
}

use std::fmt;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Error::EmptyRange => "low > high (or equal if exclusive) in uniform distribution",
            Error::NonFinite => "Non-finite range in uniform distribution",
        })
    }
}

#[test]
fn test_error_empty_range() {
    let error = Error::EmptyRange;
    let mut buffer = String::new();
    let write_result = error.fmt(&mut buffer);
    
    assert!(write_result.is_ok());
    assert_eq!(buffer, "low > high (or equal if exclusive) in uniform distribution");
}

#[test]
fn test_error_non_finite() {
    let error = Error::NonFinite;
    let mut buffer = String::new();
    let write_result = error.fmt(&mut buffer);
    
    assert!(write_result.is_ok());
    assert_eq!(buffer, "Non-finite range in uniform distribution");
}

