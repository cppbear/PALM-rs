// Answer 0

fn is_visible_ascii(b: u8) -> bool {
    (0x20..=0x7E).contains(&b)
}

struct HeaderValue {
    is_sensitive: bool,
    bytes: Vec<u8>,
}

impl HeaderValue {
    fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

use std::fmt;

impl fmt::Debug for HeaderValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_sensitive {
            f.write_str("Sensitive")
        } else {
            f.write_str("\"")?;
            let mut from = 0;
            let bytes = self.as_bytes();
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
}

#[test]
fn test_header_value_with_non_visible_ascii() {
    let header_value = HeaderValue {
        is_sensitive: false,
        bytes: vec![b'\\', b'\x00', b'"', b'\xFF'], // Contains non-visible ASCII and a quote
    };
    let result = format!("{:?}", header_value);
    assert_eq!(result, "\"\\x5c\\x0\\\"\\xff\"");
}

#[test]
fn test_header_value_with_only_visible_ascii() {
    let header_value = HeaderValue {
        is_sensitive: false,
        bytes: b"visible".to_vec(),
    };
    let result = format!("{:?}", header_value);
    assert_eq!(result, "\"visible\"");
}

#[test]
fn test_header_value_with_sensitive() {
    let header_value = HeaderValue {
        is_sensitive: true,
        bytes: b"some_value".to_vec(),
    };
    let result = format!("{:?}", header_value);
    assert_eq!(result, "Sensitive");
}

#[test]
#[should_panic] // This expects a panic due to unsafe access if the range is out of bounds
fn test_header_value_panics_on_empty_bytes() {
    let header_value = HeaderValue {
        is_sensitive: false,
        bytes: vec![],
    };
    let result = format!("{:?}", header_value);
    assert_eq!(result, "\"\""); // This line may panic
}

