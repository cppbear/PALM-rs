// Answer 0

#[test]
fn test_custom() {
    use std::fmt::{self, Display};

    struct CustomDisplay {
        value: &'static str,
    }

    impl Display for CustomDisplay {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.value)
        }
    }

    let display_value = CustomDisplay { value: "Test" };
    let result = custom(display_value);
    assert_eq!(result, fmt::Error);
}

