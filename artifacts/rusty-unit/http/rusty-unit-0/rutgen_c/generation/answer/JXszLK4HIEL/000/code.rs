// Answer 0

#[test]
fn test_id_hasher_finish() {
    let hasher = IdHasher(42);
    let result = hasher.finish();
    assert_eq!(result, 42);
}

#[test]
fn test_id_hasher_finish_zero() {
    let hasher = IdHasher(0);
    let result = hasher.finish();
    assert_eq!(result, 0);
}

#[test]
fn test_id_hasher_finish_large_value() {
    let hasher = IdHasher(u64::MAX);
    let result = hasher.finish();
    assert_eq!(result, u64::MAX);
}

