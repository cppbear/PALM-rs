// Answer 0

fn is_visible_ascii(b: u8) -> bool {
    (b >= 32 && b <= 126) // ASCII printable range
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

impl fmt::Display for HeaderValue {
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
fn test_fmt_with_visible_ascii() {
    let header_value = HeaderValue {
        is_sensitive: false,
        bytes: b"Hello World!".to_vec(),
    };
    let mut output = String::new();
    let result = write!(&mut output, "{}", header_value);
    assert!(result.is_ok());
    assert_eq!(output, "\"Hello World!\"");
}

#[test]
fn test_fmt_with_quotes() {
    let header_value = HeaderValue {
        is_sensitive: false,
        bytes: b"Hello \"World\"!".to_vec(),
    };
    let mut output = String::new();
    let result = write!(&mut output, "{}", header_value);
    assert!(result.is_ok());
    assert_eq!(output, "\"Hello \\\"World\\\"!\"");
}

#[test]
fn test_fmt_with_non_visible_ascii() {
    let header_value = HeaderValue {
        is_sensitive: false,
        bytes: b"Hello\x00World!".to_vec(), // non-visible ASCII
    };
    let mut output = String::new();
    let result = write!(&mut output, "{}", header_value);
    assert!(result.is_ok());
    assert_eq!(output, "\"Hello\\x0World!\"");
}

#[should_panic]
fn test_fmt_panic_on_empty_bytes() {
    let header_value = HeaderValue {
        is_sensitive: false,
        bytes: Vec::new(),
    };
    let mut output = String::new();
    let _ = write!(&mut output, "{}", header_value);
}

#[test]
fn test_fmt_with_only_quotes() {
    let header_value = HeaderValue {
        is_sensitive: false,
        bytes: b"\"".to_vec(),
    };
    let mut output = String::new();
    let result = write!(&mut output, "{}", header_value);
    assert!(result.is_ok());
    assert_eq!(output, "\"\\\"\"");
}

