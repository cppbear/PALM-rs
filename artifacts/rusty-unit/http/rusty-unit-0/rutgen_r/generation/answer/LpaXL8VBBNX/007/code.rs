// Answer 0

#[test]
fn test_fmt_visible_ascii_no_quotes() {
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
        b.is_ascii() && !b.is_ascii_control()
    }

    let instance = TestStruct {
        is_sensitive: false,
        data: b"HelloWorld".to_vec(),
    };

    let mut output = Vec::new();
    {
        let mut formatter = std::fmt::Formatter::new();
        let result = instance.fmt(&mut formatter);
        assert!(result.is_ok());
        output.push(formatter.buffer().to_string());
    }
    assert_eq!(output[0], "\"HelloWorld\"");
}

#[test]
fn test_fmt_with_quotes() {
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
        b.is_ascii() && !b.is_ascii_control()
    }

    let instance = TestStruct {
        is_sensitive: false,
        data: b"Hello \"World\"".to_vec(),
    };

    let mut output = Vec::new();
    {
        let mut formatter = std::fmt::Formatter::new();
        let result = instance.fmt(&mut formatter);
        assert!(result.is_ok());
        output.push(formatter.buffer().to_string());
    }
    assert_eq!(output[0], "\"Hello \\\"World\\\"\"");
}

#[test]
fn test_fmt_with_non_visible_ascii() {
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
        b.is_ascii() && !b.is_ascii_control()
    }

    let instance = TestStruct {
        is_sensitive: false,
        data: b"Hello\x01World".to_vec(),
    };

    let mut output = Vec::new();
    {
        let mut formatter = std::fmt::Formatter::new();
        let result = instance.fmt(&mut formatter);
        assert!(result.is_ok());
        output.push(formatter.buffer().to_string());
    }
    assert_eq!(output[0], "\"Hello\\x1fWorld\"");
}

