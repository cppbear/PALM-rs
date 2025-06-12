// Answer 0

#[test]
fn test_fmt_sensitive() {
    struct TestStruct {
        is_sensitive: bool,
    }

    impl TestStruct {
        fn as_bytes(&self) -> &[u8] {
            b"TestValue"
        }
    }

    let test_instance = TestStruct { is_sensitive: true };
    let mut output = Vec::new();
    let result = std::fmt::write(&mut output, |f| test_instance.fmt(f));
    assert!(result.is_ok());
    assert_eq!(output, b"Sensitive");
}

#[test]
fn test_fmt_not_sensitive_with_non_visible_ascii() {
    struct TestStruct {
        is_sensitive: bool,
    }

    impl TestStruct {
        fn as_bytes(&self) -> &[u8] {
            b"\x00\x01\x02" // Non-visible ASCII
        }
    }

    let test_instance = TestStruct { is_sensitive: false };
    let mut output = Vec::new();
    let result = std::fmt::write(&mut output, |f| test_instance.fmt(f));
    assert!(result.is_ok());
    assert_eq!(output, b"\"\x00\x01\x02\"");
}

#[test]
#[should_panic]
fn test_fmt_error_on_write_str() {
    struct TestStruct {
        is_sensitive: bool,
    }

    impl TestStruct {
        fn as_bytes(&self) -> &[u8] {
            b"TestValue";
        }
    }

    let test_instance = TestStruct { is_sensitive: false };
    let mut output = Vec::new();
    let result = std::fmt::write(&mut output, |f| {
        // Simulating an error on write_str
        let _ = f.write_str("\"")?;
        // The following statement will panic due to simulated error condition.
        panic!();
    });

    assert!(result.is_err());
}

