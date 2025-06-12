// Answer 0

#[test]
fn test_custom() {
    use std::fmt::Display;
    use crate::make_error;  // Assuming make_error is accessible in the current scope
    use crate::Error;       // Assuming Error is accessible in the current scope

    struct TestDisplay {
        value: String,
    }

    impl Display for TestDisplay {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.value)
        }
    }

    let input = TestDisplay { value: "Test error message".to_string() };
    let error = custom(input);
    assert_eq!(error.to_string(), "Test error message");
}

#[test]
fn test_custom_empty_message() {
    use std::fmt::Display;
    use crate::make_error;  // Assuming make_error is accessible in the current scope
    use crate::Error;       // Assuming Error is accessible in the current scope

    struct TestDisplay {
        value: String,
    }

    impl Display for TestDisplay {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.value)
        }
    }

    let input = TestDisplay { value: "".to_string() };
    let error = custom(input);
    assert_eq!(error.to_string(), "");
}

