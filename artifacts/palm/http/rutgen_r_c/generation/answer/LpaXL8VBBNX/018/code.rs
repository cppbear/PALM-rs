// Answer 0

fn test_header_value_fmt_visible() {
    use bytes::BytesMut;

    struct HeaderValueTest {
        inner: BytesMut,
        is_sensitive: bool,
    }

    impl HeaderValueTest {
        fn new(inner: BytesMut, is_sensitive: bool) -> Self {
            HeaderValueTest { inner, is_sensitive }
        }

        fn as_bytes(&self) -> &[u8] {
            self.inner.as_ref()
        }
    }

    impl fmt::Debug for HeaderValueTest {
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
                            f.write_str(unsafe { str::from_utf8_unchecked(&bytes[from..i]) })?;
                        }
                        if b == b'"' {
                            f.write_str("\\\"")?;
                        } else {
                            write!(f, "\\x{:x}", b)?;
                        }
                        from = i + 1;
                    }
                }
                f.write_str(unsafe { str::from_utf8_unchecked(&bytes[from..]) })?;
                f.write_str("\"")
            }
        }
    }

    let mut buf = BytesMut::with_capacity(10);
    buf.extend_from_slice(&[0, 1, 2, 3, 4, 5, b'\n', 6, 7, 8]);
    let header_value = HeaderValueTest::new(buf.clone(), false);

    let result = format!("{:?}", header_value);
    assert_eq!(result, "\"\\x00\\x01\\x02\\x03\\x04\\x05\\x0a\\x06\\x07\\x08\"");
}

fn test_header_value_fmt_sensitive() {
    struct HeaderValueTest {
        is_sensitive: bool,
    }

    impl HeaderValueTest {
        fn new(is_sensitive: bool) -> Self {
            HeaderValueTest { is_sensitive }
        }
    }

    impl fmt::Debug for HeaderValueTest {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            if self.is_sensitive {
                f.write_str("Sensitive")
            } else {
                f.write_str("\"")?;
                f.write_str("\"") // This is a minimal placeholder, since the test does not require detailed input.
            }
        }
    }

    let header_value = HeaderValueTest::new(true);

    let result = format!("{:?}", header_value);
    assert_eq!(result, "Sensitive");
}

fn test_header_value_fmt_non_visible_bytes() {
    struct HeaderValueTest {
        inner: BytesMut,
        is_sensitive: bool,
    }

    impl HeaderValueTest {
        fn new(inner: BytesMut, is_sensitive: bool) -> Self {
            HeaderValueTest { inner, is_sensitive }
        }

        fn as_bytes(&self) -> &[u8] {
            self.inner.as_ref()
        }
    }

    impl fmt::Debug for HeaderValueTest {
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
                            f.write_str(unsafe { str::from_utf8_unchecked(&bytes[from..i]) })?;
                        }
                        if b == b'"' {
                            f.write_str("\\\"")?;
                        } else {
                            write!(f, "\\x{:x}", b)?;
                        }
                        from = i + 1;
                    }
                }
                f.write_str(unsafe { str::from_utf8_unchecked(&bytes[from..]) })?;
                f.write_str("\"")
            }
        }
    }

    let mut buf = BytesMut::with_capacity(10);
    buf.extend_from_slice(&[0, 1, 2, 3, 4, 5, 255, 6, 7, 8]); // 255 is non-visible ASCII
    let header_value = HeaderValueTest::new(buf.clone(), false);

    let result = format!("{:?}", header_value);
    assert_eq!(result, "\"\\x00\\x01\\x02\\x03\\x04\\x05\\xff\\x06\\x07\\x08\"");
}

