// Answer 0

#[derive(Debug)]
enum Value {
    U64(u64),
    None,
}

impl Value {
    fn as_u64(&self) -> Option<u64> {
        match self {
            Value::U64(val) => Some(*val),
            Value::None => None,
        }
    }
}

#[test]
fn test_eq_u64_equal() {
    let value = Value::U64(100);
    assert!(eq_u64(&value, 100));
}

#[test]
fn test_eq_u64_not_equal() {
    let value = Value::U64(100);
    assert!(!eq_u64(&value, 99));
}

#[test]
fn test_eq_u64_none() {
    let value = Value::None;
    assert!(!eq_u64(&value, 100));
}

#[test]
fn test_eq_u64_zero() {
    let value = Value::U64(0);
    assert!(eq_u64(&value, 0));
}

