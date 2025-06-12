// Answer 0

#[test]
fn test_invalid_type_with_unexpected_string() {
    struct TestExpected;

    impl std::fmt::Display for TestExpected {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "a string")
        }
    }

    let unexpected = de::Unexpected::Str("unexpected value");
    let expected: &dyn de::Expected = &TestExpected;

    let result = invalid_type(unexpected, expected);
    assert_eq!(result.to_string(), "invalid type: unexpected value, expected a string");
}

#[test]
fn test_invalid_type_with_unexpected_number() {
    struct TestExpected;

    impl std::fmt::Display for TestExpected {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "a number")
        }
    }

    let unexpected = de::Unexpected::Number(42.0);
    let expected: &dyn de::Expected = &TestExpected;

    let result = invalid_type(unexpected, expected);
    assert_eq!(result.to_string(), "invalid type: 42, expected a number");
}

#[test]
fn test_invalid_type_with_unexpected_bool() {
    struct TestExpected;

    impl std::fmt::Display for TestExpected {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "a boolean")
        }
    }

    let unexpected = de::Unexpected::Bool(true);
    let expected: &dyn de::Expected = &TestExpected;

    let result = invalid_type(unexpected, expected);
    assert_eq!(result.to_string(), "invalid type: true, expected a boolean");
}

#[should_panic]
fn test_invalid_type_with_empty_string() {
    struct TestExpected;

    impl std::fmt::Display for TestExpected {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "non-empty string")
        }
    }

    let unexpected = de::Unexpected::Str("");
    let expected: &dyn de::Expected = &TestExpected;

    let _ = invalid_type(unexpected, expected); // Expect this to panic
}

