// Answer 0

#[test]
fn test_fmt() {
    struct TestStruct {
        value: String,
    }

    impl TestStruct {
        fn as_str(&self) -> &str {
            &self.value
        }
    }

    impl std::fmt::Display for TestStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.as_str())
        }
    }

    let test_cases = vec![
        TestStruct { value: String::new() }, // Test with an empty string
        TestStruct { value: String::from("test") }, // Test with a simple string
        TestStruct { value: String::from("1234567890") }, // Test with numeric string
        TestStruct { value: String::from("Special characters !@#$%^&*()") }, // Test with special characters
    ];

    for case in test_cases {
        let result = format!("{}", case);
        assert_eq!(result, case.as_str());
    }
}

#[test]
#[should_panic]
fn test_fmt_panic() {
    struct PanicStruct;

    impl PanicStruct {
        fn as_str(&self) -> &str {
            panic!("Intentional panic for testing")
        }
    }

    impl std::fmt::Display for PanicStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.as_str())
        }
    }

    let panic_case = PanicStruct;
    let _ = format!("{}", panic_case); // This should trigger a panic
}

