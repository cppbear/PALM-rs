// Answer 0

#[test]
fn test_from_str_success() {
    struct TestType(String);
    
    impl std::convert::TryFrom<&str> for TestType {
        type Error = &'static str;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            if value.is_empty() {
                Err("Empty string")
            } else {
                Ok(TestType(value.to_string()))
            }
        }
    }

    let result = TestType::try_from("valid string");
    assert!(result.is_ok());
    assert_eq!(result.unwrap().0, "valid string");
}

#[test]
fn test_from_str_empty_string() {
    struct TestType(String);
    
    impl std::convert::TryFrom<&str> for TestType {
        type Error = &'static str;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            if value.is_empty() {
                Err("Empty string")
            } else {
                Ok(TestType(value.to_string()))
            }
        }
    }

    let result = TestType::try_from("");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Empty string");
}

