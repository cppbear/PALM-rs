// Answer 0

#[test]
fn test_invalid_value() {
    use serde_json::de;
    use serde_json::Error;
    
    struct JsonUnexpected(de::Unexpected);
    
    impl std::fmt::Display for JsonUnexpected {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }

    struct ExpectedType;

    impl de::Expected for ExpectedType {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Expected description")
        }
    }

    let unexpected_value = de::Unexpected::Str("unexpected");
    let expected_instance = ExpectedType;

    let error = Error::invalid_value(unexpected_value, &expected_instance);
    
    assert!(error.to_string().contains("invalid value: Str(\"unexpected\"), expected Expected description"));
}

