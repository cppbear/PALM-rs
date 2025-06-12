// Answer 0

fn test_fmt_display() {
    struct TestStruct {
        value: String,
    }

    impl TestStruct {
        fn as_str(&self) -> &str {
            &self.value
        }
    }

    let test_cases = vec![
        TestStruct { value: String::from("Hello, World!") },
        TestStruct { value: String::from("") },
        TestStruct { value: String::from("Rust") },
        TestStruct { value: String::from("12345") },
        TestStruct { value: String::from(" ") },
        TestStruct { value: String::from("Special chars: @$%^&*()") },
    ];

    for test_case in test_cases {
        let mut output = String::new();
        let result = std::fmt::write(&mut output, format_args!("{}", test_case.as_str()));
        assert!(result.is_ok());
        assert_eq!(output, test_case.as_str());
    }
}

#[test]
fn test_fmt_display_non_panic() {
    test_fmt_display();
}

