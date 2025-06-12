// Answer 0

#[test]
fn test_try_from_valid_string() {
    struct TestType;

    impl std::str::FromStr for TestType {
        type Err = std::convert::Infallible;

        fn from_str(_: &str) -> Result<Self, Self::Err> {
            Ok(TestType)
        }
    }

    let result: Result<TestType, _> = TestType::from_str("valid_string");
    assert!(result.is_ok());
}

#[test]
fn test_try_from_invalid_string() {
    struct TestType;

    impl std::str::FromStr for TestType {
        type Err = std::num::ParseIntError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if s.parse::<i32>().is_err() {
                Err(std::num::ParseIntError::from(std::num::IntErrorKind::Empty))
            } else {
                Ok(TestType)
            }
        }
    }

    let result: Result<TestType, _> = TestType::from_str("invalid_string");
    assert!(result.is_err());
}

