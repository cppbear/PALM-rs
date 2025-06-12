// Answer 0

#[test]
fn test_unexpected_with_string_value() {
    struct Value {
        string: String,
    }

    impl Value {
        fn unexpected(&self) -> Unexpected {
            match self {
                Value { string } => Unexpected::Str(string),
            }
        }
    }

    enum Unexpected {
        Str(String),
    }

    let test_string = String::from("test string");
    let value = Value { string: test_string.clone() };

    match value.unexpected() {
        Unexpected::Str(s) => assert_eq!(s, test_string),
    }
}

#[test]
#[should_panic]
fn test_unexpected_with_empty_string_value() {
    struct Value {
        string: String,
    }

    impl Value {
        fn unexpected(&self) -> Unexpected {
            match self {
                Value { string } => Unexpected::Str(string.clone()),
            }
        }
    }

    enum Unexpected {
        Str(String),
    }

    let value = Value { string: String::new() };
    
    match value.unexpected() {
        Unexpected::Str(s) if s.is_empty() => assert!(false, "Expected non-empty string"),
        _ => assert!(true),
    }
}

