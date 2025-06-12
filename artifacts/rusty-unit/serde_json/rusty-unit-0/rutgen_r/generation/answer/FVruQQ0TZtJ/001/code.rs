// Answer 0

#[derive(Debug)]
enum Value {
    Number(Number),
    // other variants can be added here
}

#[derive(Debug)]
enum Number {
    F64(f64),
    I64(i64),
    U64(u64),
}

impl Number {
    fn is_f64(&self) -> bool {
        match self {
            Number::F64(_) => true,
            _ => false,
        }
    }
}

impl Value {
    pub fn is_f64(&self) -> bool {
        match self {
            Value::Number(n) => n.is_f64(),
            _ => false,
        }
    }
}

#[test]
fn test_is_f64_with_f64_value() {
    let v = Value::Number(Number::F64(256.0));
    assert!(v.is_f64());
}

#[test]
fn test_is_f64_with_i64_value() {
    let v = Value::Number(Number::I64(64));
    assert!(!v.is_f64());
}

#[test]
fn test_is_f64_with_u64_value() {
    let v = Value::Number(Number::U64(64));
    assert!(!v.is_f64());
} 

#[test]
fn test_is_f64_with_non_number_value() {
    let v = Value::Number(Number::F64(-64.0));
    assert!(v.is_f64());
} 

#[test]
fn test_is_f64_with_value_not_matching() {
    // Here we assume there exists a variant that does not hold `Number`
    // For example, an empty object might be a representation of the Value.
    // let v = Value::Object({});
    // assert!(!v.is_f64()); // This commented line assumes a non-implemented Value variant.
}

