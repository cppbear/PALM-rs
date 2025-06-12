// Answer 0

fn is_visible_ascii(b: u8) -> bool {
    b.is_ascii_graphic()
}

struct HeaderValue {
    is_sensitive: bool,
    value: Vec<u8>,
}

impl HeaderValue {
    fn as_bytes(&self) -> &[u8] {
        &self.value
    }
}

use std::fmt::{self, Write};

fn fmt(value: &HeaderValue, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if value.is_sensitive {
        f.write_str("Sensitive")
    } else {
        f.write_str("\"")?;
        let mut from = 0;
        let bytes = value.as_bytes();
        for (i, &b) in bytes.iter().enumerate() {
            if !is_visible_ascii(b) || b == b'"' {
                if from != i {
                    f.write_str(unsafe { std::str::from_utf8_unchecked(&bytes[from..i]) })?;
                }
                if b == b'"' {
                    f.write_str("\\\"")?;
                } else {
                    write!(f, "\\x{:x}", b)?;
                }
                from = i + 1;
            }
        }

        f.write_str(unsafe { std::str::from_utf8_unchecked(&bytes[from..]) })?;
        f.write_str("\"")
    }
}

#[test]
fn test_fmt_no_sensitive_visible_bytes() {
    let header_value = HeaderValue {
        is_sensitive: false,
        value: b"Hello\x01World".to_vec(), // Non-visible byte at 0x01
    };
    let mut output = String::new();
    let result = fmt(&header_value, &mut output);
    assert!(result.is_ok());
    assert_eq!(output, "\"Hello\\x1fWorld\"");
}

#[test]
fn test_fmt_no_sensitive_multiple_non_visible_bytes() {
    let header_value = HeaderValue {
        is_sensitive: false,
        value: b"Test\x0bString\x0cHere".to_vec(), // Non-visible bytes at 0x0b and 0x0c
    };
    let mut output = String::new();
    let result = fmt(&header_value, &mut output);
    assert!(result.is_ok());
    assert_eq!(output, "\"Test\\xbString\\xcHere\"");
}

#[test]
fn test_fmt_no_sensitive_escaped_characters() {
    let header_value = HeaderValue {
        is_sensitive: false,
        value: b"Sample \"Quote\" and \x07Bell".to_vec(), // Includes visible bytes with quotes
    };
    let mut output = String::new();
    let result = fmt(&header_value, &mut output);
    assert!(result.is_ok());
    assert_eq!(output, "\"Sample \\\"Quote\\\" and \\x7Bell\"");
}

#[test]
#[should_panic]
fn test_fmt_out_of_bounds_slicing() {
    let header_value = HeaderValue {
        is_sensitive: false,
        value: vec![], // Empty value should trigger panics during slice
    };
    let mut output = String::new();
    let result = fmt(&header_value, &mut output);
    assert!(result.is_err());
}

