// Answer 0

#[test]
fn test_captures_len_with_no_captures() {
    struct TestRegex(Regex);
    impl TestRegex {
        fn new(pattern: &str) -> Self {
            TestRegex(Regex::new(pattern).unwrap())
        }
    }

    let regex = TestRegex::new("abc");
    assert_eq!(regex.captures_len(), 0);
}

#[test]
fn test_captures_len_with_named_captures() {
    struct TestRegex(Regex);
    impl TestRegex {
        fn new(pattern: &str) -> Self {
            TestRegex(Regex::new(pattern).unwrap())
        }
    }

    let regex = TestRegex::new(r"(?P<name1>\w+)(?P<name2>\w+)");
    assert_eq!(regex.captures_len(), 2);
}

#[test]
fn test_captures_len_with_empty_pattern() {
    struct TestRegex(Regex);
    impl TestRegex {
        fn new(pattern: &str) -> Self {
            TestRegex(Regex::new(pattern).unwrap())
        }
    }

    let regex = TestRegex::new("");
    assert_eq!(regex.captures_len(), 0);
}

#[test]
#[should_panic]
fn test_captures_len_with_invalid_pattern() {
    struct TestRegex(Regex);
    impl TestRegex {
        fn new(pattern: &str) -> Self {
            TestRegex(Regex::new(pattern).unwrap())
        }
    }

    let _regex = TestRegex::new("["); // Invalid regex pattern
    // The panic would happen when trying to instantiate the TestRegex
}

