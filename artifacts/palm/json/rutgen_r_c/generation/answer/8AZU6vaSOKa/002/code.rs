// Answer 0

#[test]
fn test_is_i64_with_valid_i64() {
    let valid_i64_value = Value::Number(Number { n: N::PosInt(64) });
    assert!(valid_i64_value.is_i64());
}

#[test]
fn test_is_i64_with_negative_i64() {
    let valid_negative_i64_value = Value::Number(Number { n: N::NegInt(-128) });
    assert!(valid_negative_i64_value.is_i64());
}

#[test]
fn test_is_i64_with_large_u64() {
    let large_u64_value = Value::Number(Number { n: N::PosInt(u64::MAX) });
    assert!(!large_u64_value.is_i64());
}

#[test]
fn test_is_i64_with_float() {
    let float_value = Value::Number(Number { n: N::Float(256.0) });
    assert!(!float_value.is_i64());
}

#[test]
fn test_is_i64_with_large_i64() {
    let large_i64_value = Value::Number(Number { n: N::PosInt(i64::MAX as u64) });
    assert!(large_i64_value.is_i64());
}

#[test]
fn test_is_i64_with_overflow_i64() {
    let overflow_value = Value::Number(Number { n: N::PosInt(i64::MAX as u64 + 1) });
    assert!(!overflow_value.is_i64());
}

