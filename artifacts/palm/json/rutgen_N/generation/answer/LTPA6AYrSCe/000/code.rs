// Answer 0

#[derive(Debug)]
struct Value {
    number: Option<u64>,
}

impl Value {
    fn as_i64(&self) -> Option<i64> {
        match &self.number {
            Some(n) if *n <= i64::max_value() as u64 => Some(*n as i64),
            _ => None,
        }
    }
}

#[test]
fn test_as_i64_with_valid_integer() {
    let v = Value { number: Some(64) };
    assert_eq!(v.as_i64(), Some(64));
}

#[test]
fn test_as_i64_with_large_integer() {
    let big = i64::max_value() as u64 + 10;
    let v = Value { number: Some(big) };
    assert_eq!(v.as_i64(), None);
}

#[test]
fn test_as_i64_with_none_value() {
    let v = Value { number: None };
    assert_eq!(v.as_i64(), None);
}

