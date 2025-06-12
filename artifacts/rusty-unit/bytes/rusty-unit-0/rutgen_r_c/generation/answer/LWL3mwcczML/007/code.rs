// Answer 0

#[test]
fn test_debug_fmt_with_newline() {
    struct TestBytesRef<'a> {
        data: &'a [u8],
    }
    
    impl Debug for TestBytesRef<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "b\"")?;
            for &b in self.data {
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

    let test_bytes = TestBytesRef { data: &[b'h', b'e', b'l', b'l', b'o', b'\n'] };
    let debug_result = format!("{:?}", test_bytes);
    assert_eq!(debug_result, "b\"hello\\n\"");
}

#[test]
fn test_debug_fmt_with_varied_chars() {
    struct TestBytesRef<'a> {
        data: &'a [u8],
    }

    impl Debug for TestBytesRef<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "b\"")?;
            for &b in self.data {
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

    let test_bytes = TestBytesRef { data: &[b'a', b'\n', b'\r', b'b', b'\0', b'c', b'\x01'] };
    let debug_result = format!("{:?}", test_bytes);
    assert_eq!(debug_result, "b\"a\\nb\\0c\\x01\"");
}

#[test]
#[should_panic]
fn test_debug_fmt_with_invalid_internal_condition() {
    struct TestBytesRef<'a> {
        data: &'a [u8],
    }

    impl Debug for TestBytesRef<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "b\"")?;
            for &b in self.data {
                if b == b'\n' {
                    write!(f, "\\n")?;
                } else if b == b'\r' {
                    write!(f, "\\r")?;
                    // Here we purposely have a wrong condition to trigger panic
                    panic!("This panic is intentional for testing.");
                } else {
                    write!(f, "{}", b as char)?;
                }
            }
            write!(f, "\"")?;
            Ok(())
        }
    }

    let test_bytes = TestBytesRef { data: &[b'a', b'\n', b'\r'] };
    let _ = format!("{:?}", test_bytes);
}

