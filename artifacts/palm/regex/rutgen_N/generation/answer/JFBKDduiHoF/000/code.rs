// Answer 0

#[test]
fn test_fmt() {
    struct TestRegex {
        original: String,
    }

    impl TestRegex {
        fn as_str(&self) -> &str {
            &self.original
        }
    }

    let regex = TestRegex {
        original: String::from("^[a-z]+$"),
    };

    let mut output = String::new();
    let result = regex::fmt(&regex, &mut output);

    assert!(result.is_ok());
    assert_eq!(output, "^[a-z]+$");
}

