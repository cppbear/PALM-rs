// Answer 0

#[derive(Debug)]
struct HeaderValue {
    is_sensitive: bool,
    bytes: Vec<u8>,
}

impl HeaderValue {
    fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

fn is_visible_ascii(b: u8) -> bool {
    (0x20..=0x7E).contains(&b)
}

impl std::fmt::Display for HeaderValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
fn test_fmt_sensitive() {
    let value = HeaderValue {
        is_sensitive: true,
        bytes: vec![],
    };
    let result = format!("{}", value);
    assert_eq!(result, "Sensitive");
}

#[test]
fn test_fmt_visible_ascii() {
    let value = HeaderValue {
        is_sensitive: false,
        bytes: b"Hello World!".to_vec(),
    };
    let result = format!("{}", value);
    assert_eq!(result, "\"Hello World!\"");
}

#[test]
fn test_fmt_contains_quote() {
    let value = HeaderValue {
        is_sensitive: false,
        bytes: b"Hello \"World\"".to_vec(),
    };
    let result = format!("{}", value);
    assert_eq!(result, "\"Hello \\\"World\\\"\"");
}

#[test]
fn test_fmt_contains_non_visible() {
    let value = HeaderValue {
        is_sensitive: false,
        bytes: b"Bytes\x00\x01\x02".to_vec(),
    };
    let result = format!("{}", value);
    assert_eq!(result, "\"Bytes\\x00\\x1\\x2\"");
}

