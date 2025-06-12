// Answer 0

#[derive(Debug)]
struct Value {
    data: Option<String>,
}

impl Value {
    fn as_str(&self) -> Option<&str> {
        self.data.as_deref()
    }

    fn new_str(value: &str) -> Self {
        Value {
            data: Some(value.to_string()),
        }
    }

    fn new_none() -> Self {
        Value { data: None }
    }
}

#[test]
fn test_eq_str_equal() {
    let value = Value::new_str("test");
    let result = eq_str(&value, "test");
    assert!(result);
}

#[test]
fn test_eq_str_not_equal() {
    let value = Value::new_str("test");
    let result = eq_str(&value, "not_test");
    assert!(!result);
}

#[test]
fn test_eq_str_none_value() {
    let value = Value::new_none();
    let result = eq_str(&value, "test");
    assert!(!result);
}

#[test]
fn test_eq_str_none_comparison() {
    let value = Value::new_none();
    let result = eq_str(&value, "");
    assert!(!result);
}

