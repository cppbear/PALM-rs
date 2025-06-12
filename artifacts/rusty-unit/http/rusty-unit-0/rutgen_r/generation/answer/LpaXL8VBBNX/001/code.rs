// Answer 0

#[test]
fn test_fmt_sensitive() {
    struct HeaderValue {
        is_sensitive: bool,
        data: Vec<u8>,
    }

    impl HeaderValue {
        fn new(is_sensitive: bool, data: Vec<u8>) -> Self {
            HeaderValue { is_sensitive, data }
        }

        fn as_bytes(&self) -> &[u8] {
            &self.data
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

    fn is_visible_ascii(b: u8) -> bool {
        b.is_ascii() && (b >= 32 && b <= 126)
    }

    let sensitive_value = HeaderValue::new(true, b"Sensitive Information".to_vec());
    assert_eq!(format!("{}", sensitive_value), "Sensitive");
}

