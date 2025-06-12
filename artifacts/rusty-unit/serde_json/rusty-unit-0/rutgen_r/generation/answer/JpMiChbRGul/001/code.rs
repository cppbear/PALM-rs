// Answer 0

#[test]
fn test_collect_str_with_displayable_value() {
    struct Displayable {
        value: String,
    }

    impl std::fmt::Display for Displayable {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.value)
        }
    }

    let displayable = Displayable {
        value: String::from("test string"),
    };

    let result = collect_str(displayable);
    assert_eq!(result, Ok(String::from("test string")));
}

#[test]
fn test_collect_str_with_empty_string() {
    struct Displayable {
        value: String,
    }

    impl std::fmt::Display for Displayable {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.value)
        }
    }

    let displayable = Displayable {
        value: String::from(""),
    };

    let result = collect_str(displayable);
    assert_eq!(result, Ok(String::from("")));
}

#[test]
fn test_collect_str_with_special_characters() {
    struct Displayable {
        value: String,
    }

    impl std::fmt::Display for Displayable {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.value)
        }
    }

    let displayable = Displayable {
        value: String::from("!@#$%^&*()"),
    };

    let result = collect_str(displayable);
    assert_eq!(result, Ok(String::from("!@#$%^&*()")));
}

#[test]
fn test_collect_str_with_numeric_string() {
    struct Displayable {
        value: String,
    }

    impl std::fmt::Display for Displayable {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.value)
        }
    }

    let displayable = Displayable {
        value: String::from("12345"),
    };

    let result = collect_str(displayable);
    assert_eq!(result, Ok(String::from("12345")));
}

