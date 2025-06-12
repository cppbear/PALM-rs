// Answer 0

#[test]
fn test_eq_str_value_matches() {
    struct Value {
        data: Option<String>,
    }

    impl Value {
        fn as_str(&self) -> Option<&str> {
            self.data.as_deref()
        }
    }

    let value = Value { data: Some("test".to_string()) };
    let other = "test";
    assert!(eq_str(&value, other));
}

#[test]
fn test_eq_str_value_does_not_match() {
    struct Value {
        data: Option<String>,
    }

    impl Value {
        fn as_str(&self) -> Option<&str> {
            self.data.as_deref()
        }
    }

    let value = Value { data: Some("test".to_string()) };
    let other = "not_test";
    assert!(!eq_str(&value, other));
}

#[test]
fn test_eq_str_value_is_none() {
    struct Value {
        data: Option<String>,
    }

    impl Value {
        fn as_str(&self) -> Option<&str> {
            self.data.as_deref()
        }
    }

    let value = Value { data: None };
    let other = "test";
    assert!(!eq_str(&value, other));
}

#[test]
fn test_eq_str_empty_value() {
    struct Value {
        data: Option<String>,
    }

    impl Value {
        fn as_str(&self) -> Option<&str> {
            self.data.as_deref()
        }
    }

    let value = Value { data: Some("".to_string()) };
    let other = "";
    assert!(eq_str(&value, other));
}

#[test]
fn test_eq_str_empty_value_non_empty_other() {
    struct Value {
        data: Option<String>,
    }

    impl Value {
        fn as_str(&self) -> Option<&str> {
            self.data.as_deref()
        }
    }

    let value = Value { data: Some("".to_string()) };
    let other = "not_empty";
    assert!(!eq_str(&value, other));
}

