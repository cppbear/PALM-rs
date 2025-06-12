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
fn test_fmt_with_non_visible_ascii() {
    let header_value = HeaderValue {
        is_sensitive: false,
        bytes: vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, b'"', 0x0A, 0x0B, 0x0C],
    };

    let mut output = Vec::new();
    {
        let result = std::fmt::write(&mut output, |f| header_value.fmt(f));
        assert!(result.is_ok());
    }
    
    let expected_output = r#""\x0\x1\x2\x3\x4\x5\x6\x7\x8\"#;
    assert_eq!(String::from_utf8(output).unwrap(), expected_output);
}

#[test]
fn test_fmt_with_all_visible() {
    let header_value = HeaderValue {
        is_sensitive: false,
        bytes: b"VisibleText".to_vec(),
    };

    let mut output = Vec::new();
    {
        let result = std::fmt::write(&mut output, |f| header_value.fmt(f));
        assert!(result.is_ok());
    }
    
    let expected_output = r#""VisibleText""#;
    assert_eq!(String::from_utf8(output).unwrap(), expected_output);
}

#[test]
fn test_fmt_with_empty_bytes() {
    let header_value = HeaderValue {
        is_sensitive: false,
        bytes: Vec::new(),
    };

    let mut output = Vec::new();
    {
        let result = std::fmt::write(&mut output, |f| header_value.fmt(f));
        assert!(result.is_ok());
    }
    
    let expected_output = "\"\"";
    assert_eq!(String::from_utf8(output).unwrap(), expected_output);
}

