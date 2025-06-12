// Answer 0

#[test]
fn test_fmt_non_empty_str() {
    struct TestStr(&'static str);
    
    impl AsRef<str> for TestStr {
        fn as_ref(&self) -> &str {
            self.0
        }
    }
    
    let test_string = TestStr("Hello, World!");
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", test_string);
    assert!(result.is_ok());
    assert_eq!(buffer, "Hello, World!");
}

#[test]
fn test_fmt_empty_str() {
    struct TestStr(&'static str);
    
    impl AsRef<str> for TestStr {
        fn as_ref(&self) -> &str {
            self.0
        }
    }
    
    let test_string = TestStr("");
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", test_string);
    assert!(result.is_ok());
    assert_eq!(buffer, "");
}

#[test]
fn test_fmt_long_str() {
    struct TestStr(&'static str);
    
    impl AsRef<str> for TestStr {
        fn as_ref(&self) -> &str {
            self.0
        }
    }
    
    let test_string = TestStr("This is a long string to test the formatting function properly.");
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", test_string);
    assert!(result.is_ok());
    assert_eq!(buffer, "This is a long string to test the formatting function properly.");
}

#[should_panic]
fn test_fmt_non_ascii_str() {
    struct TestStr(&'static str);
    
    impl AsRef<str> for TestStr {
        fn as_ref(&self) -> &str {
            self.0
        }
    }
    
    let test_string = TestStr("🙂");
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{}", test_string);
}

