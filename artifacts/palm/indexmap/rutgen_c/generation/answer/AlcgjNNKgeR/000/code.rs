// Answer 0

#[test]
fn test_hash_value_zero() {
    let hv = HashValue(0);
    assert_eq!(hv.get(), 0);
}

#[test]
fn test_hash_value_positive() {
    let hv = HashValue(42);
    assert_eq!(hv.get(), 42u64);
}

#[test]
fn test_hash_value_large() {
    let hv = HashValue(usize::MAX);
    assert_eq!(hv.get(), usize::MAX as u64);
}

#[test]
fn test_hash_value_negative() {
    let hv = HashValue(usize::MAX - 1);
    assert_eq!(hv.get(), (usize::MAX - 1) as u64);
}

