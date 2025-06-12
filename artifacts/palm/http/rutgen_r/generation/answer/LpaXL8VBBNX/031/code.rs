// Answer 0

#[test]
fn test_fmt_with_non_sensitive_data() {
    struct TestStruct {
        is_sensitive: bool,
        data: Vec<u8>,
    }

    impl TestStruct {
        fn as_bytes(&self) -> &[u8] {
            &self.data
        }
    }

    fn is_visible_ascii(b: u8) -> bool {
        (b >= 32 && b <= 126) // Printable ASCII range
    }

    let instance = TestStruct {
        is_sensitive: false,
        data: b"Hello, world!".to_vec(),
    };

    let mut output = Vec::new();
    let result = std::fmt::Write::write_str(&mut output, "\"").is_ok();
    
    assert!(result);

    let fmt_result = {
        let mut formatter = std::fmt::Formatter::new();
        instance.fmt(&mut formatter)
    };
    
    assert!(fmt_result.is_ok());
}

#[test]
#[should_panic]
fn test_fmt_with_sensitive_data() {
    struct TestStruct {
        is_sensitive: bool,
        data: Vec<u8>,
    }

    impl TestStruct {
        fn as_bytes(&self) -> &[u8] {
            &self.data
        }
    }

    let instance = TestStruct {
        is_sensitive: true,
        data: b"Sensitive content".to_vec(),
    };

    let mut output = Vec::new();
    let result = std::fmt::Write::write_str(&mut output, "\"").is_ok();
    
    assert!(result);

    let fmt_result = {
        let mut formatter = std::fmt::Formatter::new();
        instance.fmt(&mut formatter)
    };

    assert!(fmt_result.is_ok());
}

#[test]
fn test_fmt_with_control_characters() {
    struct TestStruct {
        is_sensitive: bool,
        data: Vec<u8>,
    }

    impl TestStruct {
        fn as_bytes(&self) -> &[u8] {
            &self.data
        }
    }

    let instance = TestStruct {
        is_sensitive: false,
        data: b"Hello\x00World\x07!".to_vec(),
    };

    let mut output = Vec::new();
    let result = std::fmt::Write::write_str(&mut output, "\"").is_ok();
    
    assert!(result);

    let fmt_result = {
        let mut formatter = std::fmt::Formatter::new();
        instance.fmt(&mut formatter)
    };

    assert!(fmt_result.is_ok());
}

