// Answer 0

#[test]
fn test_f64() {
    let mut rng = Rng::with_seed(42);
    let result = rng.f64();
    assert!(result >= 0.0 && result < 1.0);
}

#[test]
fn test_f64_boundary_conditions() {
    let mut rng = Rng::with_seed(0);
    for _ in 0..1000 {
        let result = rng.f64();
        assert!(result >= 0.0 && result < 1.0);
    }

    let mut rng_high = Rng::with_seed(u64::MAX);
    for _ in 0..1000 {
        let result_high = rng_high.f64();
        assert!(result_high >= 0.0 && result_high < 1.0);
    }
}

