// Answer 0

#[test]
fn test_fmt_display() {
    struct TestStruct {
        value: &'static str,
    }

    impl TestStruct {
        fn as_str(&self) -> &str {
            self.value
        }
    }

    let test_instance = TestStruct { value: "Hello, world!" };
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", test_instance);
    
    assert!(result.is_ok());
    assert_eq!(buffer, "Hello, world!");
}

