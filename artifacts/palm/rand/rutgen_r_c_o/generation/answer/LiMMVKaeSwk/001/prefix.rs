// Answer 0

#[test]
fn test_next_u64() {
    let mut rng = StdRng(Rng::from_entropy());
    let result = rng.next_u64();
}

#[test]
fn test_next_u64_max() {
    let mut rng = StdRng(Rng::from_entropy());
    for _ in 0..10 {
        let result = rng.next_u64();
        assert!(result <= u64::MAX);
    }
}

#[test]
fn test_next_u64_multiple_calls() {
    let mut rng = StdRng(Rng::from_entropy());
    let result1 = rng.next_u64();
    let result2 = rng.next_u64();
    let result3 = rng.next_u64();
}

#[test]
fn test_next_u64_zero() {
    let mut rng = StdRng(Rng::from_entropy());
    let result = rng.next_u64();
    assert!(result >= 0);
}

#[should_panic]
fn test_next_u64_panic() {
    let mut rng = StdRng(Rng::from_entropy());
    rng.next_u64();
}

