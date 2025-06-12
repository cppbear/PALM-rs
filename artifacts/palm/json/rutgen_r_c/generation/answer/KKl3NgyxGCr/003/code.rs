// Answer 0

#[test]
fn test_is_i64_positive_integer_within_bounds() {
    let num = Number::from(u64::MAX); // boundary value
    assert!(!num.is_i64());
    let num = Number::from(u64::MAX - 1); // boundary case
    assert!(num.is_i64());
    let num = Number::from(0); // minimum boundary
    assert!(num.is_i64());
}

#[test]
fn test_is_i64_negative_integer() {
    let num = Number::from(-1i64);
    assert!(num.is_i64());
}

#[test]
fn test_is_i64_positive_float() {
    let num = Number::from(1.0);
    assert!(!num.is_i64());
}

#[test]
fn test_is_i64_negative_float() {
    let num = Number::from(-1.0);
    assert!(!num.is_i64());
}

#[test]
fn test_is_i64_large_positive_number() {
    let num = Number::from(u64::MAX); // exceeds i64::MAX
    assert!(!num.is_i64());
}

#[test]
fn test_is_i64_zero() {
    let num = Number::from(0u64);
    assert!(num.is_i64());
}

