// Answer 0

#[derive(Debug)]
struct Value {
    number: Option<f64>,
}

impl Value {
    fn as_f64(&self) -> Option<f64> {
        self.number
    }

    fn new(number: Option<f64>) -> Self {
        Value { number }
    }
}

#[test]
fn test_eq_f64_equal() {
    let value = Value::new(Some(3.14));
    let result = eq_f64(&value, 3.14);
    assert!(result);
}

#[test]
fn test_eq_f64_not_equal() {
    let value = Value::new(Some(2.71));
    let result = eq_f64(&value, 3.14);
    assert!(!result);
}

#[test]
fn test_eq_f64_none() {
    let value = Value::new(None);
    let result = eq_f64(&value, 3.14);
    assert!(!result);
}

#[test]
fn test_eq_f64_negative() {
    let value = Value::new(Some(-2.0));
    let result = eq_f64(&value, -2.0);
    assert!(result);
}

#[test]
fn test_eq_f64_zero() {
    let value = Value::new(Some(0.0));
    let result = eq_f64(&value, 0.0);
    assert!(result);
}

