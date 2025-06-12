// Answer 0

#[test]
fn test_eq_str_match() {
    struct Value {
        val: Option<String>,
    }

    impl Value {
        fn as_str(&self) -> Option<&str> {
            self.val.as_deref()
        }
    }

    let value = Value { val: Some("test".to_string()) };
    let result = eq_str(&value, "test");
    assert!(result);
}

#[test]
fn test_eq_str_no_match() {
    struct Value {
        val: Option<String>,
    }

    impl Value {
        fn as_str(&self) -> Option<&str> {
            self.val.as_deref()
        }
    }

    let value = Value { val: Some("test".to_string()) };
    let result = eq_str(&value, "not_test");
    assert!(!result);
}

#[test]
fn test_eq_str_none_value() {
    struct Value {
        val: Option<String>,
    }

    impl Value {
        fn as_str(&self) -> Option<&str> {
            self.val.as_deref()
        }
    }

    let value = Value { val: None };
    let result = eq_str(&value, "test");
    assert!(!result);
}

#[test]
fn test_eq_str_empty_string() {
    struct Value {
        val: Option<String>,
    }

    impl Value {
        fn as_str(&self) -> Option<&str> {
            self.val.as_deref()
        }
    }

    let value = Value { val: Some("".to_string()) };
    let result = eq_str(&value, "");
    assert!(result);
}

#[test]
fn test_eq_str_empty_none() {
    struct Value {
        val: Option<String>,
    }

    impl Value {
        fn as_str(&self) -> Option<&str> {
            self.val.as_deref()
        }
    }

    let value = Value { val: None };
    let result = eq_str(&value, "");
    assert!(!result);
}

