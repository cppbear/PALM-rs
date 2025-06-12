// Answer 0

#[test]
fn test_fmt_with_non_sensitive_visible_bytes() {
    struct TestStruct {
        is_sensitive: bool,
        bytes: Vec<u8>,
    }
    
    impl TestStruct {
        fn as_bytes(&self) -> &[u8] {
            &self.bytes
        }
    }

    let value = TestStruct {
        is_sensitive: false,
        bytes: b"Hello, World!".to_vec(),
    };
    
    let mut output = Vec::new();
    let result = std::fmt::write(&mut output, |f| value.fmt(f));
    
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "\"Hello, World!\"");
}

#[test]
fn test_fmt_with_non_sensitive_bytes_containing_non_visible_characters() {
    struct TestStruct {
        is_sensitive: bool,
        bytes: Vec<u8>,
    }
    
    impl TestStruct {
        fn as_bytes(&self) -> &[u8] {
            &self.bytes
        }
    }

    let value = TestStruct {
        is_sensitive: false,
        bytes: b"Hello\x00World\x7F".to_vec(),
    };
    
    let mut output = Vec::new();
    let result = std::fmt::write(&mut output, |f| value.fmt(f));
    
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "\"Hello\\x0\\x7fWorld\"");
}

#[test]
fn test_fmt_with_non_sensitive_bytes_containing_quotes() {
    struct TestStruct {
        is_sensitive: bool,
        bytes: Vec<u8>,
    }
    
    impl TestStruct {
        fn as_bytes(&self) -> &[u8] {
            &self.bytes
        }
    }

    let value = TestStruct {
        is_sensitive: false,
        bytes: b"Hello \"World\"".to_vec(),
    };
    
    let mut output = Vec::new();
    let result = std::fmt::write(&mut output, |f| value.fmt(f));
    
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "\"Hello \\\"World\\\"\"");
}

