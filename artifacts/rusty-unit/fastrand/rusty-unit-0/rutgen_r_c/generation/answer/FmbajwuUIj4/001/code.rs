// Answer 0

#[test]
fn test_f64_is_within_bounds() {
    let mut rng = Rng::with_seed(0);
    let result = rng.f64();
    assert!(result >= 0.0 && result < 1.0);
}

#[test]
fn test_f64_has_correct_distribution() {
    let mut rng = Rng::with_seed(1);
    let counts = (0..10000).map(|_| rng.f64()).fold([0; 10], |mut acc, val| {
        let index = (val * 10.0) as usize;
        if index < 10 { acc[index] += 1; }
        acc
    });
    assert!(counts.iter().all(|&count| count > 0));
}

#[test]
fn test_f64_with_different_seeds() {
    let mut rng1 = Rng::with_seed(2);
    let result1 = rng1.f64();
    
    let mut rng2 = Rng::with_seed(3);
    let result2 = rng2.f64();
    
    assert!(result1 != result2);
}

#[test]
fn test_f64_panic_on_empty_state() {
    let mut rng = Rng::with_seed(4);
    // This test assumes the internal state of the RNG could lead to a certain response.
    // Since we don't have a defined empty state behavior for `self.u64(..)`, we assume rng could return
    // a scenario that mimics unexpected behavior.
    // Here we expect no panic, but we're testing edge response.
    let _result = std::panic::catch_unwind(|| {
        let _ = rng.f64();
    });
    assert!(!_.is_err());
}

