// Answer 0

#[test]
fn test_fmt_with_empty_bytes() {
    struct TestBytes(Vec<u8>);

    impl std::fmt::Debug for TestBytes {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "b\"")?;
            for &b in &self.0 {
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

    let test_bytes = TestBytes(vec![]);
    let result = format!("{:?}", test_bytes);
    assert_eq!(result, r#"b""""#);
}

#[test]
fn test_fmt_with_non_printable_bytes() {
    struct TestBytes(Vec<u8>);

    impl std::fmt::Debug for TestBytes {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "b\"")?;
            for &b in &self.0 {
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

    let test_bytes = TestBytes(vec![0x00, 0x01, 0x02, 0x03, 0x7F, 0x80]);
    let result = format!("{:?}", test_bytes);
    assert_eq!(result, r#"b"\0\xx01\xx02\xx03\xx7f\xx80""#);
}

#[should_panic]
#[test]
fn test_fmt_should_panic_on_write_err() {
    struct TestBytes(Vec<u8>);

    impl std::fmt::Debug for TestBytes {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            // Simulating a panic by forcing a condition that will trigger an error
            if self.0.is_empty() {
                return write!(f, "\"");  // This does not start with b" so it should fail based on constraints
            }
            Ok(())
        }
    }

    let test_bytes = TestBytes(vec![]);
    let _ = format!("{:?}", test_bytes); // This should trigger a panic
}

