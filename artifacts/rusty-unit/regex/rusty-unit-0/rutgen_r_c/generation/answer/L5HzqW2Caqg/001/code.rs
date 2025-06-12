// Answer 0

#[test]
fn test_regex_display() {
    struct TestRegex;

    impl fmt::Display for TestRegex {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Test Regex")
        }
    }

    let regex = TestRegex;

    let mut output = String::new();
    let result = write!(&mut output, "{}", regex);
    assert!(result.is_ok());
    assert_eq!(output, "Test Regex");
}

