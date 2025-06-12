// Answer 0

#[test]
fn test_f32_range() {
    let value = fastrand::f32();
    assert!(value >= 0.0 && value < 1.0, "Value should be in the range [0.0, 1.0)");
}

#[test]
fn test_f32_multiple_calls() {
    let values: Vec<f32> = (0..1000).map(|_| fastrand::f32()).collect();
    for &value in &values {
        assert!(value >= 0.0 && value < 1.0, "Value should be in the range [0.0, 1.0)");
    }
}

