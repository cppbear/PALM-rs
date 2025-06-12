// Answer 0

#[test]
fn test_expecting() {
    struct TestStruct {
        expecting: &'static str,
    }

    impl TestStruct {
        fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            fmt.write_str(self.expecting)
        }
    }

    let test_instance = TestStruct {
        expecting: "Test message",
    };

    let mut output = String::new();
    let result = test_instance.expecting(&mut std::fmt::Formatter::new(&mut output));
    
    assert!(result.is_ok());
    assert_eq!(output, "Test message");
}

