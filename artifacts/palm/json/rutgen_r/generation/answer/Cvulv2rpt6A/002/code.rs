// Answer 0

#[derive(Debug)]
enum Value {
    Number(Number),
}

#[derive(Debug)]
struct Number {
    value: f32,
}

impl Number {
    fn as_f32(&self) -> Option<f32> {
        Some(self.value)
    }
}

fn eq_f32(value: &Value, other: f32) -> bool {
    match value {
        Value::Number(n) => n.as_f32() == Some(other),
        _ => false,
    }
}

#[test]
fn test_eq_f32_equal() {
    let number = Number { value: 3.14 };
    let value = Value::Number(number);
    assert_eq!(eq_f32(&value, 3.14), true);
}

#[test]
fn test_eq_f32_not_equal() {
    let number = Number { value: 2.71 };
    let value = Value::Number(number);
    assert_eq!(eq_f32(&value, 3.14), false);
}

#[test]
fn test_eq_f32_not_a_number() {
    let value = Value::Number(Number { value: 0.0 });
    assert_eq!(eq_f32(&value, 1.0), false);
}

#[test]
fn test_eq_f32_not_number_variant() {
    let other_value = Value::Number(Number { value: 9.99 });
    assert_eq!(eq_f32(&other_value, 3.14), false);
}

