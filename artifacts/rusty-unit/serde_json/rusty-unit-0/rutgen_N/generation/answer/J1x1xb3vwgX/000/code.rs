// Answer 0

#[test]
fn test_as_str_with_string_value() {
    struct Value {
        data: String,
    }

    impl Value {
        fn as_str(&self) -> Option<&str> {
            match self {
                Value { data } if !data.is_empty() => Some(data),
                _ => None,
            }
        }
    }

    let value = Value { data: "some string".to_string() };
    assert_eq!(value.as_str(), Some("some string"));
}

#[test]
fn test_as_str_with_non_string_value() {
    struct Value {
        data: bool,
    }

    impl Value {
        fn as_str(&self) -> Option<&str> {
            match self {
                Value { data: false } => None,
                _ => None,
            }
        }
    }

    let value = Value { data: false };
    assert_eq!(value.as_str(), None);
}

#[test]
fn test_as_str_with_empty_string_value() {
    struct Value {
        data: String,
    }

    impl Value {
        fn as_str(&self) -> Option<&str> {
            match self {
                Value { data } if !data.is_empty() => Some(data),
                _ => None,
            }
        }
    }

    let value = Value { data: "".to_string() };
    assert_eq!(value.as_str(), None);
}

