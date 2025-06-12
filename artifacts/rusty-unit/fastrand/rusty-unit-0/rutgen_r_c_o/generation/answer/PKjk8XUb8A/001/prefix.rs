// Answer 0

#[test]
fn test_uppercase_selects_uppercase_characters() {
    let mut rng = Rng::with_seed(42);
    for _ in 0..100 {
        let result = rng.uppercase();
        assert!(result >= 'A' && result <= 'Z');
    }
}

#[test]
#[should_panic(expected = "empty range")]
fn test_uppercase_panic_on_empty_choice() {
    let mut rng = Rng::with_seed(1);
    let empty_choice: &[u8] = &[];
    rng.choice(empty_choice).unwrap(); // This will trigger panic
}

#[test]
fn test_uppercase_functionality() {
    let mut rng = Rng::with_seed(99);
    let results: Vec<_> = (0..50).map(|_| rng.uppercase()).collect();
    for ch in results {
        assert!(ch.is_ascii_uppercase());
    }
}

#[test]
fn test_uppercase_consistency_with_seed() {
    let mut rng1 = Rng::with_seed(123);
    let mut rng2 = Rng::with_seed(123);
    let result1 = rng1.uppercase();
    let result2 = rng2.uppercase();
    assert_eq!(result1, result2);
}

#[test]
fn test_uppercase_with_high_values() {
    let mut rng = Rng::with_seed(u64::MAX);
    for _ in 0..100 {
        let result = rng.uppercase();
        assert!(result >= 'A' && result <= 'Z');
    }
}

