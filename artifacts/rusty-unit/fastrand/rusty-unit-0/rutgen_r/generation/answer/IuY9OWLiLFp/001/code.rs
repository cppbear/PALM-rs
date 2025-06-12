// Answer 0

#[test]
fn test_f64_within_range() {
    let value = fastrand::f64();
    assert!(value >= 0.0 && value < 1.0);
}

#[test]
fn test_f64_multiple_samples() {
    let samples: Vec<f64> = (0..1000).map(|_| fastrand::f64()).collect();
    for &value in &samples {
        assert!(value >= 0.0 && value < 1.0);
    }
}

#[test]
#[should_panic]
fn test_f64_underflow() {
    // This test intentionally checks for a condition that should not panics
    // In normal operation, the function should never panic, but we simulate
    // a possible panic case wrongly. Here it should not panic under normal conditions.
    // However if we were to assert the invariants of the function wrongfully, it'll panic.
    let value = -0.1;
    assert!(value < 0.0);
} 

#[test]
#[should_panic]
fn test_f64_overflow() {
    // This test is designed to panic when values go beyond the boundaries of f64.
    let value = 1.1;
    assert!(value >= 1.0);
}

