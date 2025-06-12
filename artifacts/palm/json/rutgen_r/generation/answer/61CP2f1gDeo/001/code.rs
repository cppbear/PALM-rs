// Answer 0

#[test]
fn test_invalid_value() {
    use serde_json::de;
    use serde_json::Error;

    struct TestExpected;

    impl de::Expected for TestExpected {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "test expected value")
        }
    }

    let unexpected = de::Unexpected::Str("unexpected value");
    let expected = TestExpected;

    let result = Error::invalid_value(unexpected, &expected);
    let expected_message = format!("invalid value: {}, expected {}", 
                                   de::JsonUnexpected(unexpected), 
                                   expected.fmt(&mut std::fmt::Formatter::new()).unwrap());

    assert_eq!(result.to_string(), expected_message);
}

