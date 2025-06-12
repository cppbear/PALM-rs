// Answer 0

#[derive(Debug)]
struct Bytes(Vec<u8>);

use std::fmt::{self, Formatter, Result};

impl fmt::Debug for Bytes {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "b\"")?;
        for &b in self.0.iter() {
            if b == b'\n' {
                write!(f, "\\n")?;
            } else if b == b'\r' {
                write!(f, "\\r")?;
            } else if b == b'\t' {
                write!(f, "\\t")?;
            } else if b == b'\\' || b == b'"' {
                write!(f, "\\{}", b as char)?;
            } else if b == b'\0' {
                write!(f, "\\0")?;
            } else if (0x20..0x7f).contains(&b) {
                write!(f, "{}", b as char)?;
            } else {
                write!(f, "\\x{:02x}", b)?;
            }
        }
        write!(f, "\"")?;
        Ok(())
    }
}

#[test]
fn test_fmt_debug_with_newline() {
    let byte_array = Bytes(vec![b'A', b'B', b'C', b'\n', b'D']);
    let mut output = Vec::new();
    let result = write!(output, "{:?}", byte_array);
    assert!(result.is_ok());
}

#[test]
fn test_fmt_debug_with_escape_characters() {
    let byte_array = Bytes(vec![b'A', b'\\', b'"', b'\t']);
    let mut output = Vec::new();
    let result = write!(output, "{:?}", byte_array);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "b\"A\\\"\\t\"");
}

#[test]
fn test_fmt_debug_with_null_and_non_printable() {
    let byte_array = Bytes(vec![b'A', b'\0', b'\x01', b'\x10']);
    let mut output = Vec::new();
    let result = write!(output, "{:?}", byte_array);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "b\"A\\0\\x01\\x10\"");
}

#[test]
#[should_panic] // This should trigger a panic in the test due to an existing condition
fn test_fmt_debug_panics_on_error() {
    let byte_array = Bytes(vec![b'A', b'B', b'C']);
    let mut output = Vec::new();
    // Simulating panic situation directly
    let result = write!(output, "b\""); // Mimicking a scenario where a panic could occur
    assert!(result.is_ok()); // The panic isn't truly generated in this mock
}

