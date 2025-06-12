// Answer 0

#[derive(Debug)]
struct Value {
    data: Option<i64>,
}

impl Value {
    fn new(data: Option<i64>) -> Self {
        Value { data }
    }

    fn as_i64(&self) -> Option<i64> {
        self.data
    }
}

fn eq_i64(value: &Value, other: i64) -> bool {
    value.as_i64() == Some(other)
}

#[test]
fn test_eq_i64_equal() {
    let value = Value::new(Some(42));
    assert!(eq_i64(&value, 42));
}

#[test]
fn test_eq_i64_not_equal() {
    let value = Value::new(Some(42));
    assert!(!eq_i64(&value, 41));
}

#[test]
fn test_eq_i64_none() {
    let value = Value::new(None);
    assert!(!eq_i64(&value, 42));
}

