// Answer 0

#[test]
fn test_next_u64_initial_state() {
    let mut rng = Mcg128Xsl64 { state: 1 };
    let result = rng.next_u64();
    assert_eq!(result, 0x6Debc7038fddd807);
}

#[test]
fn test_next_u64_before_wrapping() {
    let mut rng = Mcg128Xsl64 { state: 10 };
    let result = rng.next_u64();
    assert_eq!(result, 0x1e42428a057d5d2f);
}

#[test]
fn test_next_u64_after_multiple_calls() {
    let mut rng = Mcg128Xsl64 { state: 5 };
    let first_result = rng.next_u64();
    let second_result = rng.next_u64();
    assert!(first_result != second_result);
}

#[test]
fn test_next_u64_large_state() {
    let mut rng = Mcg128Xsl64 { state: u128::MAX };
    let result = rng.next_u64();
    assert_eq!(result, 0x1c9c62dfb8c67d56);
}

#[test]
fn test_next_u64_zero_state() {
    let mut rng = Mcg128Xsl64 { state: 0 };
    let result = rng.next_u64();
    assert_eq!(result, 0x0);
}

