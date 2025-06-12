// Answer 0

#[test]
fn test_len() {
    let pattern: Vec<u8> = b"abc".to_vec();
    let search = BoyerMooreSearch::new(pattern);
    assert_eq!(search.len(), 3);
}

#[test]
fn test_len_empty_pattern() {
    let pattern: Vec<u8> = b"".to_vec();
    let result = std::panic::catch_unwind(|| {
        BoyerMooreSearch::new(pattern);
    });
    assert!(result.is_err());
}

