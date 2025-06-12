// Answer 0

#[test]
fn test_hash_with_lower_case() {
    use std::collections::hash_map::DefaultHasher;

    let input = b"example";
    let maybe_lower = MaybeLower { buf: input, lower: true };
    let mut hasher = DefaultHasher::new();
    maybe_lower.hash(&mut hasher);
    let result = hasher.finish();

    assert!(result > 0); // Just checking that the result is non-zero
}

#[test]
fn test_hash_with_upper_case() {
    use std::collections::hash_map::DefaultHasher;

    let input = b"EXAMPLE";
    let maybe_lower = MaybeLower { buf: input, lower: false };
    let mut hasher = DefaultHasher::new();
    maybe_lower.hash(&mut hasher);
    let result = hasher.finish();

    assert!(result > 0); // Just checking that the result is non-zero
}

#[test]
fn test_hash_with_empty_buf() {
    use std::collections::hash_map::DefaultHasher;

    let input: &[u8] = &[];
    let maybe_lower = MaybeLower { buf: input, lower: false };
    let mut hasher = DefaultHasher::new();
    maybe_lower.hash(&mut hasher);
    let result = hasher.finish();

    assert_eq!(result, 0); // An empty input should lead to a zero hash
}

